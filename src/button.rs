use anyhow::Result;
use rppal::gpio::Gpio;
use rppal::gpio::InputPin;
use rppal::gpio::Level;

const KEY1_PIN: u8 = 21;
const KEY2_PIN: u8 = 20;
const KEY3_PIN: u8 = 16;

#[derive(Debug)]
pub enum State {
    Key1,
    Key2,
    Key3,
}

pub struct ButtonController {
    pin1: InputPin,
    pin2: InputPin,
    pin3: InputPin
}

impl ButtonController {
    pub fn new() -> Result<Self> {
        let gpio = Gpio::new()?;
        let pin1 = gpio.get(KEY1_PIN)?.into_input_pullup();
        let pin2 = gpio.get(KEY2_PIN)?.into_input_pullup();
        let pin3 = gpio.get(KEY3_PIN)?.into_input_pullup();

        Ok(Self { pin1, pin2, pin3 })
    }

    pub fn read(&self) -> Option<State> {
        if self.pin1.read() == Level::High { return Some(State::Key1); }
        if self.pin2.read() == Level::High { return Some(State::Key2); }
        if self.pin3.read() == Level::High { return Some(State::Key3); }

        None
    }
}
