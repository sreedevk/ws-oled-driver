use ws_oled_driver::Device;
use ws_oled_driver::gfx;
use anyhow::Result;

fn main() -> Result<()> {
    let mut device = Device::new()?;
    device.initialize_components()?;
    
    gfx::fill(&mut device.display, 0xFF);
    device.display.render()?;

    Ok(())
}
