#![no_std]
#![no_main]

#[allow(unused_imports)]
// Halt on panic
use panic_probe as _;
use stm32f1xx_hal as hal;

use cortex_m_rt::entry;
use hal::{
    stm32,
    prelude::*,
};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    // let mut rcc = dp.RCC.constrain();
    // let cp = cortex_m::Peripherals::take().unwrap();
    let mut gpiob = dp.GPIOB.split();

    // Configure the PB2 pin as an output
    let mut led = gpiob.pb2.into_push_pull_output(&mut gpiob.crl);

    loop {
        led.set_high();
        cortex_m::asm::delay(1_000_000);
        led.set_low();
        cortex_m::asm::delay(2_000_000);
    }
}