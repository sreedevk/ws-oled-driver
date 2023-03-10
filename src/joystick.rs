use anyhow::Result;
use rppal::gpio::{Gpio, InputPin, Level};

const KEY_UP_PIN: u8 = 6;
const KEY_DOWN_PIN: u8 = 19;
const KEY_LEFT_PIN: u8 = 5;
const KEY_RIGHT_PIN: u8 = 26;
const KEY_PRESS_PIN: u8 = 13;

/// # Joystick
/// the Joystick struct is a controller for the Joystick on the Device. It has 4 private fields to
/// store the interface pins. These fields are used internally and do not offer direct access.
pub struct Joystick {
    up_pin: InputPin,
    down_pin: InputPin,
    left_pin: InputPin,
    right_pin: InputPin,
    click_pin: InputPin,
}

/// # Joystick State
/// The State enum is used to represent the state of the Joystick. Which is one of:
/// - Up  // When the joystick is pushed up
/// - Down // when the joystick is pulled down
/// - Left // when the joystick is pushed left
/// - Right // when the joystick is pushed right
/// - Click // when the joystick is clicked.
#[derive(Debug)]
pub enum State {
    Up,
    Down,
    Left,
    Right,
    Click,
}

impl Joystick {
    pub fn new() -> Result<Self> {
        let gpio_controller = Gpio::new()?;
        let up_pin = gpio_controller.get(KEY_UP_PIN)?.into_input_pullup();
        let down_pin = gpio_controller.get(KEY_DOWN_PIN)?.into_input_pullup();
        let left_pin = gpio_controller.get(KEY_LEFT_PIN)?.into_input_pullup();
        let right_pin = gpio_controller.get(KEY_RIGHT_PIN)?.into_input_pullup();
        let click_pin = gpio_controller.get(KEY_PRESS_PIN)?.into_input_pullup();

        Ok(Self {
            up_pin,
            down_pin,
            left_pin,
            right_pin,
            click_pin,
        })
    }

    /// Returns `Option<State>`. When no reportable state is detected, it returns None.
    pub fn read(&self) -> Option<State> {
        if self.up_pin.read() == Level::Low { return Some(State::Up) }
        if self.down_pin.read() == Level::Low { return Some(State::Down) }
        if self.left_pin.read() == Level::Low { return Some(State::Left) }
        if self.right_pin.read() == Level::Low { return Some(State::Right) }
        if self.click_pin.read() == Level::Low { return Some(State::Click) }

        None
    }
}
