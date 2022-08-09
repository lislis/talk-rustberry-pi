use anyhow::Result;

use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_LED: u8 = 23;

fn main() -> Result<()> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?
        .get(GPIO_LED)?
        .into_output();

    loop {
        pin.toggle();
        thread::sleep(Duration::from_millis(500));
    }
}
