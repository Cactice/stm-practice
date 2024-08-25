#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::{self, entry};
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f4xx_hal::gpio::GpioExt;

#[entry]
fn main() -> ! {
    if let Some(peripherals) = stm32f4xx_hal::pac::Peripherals::take() {
        let gpioa = peripherals.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        let mut sec = 0;
        let gpioc = peripherals.GPIOC.split();
        let button = gpioc.pc13;

        loop {
            if button.is_high() {
                led.set_low();
            } else {
                sec = sec + 1;
                hprintln!("Seconds: {}", sec);

                led.set_high();
            }
        }
    }
    loop {}
}
