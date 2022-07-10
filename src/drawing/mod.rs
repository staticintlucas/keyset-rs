mod svg;

use crate::layout::Layout;
// use crate::profile::Profile;

pub struct Drawing {
    layout: Layout,
    // profile: Profile,
    dpi: f32,
}

impl Drawing {
    #[inline]
    #[must_use]
    pub fn new(layout: Layout /* profile: Profile */) -> Self {
        Self {
            layout,
            // profile,
            dpi: 96.,
        }
    }

    #[inline]
    #[must_use]
    pub fn with_dpi(self, dpi: f32) -> Self {
        Self { dpi, ..self }
    }

    #[inline]
    pub fn set_dpi(&mut self, dpi: f32) {
        self.dpi = dpi;
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::Size;
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_drawing_new() {
        let layout = Layout {
            size: Size::new(1., 1.),
            keys: vec![],
        };
        let drawing = Drawing::new(layout);

        assert_approx_eq!(drawing.dpi, 96.);
        assert_eq!(drawing.layout.keys.len(), 0);
    }

    #[test]
    fn test_drawing_with_dpi() {
        let layout = Layout {
            size: Size::new(1., 1.),
            keys: vec![],
        };
        let drawing = Drawing::new(layout);

        assert_approx_eq!(drawing.dpi, 96.);
        assert_approx_eq!(drawing.with_dpi(144.).dpi, 144.);
    }

    #[test]
    fn test_drawing_set_dpi() {
        let layout = Layout {
            size: Size::new(1., 1.),
            keys: vec![],
        };
        let mut drawing = Drawing::new(layout);
        drawing.set_dpi(192.);

        assert_approx_eq!(drawing.dpi, 192.);
    }
}
