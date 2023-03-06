pub mod button;
pub mod display;
pub mod gfx;
pub mod joystick;

use anyhow::Result;
use button::ButtonController;
use display::Display;
use joystick::Joystick;

pub struct Device {
    pub display: Display,
    pub joystick: Joystick,
    pub button_controller: ButtonController,
}

impl Device {
    pub fn new() -> Result<Self> {
        Ok(Self {
            display: Display::new()?,
            joystick: Joystick::new()?,
            button_controller: ButtonController::new()?,
        })
    }

    pub fn initialize_components(&mut self) -> Result<()> {
        self.display.initialize()?;
        Ok(())
    }
}
