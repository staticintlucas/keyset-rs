use std::f64::consts::{FRAC_PI_2, PI};

use kurbo::{Arc, BezPath, Circle, Point, Rect, Shape};

use crate::key::{Homing, KeyShape, KeyType};
use crate::utils::RoundRect;
use crate::{DrawingOptions, Key, Profile};

use super::{Outline, Path, ARC_TOL};

pub(crate) fn top(key: &Key, options: &DrawingOptions) -> Path {
    let path = match key.shape {
        KeyShape::Normal(size) => options.profile.top_with_size(size).to_path(ARC_TOL),
        KeyShape::SteppedCaps => options.profile.top_with_size((1.25, 1.)).to_path(ARC_TOL),
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
        KeyShape::Normal(size) => options.profile.bottom_with_size(size).to_path(ARC_TOL),
        KeyShape::SteppedCaps => options
            .profile
            .bottom_with_size((1.75, 1.))
            .to_path(ARC_TOL),
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

    let KeyType::Homing(homing) = key.typ else {
        return None;
    };
    let homing = homing.unwrap_or(profile.homing.default);

    let center = profile.top_with_size(key.shape.size()).center();

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
    let rect150 = profile.bottom_with_size((1.5, 1.)).rect();
    let rect125 = profile
        .bottom_with_size((1.25, 2.))
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
    let rect150 = profile.top_with_size((1.5, 1.)).rect();
    let rect125 = profile
        .top_with_size((1.25, 2.))
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

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::utils::KurboAbs;

    use super::*;

    #[test]
    fn test_top() {
        let options = DrawingOptions::default();

        // Regular 1u key
        let key = Key::example();
        let path = top(&key, &options);
        let bounds = path.path.bounding_box();

        assert_eq!(path.fill.unwrap(), key.color);
        assert_eq!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_eq!(path.outline.unwrap().width, options.outline_width);
        assert_approx_eq!(bounds.origin(), options.profile.top_rect.origin());
        assert_approx_eq!(bounds.size(), options.profile.top_rect.size());

        // Stepped caps
        let key = Key {
            shape: KeyShape::SteppedCaps,
            ..Key::example()
        };
        let path = top(&key, &options);
        let bounds = path.path.bounding_box();
        assert_approx_eq!(
            bounds.origin(),
            options.profile.top_with_size((1.25, 1.)).origin()
        );
        assert_approx_eq!(
            bounds.size(),
            options.profile.top_with_size((1.25, 1.)).size()
        );

        // ISO enter
        let key = Key {
            shape: KeyShape::IsoVertical,
            ..Key::example()
        };
        let path = top(&key, &options);
        let bounds = path.path.bounding_box();
        assert_approx_eq!(
            bounds.origin(),
            options.profile.top_with_size((1.5, 2.)).origin()
        );
        assert_approx_eq!(
            bounds.size(),
            options.profile.top_with_size((1.5, 2.)).size()
        );
    }

    #[test]
    fn test_bottom() {
        let key = Key::example();
        let options = DrawingOptions::default();

        let path = bottom(&key, &options);
        let bounds = path.path.bounding_box();

        assert_eq!(path.fill.unwrap(), key.color);
        assert_eq!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_eq!(path.outline.unwrap().width, options.outline_width);
        assert_approx_eq!(bounds.origin(), options.profile.bottom_rect.origin());
        assert_approx_eq!(bounds.size(), options.profile.bottom_rect.size());

        // Stepped caps
        let key = Key {
            shape: KeyShape::SteppedCaps,
            ..Key::example()
        };
        let path = bottom(&key, &options);
        let bounds = path.path.bounding_box();
        assert_approx_eq!(
            bounds.origin(),
            options.profile.bottom_with_size((1.75, 1.)).origin()
        );
        assert_approx_eq!(
            bounds.size(),
            options.profile.bottom_with_size((1.75, 1.)).size()
        );

        // ISO enter
        let key = Key {
            shape: KeyShape::IsoVertical,
            ..Key::example()
        };
        let path = bottom(&key, &options);
        let bounds = path.path.bounding_box();
        assert_approx_eq!(
            bounds.origin(),
            options.profile.bottom_with_size((1.5, 2.)).origin()
        );
        assert_approx_eq!(
            bounds.size(),
            options.profile.bottom_with_size((1.5, 2.)).size()
        );
    }

    #[test]
    fn test_homing() {
        let options = DrawingOptions::default();

        // Scoop
        let scoop = Key {
            typ: KeyType::Homing(Some(Homing::Scoop)),
            ..Key::example()
        };

        let path = homing(&scoop, &options);
        assert!(path.is_none()); // Top is already scooped; no additional feature to draw

        // Bar
        let bar = Key {
            typ: KeyType::Homing(Some(Homing::Bar)),
            ..Key::example()
        };

        let path = homing(&bar, &options);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.path.bounding_box();

        assert_eq!(path.fill.unwrap(), bar.color);
        assert_eq!(path.outline.unwrap().color, bar.color.highlight(0.15));
        assert_eq!(path.outline.unwrap().width, options.outline_width);
        assert_approx_eq!(
            bounds.center(),
            options.profile.top_rect.center() + (0., options.profile.homing.bar.y_offset)
        );
        assert_approx_eq!(bounds.size(), options.profile.homing.bar.size);

        // Bump
        let bump = Key {
            typ: KeyType::Homing(Some(Homing::Bump)),
            ..Key::example()
        };

        let path = homing(&bump, &options);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.path.bounding_box();

        assert_eq!(path.fill.unwrap(), bump.color);
        assert_eq!(path.outline.unwrap().color, bump.color.highlight(0.15));
        assert_eq!(path.outline.unwrap().width, options.outline_width);
        assert_approx_eq!(
            bounds.center(),
            options.profile.top_rect.center() + (0., options.profile.homing.bump.y_offset)
        );
        assert_approx_eq!(bounds.width(), options.profile.homing.bump.diameter);
        assert_approx_eq!(bounds.height(), options.profile.homing.bump.diameter);

        // Non-homing key
        let none = Key::example();

        let path = homing(&none, &options);
        assert!(path.is_none()); // No additional feature to draw
    }

    #[test]
    fn test_step() {
        let key = Key {
            shape: KeyShape::SteppedCaps,
            ..Key::example()
        };
        let options = DrawingOptions::default();

        let path = step(&key, &options);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.path.bounding_box();

        assert_eq!(path.fill.unwrap(), key.color);
        assert_eq!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_eq!(path.outline.unwrap().width, options.outline_width);

        let rect = RoundRect::from_origin_size(
            options
                .profile
                .top_rect
                .origin()
                .midpoint(options.profile.bottom_rect.origin()),
            (options.profile.top_rect.size() + options.profile.bottom_rect.size()) / 2.,
            (options.profile.top_rect.radii() + options.profile.bottom_rect.radii()) / 2.,
        );
        let rect = rect
            .with_origin((1250. - rect.origin().x - rect.radii().x, rect.origin().y))
            .with_size((500. + rect.radii().x, rect.size().height));

        assert_approx_eq!(bounds.origin(), rect.origin());
        assert_approx_eq!(bounds.size(), rect.size());
    }
}
