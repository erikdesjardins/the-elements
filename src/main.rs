#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f103xx;

extern crate panic_halt;

use cortex_m::asm::wfi;
use cortex_m::interrupt;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use stm32f103xx::GPIOC;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    dp.RCC.apb2enr.modify(|_, w| w.iopcen().enabled());

    dp.GPIOC
        .crh
        .modify(|_, w| w.mode13().output2().cnf13().open());

    interrupt::free(|_| unsafe {
        GPIOC = Some(dp.GPIOC);
    });

    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(10_000_000);
    syst.enable_counter();
    syst.enable_interrupt();

    loop {
        wfi();
    }
}

static mut GPIOC: Option<GPIOC> = None;

#[exception]
fn SysTick() {
    if let Some(gpioc) = unsafe { &GPIOC } {
        gpioc.odr.modify(|r, w| w.odr13().bit(!r.odr13().bit()));
    }
}
