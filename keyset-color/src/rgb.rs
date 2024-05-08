use super::Color;

use ::rgb::{RGB16, RGB8};
type RGBf32 = ::rgb::RGB<f32>;

impl From<RGB16> for Color {
    #[inline]
    fn from(value: RGB16) -> Self {
        Self::from_rgb16(value.into())
    }
}

impl From<Color> for RGB16 {
    #[inline]
    fn from(value: Color) -> Self {
        value.as_rgb16().into()
    }
}

impl From<RGB8> for Color {
    #[inline]
    fn from(value: RGB8) -> Self {
        Self::from_rgb8(value.into())
    }
}

impl From<Color> for RGB8 {
    #[inline]
    fn from(value: Color) -> Self {
        value.as_rgb8().into()
    }
}

impl From<RGBf32> for Color {
    #[inline]
    fn from(value: RGBf32) -> Self {
        let (r, g, b) = value.into();
        (r, g, b).into()
    }
}

impl From<Color> for RGBf32 {
    #[inline]
    fn from(value: Color) -> Self {
        let (r, g, b) = value.into();
        (r, g, b).into()
    }
}

#[cfg(test)]
mod tests {
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn from_rgb16() {
        let rgb = RGB16::new(0x3333, 0x6666, 0x9999);
        let color = Color::from(rgb);

        assert_is_close!(color.0[0], 0.2);
        assert_is_close!(color.0[1], 0.4);
        assert_is_close!(color.0[2], 0.6);
    }

    #[test]
    fn into_rgb16() {
        let color = Color::new(0.2, 0.4, 0.6);
        let rgb: RGB16 = color.into();

        assert_eq!(rgb.r, 0x3333);
        assert_eq!(rgb.g, 0x6666);
        assert_eq!(rgb.b, 0x9999);
    }

    #[test]
    fn from_rgb8() {
        let rgb = RGB8::new(0x33, 0x66, 0x99);
        let color = Color::from(rgb);

        assert_is_close!(color.0[0], 0.2);
        assert_is_close!(color.0[1], 0.4);
        assert_is_close!(color.0[2], 0.6);
    }

    #[test]
    fn into_rgb8() {
        let color = Color::new(0.2, 0.4, 0.6);
        let rgb: RGB8 = color.into();

        assert_eq!(rgb.r, 0x33);
        assert_eq!(rgb.g, 0x66);
        assert_eq!(rgb.b, 0x99);
    }

    #[test]
    fn from_rgbf32() {
        let rgb = RGBf32::new(0.2, 0.4, 0.6);
        let color = Color::from(rgb);

        assert_is_close!(color.0[0], 0.2);
        assert_is_close!(color.0[1], 0.4);
        assert_is_close!(color.0[2], 0.6);
    }

    #[test]
    fn into_rgbf32() {
        let color = Color::new(0.2, 0.4, 0.6);
        let rgb: RGBf32 = color.into();

        assert_is_close!(rgb.r, 0.2);
        assert_is_close!(rgb.g, 0.4);
        assert_is_close!(rgb.b, 0.6);
    }
}
