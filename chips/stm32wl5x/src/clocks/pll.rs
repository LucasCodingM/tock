use crate::chip_specific::clock_constants;
use crate::rcc::registers::Rcc;
use crate::rcc::pll::PllSource;
use crate::clocks::hsi::{HSI_FREQUENCY_MHZ,HSE_FREQUENCY_MHZ};
use crate::rcc::system_clock::SysClockSource;

use core::cell::Cell;
use core::marker::PhantomData;
use core::ops::RangeInclusive;


/// Main PLL clock structure.
pub struct Pll<'a, PllConstants> {
    rcc: &'a Rcc,
    pll_source: PllSource,
    pllp_frequency_mhz: usize,
    pllq_frequency_mhz: usize,
    pllr_frequency_mhz: usize,

    pll48_calibrated: Cell<bool>,
    _marker: PhantomData<PllConstants>,
}

impl<'a, PllConstants: clock_constants::PllConstants> Pll<'a, PllConstants> {

     pub fn new(rcc: &'a Rcc) -> Self {
        Self {
            rcc,
            pll_source: PllSource::HSI16,
            pllp_frequency_mhz: 0,
            pllq_frequency_mhz: 0,
            pllr_frequency_mhz: 0,
            pll48_calibrated: Cell::new(false),
            _marker: PhantomData,
        }
    }

    /// Configure le PLL à partir d'une source et de cibles P/Q/R (en MHz).
    /// Ppasser `None` si tu ne veux pas activer une sortie.
    /// Retourne Err si aucune combinaison (M,N,diviseurs) ne respecte les contraintes.
    pub fn configure_from_targets_mhz(
        &mut self,
        source: PllSource,
        target_p_mhz: Option<usize>,
        target_q_mhz: Option<usize>,
        target_r_mhz: Option<usize>,
        max_ppm_error: u32, // tolérance d'erreur relative (ex: 10000 = 10 000 ppm = 1%)
    ) -> Result<(), PllError> {
        let src_mhz = self.source_freq_mhz(source)?;
        let solution = Self::solve_pll_for_targets::<PllConstants>(
            src_mhz,
            target_p_mhz,
            target_q_mhz,
            target_r_mhz,
            max_ppm_error,
        ).ok_or(PllError::NoSolution)?;

        // Stop PLL, programme la config, relance et active les sorties nécessaires
        self.rcc.disable_pll_and_wait();
        self.rcc.configure_pll(
            source,
            solution.m,
            solution.n,
            solution.p.unwrap_or(0), // si non utilisé, valeur placeholder
            solution.q.unwrap_or(0),
            solution.r.unwrap_or(0),
        );

        self.rcc.set_pll_enable(true);
        self.rcc.wait_pll_ready();

        // Active les sorties demandées
        self.rcc.enable_pll_outputs(
            solution.p.is_some(),
            solution.q.is_some(),
            solution.r.is_some(),
        );

        // Mets à jour l'état local
        self.pll_source = source;
        if let Some(_p) = solution.p {
            self.pllp_frequency_mhz = solution.actual_p_mhz.unwrap();
        }
        if let Some(_q) = solution.q {
            self.pllq_frequency_mhz = solution.actual_q_mhz.unwrap();
        }
        if let Some(_r) = solution.r {
            self.pllr_frequency_mhz = solution.actual_r_mhz.unwrap();
        }

        Ok(())
    }

    /// Raccourcis si tu veux ne cibler que R (souvent SYSCLK) et en option Q=48 MHz.
    pub fn configure_sysclk_and_optional_48mhz(
        &mut self,
        source: PllSource,
        sysclk_mhz: usize,
        need_48mhz_q: bool,
    ) -> Result<(), PllError> {
        let q = if need_48mhz_q { Some(48) } else { None };
        self.configure_from_targets_mhz(source, None, q, Some(sysclk_mhz), 10_000)
    }

    /// (Optionnel) sélectionner SYSCLK = PLLR si ce n'est pas fait côté `configure_pll`.
    pub fn switch_sysclk_to_pllr(&self) {
        self.rcc.set_sys_clock_source(SysClockSource::PLLRCLK);
    }

    // --- Helpers ---

    fn source_freq_mhz(&self, src: PllSource) -> Result<usize, PllError> {
        let mhz = match src {
            PllSource::HSI16 => HSI_FREQUENCY_MHZ,
            PllSource::HSE32 => HSE_FREQUENCY_MHZ,
            PllSource::MSI   => self.rcc.get_msi_frequency().frequency_mhz() as usize 
        };
        Ok(mhz)
    }
    

    #[inline]
    fn for_each_opt_div<F>(enabled: bool, range: RangeInclusive<u8>, mut f: F)
    where
        F: FnMut(Option<u8>),
    {
        if enabled {
            for d in range {
                f(Some(d));
            }
        } else {
            f(None);
        }
    }

