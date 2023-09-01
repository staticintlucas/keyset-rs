//! This crate contains the [`Color`] type implementation used internally in [keyset].
//!
//! ## *But why not use an existing crate like [`rgb`]???*
//!
//! [`rgb`] has great storage containers for colours, and many convenience methods for converting
//! between different pixel formats. Unfortunately one thing it doesn't do well is convert between
//! different component types. For example `From<RGB<f32>>` is implemented for `RGB<u8>` but the
//! conversion does not scale the component ranges.
//!
//! [Keyset] uses several different libraries internally and multiple different component types.
//! This crate is designed to scale `0.0f32..1.0f32` to `0u8..255u8` and `0u16..65535u16` as is
//! commonly expected. It also provides conversion traits to [`RGB<u8>`], [`RGB<u16>`], and
//! [`RGB<f32>`] for interoperability; and supports direct conversion to other colour types
//! used by dependencies of [keyset].
//!
//! [keyset]: https://crates.io/crates/keyset
//! [`rgb`]: https://crates.io/crates/rgb
//! [`RGB<u8>`]: ::rgb::RGB
//! [`RGB<u16>`]: ::rgb::RGB
//! [`RGB<f32>`]: ::rgb::RGB

#![warn(
    missing_docs,
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
#![allow(
    clippy::suboptimal_flops // Optimiser is pretty good, and mul_add is pretty ugly
)]

#[cfg(feature = "tiny-skia")]
mod skia;

#[cfg(feature = "rgb")]
mod rgb;

use std::fmt::{Display, LowerHex, UpperHex};

#[allow(clippy::doc_markdown)] // Otherwise it thinks sRGB need backticks
/// sRGB Color type.
///
/// Internally stores red, green, and blue components as [`f32`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color([f32; 3]); // r, g, b in that order

impl Color {
    /// Creates a new [`Color`] value with the given red, green, and blue component values.
    ///
    /// The components should be in the range `0.0..1.0` for a semantically valid colour, although
    /// this function does not perform any range checks.
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self([r, g, b])
    }

    /// Returns the red component.
    #[must_use]
    pub const fn r(&self) -> f32 {
        self.0[0]
    }

    /// Returns the green component.
    #[must_use]
    pub const fn g(&self) -> f32 {
        self.0[1]
    }

    /// Returns the blue component.
    #[must_use]
    pub const fn b(&self) -> f32 {
        self.0[2]
    }

    /// Returns an iterator over the colour's components.
    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.0.iter()
    }

    /// Returns an iterator that allows modifying the colour's components.
    ///
    /// Modified components should be in the range `0.0..1.0` for a semantically valid colour,
    /// although this function does not perform any range checks.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.0.iter_mut()
    }

    /// Applies the function `f` to each of the colour's components.
    ///
    /// The resulting components should be in the range `0.0..1.0` for a semantically valid colour,
    /// although this function does not perform any range checks.
    #[must_use]
    pub fn map(self, f: impl FnMut(f32) -> f32) -> Self {
        Self(self.0.map(f))
    }

    /// Returns a tuple containing the red, green, and blue components as [`u8`].
    #[must_use]
    pub fn as_rgb8(&self) -> (u8, u8, u8) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // We want truncation
        self.0.map(|c| (c * 256.0) as u8).into()
    }

    /// Returns a tuple containing the red, green, and blue components as [`u16`].
    #[must_use]
    pub fn as_rgb16(&self) -> (u16, u16, u16) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // We want truncation
        self.0.map(|c| (c * 65536.0) as u16).into()
    }

    /// Creates a new [`Color`] from a tuple containing the red, green, and blue
    /// components as [`u8`].
    #[must_use]
    pub fn from_rgb8((r, g, b): (u8, u8, u8)) -> Self {
        [r, g, b].map(|c| f32::from(c) / 255.0).into()
    }

    /// Creates a new [`Color`] from a tuple containing the red, green, and blue
    /// components as [`u16`].
    #[must_use]
    pub fn from_rgb16((r, g, b): (u16, u16, u16)) -> Self {
        [r, g, b].map(|c| f32::from(c) / 65535.0).into()
    }

    /// Returns a slice containing the red, green, and blue components of the colour.
    #[must_use]
    pub const fn as_slice(&self) -> &[f32] {
        &self.0
    }

    /// Returns a mutable slice containing the red, green, and blue components of the colour.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        &mut self.0
    }
}

