use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};

impl Rcc {
        //*******************************************************************//
//                                 PLL Methods                           //
    //*******************************************************************//
    pub(crate) fn configure_pll(&self, pll_source: PllSource, m: u8, n: u16, p: u8, q: u8, r: u8) {
        self.set_pll_clocks_source(pll_source);
        self.set_pll_clocks_m_divider(m);
        self.set_pll_clock_n_multiplier(n);
        self.set_pll_clock_p_divider(p);
        self.set_pll_clock_q_divider(q);
        self.set_pll_clock_r_divider(r);
    }

    pub(crate) fn enable_pll_outputs(&self,enable_p: bool, enable_q: bool, enable_r: bool) {
        if enable_p {
            self.registers().pllcfgr.modify(PLLCFGR::PLLPEN::SET);
        }
        if enable_q {
            self.registers().pllcfgr.modify(PLLCFGR::PLLQEN::SET);
        }
        if enable_r {
            self.registers().pllcfgr.modify(PLLCFGR::PLLREN::SET);
        }
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
    pub(crate) fn set_pll_clocks_m_divider(&self, m: u8) {
        self.registers().pllcfgr.modify(PLLCFGR::PLLM.val(m.into()));
    }

    pub(crate) fn get_pll_clocks_m_divider(&self) -> u32 {
        let pll_m = self.registers().pllcfgr.read(PLLCFGR::PLLM);
        match pll_m {
            1..=8 => pll_m,
            _ => panic!("Unexpected PLLM divider"),
        }
    }

    pub (crate) fn set_pll_clock_n_multiplier(&self, n: u16) {
        if n < 8 || n > 127 {
            panic!("Invalid PLLN multiplier value: {}", n);
        }
        self.registers().pllcfgr.modify(PLLCFGR::PLLN.val(n.into()));
    }

    pub (crate) fn get_pll_clock_n_mulltiplier(&self) -> u32 {
        self.registers().pllcfgr.read(PLLCFGR::PLLN)
    }

    pub (crate) fn set_pll_clock_p_divider(&self, p: u8) {
        match p {
            2..32 => self.registers().pllcfgr.modify(PLLCFGR::PLLP.val(p.into())),
            _ => panic!("Invalid PLLP divider value: {}", p),
        }
    }

    pub (crate) fn get_pll_clock_p_divider(&self) -> u32 {
        self.registers().pllcfgr.read(PLLCFGR::PLLP)
    }
    
    pub (crate) fn set_pll_clock_q_divider(&self, q: u8) {
        match q {
            2..=8 => self.registers().pllcfgr.modify(PLLCFGR::PLLQ.val(q.into())),
            _ => panic!("Invalid PLLQ divider value: {}", q),
            
        }
    }

    pub (crate) fn get_pll_clock_r_divider(&self) -> u32 {
        self.registers().pllcfgr.read(PLLCFGR::PLLR)
    }
    
    pub (crate) fn set_pll_clock_r_divider(&self, r: u8) {
        match r {
            2..=8 => self.registers().pllcfgr.modify(PLLCFGR::PLLR.val(r.into())),
            _ => panic!("Invalid PLLQ divider value: {}", r),            
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

    pub(crate) fn disable_pll_and_wait(&self) {
        self.set_pll_enable(false);
        while self.is_pll_ready() {}
    }
    pub(crate) fn wait_pll_ready(&self) {
        while !self.is_pll_ready() {}
    }

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub (crate) enum PllSource {
    MSI = 0b01,
    HSI16 = 0b10,
    HSE32 = 0b11,
}

pub (crate) enum PLLM {
    Div1 = 1,
    Div2 = 2,
    Div3 = 3,
    Div4 = 4,
    Div5 = 5,
    Div6 = 6,
    Div7 = 7,
    Div8 = 8,
}
pub (crate) enum PLLN {
    Mul6 = 0b0000110,
    Mul7 = 0b0000111,
    Mul8 = 0b0001000,
    Mul9 = 0b0001001,
    Mul10 = 0b0001010,
    Mul11 = 0b0001011,
    Mul12 = 0b0001100,
    Mul13 = 0b0001101,
    Mul14 = 0b0001110, 
    Mul15 = 0b0001111,
    Mul16 = 0b0010000,
    Mul17 = 0b0010001,
    Mul18 = 0b0010010,
    Mul19 = 0b0010011,
    Mul20 = 0b0010100,
    Mul21 = 0b0010101,
    Mul22 = 0b0010110,
    Mul23 = 0b0010111,
    Mul24 = 0b0011000,
    Mul25 = 0b0011001,
    Mul26 = 0b0011010,
    Mul27 = 0b0011011,
    Mul28 = 0b0011100,
    Mul29 = 0b0011101,
    Mul30 = 0b0011110,
    Mul31 = 0b0011111,
    Mul32 = 0b0100000,
    Mul33 = 0b0100001,
    Mul34 = 0b0100010,
    Mul35 = 0b0100011,
    Mul36 = 0b0100100,
    Mul37 = 0b0100101,
    Mul38 = 0b0100110,
    Mul39 = 0b0100111,
    Mul40 = 0b0101000,
    Mul41 = 0b0101001,
    Mul42 = 0b0101010,
    Mul43 = 0b0101011,
    Mul44 = 0b0101100,
    Mul45 = 0b0101101,
    Mul46 = 0b0101110,
    Mul47 = 0b0101111,
    Mul48 = 0b0110000,
    Mul49 = 0b0110001,
    Mul50 = 0b0110010, 
    Mul51 = 0b0110011,
    Mul52 = 0b0110100,
    Mul53 = 0b0110101,
    Mul54 = 0b0110110,
    Mul55 = 0b0110111,
    Mul56 = 0b0111000,
    Mul57 = 0b0111001,
    Mul58 = 0b0111010,
    Mul59 = 0b0111011,
    Mul60 = 0b0111100,
    Mul61 = 0b0111101,
    Mul62 = 0b0111110,
    Mul63 = 0b0111111,
    Mul64 = 0b1000000,
    Mul65 = 0b1000001,
    Mul66 = 0b1000010,
    Mul67 = 0b1000011,
    Mul68 = 0b1000100,
    Mul69 = 0b1000101,
    Mul70 = 0b1000110,
    Mul71 = 0b1000111,
    Mul72 = 0b1001000,
    Mul73 = 0b1001001,
    Mul74 = 0b1001010,
    Mul75 = 0b1001011,
    Mul76 = 0b1001100,
    Mul77 = 0b1001101,
    Mul78 = 0b1001110,
    Mul79 = 0b1001111,
    Mul80 = 0b1010000,
    Mul81 = 0b1010001,
    Mul82 = 0b1010010,
    Mul83 = 0b1010011,
    Mul84 = 0b1010100,
    Mul85 = 0b1010101,
    Mul86 = 0b1010110,
    Mul87 = 0b1010111,
    Mul88 = 0b1011000,
    Mul89 = 0b1011001,
    Mul90 = 0b1011010,
    Mul91 = 0b1011011,
    Mul92 = 0b1011100,
    Mul93 = 0b1011101,
    Mul94 = 0b1011110,
    Mul95 = 0b1011111,
    Mul96 = 0b1100000,
    Mul97 = 0b1100001,
    Mul98 = 0b1100010,
    Mul99 = 0b1100011,
    Mul100 = 0b1100100,
    Mul101 = 0b1100101,
    Mul102 = 0b1100110,
    Mul103 = 0b1100111,
    Mul104 = 0b1101000,
    Mul105 = 0b1101001,
    Mul106 = 0b1101010,
    Mul107 = 0b1101011,
    Mul108 = 0b1101100,
    Mul109 = 0b1101101,
    Mul110 = 0b1101110,
    Mul111 = 0b1101111,
    Mul112 = 0b1110000,
    Mul113 = 0b1110001,
    Mul114 = 0b1110010,
    Mul115 = 0b1110011,
    Mul116 = 0b1110100,
    Mul117 = 0b1110101,
    Mul118 = 0b1110110,
    Mul119 = 0b1110111,
    Mul120 = 0b1111000,
    Mul121 = 0b1111001,
    Mul122 = 0b1111010,
    Mul123 = 0b1111011,
    Mul124 = 0b1111100,
    Mul125 = 0b1111101,
    Mul126 = 0b1111110,
    Mul127 = 0b1111111,
}

pub(crate) enum PLLP {
    Div2 = 0b001,
    Div3 = 0b010,
    Div4 = 0b011,
    Div5 = 0b100,
    Div6 = 0b101,
    Div7 = 0b110,
    Div8 = 0b111,
}

pub(crate) enum PLLQ {
    Div2 = 0b001,
    Div3 = 0b010,
    Div4 = 0b011,
    Div5 = 0b100,
    Div6 = 0b101,
    Div7 = 0b110,
    Div8 = 0b111,
}

pub(crate) enum PLLR {
    Div2 = 0b00001,
    Div3 = 0b00010,
    Div4 = 0b00011,
    Div5 = 0b00100,
    Div6 = 0b00101,
    Div7 = 0b00110,
    Div8 = 0b00111,
    Div9 = 0b01000,
    Div10 = 0b01001,
    Div11 = 0b01010,
    Div12 = 0b01011,
    Div13 = 0b01100,                
    Div14 = 0b01101,
    Div15 = 0b01110,
    Div16 = 0b01111,
    Div17 = 0b10000,
    Div18 = 0b10001,
    Div19 = 0b10010,
    Div20 = 0b10011,
    Div21 = 0b10100,
    Div22 = 0b10101,
    Div23 = 0b10110,
    Div24 = 0b10111,
    Div25 = 0b11000,
    Div26 = 0b11001,
    Div27 = 0b11010,
    Div28 = 0b11011,
    Div29 = 0b11100,
    Div30 = 0b11101,
    Div31 = 0b11110,
    Div32 = 0b11111,    
}