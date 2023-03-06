use anyhow::Result;
use ws_oled_driver::joystick;
use ws_oled_driver::Device; /* HAT Device */

fn main() -> Result<()> {
    let mut device = Device::new()?;
    device.initialize_components()?;

    loop {
        if let Some(joystick_state) = device.joystick.read() {
            match joystick_state {
                joystick::State::Up => {
                    println!("You Pressed Up");
                }
                joystick::State::Down => {
                    println!("You Pressed Down");
                }
                joystick::State::Left => {
                    println!("You Pressed Left");
                }
                joystick::State::Right => {
                    println!("You Pressed Right");
                }
                joystick::State::Click => {
                    println!("You Clicked!");
                }
            }
        }
    }
}
