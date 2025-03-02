use tiny_skia::Color as SkiaColor;

use super::Color;

impl From<Color> for SkiaColor {
    #[inline]
    fn from(value: Color) -> Self {
        // Use set_* here rather than Self::from_rgba to avoid clippy::fallible_impl_from
        let mut result = Self::BLACK;
        result.set_red(value.r());
        result.set_green(value.g());
        result.set_blue(value.b());
        result
    }
}

impl From<SkiaColor> for Color {
    #[inline]
    fn from(value: SkiaColor) -> Self {
        let (r, g, b) = (value.red(), value.green(), value.blue());
        Self::new(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn from_skia() {
        let skia = SkiaColor::from_rgba(0.2, 0.4, 0.6, 1.0).unwrap();
        let color = Color::from(skia);

        assert_is_close!(color.0[0], 0.2);
        assert_is_close!(color.0[1], 0.4);
        assert_is_close!(color.0[2], 0.6);
    }

    #[test]
    fn into_rgbf32() {
        let color = Color::new(0.2, 0.4, 0.6);
        let skia: SkiaColor = color.into();

        assert_is_close!(skia.red(), 0.2);
        assert_is_close!(skia.green(), 0.4);
        assert_is_close!(skia.blue(), 0.6);
        assert_is_close!(skia.alpha(), 1.0);
    }
}
