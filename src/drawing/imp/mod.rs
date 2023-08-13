mod key;
mod legend;

use itertools::Itertools;
use kurbo::{BezPath, Point, Shape, Vec2};

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

        let top_rect = match key.shape {
            KeyShape::Normal(size) => options.profile.top_with_size(size).rect(),
            KeyShape::SteppedCaps => options.profile.top_with_size((1.25, 1.)).rect(),
            KeyShape::IsoHorizontal => options.profile.top_with_size((1.5, 1.)).rect(),
            KeyShape::IsoVertical => options
                .profile
                .top_with_size((1.25, 2.))
                .with_origin(options.profile.top_rect.origin() + (250., 0.))
                .rect(),
        };

        let margin = options.show_margin.then(|| {
            let path = key
                .legends
                .iter()
                .flatten()
                .filter_map(|l| l.as_ref().map(|l| l.size))
                .unique()
                .map(|s| (top_rect + options.profile.text_margin.get(s)).into_path(ARC_TOL))
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

        let align = |row_idx, col_idx| {
            Vec2::new(
                (col_idx as f64) / ((key.legends.len() - 1) as f64),
                (row_idx as f64) / ((key.legends[0].len() - 1) as f64),
            )
        };

        let legends = key
            .legends
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter().enumerate().filter_map(move |(col_idx, legend)| {
                    legend.as_ref().map(|l| (align(row_idx, col_idx), l))
                })
            })
            .map(|(align, legend)| {
                legend::draw(legend, &options.font, &options.profile, top_rect, align)
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
