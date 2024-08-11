#![no_std]
#![no_main]

use panic_halt as _; // A simple panic handler for embedded applications
use cortex_m_rt::entry; // The entry point for the program
use stm32f1xx_hal::{pac, prelude::*, delay::Delay}; // HAL for the specific microcontroller

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the Cortex-M processor
    let cp = pac::CorePeripherals::take().unwrap();

    // Get access to the device specific peripherals from the microcontroller
    let dp = pac::Peripherals::take().unwrap();

    // Set up the system clock. We want to run at 8 MHz for this example.
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Set up the delay provider
    let mut delay = Delay::new(cp.SYST, clocks);

    // Set up the GPIO pin
    let mut gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
        // Turn the LED on
        led.set_high().unwrap();
        delay.delay_ms(1_000_u16);

        // Turn the LED off
        led.set_low().unwrap();
        delay.delay_ms(1_000_u16);
    }
}
