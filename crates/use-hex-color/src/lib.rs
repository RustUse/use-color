//! Hex color parsing and formatting.
//!
//! Accepted input forms are `#RRGGBB`, `RRGGBB`, `#RGB`, and `RGB`.
//! All parsed values are normalized to uppercase six-digit strings with a
//! leading `#`.
//!
//! # Examples
//!
//! ```rust
//! use use_hex_color::HexColor;
//! use use_rgb::Rgb;
//!
//! let color = HexColor::parse("#fff").unwrap();
//! assert_eq!(color.as_str(), "#FFFFFF");
//! assert_eq!(color.to_rgb(), Rgb::white());
//! ```

use std::error::Error;
use std::fmt;

use use_rgb::Rgb;

/// A normalized hex color string such as `#FF4500`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexColor(String);

/// An error returned when parsing a hex color fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexColorError {
    /// The input was empty.
    Empty,
    /// The trimmed input had an unsupported length.
    InvalidLength(usize),
    /// The input contained a non-hexadecimal character.
    InvalidCharacter { character: char, index: usize },
}

impl HexColor {
    /// Parses a hex color from `#RRGGBB`, `RRGGBB`, `#RGB`, or `RGB` input.
    ///
    /// The returned color is normalized to uppercase six-digit form with a
    /// leading `#`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_hex_color::HexColor;
    ///
    /// let color = HexColor::parse("fff").unwrap();
    /// assert_eq!(color.as_str(), "#FFFFFF");
    /// ```
    pub fn parse(input: &str) -> Result<Self, HexColorError> {
        if input.is_empty() {
            return Err(HexColorError::Empty);
        }

        let trimmed = input.strip_prefix('#').unwrap_or(input);
        if trimmed.is_empty() {
            return Err(HexColorError::InvalidLength(0));
        }

        for (index, character) in trimmed.char_indices() {
            if !character.is_ascii_hexdigit() {
                return Err(HexColorError::InvalidCharacter { character, index });
            }
        }

        let normalized = match trimmed.len() {
            3 => {
                let mut expanded = String::with_capacity(7);
                expanded.push('#');
                for character in trimmed.chars() {
                    expanded.push(character.to_ascii_uppercase());
                    expanded.push(character.to_ascii_uppercase());
                }
                expanded
            }
            6 => format!("#{}", trimmed.to_ascii_uppercase()),
            length => return Err(HexColorError::InvalidLength(length)),
        };

        Ok(Self(normalized))
    }

    /// Creates a normalized hex color from an [`Rgb`] value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_hex_color::HexColor;
    /// use use_rgb::Rgb;
    ///
    /// let color = HexColor::from_rgb(Rgb::new(255, 69, 0));
    /// assert_eq!(color.as_str(), "#FF4500");
    /// ```
    #[must_use]
    pub fn from_rgb(rgb: Rgb) -> Self {
        Self(format!("#{:02X}{:02X}{:02X}", rgb.r(), rgb.g(), rgb.b()))
    }

    /// Converts the hex color to an [`Rgb`] value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_hex_color::HexColor;
    /// use use_rgb::Rgb;
    ///
    /// let color = HexColor::parse("#00FF00").unwrap();
    /// assert_eq!(color.to_rgb(), Rgb::green());
    /// ```
    #[must_use]
    pub fn to_rgb(&self) -> Rgb {
        let value = &self.0[1..];
        let r = u8::from_str_radix(&value[0..2], 16).expect("normalized hex red channel");
        let g = u8::from_str_radix(&value[2..4], 16).expect("normalized hex green channel");
        let b = u8::from_str_radix(&value[4..6], 16).expect("normalized hex blue channel");
        Rgb::new(r, g, b)
    }

    /// Returns the normalized hex color string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_hex_color::HexColor;
    ///
    /// let color = HexColor::parse("#abc").unwrap();
    /// assert_eq!(color.as_str(), "#AABBCC");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for HexColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "hex color input cannot be empty"),
            Self::InvalidLength(length) => {
                write!(f, "hex color input must have 3 or 6 digits, found {length}")
            }
            Self::InvalidCharacter { character, index } => {
                write!(f, "invalid hex character '{character}' at index {index}")
            }
        }
    }
}

impl Error for HexColorError {}

#[cfg(test)]
mod tests {
    use super::{HexColor, HexColorError};
    use use_rgb::Rgb;

    #[test]
    fn parses_supported_formats() {
        assert_eq!(HexColor::parse("#FF4500").unwrap().as_str(), "#FF4500");
        assert_eq!(HexColor::parse("FF4500").unwrap().as_str(), "#FF4500");
        assert_eq!(HexColor::parse("#fff").unwrap().as_str(), "#FFFFFF");
        assert_eq!(HexColor::parse("fff").unwrap().as_str(), "#FFFFFF");
    }

    #[test]
    fn invalid_inputs_return_explicit_errors() {
        assert_eq!(HexColor::parse(""), Err(HexColorError::Empty));
        assert_eq!(HexColor::parse("#12"), Err(HexColorError::InvalidLength(2)));
        assert_eq!(
            HexColor::parse("#GGGGGG"),
            Err(HexColorError::InvalidCharacter {
                character: 'G',
                index: 0,
            })
        );
    }

    #[test]
    fn rgb_conversion_round_trips() {
        let rgb = Rgb::new(255, 69, 0);
        let hex = HexColor::from_rgb(rgb);

        assert_eq!(hex.as_str(), "#FF4500");
        assert_eq!(hex.to_string(), "#FF4500");
        assert_eq!(hex.to_rgb(), rgb);
    }
}
