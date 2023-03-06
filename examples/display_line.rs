use ws_oled_driver::Device;
use ws_oled_driver::gfx;
use anyhow::Result;

fn main() -> Result<()> {
    let mut device = Device::new()?;
    device.initialize_components()?;
    
    gfx::draw_line(&mut device.display, (0, 0), (127, 63));
    device.display.render()?;

    Ok(())
}
