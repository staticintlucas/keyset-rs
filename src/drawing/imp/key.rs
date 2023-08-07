use std::f64::consts::{FRAC_PI_2, PI};

use kurbo::{Arc, BezPath, Circle, Point, Rect, Shape};

use crate::key::{Homing, Shape as KeyShape, Type as KeyType};
use crate::utils::RoundRect;
use crate::{DrawingOptions, Key, Profile};

use super::{Outline, Path, ARC_TOL};

pub(crate) fn top(key: &Key, options: &DrawingOptions) -> Path {
    let path = match key.shape {
        KeyShape::Normal(size) => options.profile.top_rect(size).to_path(ARC_TOL),
        KeyShape::SteppedCaps => options.profile.top_rect((1.25, 1.)).to_path(ARC_TOL),
        KeyShape::IsoHorizontal | KeyShape::IsoVertical => iso_top_path(&options.profile),
    };

    Path {
        path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: options.outline_width,
        }),
    }
}

pub(crate) fn bottom(key: &Key, options: &DrawingOptions) -> Path {
    let path = match key.shape {
        KeyShape::Normal(size) => options.profile.bottom_rect(size).to_path(ARC_TOL),
        KeyShape::SteppedCaps => options.profile.bottom_rect((1.75, 1.)).to_path(ARC_TOL),
        KeyShape::IsoHorizontal | KeyShape::IsoVertical => iso_bottom_path(&options.profile),
    };

    Path {
        path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: options.outline_width,
        }),
    }
}

pub(crate) fn homing(key: &Key, options: &DrawingOptions) -> Option<Path> {
    let profile = &options.profile;

    let KeyType::Homing(homing) = key.typ else { return None };
    let homing = homing.unwrap_or(profile.homing.default);

    let center = profile.top_rect(key.shape.size()).center();

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

    bez_path.map(|path| Path {
        path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: options.outline_width,
        }),
    })
}

pub(crate) fn step(key: &Key, options: &DrawingOptions) -> Option<Path> {
    matches!(key.shape, KeyShape::SteppedCaps).then(|| {
        let profile = &options.profile;

        // Take average dimensions of top and bottom
        let rect = RoundRect::from_origin_size(
            ((profile.top_rect.origin().to_vec2() + profile.bottom_rect.origin().to_vec2()) / 2.)
                .to_point(),
            (profile.top_rect.size() + profile.bottom_rect.size()) / 2.,
            (profile.top_rect.radii() + profile.bottom_rect.radii()) / 2.,
        );

        Path {
            path: step_path(rect),
            fill: Some(key.color),
            outline: Some(Outline {
                color: key.color.highlight(0.15),
                width: options.outline_width,
            }),
        }
    })
}

fn iso_bottom_path(profile: &Profile) -> BezPath {
    let rect150 = profile.bottom_rect((1.5, 1.)).rect();
    let rect125 = profile
        .bottom_rect((1.25, 2.))
        .with_origin(profile.bottom_rect.origin() + (250., 0.))
        .rect();
    let radii = profile.bottom_rect.radii();

    // TODO ensure Arc is transformed to single PathEl::CurveTo. Then we can avoid using
    // extend and use [PathEl].into_iter().collect() and avoid reallocations.
    let mut path = BezPath::new();
    path.move_to(rect150.origin() + (0., radii.y));
    path.extend(
        Arc::new(
            rect150.origin() + (radii.x, radii.y),
            radii,
            PI,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (rect150.width() - radii.x, 0.));
    path.extend(
        Arc::new(
            rect150.origin() + (rect150.width() - radii.x, radii.y),
            radii,
            -FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect125.origin() + (rect125.width(), rect125.height() - radii.y));
    path.extend(
        Arc::new(
            rect125.origin() + (rect125.width() - radii.x, rect125.height() - radii.y),
            radii,
            0.,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect125.origin() + (radii.x, rect125.height()));
    path.extend(
        Arc::new(
            rect125.origin() + (radii.x, rect125.height() - radii.y),
            radii,
            FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(
        Point::new(rect125.origin().x, rect150.origin().y) + (0., rect150.height() + radii.y),
    );
    path.extend(
        Arc::new(
            Point::new(rect125.origin().x, rect150.origin().y)
                + (-radii.x, rect150.height() + radii.y),
            radii,
            0.,
            -FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (radii.x, rect150.height()));
    path.extend(
        Arc::new(
            rect150.origin() + (radii.x, rect150.height() - radii.y),
            radii,
            FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (0., radii.y));
    path.close_path();

    path
}

fn iso_top_path(profile: &Profile) -> BezPath {
    let rect150 = profile.top_rect((1.5, 1.)).rect();
    let rect125 = profile
        .top_rect((1.25, 2.))
        .with_origin(profile.top_rect.origin() + (250., 0.))
        .rect();
    let radii = profile.top_rect.radii();

    // TODO ensure Arc is transformed to single PathEl::CurveTo. Then we can avoid using
    // extend and use [PathEl].into_iter().collect() and avoid reallocations.
    let mut path = BezPath::new();
    path.move_to(rect150.origin() + (0., radii.y));
    path.extend(
        Arc::new(
            rect150.origin() + (radii.x, radii.y),
            radii,
            PI,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (rect150.width() - radii.x, 0.));
    path.extend(
        Arc::new(
            rect150.origin() + (rect150.width() - radii.x, radii.y),
            radii,
            -FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect125.origin() + (rect125.width(), rect125.height() - radii.y));
    path.extend(
        Arc::new(
            rect125.origin() + (rect125.width() - radii.x, rect125.height() - radii.y),
            radii,
            0.,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect125.origin() + (radii.x, rect125.height()));
    path.extend(
        Arc::new(
            rect125.origin() + (radii.x, rect125.height() - radii.y),
            radii,
            FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(
        Point::new(rect125.origin().x, rect150.origin().y) + (0., rect150.height() + radii.y),
    );
    path.extend(
        Arc::new(
            Point::new(rect125.origin().x, rect150.origin().y)
                + (-radii.x, rect150.height() + radii.y),
            radii,
            0.,
            -FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (radii.x, rect150.height()));
    path.extend(
        Arc::new(
            rect150.origin() + (radii.x, rect150.height() - radii.y),
            radii,
            FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect150.origin() + (0., radii.y));
    path.close_path();

    path
}

fn step_path(rect: RoundRect) -> BezPath {
    let rect = rect
        .with_origin((1250. - rect.origin().x, rect.origin().y))
        .with_size((500., rect.height()));
    let radii = rect.radii();

    let mut path = BezPath::new();
    path.move_to(rect.origin() + (0., radii.y));
    path.extend(
        Arc::new(
            rect.origin() + (-radii.x, radii.y),
            radii,
            0.,
            -FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect.origin() + (rect.width() - radii.x, 0.));
    path.extend(
        Arc::new(
            rect.origin() + (rect.width() - radii.x, radii.y),
            radii,
            -FRAC_PI_2,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect.origin() + (rect.width(), rect.height() - radii.y));
    path.extend(
        Arc::new(
            rect.origin() + (rect.width() - radii.x, rect.height() - radii.y),
            radii,
            0.,
            FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect.origin() + (-radii.x, rect.height()));
    path.extend(
        Arc::new(
            rect.origin() + (-radii.x, rect.height() - radii.y),
            radii,
            FRAC_PI_2,
            -FRAC_PI_2,
            0.,
        )
        .append_iter(ARC_TOL),
    );
    path.line_to(rect.origin() + (0., radii.y));
    path.close_path();

    path
}