impl IntoIterator for Color {
    type Item = f32;
    type IntoIter = std::array::IntoIter<f32, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl AsMut<[f32; 3]> for Color {
    fn as_mut(&mut self) -> &mut [f32; 3] {
        &mut self.0
    }
}

impl AsMut<[f32]> for Color {
    fn as_mut(&mut self) -> &mut [f32] {
        &mut self.0
    }
}

impl AsRef<[f32; 3]> for Color {
    fn as_ref(&self) -> &[f32; 3] {
        &self.0
    }
}

impl AsRef<[f32]> for Color {
    fn as_ref(&self) -> &[f32] {
        &self.0
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [r, g, b] = self.0.map(|c| (c * 1e3).round() / 1e3);
        write!(f, "rgb({r},{g},{b})")
    }
}

impl LowerHex for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if f.alternate() { "0x" } else { "#" };
        let (r, g, b) = self.as_rgb8();
        write!(f, "{prefix}{r:02x}{g:02x}{b:02x}")
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if f.alternate() { "0x" } else { "#" };
        let (r, g, b) = self.as_rgb8();
        write!(f, "{prefix}{r:02X}{g:02X}{b:02X}")
    }
}

impl From<[f32; 3]> for Color {
    fn from(value: [f32; 3]) -> Self {
        Self(value)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::new(r, g, b)
    }
}

impl From<Color> for [f32; 3] {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl From<Color> for (f32, f32, f32) {
    fn from(value: Color) -> Self {
        (value.r(), value.g(), value.b())
    }
}

impl Color {
    /// Lightens the colour by a given amount.
    ///
    /// `val` should be in the range `0.0..1.0` for a semantically valid factor, although this
    /// function does not perform any range checks.
    #[must_use]
    pub fn lighter(self, val: f32) -> Self {
        self.map(|c| val + c * (1.0 - val))
    }

    /// Darkens the colour by a given amount.
    ///
    /// `val` should be in the range `0.0..1.0` for a semantically valid factor, although this
    /// function does not perform any range checks.
    #[must_use]
    pub fn darker(self, val: f32) -> Self {
        self.map(|c| c * (1.0 - val))
    }

