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
pub fn draw_point(display: &mut Display, point: Point) {
    let display_width = display.width as isize;
    let display_height = display.height as isize;

    if point.0 >= display_width || point.1 >= display_height {
        return;
    }

    let index = point.1 * display_width + point.0;
    display.memory[index as usize] = 0xFF;
}

/// Draws a line on the display buffer.
pub fn draw_line(display: &mut Display, begin: Point, end: Point) {
    let display_width = display.width as isize;
    let display_height = display.height as isize;

    if begin.0 >= display_width
        || begin.1 >= display_height
            || end.0 >= display_width
            || end.1 >= display_height
            {
                return;
            }

    let dx = if begin.0 > end.0 {
        begin.0 - end.0
    } else {
        end.0 - begin.0
    };
    let dy = if begin.1 > end.1 {
        begin.1 - end.1
    } else {
        end.1 - begin.1
    };
    let sx = if begin.0 < end.0 { 1 } else { -1 };
    let sy = if begin.1 < end.1 { 1 } else { -1 };

    let mut err = dx - dy;
    let mut x = begin.0;
    let mut y = begin.1;

    while x != end.0 || y != end.1 {
        let index = y * display_width + x;
        display.memory[index as usize] = 255;
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    // Draw the last pixel of the line.
    let index = y * display_width + x;
    display.memory[index as usize] = 255;
}

/// Clears the display buffer. Fills it with 0x00.
pub fn clear(display: &mut Display) -> Result<()> {
    fill(display, 0x00);

    Ok(())
}
