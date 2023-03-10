mod svg;

pub use self::svg::ToSvg;

use crate::layout::Layout;
use crate::profile::Profile;

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
    options: DrawingOptions,
}

impl Drawing {
    #[inline]
    #[must_use]
    pub fn new(layout: Layout, profile: Profile, options: DrawingOptions) -> Self {
        Self {
            layout,
            profile,
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
            size: Size::new(1., 1.),
            keys: vec![],
        };
        let profile = Profile::default();
        let options = DrawingOptions::default();
        let drawing = Drawing::new(layout, profile, options);

        assert_approx_eq!(drawing.options.dpi, 96.);
        assert_eq!(drawing.layout.keys.len(), 0);
    }
}
