mod svg;

pub use self::svg::ToSvg;

use crate::layout::Layout;
use crate::profile::Profile;
use crate::Font;

pub struct DrawingOptions {
    pub dpi: f32,
    pub show_keys: bool,
    pub show_margin: bool,
}

impl Default for DrawingOptions {
    fn default() -> Self {
        Self {
            dpi: 96.,
            show_keys: true,
            show_margin: false,
        }
    }
}

pub struct Drawing {
    layout: Layout,
    profile: Profile,
    font: Font,
    options: DrawingOptions,
}

impl Drawing {
    #[inline]
    #[must_use]
    pub fn new(layout: Layout, profile: Profile, font: Font, options: DrawingOptions) -> Self {
        Self {
            layout,
            profile,
            font,
            options,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::profile::*;
    use crate::utils::*;
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_drawing_new() {
        let layout = Layout {
            size: Vec2::new(1., 1.),
            keys: vec![],
        };
        let profile = Profile::default();
        let font = Font::from_ttf(&std::fs::read("tests/fonts/demo.ttf").unwrap()).unwrap();
        let options = DrawingOptions::default();
        let drawing = Drawing::new(layout, profile, font, options);

        assert_approx_eq!(drawing.options.dpi, 96.);
        assert_eq!(drawing.layout.keys.len(), 0);
    }
}
