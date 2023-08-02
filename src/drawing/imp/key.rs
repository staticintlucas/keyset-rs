use std::f64::consts::{FRAC_PI_2, PI};

use kurbo::{Arc, BezPath, Circle, Point, Rect, RoundedRect, RoundedRectRadii, Shape, Size};

use crate::{
    key::Homing, key::Shape as KeyShape, key::Type as KeyType, utils::Color, DrawingOptions, Key,
};

use super::ARC_TOL;

// Quick trait to implement some funcs that are otherwise not implemented for RoundedRect in Kurbo
// and some extras I just needed
pub trait RoundedRectHelper {
    fn with_origin(self, origin: impl Into<Point>) -> Self;
    fn size(&self) -> Size;
    fn with_size(self, size: impl Into<Size>) -> Self;
    fn average(&self, other: &Self) -> Self;
}

impl RoundedRectHelper for RoundedRect {
    fn with_origin(self, origin: impl Into<Point>) -> Self {
        self.rect()
            .with_origin(origin)
            .to_rounded_rect(self.radii())
    }

    fn size(&self) -> Size {
        self.rect().size()
    }

    fn with_size(self, size: impl Into<Size>) -> Self {
        self.rect().with_size(size).to_rounded_rect(self.radii())
    }

    fn average(&self, other: &Self) -> Self {
        let origin = ((self.origin().to_vec2() + other.origin().to_vec2()) / 2.).to_point();
        let size = (self.size() + other.size()) / 2.;
        let RoundedRectRadii {
            top_left: top_left_1,
            top_right: top_right_1,
            bottom_right: bottom_right_1,
            bottom_left: bottom_left_1,
        } = self.radii();
        let RoundedRectRadii {
            top_left: top_left_2,
            top_right: top_right_2,
            bottom_right: bottom_right_2,
            bottom_left: bottom_left_2,
        } = other.radii();
        let radii = (
            (top_left_1 + top_left_2) / 2.,
            (top_right_1 + top_right_2) / 2.,
            (bottom_right_1 + bottom_right_2) / 2.,
            (bottom_left_1 + bottom_left_2) / 2.,
        );
        RoundedRect::from_origin_size(origin, size, radii)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct KeyPath {
    pub path: BezPath,
    pub fill: Color,
    pub outline: Color,
}

impl KeyPath {
    pub fn top(key: &Key, options: &DrawingOptions) -> Self {
        let top_rect = options.profile.top_rect;

        let path = match key.shape {
            KeyShape::Normal(size) => top_rect
                .with_size(top_rect.size() + 1e3 * (size - Size::new(1., 1.)))
                .to_path(ARC_TOL),
            KeyShape::SteppedCaps => top_rect
                .with_size(top_rect.size() + 1e3 * (Size::new(0.25, 0.)))
                .to_path(ARC_TOL),
            KeyShape::IsoHorizontal | KeyShape::IsoVertical => iso_top_path(top_rect),
        };

        Self {
            path,
            fill: key.color,
            outline: key.color.highlight(0.15),
        }
    }

    pub fn bottom(key: &Key, options: &DrawingOptions) -> Self {
        let bottom_rect = options.profile.bottom_rect;

        let path = match key.shape {
            KeyShape::Normal(size) => bottom_rect
                .with_size(bottom_rect.size() + 1e3 * (size - Size::new(1., 1.)))
                .to_path(ARC_TOL),
            KeyShape::SteppedCaps => bottom_rect
                .with_size(bottom_rect.size() + 1e3 * (Size::new(0.75, 0.)))
                .to_path(ARC_TOL),
            KeyShape::IsoHorizontal | KeyShape::IsoVertical => iso_bottom_path(bottom_rect),
        };

        Self {
            path,
            fill: key.color,
            outline: key.color.highlight(0.15),
        }
    }

    pub fn homing(key: &Key, options: &DrawingOptions) -> Option<Self> {
        let profile = &options.profile;

        let KeyType::Homing(homing) = key.typ else { return None };
        let homing = homing.unwrap_or(profile.homing.default);

        let center = profile
            .top_rect
            .rect()
            .with_size(profile.top_rect.size() + 1e3 * (key.shape.size() - Size::new(1., 1.)))
            .center();

        let bez_path = match homing {
            Homing::Scoop => None,
            Homing::Bar => Some(
                Rect::from_center_size(
                    center + (0., profile.homing.bar.y_offset),
                    profile.homing.bar.size,
                )
                .into_path(ARC_TOL),
            ),
            Homing::Bump => Some(
                Circle::new(
                    center + (0., profile.homing.bump.y_offset),
                    profile.homing.bump.diameter / 2.,
                )
                .into_path(ARC_TOL),
            ),
        };

        bez_path.map(|path| Self {
            path,
            fill: key.color,
            outline: key.color.highlight(0.15),
        })
    }

    pub fn step(key: &Key, options: &DrawingOptions) -> Option<Self> {
        matches!(key.shape, KeyShape::SteppedCaps).then(|| {
            let profile = &options.profile;

            // Take average dimensions of top and bottom
            let rect = profile.top_rect.average(&profile.bottom_rect);

            Self {
                path: step_path(rect),
                fill: key.color,
                outline: key.color.highlight(0.15),
            }
        })
    }
}

fn iso_bottom_path(rect: RoundedRect) -> BezPath {
    let rect150 = rect.with_size(rect.size() + Size::new(500., 0.));
    let rect125 = rect
        .with_origin(rect.origin() + (250., 1000.))
        .with_size(rect.size() + Size::new(250., 0.));

    let RoundedRectRadii {
        top_left: r_top_left,
        top_right: r_top_right,
        bottom_right: r_bottom_right,
        bottom_left: r_bottom_left,
    } = rect.radii();

    // TODO ensure Arc is transformed to single PathEl::CurveTo. Then we can avoid using
    // extend and use [PathEl].into_iter().collect() and avoid reallocations.
    let mut path = BezPath::new();
    path.move_to(rect150.origin() + (0., r_top_left));
    path.extend(
        Arc::new(
            rect150.origin() + (r_top_left, r_top_left),
            (r_top_left, r_top_left),
            PI,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (rect150.width() - r_top_right, 0.));
    path.extend(
        Arc::new(
            rect150.origin() + (rect150.width() - r_top_right, r_top_right),
            (r_top_right, r_top_right),
            -FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect125.origin() + (rect125.width(), rect125.height() - r_bottom_right));
    path.extend(
        Arc::new(
            rect125.origin()
                + (
                    rect125.width() - r_bottom_right,
                    rect125.height() - r_bottom_right,
                ),
            (r_bottom_right, r_bottom_right),
            0.,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect125.origin() + (r_bottom_left, rect125.height()));
    path.extend(
        Arc::new(
            rect125.origin() + (r_bottom_left, rect125.height() - r_bottom_left),
            (r_bottom_left, r_bottom_left),
            FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(
        Point::new(rect125.origin().x, rect150.origin().y) + (0., rect150.height() + r_top_right),
    );
    path.extend(
        Arc::new(
            Point::new(rect125.origin().x, rect150.origin().y)
                + (-r_top_right, rect150.height() + r_top_right),
            (r_top_right, r_top_right),
            0.,
            -FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (r_bottom_left, rect150.height()));
    path.extend(
        Arc::new(
            rect150.origin() + (r_bottom_left, rect150.height() - r_bottom_left),
            (r_bottom_left, r_bottom_left),
            FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (0., r_top_left));
    path.close_path();

    path
}

fn iso_top_path(rect: RoundedRect) -> BezPath {
    let rect150 = rect.with_size(rect.size() + Size::new(500., 0.));
    let rect125 = rect
        .with_origin(rect.origin() + (250., 1000.))
        .with_size(rect.size() + Size::new(250., 0.));

    let RoundedRectRadii {
        top_left: r_top_left,
        top_right: r_top_right,
        bottom_right: r_bottom_right,
        bottom_left: r_bottom_left,
    } = rect.radii();

    // TODO ensure Arc is transformed to single PathEl::CurveTo. Then we can avoid using
    // extend and use [PathEl].into_iter().collect() and avoid reallocations.
    let mut path = BezPath::new();
    path.move_to(rect150.origin() + (0., r_top_left));
    path.extend(
        Arc::new(
            rect150.origin() + (r_top_left, r_top_left),
            (r_top_left, r_top_left),
            PI,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (rect150.width() - r_top_right, 0.));
    path.extend(
        Arc::new(
            rect150.origin() + (rect150.width() - r_top_right, r_top_right),
            (r_top_right, r_top_right),
            -FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect125.origin() + (rect125.width(), rect125.height() - r_bottom_right));
    path.extend(
        Arc::new(
            rect125.origin()
                + (
                    rect125.width() - r_bottom_right,
                    rect125.height() - r_bottom_right,
                ),
            (r_bottom_right, r_bottom_right),
            0.,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect125.origin() + (r_bottom_left, rect125.height()));
    path.extend(
        Arc::new(
            rect125.origin() + (r_bottom_left, rect125.height() - r_bottom_left),
            (r_bottom_left, r_bottom_left),
            FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(
        Point::new(rect125.origin().x, rect150.origin().y) + (0., rect150.height() + r_top_right),
    );
    path.extend(
        Arc::new(
            Point::new(rect125.origin().x, rect150.origin().y)
                + (-r_top_right, rect150.height() + r_top_right),
            (r_top_right, r_top_right),
            0.,
            -FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (r_bottom_left, rect150.height()));
    path.extend(
        Arc::new(
            rect150.origin() + (r_bottom_left, rect150.height() - r_bottom_left),
            (r_bottom_left, r_bottom_left),
            FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (0., r_top_left));
    path.close_path();

    path
}

fn step_path(rect: RoundedRect) -> BezPath {
    let rect = rect
        .with_origin((1250. - rect.origin().x, rect.origin().y))
        .with_size((500., rect.height()));

    let RoundedRectRadii {
        top_left: _,
        top_right: r_top,
        bottom_right: r_bottom,
        bottom_left: _,
    } = rect.radii();

    let mut path = BezPath::new();
    path.move_to(rect.origin() + (0., r_top));
    path.extend(
        Arc::new(
            rect.origin() + (-r_top, r_top),
            (r_top, r_top),
            0.,
            -FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect.origin() + (rect.width() - r_top, 0.));
    path.extend(
        Arc::new(
            rect.origin() + (rect.width() - r_top, r_top),
            (r_top, r_top),
            -FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect.origin() + (rect.width(), rect.height() - r_bottom));
    path.extend(
        Arc::new(
            rect.origin() + (rect.width() - r_bottom, rect.height() - r_bottom),
            (r_bottom, r_bottom),
            0.,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect.origin() + (-r_bottom, rect.height()));
    path.extend(
        Arc::new(
            rect.origin() + (-r_bottom, rect.height() - r_bottom),
            (r_bottom, r_bottom),
            FRAC_PI_2,
            -FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect.origin() + (0., r_top));
    path.close_path();

    path
}
