use crate::display::Display;
use anyhow::Result;
/// Point type is used to address a pixel on the display buffer. The Point type is a `(isize, isize)` which
/// can be used to address a pixel (x, y).
type Point = (isize, isize);

/// Fill the display buffer with a value which maybe either `0x00` or `0xFF`.
pub fn fill(display: &mut Display, value: u8) {
    display.memory.iter_mut().for_each(|mem| *mem = value);
}

/// Draws a single point onto the display buffer.
pub fn draw_point(display: &mut Display, point: Point, color: u8) {
    let index = point.0 + (point.1 / 8) * display.width as isize;
    display.memory[index as usize] = color;
}

pub fn draw_line(display: &mut Display, (x1, y1): Point, (x2, y2): Point) {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();

    let mut x = x1;
    let mut y = y1;

    let x_inc = if x2 > x1 { 1 } else { -1 };
    let y_inc = if y2 > y1 { 1 } else { -1 };

    let mut error = dx - dy;

    while x != x2 || y != y2 {
        draw_point(display, (x, y), 0xFF);

        let double_error = error * 2;

        if double_error > -dy {
            error -= dy;
            x += x_inc;
        }

        if double_error < dx {
            error += dx;
            y += y_inc;
        }
    }

    draw_point(display, (x2, y2), 0xFF);
}


/// Clears the display buffer. Fills it with 0x00.
pub fn clear(display: &mut Display) -> Result<()> {
    fill(display, 0x00);

    Ok(())
}
