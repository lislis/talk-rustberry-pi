use anyhow::Result;

use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_TOUCH: u8 = 24;

fn main() -> Result<()> {
    println!("Using touch sensor on a {}.", DeviceInfo::new()?.model());

    let touch = Gpio::new()?
        .get(GPIO_TOUCH)?
        .into_input();

    loop {
        if touch.is_high() {
            println!("HIGH");
        } else {
            println!("LOW");
        }
        thread::sleep(Duration::from_millis(200));
    }
}
