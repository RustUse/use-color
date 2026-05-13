//! HSL color primitives and RGB conversion.
//!
//! Hue is expressed in degrees in the range `[0, 360)`. Saturation and
//! lightness are expressed as fractions in the inclusive range `[0.0, 1.0]`.
//!
//! # Examples
//!
//! ```rust
//! use use_hsl::Hsl;
//! use use_rgb::Rgb;
//!
//! let hsl = Hsl::from_rgb(Rgb::blue());
//! assert_eq!(hsl.to_rgb(), Rgb::blue());
//! ```

use use_rgb::Rgb;

/// An HSL color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    h: f64,
    s: f64,
    l: f64,
}

impl Hsl {
    /// Creates an HSL color when all components are finite and in range.
    ///
    /// Hue is normalized into `[0, 360)`. Saturation and lightness must both be
    /// within `0.0..=1.0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_hsl::Hsl;
    ///
    /// let color = Hsl::new(360.0, 1.0, 0.5).unwrap();
    /// assert_eq!(color.h(), 0.0);
    /// ```
    #[must_use]
    pub fn new(h: f64, s: f64, l: f64) -> Option<Self> {
        if !h.is_finite() || !s.is_finite() || !l.is_finite() {
            return None;
        }

        if !(0.0..=1.0).contains(&s) || !(0.0..=1.0).contains(&l) {
            return None;
        }

        Some(Self {
            h: h.rem_euclid(360.0),
            s,
            l,
        })
    }

    /// Converts an [`Rgb`] color into HSL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_hsl::Hsl;
    /// use use_rgb::Rgb;
    ///
    /// let color = Hsl::from_rgb(Rgb::red());
    /// assert_eq!(color.h(), 0.0);
    /// assert_eq!(color.s(), 1.0);
    /// assert_eq!(color.l(), 0.5);
    /// ```
    #[must_use]
    pub fn from_rgb(rgb: Rgb) -> Self {
        let r = f64::from(rgb.r()) / 255.0;
        let g = f64::from(rgb.g()) / 255.0;
        let b = f64::from(rgb.b()) / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;
        let delta = max - min;

        if delta.abs() < f64::EPSILON {
            return Self { h: 0.0, s: 0.0, l };
        }

        let s = delta / (1.0 - (2.0 * l - 1.0).abs());
        let h = if (max - r).abs() < f64::EPSILON {
            60.0 * ((g - b) / delta).rem_euclid(6.0)
        } else if (max - g).abs() < f64::EPSILON {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        Self { h, s, l }
    }

    /// Converts the HSL color into [`Rgb`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_hsl::Hsl;
    /// use use_rgb::Rgb;
    ///
    /// let color = Hsl::new(120.0, 1.0, 0.5).unwrap();
    /// assert_eq!(color.to_rgb(), Rgb::green());
    /// ```
    #[must_use]
    pub fn to_rgb(&self) -> Rgb {
        if self.s.abs() < f64::EPSILON {
            let channel = to_channel(self.l);
            return Rgb::new(channel, channel, channel);
        }

        let c = (1.0 - (2.0 * self.l - 1.0).abs()) * self.s;
        let h_prime = self.h / 60.0;
        let x = c * (1.0 - ((h_prime.rem_euclid(2.0)) - 1.0).abs());

        let (r1, g1, b1) = if h_prime < 1.0 {
            (c, x, 0.0)
        } else if h_prime < 2.0 {
            (x, c, 0.0)
        } else if h_prime < 3.0 {
            (0.0, c, x)
        } else if h_prime < 4.0 {
            (0.0, x, c)
        } else if h_prime < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let m = self.l - c / 2.0;
        Rgb::new(to_channel(r1 + m), to_channel(g1 + m), to_channel(b1 + m))
    }

    /// Returns the hue in degrees.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_hsl::Hsl;
    ///
    /// assert_eq!(Hsl::new(420.0, 1.0, 0.5).unwrap().h(), 60.0);
    /// ```
    #[must_use]
    pub fn h(&self) -> f64 {
        self.h
    }

    /// Returns the saturation in the range `0.0..=1.0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_hsl::Hsl;
    ///
    /// assert_eq!(Hsl::new(120.0, 0.25, 0.5).unwrap().s(), 0.25);
    /// ```
    #[must_use]
    pub fn s(&self) -> f64 {
        self.s
    }

    /// Returns the lightness in the range `0.0..=1.0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_hsl::Hsl;
    ///
    /// assert_eq!(Hsl::new(120.0, 0.25, 0.5).unwrap().l(), 0.5);
    /// ```
    #[must_use]
    pub fn l(&self) -> f64 {
        self.l
    }
}

fn to_channel(value: f64) -> u8 {
    (value * 255.0).round().clamp(0.0, 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::Hsl;
    use use_rgb::Rgb;

    const TOLERANCE: f64 = 1e-10;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= TOLERANCE,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn new_validates_and_normalizes_values() {
        let wrapped = Hsl::new(-30.0, 0.5, 0.25).unwrap();
        assert_close(wrapped.h(), 330.0);
        assert_close(wrapped.s(), 0.5);
        assert_close(wrapped.l(), 0.25);

        assert!(Hsl::new(f64::NAN, 0.5, 0.5).is_none());
        assert!(Hsl::new(f64::INFINITY, 0.5, 0.5).is_none());
        assert!(Hsl::new(0.0, -0.1, 0.5).is_none());
        assert!(Hsl::new(0.0, 0.5, 1.1).is_none());
    }

    #[test]
    fn rgb_to_hsl_matches_common_colors() {
        let black = Hsl::from_rgb(Rgb::black());
        assert_close(black.h(), 0.0);
        assert_close(black.s(), 0.0);
        assert_close(black.l(), 0.0);

        let white = Hsl::from_rgb(Rgb::white());
        assert_close(white.h(), 0.0);
        assert_close(white.s(), 0.0);
        assert_close(white.l(), 1.0);

        let red = Hsl::from_rgb(Rgb::red());
        assert_close(red.h(), 0.0);
        assert_close(red.s(), 1.0);
        assert_close(red.l(), 0.5);

        let green = Hsl::from_rgb(Rgb::green());
        assert_close(green.h(), 120.0);
        assert_close(green.s(), 1.0);
        assert_close(green.l(), 0.5);

        let blue = Hsl::from_rgb(Rgb::blue());
        assert_close(blue.h(), 240.0);
        assert_close(blue.s(), 1.0);
        assert_close(blue.l(), 0.5);
    }

    #[test]
    fn hsl_to_rgb_round_trips_common_colors() {
        let colors = [
            Rgb::black(),
            Rgb::white(),
            Rgb::red(),
            Rgb::green(),
            Rgb::blue(),
        ];

        for color in colors {
            assert_eq!(Hsl::from_rgb(color).to_rgb(), color);
        }
    }
}
