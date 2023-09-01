use super::Color;

use tiny_skia::Color as SkiaColor;

#[allow(clippy::fallible_impl_from)] // It's not really fallible due to the clamp
impl From<Color> for SkiaColor {
    fn from(value: Color) -> Self {
        let (r, g, b) = value.map(|c| c.clamp(0.0, 1.0)).into();
        Self::from_rgba(r, g, b, 1.0).unwrap()
    }
}

impl From<SkiaColor> for Color {
    fn from(value: SkiaColor) -> Self {
        let (r, g, b) = (value.red(), value.green(), value.blue());
        Self::new(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_skia() {
        let skia = SkiaColor::from_rgba(0.2, 0.4, 0.6, 1.0).unwrap();
        let color = Color::from(skia);

        assert_eq!(color.0[0], 0.2);
        assert_eq!(color.0[1], 0.4);
        assert_eq!(color.0[2], 0.6);
    }

    #[test]
    fn into_rgbf32() {
        let color = Color::new(0.2, 0.4, 0.6);
        let skia: SkiaColor = color.into();

        assert_eq!(skia.red(), 0.2);
        assert_eq!(skia.green(), 0.4);
        assert_eq!(skia.blue(), 0.6);
        assert_eq!(skia.alpha(), 1.0);
    }
}
