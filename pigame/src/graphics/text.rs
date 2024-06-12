use std::{fs::read, path::Path};

pub use fontdue::{Font, FontSettings};

use crate::{context::get, error::Error};

use super::colour::Colour;

/// Load a ttf font and return the index of the font in the internal font list.
///
/// # Errors
///
/// If the font file cannot be read or the font cannot be loaded, an error is returned.
pub fn load_ttf_font<P: AsRef<Path>>(path: P, settings: FontSettings) -> Result<usize, Error> {
    let fonts = &mut get().fonts;
    fonts.push(Font::from_bytes(read(path)?, settings).map_err(Error::Font)?);
    Ok(fonts.len() - 1)
}

/// Draw text to the screen at the specified position.
#[allow(unused_variables)]
pub fn draw_text_ex(text: &str, x: u32, y: u32, font: usize, size: f32, colour: Colour) {
    let font = &get().fonts[font];
    let frame_buffer = &mut get().frame_buffer;
    for char in text.chars() {
        let (metrics, raster) = font.rasterize(char, font.scale_factor(size));
        let rows = raster.chunks_exact(metrics.width);
        for (dy, row) in rows.enumerate() {
            for (dx, pixel) in row.iter().enumerate() {
                let start = ((y as usize + dy) * frame_buffer.variable_info.xres as usize
                    + x as usize
                    + dx)
                    * 4;
                let slice = frame_buffer.buffer.get_mut(start..start + 4);
                if let Some(slice) = slice {
                    slice.copy_from_slice(&(colour * ((f32::from(*pixel)) / 255.)).to_bgra_bytes());
                } else {
                    break;
                }
            }
        }
    }
}
