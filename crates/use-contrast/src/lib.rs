//! Contrast ratio helpers for sRGB colors.
//!
//! Contrast ratio is computed using the WCAG formula:
//!
//! `(L1 + 0.05) / (L2 + 0.05)`
//!
//! where `L1` is the lighter relative luminance and `L2` is the darker relative
//! luminance.
//!
//! # Examples
//!
//! ```rust
//! use use_contrast::{contrast_ratio, passes_aa};
//! use use_rgb::Rgb;
//!
//! let ratio = contrast_ratio(Rgb::black(), Rgb::white());
//! assert!(ratio > 20.0);
//! assert!(passes_aa(Rgb::black(), Rgb::white()));
//! ```

use use_luminance::relative_luminance;
use use_rgb::Rgb;

const AA_THRESHOLD: f64 = 4.5;
const AA_LARGE_TEXT_THRESHOLD: f64 = 3.0;
const AAA_THRESHOLD: f64 = 7.0;
const AAA_LARGE_TEXT_THRESHOLD: f64 = 4.5;

/// Computes the WCAG-style contrast ratio between two colors.
///
/// The lighter luminance is always used as the numerator.
///
/// # Examples
///
/// ```rust
/// use use_contrast::contrast_ratio;
/// use use_rgb::Rgb;
///
/// assert_eq!(contrast_ratio(Rgb::black(), Rgb::white()), 21.0);
/// ```
#[must_use]
pub fn contrast_ratio(foreground: Rgb, background: Rgb) -> f64 {
    let foreground_luminance = relative_luminance(foreground);
    let background_luminance = relative_luminance(background);
    let lighter = foreground_luminance.max(background_luminance);
    let darker = foreground_luminance.min(background_luminance);

    (lighter + 0.05) / (darker + 0.05)
}

/// Returns `true` when the contrast ratio meets the WCAG AA threshold for
/// normal text (`4.5`).
///
/// # Examples
///
/// ```rust
/// use use_contrast::passes_aa;
/// use use_rgb::Rgb;
///
/// assert!(passes_aa(Rgb::black(), Rgb::white()));
/// ```
#[must_use]
pub fn passes_aa(foreground: Rgb, background: Rgb) -> bool {
    contrast_ratio(foreground, background) >= AA_THRESHOLD
}

/// Returns `true` when the contrast ratio meets the WCAG AA threshold for
/// large text (`3.0`).
///
/// # Examples
///
/// ```rust
/// use use_contrast::passes_aa_large_text;
/// use use_rgb::Rgb;
///
/// assert!(passes_aa_large_text(Rgb::new(119, 119, 119), Rgb::white()));
/// ```
#[must_use]
pub fn passes_aa_large_text(foreground: Rgb, background: Rgb) -> bool {
    contrast_ratio(foreground, background) >= AA_LARGE_TEXT_THRESHOLD
}

/// Returns `true` when the contrast ratio meets the WCAG AAA threshold for
/// normal text (`7.0`).
///
/// # Examples
///
/// ```rust
/// use use_contrast::passes_aaa;
/// use use_rgb::Rgb;
///
/// assert!(passes_aaa(Rgb::blue(), Rgb::white()));
/// ```
#[must_use]
pub fn passes_aaa(foreground: Rgb, background: Rgb) -> bool {
    contrast_ratio(foreground, background) >= AAA_THRESHOLD
}

/// Returns `true` when the contrast ratio meets the WCAG AAA threshold for
/// large text (`4.5`).
///
/// # Examples
///
/// ```rust
/// use use_contrast::passes_aaa_large_text;
/// use use_rgb::Rgb;
///
/// assert!(passes_aaa_large_text(Rgb::blue(), Rgb::white()));
/// ```
#[must_use]
pub fn passes_aaa_large_text(foreground: Rgb, background: Rgb) -> bool {
    contrast_ratio(foreground, background) >= AAA_LARGE_TEXT_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::{
        contrast_ratio, passes_aa, passes_aa_large_text, passes_aaa, passes_aaa_large_text,
    };
    use use_rgb::Rgb;

    const TOLERANCE: f64 = 1e-12;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= TOLERANCE,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn black_and_white_have_expected_contrast() {
        assert_close(contrast_ratio(Rgb::black(), Rgb::white()), 21.0);
        assert_close(contrast_ratio(Rgb::white(), Rgb::black()), 21.0);
    }

    #[test]
    fn threshold_helpers_match_wcag_levels() {
        let blue_on_white = (Rgb::blue(), Rgb::white());
        assert!(passes_aa(blue_on_white.0, blue_on_white.1));
        assert!(passes_aa_large_text(blue_on_white.0, blue_on_white.1));
        assert!(passes_aaa(blue_on_white.0, blue_on_white.1));
        assert!(passes_aaa_large_text(blue_on_white.0, blue_on_white.1));

        let gray_on_white = (Rgb::new(119, 119, 119), Rgb::white());
        assert!(!passes_aa(gray_on_white.0, gray_on_white.1));
        assert!(passes_aa_large_text(gray_on_white.0, gray_on_white.1));
        assert!(!passes_aaa(gray_on_white.0, gray_on_white.1));
        assert!(!passes_aaa_large_text(gray_on_white.0, gray_on_white.1));
    }
}