    #[inline]
    fn eval_and_update<C: clock_constants::PllConstants>(
        m: u8,
        n: u16,
        vco_out: f32,
        p_div: Option<u8>,
        q_div: Option<u8>,
        r_div: Option<u8>,
        t_p_mhz: Option<usize>,
        t_q_mhz: Option<usize>,
        t_r_mhz: Option<usize>,
        max_ppm_error: u32,
        best_score: &mut u64,
        best: &mut Option<PllSolution>,
    ) {
        // Frequencies only if requested
        let (p_freq, q_freq, r_freq) = (
            p_div.map(|d| vco_out / d as f32),
            q_div.map(|d| vco_out / d as f32),
            r_div.map(|d| vco_out / d as f32),
        );

        // Limits
        if let (Some(_), Some(f)) = (t_p_mhz, p_freq) {
            if f as usize > C::PLL_P_MAX_MHZ as usize { return; }
        }
        if let (Some(_), Some(f)) = (t_q_mhz, q_freq) {
            if f as usize > C::PLL_QR_MAX_MHZ as usize { return; }
        }
        if let (Some(_), Some(f)) = (t_r_mhz, r_freq) {
            if f as usize > C::PLL_QR_MAX_MHZ as usize { return; }
        }

        // Scoring (sum of ppm on requested outputs)
        let mut score: u64 = 0;
        let mut ok = true;

        if let (Some(t), Some(f)) = (t_p_mhz, p_freq) {
            let ppm = Self::ppm_error(f, t as f32);
            if ppm > max_ppm_error { ok = false; }
            score += ppm as u64;
        }
        if let (Some(t), Some(f)) = (t_q_mhz, q_freq) {
            let ppm = Self::ppm_error(f, t as f32);
            if ppm > max_ppm_error { ok = false; }
            score += ppm as u64;
        }
        if let (Some(t), Some(f)) = (t_r_mhz, r_freq) {
            let ppm = Self::ppm_error(f, t as f32);
            if ppm > max_ppm_error { ok = false; }
            score += ppm as u64;
        }

        if !ok { return; }

        if score < *best_score {
            *best_score = score;
            *best = Some(PllSolution {
                m,
                n,
                p: p_div,
                q: q_div,
                r: r_div,
                actual_p_mhz: p_freq.map(|f| f as usize),
                actual_q_mhz: q_freq.map(|f| f as usize),
                actual_r_mhz: r_freq.map(|f| f as usize),
            });
        }
    }

    fn solve_pll_for_targets<C: clock_constants::PllConstants>(
        src_mhz: usize,
        t_p_mhz: Option<usize>,
        t_q_mhz: Option<usize>,
        t_r_mhz: Option<usize>,
        max_ppm_error: u32,
    ) -> Option<PllSolution> {
        let mut best: Option<PllSolution> = None;
        let mut best_score: u64 = u64::MAX;

        for m in C::PLL_M_RANGE {
            let vco_in = src_mhz as f32 / m as f32;
            if vco_in < C::PLL_VCO_IN_MIN_MHZ || vco_in > C::PLL_VCO_IN_MAX_MHZ {
                continue;
            }

            for n in C::PLL_N_RANGE {
                let vco_out = vco_in * n as f32;

                Self::for_each_opt_div(t_p_mhz.is_some(), C::PLL_P_ALLOWED.clone(), |p_opt| {
                    Self::for_each_opt_div(t_q_mhz.is_some(), C::PLL_Q_ALLOWED.clone(), |q_opt| {
                        Self::for_each_opt_div(t_r_mhz.is_some(), C::PLL_R_ALLOWED.clone(), |r_opt| {
                            Self::eval_and_update::<C>(
                                m,
                                n,
                                vco_out,
                                p_opt,
                                q_opt,
                                r_opt,
                                t_p_mhz,
                                t_q_mhz,
                                t_r_mhz,
                                max_ppm_error,
                                &mut best_score,
                                &mut best,
                            );
                        });
                    });
                });
            }
        }

        best
    }

    fn ppm_error(actual_mhz: f32, target_mhz: f32) -> u32 {
    let diff = (actual_mhz - target_mhz).abs();
    ((diff / target_mhz) * 1_000_000.0) as u32
}
}
// --- Types utilitaires ---

#[derive(Debug)]
pub enum PllError {
    NoSolution,
    InvalidSource,
}

#[derive(Clone, Copy, Debug)]
struct PllSolution {
    m: u8,
    n: u16,
    p: Option<u8>,
    q: Option<u8>,
    r: Option<u8>,
    actual_p_mhz: Option<usize>,
    actual_q_mhz: Option<usize>,
    actual_r_mhz: Option<usize>,
}

