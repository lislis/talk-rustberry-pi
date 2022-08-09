use anyhow::Result;
use rand::prelude::*;

use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, InputPin, OutputPin};
use rppal::system::DeviceInfo;
use rppal::pwm::{Channel, Polarity, Pwm};

const PERIOD_MS: u64 = 50;
const PULSE_MIN_US: u64 = 500;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 2500;

const BTN: u8 = 23;
const INDICATOR_LED: u8 = 12;
const POINT1_LED: u8 = 16;
const POINT2_LED: u8 = 20;
const POINT3_LED: u8 = 21;

#[derive(Debug)]
struct Game {
    led1: OutputPin,
    led2: OutputPin,
    led3: OutputPin,
    indicator: OutputPin,
    btn: InputPin,
    pwm: Pwm,
    data: GameData,
    state: State
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct GameData {
    current_num: u64,
    current_pulse: u64,
    max_led: usize,
    current_led: usize,
    clockwise: bool
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum State {
    Setup,
    Play,
    Win
}


fn main() -> Result<()> {
    println!("Hack into the vault! {}.", DeviceInfo::new()?.model());

    let led1 = Gpio::new()?
        .get(POINT1_LED)?
        .into_output();
    let led2 = Gpio::new()?
        .get(POINT2_LED)?
        .into_output();
    let led3 = Gpio::new()?
        .get(POINT3_LED)?
        .into_output();
    let indicator = Gpio::new()?
        .get(INDICATOR_LED)?
        .into_output();
    let btn = Gpio::new()?
        .get(BTN)?
        .into_input();

    let pwm = Pwm::with_period(
        // GPIO18, physical 12
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_NEUTRAL_US),
        Polarity::Normal,
        true
    )?;

    let mut rnd = thread_rng();
    let data = GameData {
        current_num: rnd.gen_range(500..=2500),
        max_led: 3,
        current_led: 0,
        current_pulse: PULSE_MAX_US,
        clockwise: true
    };
    let state = State::Setup;
    let mut game = Game {
        led1,
        led2,
        led3,
        indicator,
        pwm,
        btn,
        data,
        state
   };

    loop {
        match game.state {
            State::Setup => setup(&mut game)?,
            State::Play =>  play(&mut game)?,
            State::Win => win(&mut game)?
        }

    }
}

fn setup(game: &mut Game) -> Result<()> {
    game.led1.set_low();
    game.led2.set_low();
    game.led3.set_low();
    game.indicator.set_low();
    thread::sleep(Duration::from_millis(1000));
    game.pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US))?;
    game.indicator.set_high();
    thread::sleep(Duration::from_millis(1000));
    game.pwm.set_pulse_width(Duration::from_micros(PULSE_MAX_US))?;
    game.indicator.set_low();
    thread::sleep(Duration::from_millis(1000));
    game.pwm.set_pulse_width(Duration::from_micros(PULSE_NEUTRAL_US))?;
    game.indicator.set_high();
    thread::sleep(Duration::from_millis(1000));
    game.state = State::Play;
    game.data.current_pulse = PULSE_NEUTRAL_US;
    Ok(())
}

fn play(game: &mut Game) -> Result<()> {
    match game.data.current_led {
        0 => { game.led1.toggle() },
        1 => {
            game.led1.set_high();
            game.led2.toggle()
        },
        2 => {
            game.led1.set_high();
            game.led2.set_high();
            game.led3.toggle();
        },
        _ => { game.state = State::Win }
    };

    //println!("{:?}, {:?}", game.btn.is_high(), game.data);

    let step = 50;
    if game.data.current_num >= game.data.current_pulse - step
        && game.data.current_num <= game.data.current_pulse + step {
        game.indicator.set_high();
            if game.btn.is_high() {
                println!("MATCH! Next number");
                game.data.current_led += 1;
                let mut rnd = thread_rng();
                game.data.current_num = rnd.gen_range(500..=2500);
            }
        } else {
            game.indicator.set_low();
        }


    if game.data.current_pulse >= PULSE_MAX_US {
        game.data.clockwise = false;
    } else if game.data.current_pulse <= PULSE_MIN_US {
        game.data.clockwise = true;
    }

    if game.data.clockwise {
        game.data.current_pulse += step;
    } else {
        game.data.current_pulse -= step;
    }
    game.pwm.set_pulse_width(Duration::from_micros(game.data.current_pulse))?;

    thread::sleep(Duration::from_millis(500));
    Ok(())
}

fn win(game: &mut Game) -> Result<()> {
    game.led1.toggle();
    game.led2.toggle();
    game.led3.toggle();
    game.indicator.toggle();
    println!("You win!");
    thread::sleep(Duration::from_millis(300));
    Ok(())
}
