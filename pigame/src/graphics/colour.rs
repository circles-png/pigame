use std::{fmt::Display, ops::Mul};

/// A colour struct with red, green, and blue components
#[derive(Debug, Clone, Copy)]
pub struct Colour {
    /// Red component; 0-255 inclusive
    pub red: u8,
    /// Green component; 0-255 inclusive
    pub green: u8,
    /// Blue component; 0-255 inclusive
    pub blue: u8,
}

/// Create a new colour with the given red, green, and blue components
#[must_use]
pub const fn colour(red: u8, green: u8, blue: u8) -> Colour {
    Colour::new(red, green, blue)
}

impl Colour {
    /// Create a new colour with the given red, green, and blue components
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Convert the colour to a hex string (e.g. "#FF0000")
    #[must_use]
    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    /// Create a colour from a hex string (e.g. "#FF0000" or "FF0000")
    #[must_use]
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(colour(red, green, blue))
    }

    pub(crate) const fn to_bgra_bytes(self) -> [u8; 4] {
        [self.blue, self.green, self.red, 0]
    }
}

impl From<Colour> for u32 {
    fn from(colour: Colour) -> Self {
        Self::from(colour.red) << 16 | Self::from(colour.green) << 8 | Self::from(colour.blue)
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl From<[u8; 3]> for Colour {
    fn from([red, green, blue]: [u8; 3]) -> Self {
        colour(red, green, blue)
    }
}

impl From<Colour> for [u8; 3] {
    fn from(colour: Colour) -> Self {
        [colour.red, colour.green, colour.blue]
    }
}

impl From<Colour> for (u8, u8, u8) {
    fn from(colour: Colour) -> Self {
        (colour.red, colour.green, colour.blue)
    }
}

impl From<(u8, u8, u8)> for Colour {
    fn from((red, green, blue): (u8, u8, u8)) -> Self {
        colour(red, green, blue)
    }
}

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::cast_lossless
        )]
        colour(
            (self.red as f32 * rhs) as u8,
            (self.green as f32 * rhs) as u8,
            (self.blue as f32 * rhs) as u8,
        )
    }
}

#[allow(missing_docs)]
mod consts {
    use super::{colour, Colour};

    pub const RED: Colour = colour(255, 0, 0);
    pub const BLUE: Colour = colour(0, 0, 255);
    pub const ORANGE: Colour = colour(255, 165, 0);
    pub const GREEN: Colour = colour(0, 255, 0);
    pub const YELLOW: Colour = colour(255, 255, 0);
    pub const WHITE: Colour = colour(255, 255, 255);
    pub const BLACK: Colour = colour(0, 0, 0);
}

pub use consts::*;
