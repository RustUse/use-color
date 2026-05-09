//! Relative luminance helpers for sRGB colors.
//!
//! Relative luminance is computed using the standard sRGB formula:
//!
//! `0.2126 * R + 0.7152 * G + 0.0722 * B`
//!
//! where `R`, `G`, and `B` are gamma-expanded linear channel values.
//!
//! # Examples
//!
//! ```rust
//! use use_luminance::relative_luminance;
//! use use_rgb::Rgb;
//!
//! assert_eq!(relative_luminance(Rgb::black()), 0.0);
//! ```

use use_rgb::Rgb;

const LIGHT_THRESHOLD: f64 = 0.5;

/// Computes the relative luminance of an sRGB color.
///
/// # Examples
///
/// ```rust
/// use use_luminance::relative_luminance;
/// use use_rgb::Rgb;
///
/// assert_eq!(relative_luminance(Rgb::white()), 1.0);
/// ```
#[must_use]
pub fn relative_luminance(rgb: Rgb) -> f64 {
    let r = linearize(rgb.r());
    let g = linearize(rgb.g());
    let b = linearize(rgb.b());

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Returns `true` when relative luminance is at least `0.5`.
///
/// # Examples
///
/// ```rust
/// use use_luminance::is_light;
/// use use_rgb::Rgb;
///
/// assert!(is_light(Rgb::white()));
/// ```
#[must_use]
pub fn is_light(rgb: Rgb) -> bool {
    relative_luminance(rgb) >= LIGHT_THRESHOLD
}

/// Returns `true` when relative luminance is below `0.5`.
///
/// # Examples
///
/// ```rust
/// use use_luminance::is_dark;
/// use use_rgb::Rgb;
///
/// assert!(is_dark(Rgb::black()));
/// ```
#[must_use]
pub fn is_dark(rgb: Rgb) -> bool {
    !is_light(rgb)
}

fn linearize(channel: u8) -> f64 {
    let srgb = f64::from(channel) / 255.0;
    if srgb <= 0.04045 {
        srgb / 12.92
    } else {
        ((srgb + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use super::{is_dark, is_light, relative_luminance};
    use use_rgb::Rgb;

    const TOLERANCE: f64 = 1e-12;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= TOLERANCE,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn relative_luminance_matches_black_and_white() {
        assert_close(relative_luminance(Rgb::black()), 0.0);
        assert_close(relative_luminance(Rgb::white()), 1.0);
    }

    #[test]
    fn light_and_dark_helpers_use_documented_threshold() {
        assert!(is_light(Rgb::white()));
        assert!(is_dark(Rgb::black()));
        assert!(is_dark(Rgb::new(127, 127, 127)));
    }
}
