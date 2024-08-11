#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // Provides a panic handler that halts the processor on panic
use stm32f1xx_hal::{pac, prelude::*, serial::Serial};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // Configure PA9 and PA10 as UART pins
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // Configure PC13 as input (Button)
    let button = gpioc.pc13.into_pull_down_input(&mut gpioc.crh);

    // Set up UART at 9600 baud
    let serial = Serial::usart1(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );

    let (mut tx, _) = serial.split();

    loop {
        // Check if the button is pressed
        if button.is_high().unwrap() {
            // Send a message over UART
            tx.write_str("Button Pressed!\r\n").unwrap();
        }
    }
}
