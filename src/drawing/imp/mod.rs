mod key;

use itertools::Itertools;
use kurbo::{BezPath, Point, Rect, Shape, Size, Vec2};

use crate::key::{Shape as KeyShape, Type as KeyType};
use crate::utils::Color;
use crate::{DrawingOptions, Key};

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
        let show_key = options.show_keys && !matches!(key.typ, KeyType::None);

        let bottom = show_key.then(|| key::bottom(key, options));
        let top = show_key.then(|| key::top(key, options));
        let step = show_key.then(|| key::step(key, options)).flatten();
        let homing = show_key.then(|| key::homing(key, options)).flatten();

        let margin = options.show_margin.then(|| {
            let (offset, size) = match key.shape {
                KeyShape::Normal(size) => (Vec2::ZERO, 1e3 * (size - Size::new(1., 1.))),
                KeyShape::SteppedCaps => (Vec2::ZERO, Size::new(250., 0.)),
                KeyShape::IsoHorizontal => (Vec2::ZERO, Size::new(500., 0.)),
                KeyShape::IsoVertical => (Vec2::new(250., 0.), Size::new(250., 1000.)),
            };

            let path = key
                .legends
                .iter()
                .flatten()
                .filter_map(|l| l.as_ref().map(|l| l.size))
                .unique()
                .map(|s| options.profile.text_margin.get(s))
                .map(move |r| {
                    Rect::from_origin_size(r.origin() + offset, r.size() + size).into_path(ARC_TOL)
                })
                .fold(BezPath::new(), |mut p, r| {
                    p.extend(r);
                    p
                });

            Path {
                path,
                outline: Some(Outline {
                    color: Color::new(0xff, 0, 0),
                    width: 5.,
                }),
                fill: None,
            }
        });

        // Do a bunch of chaining here rather than using [...].iter().filter_map(|it| it). This
        // gives iterator a known size so it will allocate the required size when collecting to a
        // Vec<_>
        let paths = bottom
            .into_iter()
            .chain(top)
            .chain(step)
            .chain(homing)
            .chain(margin);

        Self {
            origin: key.position,
            paths: paths.collect(),
        }
    }
}
