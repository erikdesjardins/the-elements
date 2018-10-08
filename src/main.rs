#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

#[cfg(debug_assertions)]
extern crate panic_halt;
#[cfg(not(debug_assertions))]
extern crate panic_abort;

use core::sync::atomic::{compiler_fence, Ordering};
use core::fmt::Write;

use cortex_m_rt::{entry, exception};
use cortex_m::peripheral::{self, syst::SystClkSource};
use cortex_m_semihosting::hio;

const RCC: u32 = 0x4002_1000;
const GPIO_PORT_C: u32 = 0x4001_1000;

#[entry]
fn main() -> ! {
    let rcc_apb2enr = unsafe { &mut *((RCC + 0x18) as *mut u32) };

    // enable gpio clock
    *rcc_apb2enr |= 1 << 4;

    let gpio_high = unsafe { &mut *((GPIO_PORT_C + 0x04) as *mut u32) };

    // CNF13  = 0b01 (open-drain output)
    // MODE13 = 0b10 (up to 2MHz output)
    *gpio_high = (*gpio_high & 0xff0f_ffff) | 0x0060_0000;

    let p = cortex_m::Peripherals::take().unwrap();

    // not even close to accurate
    let ticks_per_10ms = peripheral::SYST::get_ticks_per_10ms();

    let mut syst = p.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(ticks_per_10ms * 1000);
    syst.enable_counter();
    syst.enable_interrupt();

    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, r#"ticks per "10" ms: {:#x}"#, ticks_per_10ms).unwrap();
    writeln!(hstdout, "precise: {}", peripheral::SYST::is_precise()).unwrap();

    loop { compiler_fence(Ordering::SeqCst); }
}

#[exception]
fn SysTick() {
    let gpio_data = unsafe { &mut *((GPIO_PORT_C + 0x0C) as *mut u32) };

    *gpio_data ^= 1 << 13;
}