// --- Tests -------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::RangeInclusive;

    /// Minimal constants set for unit testing the solver.
    /// We keep ranges small so brute-force stays fast in tests.
    struct TestWl55;

    impl clock_constants::PllConstants for TestWl55 {
        // VCOin constraints for STM32WL55: 2.66..=16 MHz
        const PLL_VCO_IN_MIN_MHZ: f32 = 2.66;
        const PLL_VCO_IN_MAX_MHZ: f32 = 16.0;

        // Max output clocks: P ≤ 62 MHz, Q/R ≤ 48 MHz
        const PLL_P_MAX_MHZ: f32 = 62.0;
        const PLL_QR_MAX_MHZ: f32 = 48.0;

        // Search domains kept small for tests (but include the classic M=2,N=12 solution).
        // NOTE: Arrays here (not slices) so `for m in C::PLL_M_RANGE` iterates by value.
        const PLL_M_RANGE: core::ops::RangeInclusive<u8> = 1..=4;
        const PLL_N_RANGE: core::ops::RangeInclusive<u16> = 8..=32;

        // Allowed output divisors as RangeInclusive<u8>.
        // On WL55 they are typically {2,4,6,8}; we keep that in tests.
        const PLL_P_ALLOWED: RangeInclusive<u8> = 2..=8;
        const PLL_Q_ALLOWED: RangeInclusive<u8> = 2..=8;
        const PLL_R_ALLOWED: RangeInclusive<u8> = 2..=8;
    }

    /// Dummy Rcc to let us instantiate Pll for API-level tests without touching hardware.
    /// Only the methods used in the tested paths are provided as no-ops.
    struct DummyRcc;

    impl DummyRcc {
        fn new() -> Self { Self }
        // Methods referenced by Pll::configure_from_targets_mhz:
        fn disable_pll_and_wait(&self) {}
        #[allow(clippy::too_many_arguments)]
        fn configure_pll(
            &self,
            _src: PllSource,
            _m: u8,
            _n: u16,
            _p: u8,
            _q: u8,
            _r: u8,
        ) {}
        fn set_pll_enable(&self, _en: bool) {}
        fn wait_pll_ready(&self) {}
        fn enable_pll_outputs(&self, _p_en: bool, _q_en: bool, _r_en: bool) {}
        fn set_sys_clock_source(&self, _src: SysClockSource) {}

        // Minimal MSI helper used by source_freq_mhz when source == MSI.
        fn get_msi_frequency(&self) -> MsiLike { MsiLike::mhz(4) } // pretend 4 MHz default
    }

    /// Minimal MSI-like object with the method used by source_freq_mhz().
    struct MsiLike { mhz: u32 }
    impl MsiLike {
        fn mhz(mhz: u32) -> Self { Self { mhz } }
        fn frequency_mhz(&self) -> u32 { self.mhz }
    }

    // Rebind Rcc type expected by Pll to our dummy for tests.
    // If your real Rcc is a trait in your codebase, prefer implementing that trait.
    impl DummyRcc {
        // Give names matching what Pll expects to call (already provided above).
    }

    /// Sanity test for ppm_error helper.
    #[test]
    fn ppm_error_sanity() {
        let ppm0 = Pll::<TestWl55>::ppm_error(48.0, 48.0);
        assert_eq!(ppm0, 0);

        // ~20.8 ppm deviation
        let ppm_small = Pll::<TestWl55>::ppm_error(48.001, 48.0);
        assert!(ppm_small > 0 && ppm_small < 100); // loose bound, no float flakiness
    }

    /// Check the solver can find the classic HSI16 -> VCOin=8 MHz (M=2), N=12, R=2/Q=2 -> 48 MHz.
    #[test]
    fn find_48mhz_sysclk_and_q48_from_hsi16() {
        // Given HSI16 = 16 MHz (from your constants import)
        let src_mhz = HSI_FREQUENCY_MHZ; // typically 16

        let best = Pll::<TestWl55>::solve_pll_for_targets::<TestWl55>(
            src_mhz,
            None,          // P not requested
            Some(48),      // Q target 48 MHz
            Some(48),      // R target 48 MHz (SYSCLK)
            50,            // max 50 ppm tolerance for test
        );

        let sol = best.expect("should find a solution for 48/48 from HSI16");

        // Expect the canonical solution M=2, N=12, Q=2, R=2 for 48/48
        assert_eq!(sol.m, 2);
        assert_eq!(sol.n, 12);
        assert_eq!(sol.q, Some(2));
        assert_eq!(sol.r, Some(2));
        assert_eq!(sol.actual_q_mhz, Some(48));
        assert_eq!(sol.actual_r_mhz, Some(48));
    }

    /// If we request an out-of-range Q target (e.g., 60 MHz), the solver should fail.
    #[test]
    fn reject_q_over_spec() {
        let src_mhz = HSI_FREQUENCY_MHZ;

        let best = Pll::<TestWl55>::solve_pll_for_targets::<TestWl55>(
            src_mhz,
            None,
            Some(60),  // exceeds 48 MHz spec
            None,
            1000,
        );

        assert!(best.is_none(), "Q target above 48 MHz must be rejected");
    }

    /// Check the high-level API updates the struct’s cached frequencies using a dummy RCC.
    #[test]
    fn configure_from_targets_updates_cached_freqs() {
        let rcc = DummyRcc::new();
        let mut pll = Pll::<TestWl55>::new(unsafe { core::mem::transmute::<&DummyRcc, &Rcc>(&rcc) });

        // Ask for R=48 MHz SYSCLK and Q=48 MHz
        pll.configure_from_targets_mhz(PllSource::HSI16, None, Some(48), Some(48), 100)
            .expect("configuration should succeed");

        assert_eq!(pll.pllq_frequency_mhz, 48);
        assert_eq!(pll.pllr_frequency_mhz, 48);
    }
}

