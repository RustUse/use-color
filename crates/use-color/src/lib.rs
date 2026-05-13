#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::{error::Error, fmt};

pub mod prelude;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    #[must_use]
    pub const fn is_grayscale(self) -> bool {
        self.red == self.green && self.green == self.blue
    }

    #[must_use]
    pub fn to_hex_rgb(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    #[must_use]
    pub fn relative_luminance(self) -> f64 {
        0.2126 * srgb_channel_to_linear(self.red)
            + 0.7152 * srgb_channel_to_linear(self.green)
            + 0.0722 * srgb_channel_to_linear(self.blue)
    }
}

pub const BLACK: Rgb = Rgb::new(0, 0, 0);
pub const WHITE: Rgb = Rgb::new(255, 255, 255);
pub const RED: Rgb = Rgb::new(255, 0, 0);
pub const GREEN: Rgb = Rgb::new(0, 255, 0);
pub const BLUE: Rgb = Rgb::new(0, 0, 255);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HexColorError {
    InvalidLength(usize),
    InvalidCharacter,
}

impl fmt::Display for HexColorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(length) => {
                write!(formatter, "hex color must contain 6 digits, got {length}")
            },
            Self::InvalidCharacter => formatter.write_str("hex color contains invalid digits"),
        }
    }
}

impl Error for HexColorError {}

pub fn parse_hex_rgb(input: &str) -> Result<Rgb, HexColorError> {
    let value = input.strip_prefix('#').unwrap_or(input);

    if value.len() != 6 {
        return Err(HexColorError::InvalidLength(value.len()));
    }

    let red = parse_hex_pair(&value[0..2])?;
    let green = parse_hex_pair(&value[2..4])?;
    let blue = parse_hex_pair(&value[4..6])?;

    Ok(Rgb::new(red, green, blue))
}

fn parse_hex_pair(pair: &str) -> Result<u8, HexColorError> {
    u8::from_str_radix(pair, 16).map_err(|_| HexColorError::InvalidCharacter)
}

fn srgb_channel_to_linear(channel: u8) -> f64 {
    let srgb = f64::from(channel) / 255.0;

    if srgb <= 0.04045 {
        srgb / 12.92
    } else {
        ((srgb + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use super::{BLACK, HexColorError, Rgb, WHITE, parse_hex_rgb};

    #[test]
    fn parses_hex_colors() {
        let color = parse_hex_rgb("#3366CC").expect("hex color should parse");

        assert_eq!(color, Rgb::new(0x33, 0x66, 0xCC));
        assert_eq!(color.to_hex_rgb(), "#3366CC");
    }

    #[test]
    fn rejects_invalid_hex_colors() {
        assert_eq!(parse_hex_rgb("#FFF"), Err(HexColorError::InvalidLength(3)));
        assert_eq!(
            parse_hex_rgb("#GG0000"),
            Err(HexColorError::InvalidCharacter)
        );
    }

    #[test]
    fn detects_grayscale_and_luminance() {
        assert!(Rgb::new(80, 80, 80).is_grayscale());
        assert!(!Rgb::new(80, 81, 80).is_grayscale());
        assert!(WHITE.relative_luminance() > BLACK.relative_luminance());
    }
}