    /// Either calls [`lighter`] or [`darker`] on the colour depending on its luminance.
    ///
    /// `val` should be in the range `0.0..1.0` for a semantically valid factor, although this
    /// function does not perform any range checks.
    ///
    /// [`lighter`]: Color::lighter
    /// [`darker`]: Color::darker
    #[must_use]
    pub fn highlight(self, val: f32) -> Self {
        let (c_max, c_min) = self
            .0
            .into_iter()
            .fold((f32::NEG_INFINITY, f32::INFINITY), |(max, min), c| {
                (max.max(c), min.min(c))
            });
        let lum_x2 = c_max + c_min;

        // If (lum * 2) > (0.5 * 2)
        if lum_x2 > 1.0 {
            self.darker(val)
        } else {
            self.lighter(val)
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn new() {
        let color = Color::new(0.2, 0.4, 0.6);
        assert_eq!(color.0[0], 0.2);
        assert_eq!(color.0[1], 0.4);
        assert_eq!(color.0[2], 0.6);
    }

    #[test]
    fn r() {
        let color = Color::new(0.2, 0.4, 0.6);
        assert_eq!(color.r(), 0.2);
    }

    #[test]
    fn g() {
        let color = Color::new(0.2, 0.4, 0.6);
        assert_eq!(color.g(), 0.4);
    }

    #[test]
    fn b() {
        let color = Color::new(0.2, 0.4, 0.6);
        assert_eq!(color.b(), 0.6);
    }

    #[test]
    fn iter() {
        let color = Color::new(0.2, 0.4, 0.6);
        let mut iter = color.iter();

        assert_eq!(iter.next(), Some(&0.2));
        assert_eq!(iter.next(), Some(&0.4));
        assert_eq!(iter.next(), Some(&0.6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut color = Color::new(0.2, 0.4, 0.6);
        let iter = color.iter_mut();
        iter.for_each(|c| *c = 1.0 - *c);

        assert_approx_eq!(color.0[0], 0.8);
        assert_approx_eq!(color.0[1], 0.6);
        assert_approx_eq!(color.0[2], 0.4);
    }

    #[test]
    fn map() {
        let color = Color::new(0.2, 0.4, 0.6).map(|c| 1.0 - c);

        assert_approx_eq!(color.0[0], 0.8);
        assert_approx_eq!(color.0[1], 0.6);
        assert_approx_eq!(color.0[2], 0.4);
    }

    #[test]
    fn as_rgb8() {
        let color = Color::new(0.2, 0.4, 0.6);
        let (r, g, b) = color.as_rgb8();

        assert_eq!(r, 0x33);
        assert_eq!(g, 0x66);
        assert_eq!(b, 0x99);
    }

    #[test]
    fn as_rgb16() {
        let color = Color::new(0.2, 0.4, 0.6);
        let (r, g, b) = color.as_rgb16();

        assert_eq!(r, 0x3333);
        assert_eq!(g, 0x6666);
        assert_eq!(b, 0x9999);
    }

    #[test]
    fn from_rgb8() {
        let rgb = (0x33, 0x66, 0x99);
        let color = Color::from_rgb8(rgb);

        assert_approx_eq!(color.0[0], 0.2);
        assert_approx_eq!(color.0[1], 0.4);
        assert_approx_eq!(color.0[2], 0.6);
    }

    #[test]
    fn from_rgb16() {
        let rgb = (0x3333, 0x6666, 0x9999);
        let color = Color::from_rgb16(rgb);

        assert_approx_eq!(color.0[0], 0.2);
        assert_approx_eq!(color.0[1], 0.4);
        assert_approx_eq!(color.0[2], 0.6);
    }

    #[test]
    fn as_slice() {
        let color = Color::new(0.2, 0.4, 0.6);
        let slice = color.as_slice();

        assert_eq!(slice.len(), 3);
        assert_eq!(slice[0], 0.2);
        assert_eq!(slice[1], 0.4);
        assert_eq!(slice[2], 0.6);
    }

    #[test]
    fn as_mut_slice() {
        let mut color = Color::new(0.2, 0.4, 0.6);
        let slice = color.as_mut_slice();

        assert_eq!(slice.len(), 3);

        slice[0] = 0.8;
        slice[1] = 0.6;
        slice[2] = 0.4;

        assert_eq!(color.0[0], 0.8);
        assert_eq!(color.0[1], 0.6);
        assert_eq!(color.0[2], 0.4);
    }

    #[test]
    fn into_iter() {
        let color = Color::new(0.2, 0.4, 0.6);
        let iter = color.into_iter();
        let vec: Vec<_> = iter.collect();

        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 0.2);
        assert_eq!(vec[1], 0.4);
        assert_eq!(vec[2], 0.6);
    }

    #[test]
    fn as_mut() {
        let mut color = Color::new(0.2, 0.4, 0.6);

        {
            let array: &mut [f32; 3] = color.as_mut();
            assert_eq!(array.len(), 3);
            array[0] = 0.8;
            array[1] = 0.6;
            array[2] = 0.4;
        }

        assert_eq!(color.0[0], 0.8);
        assert_eq!(color.0[1], 0.6);
        assert_eq!(color.0[2], 0.4);

        {
            let slice: &mut [f32] = color.as_mut();
            assert_eq!(slice.len(), 3);
            slice[0] = 0.2;
            slice[1] = 0.4;
            slice[2] = 0.6;
        }

        assert_eq!(color.0[0], 0.2);
        assert_eq!(color.0[1], 0.4);
        assert_eq!(color.0[2], 0.6);
    }

    #[test]
    fn as_ref() {
        let color = Color::new(0.2, 0.4, 0.6);

        let array: &[f32; 3] = color.as_ref();

        assert_eq!(array.len(), 3);
        assert_eq!(array[0], 0.2);
        assert_eq!(array[1], 0.4);
        assert_eq!(array[2], 0.6);

        let slice: &[f32] = color.as_ref();

        assert_eq!(slice.len(), 3);
        assert_eq!(slice[0], 0.2);
        assert_eq!(slice[1], 0.4);
        assert_eq!(slice[2], 0.6);
    }

    #[test]
    fn fmt() {
        let color = Color::new(0.6, 0.8, 1.0);

        assert_eq!(format!("{color}"), "rgb(0.6,0.8,1)");
        assert_eq!(format!("{color:x}"), "#99ccff");
        assert_eq!(format!("{color:X}"), "#99CCFF");
        assert_eq!(format!("{color:#x}"), "0x99ccff");
        assert_eq!(format!("{color:#X}"), "0x99CCFF");
    }

    #[test]
    fn from_array() {
        let array = [0.2, 0.4, 0.6];
        let color = Color::from(array);

        assert_eq!(color.0[0], 0.2);
        assert_eq!(color.0[1], 0.4);
        assert_eq!(color.0[2], 0.6);
    }

    #[test]
    fn from_tuple() {
        let tuple = (0.2, 0.4, 0.6);
        let color = Color::from(tuple);

        assert_eq!(color.0[0], 0.2);
        assert_eq!(color.0[1], 0.4);
        assert_eq!(color.0[2], 0.6);
    }

    #[test]
    fn into_array() {
        let color = Color::new(0.2, 0.4, 0.6);
        let array = <[f32; 3]>::from(color);

        assert_eq!(array.len(), 3);
        assert_eq!(array[0], 0.2);
        assert_eq!(array[1], 0.4);
        assert_eq!(array[2], 0.6);
    }

    #[test]
    fn into_tuple() {
        let color = Color::new(0.2, 0.4, 0.6);
        let tuple = <(f32, f32, f32)>::from(color);

        assert_eq!(tuple.0, 0.2);
        assert_eq!(tuple.1, 0.4);
        assert_eq!(tuple.2, 0.6);
    }

    #[test]
    fn lighter() {
        let color = Color::new(0.2, 0.4, 0.6).lighter(0.5);

        assert_approx_eq!(color.0[0], 0.6);
        assert_approx_eq!(color.0[1], 0.7);
        assert_approx_eq!(color.0[2], 0.8);
    }

    #[test]
    fn darker() {
        let color = Color::new(0.6, 0.8, 1.0).darker(0.5);

        assert_approx_eq!(color.0[0], 0.3);
        assert_approx_eq!(color.0[1], 0.4);
        assert_approx_eq!(color.0[2], 0.5);
    }

    #[test]
    fn highlight() {
        let color = Color::new(0.6, 0.8, 1.0).highlight(0.5);

        assert_approx_eq!(color.0[0], 0.3);
        assert_approx_eq!(color.0[1], 0.4);
        assert_approx_eq!(color.0[2], 0.5);

        let color = Color::new(0.2, 0.4, 0.6).highlight(0.5);

        assert_approx_eq!(color.0[0], 0.6);
        assert_approx_eq!(color.0[1], 0.7);
        assert_approx_eq!(color.0[2], 0.8);
    }
}
