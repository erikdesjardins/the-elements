use stm32f103xx::RCC;

use clock::AsDivisor;

pub fn adc_prescaler(rcc: &RCC) -> u32 {
    // sysclk
    let ahb = rcc.cfgr.read().hpre().as_divisor(); // -> through AHB prescaler
    let abp2 = rcc.cfgr.read().ppre2().as_divisor(); // -> through APB2 prescaler
    let adc = rcc.cfgr.read().adcpre().as_divisor(); // -> through ADC prescaler
    u32::from(ahb) * u32::from(abp2) * u32::from(adc)
}
