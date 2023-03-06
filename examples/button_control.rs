use ws_oled_driver::Device;  /* HAT Device */
use ws_oled_driver::button::State;
use anyhow::Result;

fn main() -> Result<()> {
  let mut device = Device::new()?;
  device.initialize_components()?;

  loop {
    if let Some(button_state) = device.button_controller.read() {
      match button_state {
        State::Key1 => println!("Key1 pressed"),
        State::Key2 => println!("Key2 pressed"),
        State::Key3 => println!("Key3 pressed"),
      }
    }
    else {
      println!("Nothing Pressed!")
    }
  }
}
