//! RGB color primitives.
//!
//! This crate provides a small, dependency-free [`Rgb`] type for working with
//! red, green, and blue channel values.
//!
//! # Examples
//!
//! ```rust
//! use use_rgb::Rgb;
//!
//! let color = Rgb::new(255, 69, 0);
//! assert_eq!(color.as_tuple(), (255, 69, 0));
//! ```

/// An RGB color with 8-bit red, green, and blue channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    /// The red channel.
    pub r: u8,
    /// The green channel.
    pub g: u8,
    /// The blue channel.
    pub b: u8,
}

impl Rgb {
    /// Creates an RGB color from red, green, and blue channel values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rgb::Rgb;
    ///
    /// let color = Rgb::new(12, 34, 56);
    /// assert_eq!(color.as_tuple(), (12, 34, 56));
    /// ```
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns black (`0, 0, 0`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rgb::Rgb;
    ///
    /// assert_eq!(Rgb::black(), Rgb::new(0, 0, 0));
    /// ```
    #[must_use]
    pub const fn black() -> Self {
        Self::new(0, 0, 0)
    }

    /// Returns white (`255, 255, 255`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rgb::Rgb;
    ///
    /// assert_eq!(Rgb::white(), Rgb::new(255, 255, 255));
    /// ```
    #[must_use]
    pub const fn white() -> Self {
        Self::new(255, 255, 255)
    }

    /// Returns red (`255, 0, 0`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rgb::Rgb;
    ///
    /// assert_eq!(Rgb::red(), Rgb::new(255, 0, 0));
    /// ```
    #[must_use]
    pub const fn red() -> Self {
        Self::new(255, 0, 0)
    }

    /// Returns green (`0, 255, 0`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rgb::Rgb;
    ///
    /// assert_eq!(Rgb::green(), Rgb::new(0, 255, 0));
    /// ```
    #[must_use]
    pub const fn green() -> Self {
        Self::new(0, 255, 0)
    }

    /// Returns blue (`0, 0, 255`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rgb::Rgb;
    ///
    /// assert_eq!(Rgb::blue(), Rgb::new(0, 0, 255));
    /// ```
    #[must_use]
    pub const fn blue() -> Self {
        Self::new(0, 0, 255)
    }

    /// Returns the red channel.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rgb::Rgb;
    ///
    /// assert_eq!(Rgb::new(1, 2, 3).r(), 1);
    /// ```
    #[must_use]
    pub const fn r(&self) -> u8 {
        self.r
    }

    /// Returns the green channel.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rgb::Rgb;
    ///
    /// assert_eq!(Rgb::new(1, 2, 3).g(), 2);
    /// ```
    #[must_use]
    pub const fn g(&self) -> u8 {
        self.g
    }

    /// Returns the blue channel.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rgb::Rgb;
    ///
    /// assert_eq!(Rgb::new(1, 2, 3).b(), 3);
    /// ```
    #[must_use]
    pub const fn b(&self) -> u8 {
        self.b
    }

    /// Returns the color as an `(r, g, b)` tuple.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_rgb::Rgb;
    ///
    /// assert_eq!(Rgb::new(1, 2, 3).as_tuple(), (1, 2, 3));
    /// ```
    #[must_use]
    pub const fn as_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::Rgb;

    #[test]
    fn constructors_and_accessors_work() {
        let color = Rgb::new(12, 34, 56);

        assert_eq!(color.r(), 12);
        assert_eq!(color.g(), 34);
        assert_eq!(color.b(), 56);
        assert_eq!(color.as_tuple(), (12, 34, 56));
        assert_eq!(color.r, 12);
        assert_eq!(color.g, 34);
        assert_eq!(color.b, 56);
    }

    #[test]
    fn named_colors_match_expected_values() {
        assert_eq!(Rgb::black(), Rgb::new(0, 0, 0));
        assert_eq!(Rgb::white(), Rgb::new(255, 255, 255));
        assert_eq!(Rgb::red(), Rgb::new(255, 0, 0));
        assert_eq!(Rgb::green(), Rgb::new(0, 255, 0));
        assert_eq!(Rgb::blue(), Rgb::new(0, 0, 255));
    }
}
