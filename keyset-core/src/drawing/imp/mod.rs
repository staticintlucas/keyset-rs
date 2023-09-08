mod key;
mod legend;

use ::key::Key;
use color::Color;
use itertools::Itertools;
use kurbo::{BezPath, Point, Shape, Vec2};

use crate::DrawingOptions;

// TODO move this somewhere?
const ARC_TOL: f64 = 1.; // Tolerance for converting Arc->Bézier with Kurbo

#[derive(Debug, Clone, Copy)]
pub(crate) struct Outline {
    pub color: Color,
    pub width: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct Path {
    pub path: BezPath,
    pub outline: Option<Outline>,
    pub fill: Option<Color>,
}

#[derive(Debug, Clone)]
pub(crate) struct KeyDrawing {
    pub origin: Point,
    pub paths: Vec<Path>,
}

impl KeyDrawing {
    pub fn new(key: &Key, options: &DrawingOptions) -> Self {
        let show_key = options.show_keys && !matches!(key.typ, ::key::Type::None);

        let bottom = show_key.then(|| key::bottom(key, options));
        let top = show_key.then(|| key::top(key, options));
        let step = show_key.then(|| key::step(key, options)).flatten();
        let homing = show_key.then(|| key::homing(key, options)).flatten();

        let top_rect = match key.shape {
            ::key::Shape::Normal(size) => options.profile.top_with_size(size).rect(),
            ::key::Shape::SteppedCaps => options.profile.top_with_size((1.25, 1.)).rect(),
            ::key::Shape::IsoHorizontal => options.profile.top_with_size((1.5, 1.)).rect(),
            ::key::Shape::IsoVertical => {
                let rect = options.profile.top_with_size((1.25, 2.)).rect();
                rect.with_origin(rect.origin() + (250., 0.))
            }
        };

        let margin = options.show_margin.then(|| {
            let path = key
                .legends
                .iter()
                .flatten()
                .map(|l| l.size_idx)
                .unique()
                .map(|s| (top_rect + options.profile.text_margin.get(s)).into_path(ARC_TOL))
                .fold(BezPath::new(), |mut p, r| {
                    p.extend(r);
                    p
                });

            Path {
                path,
                outline: Some(Outline {
                    color: Color::new(1.0, 0.0, 0.0),
                    width: 5.,
                }),
                fill: None,
            }
        });

        let legends = key.legends.iter().enumerate().filter_map(|(i, l)| {
            l.as_ref().map(|legend| {
                let align = Vec2::new(((i % 3) as f64) / 2.0, ((i / 3) as f64) / 2.0);
                legend::draw(legend, &options.font, &options.profile, top_rect, align)
            })
        });

        // Do a bunch of chaining here rather than using [...].iter().filter_map(|it| it). This
        // gives iterator a known size so it will allocate the required size when collecting to a
        // Vec<_>
        let paths = bottom
            .into_iter()
            .chain(top)
            .chain(step)
            .chain(homing)
            .chain(margin)
            .chain(legends);

        Self {
            origin: key.position,
            paths: paths.collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::utils::KurboAbs;

    use super::*;

    #[test]
    fn test_key_drawing_new() {
        // Regular 1u
        let key = Key::example();
        let options = DrawingOptions::default();
        let drawing = KeyDrawing::new(&key, &options);

        assert_eq!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 6); // top, bottom, 4x legends

        // Stepped caps
        let key = {
            let mut key = Key::example();
            key.shape = ::key::Shape::SteppedCaps;
            key
        };
        let options = DrawingOptions::default();
        let drawing = KeyDrawing::new(&key, &options);

        assert_eq!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, step, 4x legends

        // ISO H
        let key = {
            let mut key = Key::example();
            key.shape = ::key::Shape::IsoHorizontal;
            key
        };
        let options = DrawingOptions {
            show_margin: true,
            ..DrawingOptions::default()
        };
        let drawing = KeyDrawing::new(&key, &options);

        assert_eq!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, margin, 4x legends
        let font_size = key.legends[0].as_ref().unwrap().size_idx;
        let margin_rect = options.profile.top_with_size((1.5, 1.0)).rect()
            + options.profile.text_margin.get(font_size);
        assert_approx_eq!(
            drawing.paths[2].path.bounding_box().size(),
            margin_rect.size()
        );

        // ISO V
        let key = {
            let mut key = Key::example();
            key.shape = ::key::Shape::IsoVertical;
            key
        };
        let options = DrawingOptions {
            show_margin: true,
            ..DrawingOptions::default()
        };
        let drawing = KeyDrawing::new(&key, &options);

        assert_eq!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, margin, 4x legends
        let font_size = key.legends[0].as_ref().unwrap().size_idx;
        let margin_rect = options.profile.top_with_size((1.25, 2.0)).rect()
            + options.profile.text_margin.get(font_size);
        assert_approx_eq!(
            drawing.paths[2].path.bounding_box().size(),
            margin_rect.size()
        );
    }
}
