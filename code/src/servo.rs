use anyhow::Result;

use std::thread;
use std::time::Duration;

use rppal::system::DeviceInfo;
use rppal::pwm::{Channel, Polarity, Pwm};

const PERIOD_MS: u64 = 50;
const PULSE_MIN_US: u64 = 500;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 2500;

fn main() -> Result<()> {
    println!("Servo fun on a {}.", DeviceInfo::new()?.model());

    let pwm = Pwm::with_period(
        // GPIO18, physical 12
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
        Polarity::Normal,
        true
    )?;

    thread::sleep(Duration::from_millis(500));

    pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US))?;

    thread::sleep(Duration::from_millis(500));

    for pulse in (PULSE_MIN_US..=PULSE_NEUTRAL_US).step_by(10) {
        pwm.set_pulse_width(Duration::from_micros(pulse))?;
        thread::sleep(Duration::from_millis(20));
    }

    Ok(())
}
