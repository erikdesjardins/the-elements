#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f103xx;

extern crate panic_halt;

mod clock;
mod rcc;
mod ts;

use cortex_m::asm::{bkpt, nop, wfi};
use cortex_m::interrupt;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use stm32f103xx::ADC1;

use rcc::adc_prescaler;

const SYSCLK_FREQ: u32 = 8_000_000;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    // prepare adc channel 16 (temperature sensor)
    dp.RCC.apb2enr.modify(|_, w| w.adc1en().enabled());
    dp.ADC1
        .cr2
        .modify(|_, w| w.adon().set_bit().tsvrefe().set_bit());
    dp.ADC1.sqr3.modify(|_, w| unsafe { w.sq1().bits(16) });
    dp.ADC1.smpr1.modify(|_, w| unsafe {
        // recommended sample time 17.1us ~ 138 sysclk cycles @ 8MHz
        // default overall adc prescaler /2 => ~69 adc cycles
        // closest is 0b110 = 71.5 adc cycles
        // but use the max 0b111 = 239.5 adc cycles
        // for extra stability, and because we don't need the performance
        w.smp16().bits(0b111)
    });
    // calibration: "must have been in power-on state (ADON bit = ‘1’) for at least two ADC clock cycles"
    // also, must wait t_STAB = 1us for adc to stabilize in general; 1us ~ 8 sysclk cycles @ 8MHz
    for _ in 0..(2 * adc_prescaler(&dp.RCC)).max(8) {
        nop();
    }
    // start calibration and wait for it to be done
    dp.ADC1.cr2.modify(|_, w| w.cal().set_bit());
    while dp.ADC1.cr2.read().cal().bit_is_set() {}

    // run ~1Hz timer
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(SYSCLK_FREQ);
    syst.enable_counter();
    syst.enable_interrupt();

    interrupt::free(|_| unsafe {
        ADC1 = Some(dp.ADC1);
    });

    loop {
        wfi();
    }
}

static mut ADC1: Option<ADC1> = None;

#[exception]
fn SysTick() {
    if let Some(adc1) = unsafe { &ADC1 } {
        // start conversion and wait for it to be done
        adc1.cr2.modify(|_, w| w.adon().set_bit());
        while adc1.sr.read().eoc().bit_is_clear() {}
        let data = adc1.dr.read().data().bits();
        let _temp = ts::to_tenths_celsius(data);
        bkpt();
    }
}
