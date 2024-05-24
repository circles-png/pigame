use std::{fs::read, path::Path};

pub use fontdue::{Font, FontSettings};

use crate::{context::get, error::Error};

use super::colour::{Colour, WHITE};

pub fn load_ttf_font<P: AsRef<Path>>(path: P, settings: FontSettings) -> Result<usize, Error> {
    let fonts = &mut get().fonts;
    fonts.push(Font::from_bytes(read(path)?, settings).map_err(Error::Font)?);
    Ok(fonts.len() - 1)
}

pub fn draw_text_ex(text: &str, mut x: u32, y: u32, params: &Params) {
    // let font = params.font;
    // let pixels = params.scale * font.units_per_em();
    // let frame_buffer = &mut get_context().frame_buffer;
    // #[allow(clippy::cast_possible_truncation)]
    // for char in text.chars() {
    //     let (metrics, raster_serial) = font.rasterize(char, pixels);
    //     let raster_serial: Vec<_> = raster_serial
    //         .iter()
    //         .map(|coverage| {
    //             #[allow(clippy::cast_lossless)]
    //             let coverage = *coverage as f32 / 255.;
    //             (params.colour * coverage).to_bgra_bytes()
    //         })
    //         .collect();
    //     let rows = raster_serial
    //         .chunks_exact(metrics.width)
    //         .step_by(128)
    //         .map(|row| row.iter().copied().step_by(128).flatten());
    //     for (dy, row) in rows.enumerate() {
    //         let start = ((y + dy as u32) * (frame_buffer.variable_info.xres + 10) + x) as usize * 4;
    //         frame_buffer.buffer[start..start + (4 * metrics.width / 128)]
    //             .copy_from_slice(&row.collect::<Vec<_>>());
    //     }
    //     x += metrics.width as u32;
    // }
}

pub struct Params {
    pub font: usize,
    pub scale: f32,
    pub rotation: f32,
    pub colour: Colour,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            font: {
                let settings = FontSettings {
                    scale: 40.,
                    ..Default::default()
                };
                let fonts = &mut get().fonts;
                fonts.push(
                    Font::from_bytes(include_bytes!("Quinque Five Font.ttf").as_slice(), settings)
                        .map_err(Error::Font)
                        .unwrap(),
                );
                fonts.len() - 1
            },
            scale: 1.,
            colour: WHITE,
            rotation: 0.0,
        }
    }
}
