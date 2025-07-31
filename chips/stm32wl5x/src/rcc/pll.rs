use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};

impl Rcc {
        //*******************************************************************//
//                                 PLL Methods                           //
    //*******************************************************************//
    fn init_pll_clock(&self) {
        self.set_pll_clocks_source(PllSource::HSI16);
        self.set_pll_clocks_m_divider(DEFAULT_PLLM_VALUE);
        self.set_pll_clock_n_multiplier(DEFAULT_PLLN_VALUE);
        self.set_pll_clock_p_divider(DEFAULT_PLLP_VALUE);
        self.set_pll_clock_q_divider(DEFAULT_PLLQ_VALUE);
    }

    pub(crate) fn set_pll_clocks_source(&self, pll_source: PllSource) {
        self.registers().pllcfgr
            .modify(PLLCFGR::PLLSRC.val(pll_source as u32));
    }

    pub(crate) fn get_pll_clocks_source(&self) -> PllSource {
        match self.registers().pllcfgr.read(PLLCFGR::PLLSRC) {
            0b01 => PllSource::MSI,
            0b10 => PllSource::HSI16,
            0b11 => PllSource::HSE32,
            _ => panic!("Invalid PLL source"),
        }
    }

    // This method must be called only when all PLL clocks are disabled
    pub(crate) fn set_pll_clocks_m_divider(&self, m: u32) {
        self.registers().pllcfgr.modify(PLLCFGR::PLLM.val(m ));
    }

    pub(crate) fn get_pll_clocks_m_divider(&self) -> u32 {
        let pll_m = self.registers().pllcfgr.read(PLLCFGR::PLLM);
        match pll_m {
            1..=8 => pll_m,
            _ => panic!("Unexpected PLLM divider"),
        }
    }

    pub (crate) fn set_pll_clock_n_multiplier(&self, n: u32) {
        if n < 8 || n > 127 {
            panic!("Invalid PLLN multiplier value: {}", n);
        }
        self.registers().pllcfgr.modify(PLLCFGR::PLLN.val(n));
    }

    pub (crate) fn get_pll_clock_n_mulltiplier(&self) -> u32 {
        self.registers().pllcfgr.read(PLLCFGR::PLLN)
    }

    pub (crate) fn set_pll_clock_p_divider(&self, p: u32) {
        match p {
            2..32 => self.registers().pllcfgr.modify(PLLCFGR::PLLP.val(p)),
            _ => panic!("Invalid PLLP divider value: {}", p),
        }
    }

    pub (crate) fn get_pll_clock_p_divider(&self) -> u32 {
        self.registers().pllcfgr.read(PLLCFGR::PLLP)
    }
    
    pub (crate) fn set_pll_clock_q_divider(&self, q: u32) {
        match q {
            2..=8 => self.registers().pllcfgr.modify(PLLCFGR::PLLQ.val(q)),
            _ => panic!("Invalid PLLQ divider value: {}", q),
            
        }
    }

    pub(crate) fn set_pll_enable(&self, enable: bool) {
        if enable {
            self.registers().cr.modify(CR::PLLON::SET);
        } else {
            self.registers().cr.modify(CR::PLLON::CLEAR);
        }
    }

    pub(crate) fn is_pll_clock_enabled(&self) -> bool {
        self.registers().cr.is_set(CR::PLLON)
    }

    pub(crate) fn is_pll_ready(&self) -> bool {
        self.registers().cr.is_set(CR::PLLRDY)
    }
}

pub enum PllSource {
    MSI = 0b01,
    HSI16 = 0b10,
    HSE32 = 0b11,
}