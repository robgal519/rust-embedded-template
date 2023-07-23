#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin as _, Pull};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task(pool_size = 4)]
async fn button_task(n: usize, mut led: Output<'static, AnyPin>, mut btn: Input<'static, AnyPin>) {
    loop {
        btn.wait_for_low().await;
        led.set_low();
        info!("Button {:?} pressed!", n);
        btn.wait_for_high().await;
        led.set_high();
        info!("Button {:?} released!", n);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    info!("Starting!");

    let btn1 = Input::new(p.P0_11.degrade(), Pull::Up);
    let btn2 = Input::new(p.P0_12.degrade(), Pull::Up);
    let btn3 = Input::new(p.P0_24.degrade(), Pull::Up);
    let btn4 = Input::new(p.P0_25.degrade(), Pull::Up);

    let led1 = Output::new(p.P0_13.degrade(), Level::High, OutputDrive::Standard);
    let led2 = Output::new(p.P0_14.degrade(), Level::High, OutputDrive::Standard);
    let led3 = Output::new(p.P0_15.degrade(), Level::High, OutputDrive::Standard);
    let led4 = Output::new(p.P0_16.degrade(), Level::High, OutputDrive::Standard);
    unwrap!(spawner.spawn(button_task(1, led1, btn1)));
    unwrap!(spawner.spawn(button_task(2, led2, btn2)));
    unwrap!(spawner.spawn(button_task(3, led3, btn3)));
    unwrap!(spawner.spawn(button_task(4, led4, btn4)));
}
