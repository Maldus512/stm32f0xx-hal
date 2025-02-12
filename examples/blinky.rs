#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f0xx_hal as hal;

use crate::hal::{prelude::*, stm32};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(mut p) = stm32::Peripherals::take() {
        let mut led = cortex_m::interrupt::free(|cs| {
            let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

            let gpioa = p.GPIOA.split(&mut rcc);

            // (Re-)configure PA1 as output
            gpioa.pa1.into_push_pull_output(cs)
        });

        loop {
            // Turn PA1 on a million times in a row
            for _ in 0..1_000_000 {
                led.set_high();
            }
            // Then turn PA1 off a million times in a row
            for _ in 0..1_000_000 {
                led.set_low();
            }
        }
    }

    loop {
        continue;
    }
}
