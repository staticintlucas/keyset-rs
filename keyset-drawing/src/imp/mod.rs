mod key;
mod legend;

use std::collections::HashSet;

use ::key::Key;
use ::key::Shape as KeyShape;
use color::Color;
use geom::{Dot, Length, ToPath, Unit, Vector};
use geom::{Path, Point};
use saturate::SaturatingFrom;

use crate::Options;

#[derive(Debug, Clone, Copy)]
pub struct Outline {
    pub color: Color,
    pub width: Length<Dot>,
}

#[derive(Debug, Clone)]
pub struct KeyPath {
    pub data: Path<Dot>,
    pub outline: Option<Outline>,
    pub fill: Option<Color>,
}

#[derive(Debug, Clone)]
pub struct KeyDrawing {
    pub origin: Point<Unit>,
    pub paths: Box<[KeyPath]>,
}

impl KeyDrawing {
    pub fn new(key: &Key, options: &Options<'_>) -> Self {
        let show_key = options.show_keys && !matches!(key.shape, KeyShape::None(..));

        let bottom = show_key.then(|| key::bottom(key, options));
        let top = show_key.then(|| key::top(key, options));
        let step = show_key.then(|| key::step(key, options)).flatten();
        let homing = show_key.then(|| key::homing(key, options)).flatten();

        let top_rect = options.profile.top_with_rect(key.shape.inner_rect()).rect();

        let margin = options.show_margin.then(|| {
            // Cann't get unique margins because SideOffsets: !Hash, use unique size_idx's instead
            let sizes: HashSet<_> = key.legends.iter().flatten().map(|l| l.size_idx).collect();
            let paths: Vec<_> = sizes
                .into_iter()
                .map(|s| {
                    top_rect
                        .inner_box(options.profile.text_margin.get(s))
                        .to_path()
                })
                .collect();
            let path = Path::from_slice(&paths);

            KeyPath {
                data: path,
                outline: Some(Outline {
                    color: Color::new(1.0, 0.0, 0.0),
                    width: Length::new(5.0),
                }),
                fill: None,
            }
        });

        let legends = key.legends.iter().enumerate().filter_map(|(i, l)| {
            l.as_ref().map(|legend| {
                let align = Vector::new(
                    f32::saturating_from(i % 3) / 2.0,
                    f32::saturating_from(i / 3) / 2.0,
                );
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
    use geom::{Size, DOT_PER_UNIT};
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn test_key_drawing_new() {
        // Regular 1u
        let key = Key::example();
        let options = Options::default();
        let drawing = KeyDrawing::new(&key, &options);

        assert_is_close!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 6); // top, bottom, 4x legends

        // Stepped caps
        let key = {
            let mut key = Key::example();
            key.shape = ::key::Shape::SteppedCaps;
            key
        };
        let options = Options::default();
        let drawing = KeyDrawing::new(&key, &options);

        assert_is_close!(drawing.origin, key.position);
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

        assert_is_close!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, margin, 4x legends
        let bounding_box = drawing.paths[2].data.bounds;
        let font_size = key.legends[0].as_ref().unwrap().size_idx;
        let margin_rect = options
            .profile
            .top_with_size(Size::new(1.5, 1.0))
            .rect()
            .inner_box(options.profile.text_margin.get(font_size));
        assert_is_close!(bounding_box, margin_rect);

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

        assert_is_close!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, margin, 4x legends
        let bounding_box = drawing.paths[2].data.bounds;
        let font_size = key.legends[0].as_ref().unwrap().size_idx;
        let margin_rect = options
            .profile
            .top_with_size(Size::new(1.25, 2.0))
            .rect()
            .translate(Vector::new(0.25, 0.0) * DOT_PER_UNIT)
            .inner_box(options.profile.text_margin.get(font_size));
        assert_is_close!(bounding_box, margin_rect);
    }
}
