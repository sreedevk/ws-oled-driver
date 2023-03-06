## Raspberry Pi Waveshare OLED (SH1106) HAT Driver Written in Rust
Device (Front) | Device (Back) | Device (Powered)
:-------------------------:|:-------------------------:|:------------------------:|
<img src="https://user-images.githubusercontent.com/36154121/223014213-2cf0357e-4ea1-405d-9f1c-89f3bf78f9d4.jpg" width="150" /> |<img src="https://user-images.githubusercontent.com/36154121/223014227-defea413-c79a-495c-9cb4-cfe5beffcee2.jpg" width="150" /> |<img src="https://user-images.githubusercontent.com/36154121/223014239-efdec969-89af-43e9-a77e-19da345008ca.jpg" width="150" /> 

Device: https://www.waveshare.com/wiki/1.3inch_OLED_HAT  

### Usage

```bash
cargo add ws-oled-driver
```
-- or --

```toml
[dependencies]
ws-oled-driver = "0.0.2"
```

### Examples

#### Display

```rust
use ws_oled_driver::Device;  /* HAT Device */
use ws_oled_driver::gfx;     /* Graphics */
use ws_oled_driver::joystick;
use std::thread::sleep;
use std::time::Duration;

fn main() {
   let mut device = Device::new();
   device.initialize_components();

   /* FILL DISPLAY */
   gfx::fill(&mut device.display, 0xFF);
   device.display.render()?; 
}
```

#### Joystick

```rust
use ws_oled_driver::Device;  /* HAT Device */
use ws_oled_driver::joystick;

fn main() {
  let mut device = Device::new();
  device.initialize_components();

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

```

#### Buttons

```rust
use ws_oled_driver::Device;  /* HAT Device */
use button::State;

fn main() {
  let mut device = Device::new();
  device.initialize_components();

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

```


