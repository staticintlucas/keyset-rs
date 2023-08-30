use std::fmt::Display;

use rgb::{ComponentMap, RGB, RGB16, RGB8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(RGB16);

impl From<RGB16> for Color {
    fn from(value: RGB16) -> Self {
        Self(value)
    }
}

impl From<Color> for RGB16 {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl From<RGB8> for Color {
    fn from(value: RGB8) -> Self {
        RGB16::from(value).map(|c| c << 8 | c).into()
    }
}

impl From<Color> for RGB8 {
    fn from(value: Color) -> Self {
        RGB16::from(value).map(|c| (c >> 8) as u8)
    }
}

impl From<RGB<f32>> for Color {
    fn from(value: RGB<f32>) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        value.map(|c| (65536.0 * c) as u16).into()
    }
}

impl From<Color> for RGB<f32> {
    fn from(value: Color) -> Self {
        RGB16::from(value).map(|c| f32::from(c) / 65535.0)
    }
}

impl From<(u16, u16, u16)> for Color {
    fn from(value: (u16, u16, u16)) -> Self {
        RGB16::from(value).into()
    }
}

impl From<Color> for (u16, u16, u16) {
    fn from(value: Color) -> Self {
        RGB16::from(value).into()
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        RGB8::from(value).into()
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(value: Color) -> Self {
        RGB8::from(value).into()
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(value: (f32, f32, f32)) -> Self {
        RGB::<f32>::from(value).into()
    }
}

impl From<Color> for (f32, f32, f32) {
    fn from(value: Color) -> Self {
        RGB::<f32>::from(value).into()
    }
}

#[allow(clippy::fallible_impl_from)] // It's not really fallible
impl From<Color> for tiny_skia::Color {
    fn from(value: Color) -> Self {
        let (r, g, b) = value.into();
        Self::from_rgba(r, g, b, 1.0).unwrap()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (r, g, b) = RGB8::from(*self).into();
        write!(f, "#{r:02x}{g:02x}{b:02x}")
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self::from(RGB8::new(r, g, b))
    }

    pub fn new16(r: u16, g: u16, b: u16) -> Self {
        Self::from(RGB16::new(r, g, b))
    }

    // TODO implement this with val: u16
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn lighter(self, val: f32) -> Self {
        Self(self.0.map(|c| c + (f32::from(u16::MAX - c) * val) as u16))
    }

    // TODO implement this with val: u16
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn darker(self, val: f32) -> Self {
        Self(self.0.map(|c| (f32::from(c) * (1.0 - val)) as u16))
    }

    // TODO implement this with val: u16
    pub fn highlight(self, val: f32) -> Self {
        let c_max = self.0.iter().max().unwrap();
        let c_min = self.0.iter().min().unwrap();
        let lum_x2 = u32::from(c_max) + u32::from(c_min);

        // If (lum * 2) > (u16::MAX / 2 * 2)
        if lum_x2 > u32::from(u16::MAX) {
            self.darker(val)
        } else {
            self.lighter(val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_rgb() {
        assert_eq!(
            Color::from(RGB8::new(0x12, 0x34, 0x56)),
            Color::new16(0x1212, 0x3434, 0x5656)
        );
        assert_eq!(
            Color::from(RGB16::new(0x1234, 0x5678, 0x9abc)),
            Color::new16(0x1234, 0x5678, 0x9abc)
        );
        assert_eq!(
            Color::from(RGB::<f32>::new(0.2, 0.4, 0.6)),
            Color::new16(0x3333, 0x6666, 0x9999)
        )
    }

    #[test]
    fn test_color_into_rgb() {
        assert_eq!(
            RGB8::from(Color::new16(0x1212, 0x3434, 0x5656)),
            RGB8::new(0x12, 0x34, 0x56)
        );
        assert_eq!(
            RGB16::from(Color::new16(0x1234, 0x5678, 0x9abc)),
            RGB16::new(0x1234, 0x5678, 0x9abc)
        );
        assert_eq!(
            RGB::<f32>::from(Color::new16(0x6666, 0x9999, 0xcccc)),
            RGB::<f32>::new(0.4, 0.6, 0.8)
        );
    }

    #[test]
    fn test_color_from_tuple() {
        assert_eq!(
            Color::from((0x12_u8, 0x34, 0x56)),
            Color::new16(0x1212, 0x3434, 0x5656)
        );
        assert_eq!(
            Color::from((0x1234_u16, 0x5678, 0x9abc)),
            Color::new16(0x1234, 0x5678, 0x9abc)
        );
        assert_eq!(
            Color::from((0.0, 0.2, 0.4)),
            Color::new16(0x0000, 0x3333, 0x6666)
        );
    }

    #[test]
    fn test_color_into_tuple() {
        assert_eq!(
            <(u8, u8, u8)>::from(Color::new16(0x1212, 0x3434, 0x5656)),
            (0x12, 0x34, 0x56)
        );
        assert_eq!(
            <(u16, u16, u16)>::from(Color::new16(0x1234, 0x5678, 0x9abc)),
            (0x1234, 0x5678, 0x9abc)
        );
        assert_eq!(
            <(f32, f32, f32)>::from(Color::new16(0x9999, 0xcccc, 0xffff)),
            (0.6, 0.8, 1.0)
        );
    }

    #[test]
    fn test_skia_from_color() {
        assert_eq!(
            tiny_skia::Color::from(Color::new16(0x3333, 0x6666, 0x9999)),
            tiny_skia::Color::from_rgba(0.2, 0.4, 0.6, 1.0).unwrap()
        )
    }

    #[test]
    fn test_color_display() {
        let hex = Color::new(51, 102, 153).to_string();
        assert_eq!(hex, "#336699");

        let hex = Color::new(204, 255, 0).to_string();
        assert_eq!(hex, "#ccff00");
    }

    #[test]
    fn test_color_lighter() {
        let color: (u8, u8, u8) = Color::new(102, 51, 153).lighter(0.5).into();
        assert_eq!(color.0, 179);
        assert_eq!(color.1, 153);
        assert_eq!(color.2, 204);
    }

    #[test]
    fn test_color_darker() {
        let color: (u8, u8, u8) = Color::new(102, 51, 153).darker(0.5).into();
        assert_eq!(color.0, 51);
        assert_eq!(color.1, 25);
        assert_eq!(color.2, 76);
    }

    #[test]
    fn test_color_highlight() {
        let color1: (u8, u8, u8) = Color::new(102, 51, 153).highlight(0.5).into();
        assert_eq!(color1.0, 179);
        assert_eq!(color1.1, 153);
        assert_eq!(color1.2, 204);

        let color2: (u8, u8, u8) = Color::new(204, 102, 153).highlight(0.5).into();
        assert_eq!(color2.0, 102);
        assert_eq!(color2.1, 51);
        assert_eq!(color2.2, 76);
    }
}
