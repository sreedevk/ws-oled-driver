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

### Example

```rust
use ws_oled_driver::Device;  /* HAT Device */
use ws_oled_driver::gfx;     /* Graphics */

fn main() {
   let mut device = Device::new();
   device.initialize_components();
   
   
}
```


