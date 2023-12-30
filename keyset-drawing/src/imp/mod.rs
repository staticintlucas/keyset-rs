mod key;
mod legend;

use std::collections::HashSet;

use ::key::Key;
use ::key::Shape as KeyShape;
use color::Color;
use geom::{BezPath, Point, Shape, Vec2};

use crate::Options;

// TODO move this somewhere?
const ARC_TOL: f64 = 1.; // Tolerance for converting Arc->Bézier with Kurbo

#[derive(Debug, Clone, Copy)]
pub struct Outline {
    pub color: Color,
    pub width: f64,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub data: BezPath,
    pub outline: Option<Outline>,
    pub fill: Option<Color>,
}

#[derive(Debug, Clone)]
pub struct KeyDrawing {
    pub origin: Point,
    pub paths: Vec<Path>,
}

impl KeyDrawing {
    pub fn new(key: &Key, options: &Options) -> Self {
        let show_key = options.show_keys && !matches!(key.shape, KeyShape::None(..));

        let bottom = show_key.then(|| key::bottom(key, options));
        let top = show_key.then(|| key::top(key, options));
        let step = show_key.then(|| key::step(key, options)).flatten();
        let homing = show_key.then(|| key::homing(key, options)).flatten();

        let top_rect = options.profile.top_with_rect(key.shape.inner_rect()).rect();

        let margin = options.show_margin.then(|| {
            // TODO get unique margins, not size_idx's. Currently impossible because Insets: !Hash
            let sizes: HashSet<_> = key.legends.iter().flatten().map(|l| l.size_idx).collect();
            let path = sizes
                .into_iter()
                .map(|s| (top_rect + options.profile.text_margin.get(s)).into_path(ARC_TOL))
                .fold(BezPath::new(), |mut p, r| {
                    p.extend(r);
                    p
                });

            Path {
                data: path,
                outline: Some(Outline {
                    color: Color::new(1.0, 0.0, 0.0),
                    width: 5.,
                }),
                fill: None,
            }
        });

        let legends = key.legends.iter().enumerate().filter_map(|(i, l)| {
            l.as_ref().map(|legend| {
                #[allow(clippy::cast_precision_loss)]
                let align = Vec2::new(((i % 3) as f64) / 2.0, ((i / 3) as f64) / 2.0);
                legend::draw(legend, options.font, options.profile, top_rect, align)
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

    use super::*;

    #[test]
    fn test_key_drawing_new() {
        // Regular 1u
        let key = Key::example();
        let options = Options::default();
        let drawing = KeyDrawing::new(&key, &options);

        assert_eq!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 6); // top, bottom, 4x legends

        // Stepped caps
        let key = {
            let mut key = Key::example();
            key.shape = ::key::Shape::SteppedCaps;
            key
        };
        let options = Options::default();
        let drawing = KeyDrawing::new(&key, &options);

        assert_eq!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, step, 4x legends

        // ISO H
        let key = {
            let mut key = Key::example();
            key.shape = ::key::Shape::IsoHorizontal;
            key
        };
        let options = Options {
            show_margin: true,
            ..Options::default()
        };
        let drawing = KeyDrawing::new(&key, &options);

        assert_eq!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, margin, 4x legends
        let bounding_box = drawing.paths[2].data.bounding_box();
        let font_size = key.legends[0].as_ref().unwrap().size_idx;
        let margin_rect = options.profile.top_with_size((1.5, 1.0)).rect()
            + options.profile.text_margin.get(font_size);
        assert_approx_eq!(bounding_box.size().width, margin_rect.size().width);
        assert_approx_eq!(bounding_box.size().height, margin_rect.size().height);

        // ISO V
        let key = {
            let mut key = Key::example();
            key.shape = ::key::Shape::IsoVertical;
            key
        };
        let options = Options {
            show_margin: true,
            ..Options::default()
        };
        let drawing = KeyDrawing::new(&key, &options);

        assert_eq!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, margin, 4x legends
        let bounding_box = drawing.paths[2].data.bounding_box();
        let font_size = key.legends[0].as_ref().unwrap().size_idx;
        let margin_rect = options.profile.top_with_size((1.25, 2.0)).rect()
            + options.profile.text_margin.get(font_size);
        assert_approx_eq!(bounding_box.size().width, margin_rect.size().width);
        assert_approx_eq!(bounding_box.size().height, margin_rect.size().height);
    }
}
