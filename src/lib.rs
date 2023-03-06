pub mod button;
pub mod display;
pub mod gfx;
pub mod joystick;

use anyhow::Result;
use button::ButtonController;
use display::Display;
use joystick::Joystick;

/// # Device
/// the `Device` struct offers an organized way to access various componenets on the Waveshare OLED
/// Display Hat. It has three fields, `display`, `joystick` and `button_controller`
/// - The `display` field is an instance of the `Display` struct that let's you initialze and control
/// the display
/// - The `joystick` is an instance of the `Joystick` struct that let's you initalize and read
/// input from the joystick on the device.
/// - The button_controller field is an instance of the ButtonController Struct that let's you
/// initalize and read input from the three buttons on the device 
///
/// # Usage
/// ```
/// use ws_oled_driver::Device;
/// let mut device = Device::new()?.initialize_components()?;
/// ````
pub struct Device {
    pub display: Display,
    pub joystick: Joystick,
    pub button_controller: ButtonController,
}

impl Device {
    /// Creates a new `Device`. It returns `Result<Device, anyhow::Error>`.
    pub fn new() -> Result<Self> {
        Ok(Self {
            display: Display::new()?,
            joystick: Joystick::new()?,
            button_controller: ButtonController::new()?,
        })
    }

    /// Intializes components - display, joystick & buttons.
    pub fn initialize_components(&mut self) -> Result<()> {
        self.display.initialize()?;
        Ok(())
    }
}
