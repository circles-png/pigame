use std::{fs::read, path::Path};

pub use fontdue::{Font, FontSettings};

use crate::{context::get, error::Error};

use super::colour::{Colour, WHITE};

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
pub fn draw_text_ex(text: &str, x: u32, y: u32, params: &Properties) {
}

/// Properties for drawing text.
#[derive(Debug, Clone, Copy)]
pub struct Properties {
    /// The font to use; index of internal font list.
    pub font: usize,
    /// The scale of the text.
    pub scale: f32,
    /// The rotation of the text in radians.
    pub rotation: f32,
    /// The colour of the text.
    pub colour: Colour,
}

impl Default for Properties {
    /// Returns the "default properties".
    ///
    /// It uses the Quinque Five font, a scale of 1, white colour, and no rotation.
    fn default() -> Self {
        Self {
            font: {
                let settings = FontSettings {
                    scale: 40.,
                    ..Default::default()
                };
                let fonts = &mut get().fonts;
                fonts.push(unsafe {
                    Font::from_bytes(include_bytes!("Quinque Five Font.ttf").as_slice(), settings)
                        .map_err(Error::Font)
                        .unwrap_unchecked()
                });
                fonts.len() - 1
            },
            scale: 1.,
            colour: WHITE,
            rotation: 0.0,
        }
    }
}
