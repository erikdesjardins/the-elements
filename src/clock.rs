use stm32f103xx::rcc::cfgr::{ADCPRER, HPRER, PPRE2R};

pub trait AsDivisor {
    fn as_divisor(&self) -> u16;
}

impl AsDivisor for HPRER {
    fn as_divisor(&self) -> u16 {
        match self {
            HPRER::NODIV => 1,
            HPRER::DIV2 => 2,
            HPRER::DIV4 => 4,
            HPRER::DIV8 => 8,
            HPRER::DIV16 => 16,
            HPRER::DIV64 => 64,
            HPRER::DIV128 => 128,
            HPRER::DIV256 => 256,
            HPRER::DIV512 => 512,
            HPRER::_Reserved(_) => unreachable!(),
        }
    }
}

impl AsDivisor for PPRE2R {
    fn as_divisor(&self) -> u16 {
        match self {
            PPRE2R::NODIV => 1,
            PPRE2R::DIV2 => 2,
            PPRE2R::DIV4 => 4,
            PPRE2R::DIV8 => 8,
            PPRE2R::DIV16 => 16,
            PPRE2R::_Reserved(_) => unreachable!(),
        }
    }
}

impl AsDivisor for ADCPRER {
    fn as_divisor(&self) -> u16 {
        match self {
            ADCPRER::DIV2 => 2,
            ADCPRER::DIV4 => 4,
            ADCPRER::DIV6 => 6,
            ADCPRER::DIV8 => 8,
        }
    }
}
