use std::f64::consts::{FRAC_PI_2, PI};

use geom::{Arc, BezPath, Circle, Point, Rect, RoundRect, Shape};
use profile::Profile;

use crate::Options;

use super::{Outline, Path, ARC_TOL};

pub fn top(key: &key::Key, options: &Options) -> Path {
    let path = match key.shape {
        key::Shape::None(..) => BezPath::new(),
        key::Shape::Normal(size) | key::Shape::Space(size) => {
            options.profile.top_with_size(size).to_path(ARC_TOL)
        }
        key::Shape::Homing(..) => options.profile.top_with_size((1.0, 1.0)).to_path(ARC_TOL),
        key::Shape::SteppedCaps => options.profile.top_with_size((1.25, 1.0)).to_path(ARC_TOL),
        key::Shape::IsoHorizontal | key::Shape::IsoVertical => iso_top_path(options.profile),
    };

    Path {
        data: path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: options.outline_width,
        }),
    }
}

pub fn bottom(key: &key::Key, options: &Options) -> Path {
    let path = match key.shape {
        key::Shape::None(..) => BezPath::new(),
        key::Shape::Normal(size) | key::Shape::Space(size) => {
            options.profile.bottom_with_size(size).to_path(ARC_TOL)
        }
        key::Shape::Homing(..) => options
            .profile
            .bottom_with_size((1.0, 1.0))
            .to_path(ARC_TOL),
        key::Shape::SteppedCaps => options
            .profile
            .bottom_with_size((1.75, 1.))
            .to_path(ARC_TOL),
        key::Shape::IsoHorizontal | key::Shape::IsoVertical => iso_bottom_path(options.profile),
    };

    Path {
        data: path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: options.outline_width,
        }),
    }
}

pub fn homing(key: &key::Key, options: &Options) -> Option<Path> {
    let profile = &options.profile;

    let key::Shape::Homing(homing) = key.shape else {
        return None;
    };
    let homing = homing.unwrap_or(profile.homing.default);

    let center = profile
        .top_with_size(key.shape.inner_rect().size())
        .center();

    let bez_path = match homing {
        key::Homing::Scoop => None,
        key::Homing::Bar => Some(
            Rect::from_center_size(
                center + (0., profile.homing.bar.y_offset),
                profile.homing.bar.size,
            )
            .into_path(ARC_TOL),
        ),
        key::Homing::Bump => Some(
            Circle::new(
                center + (0., profile.homing.bump.y_offset),
                profile.homing.bump.diameter / 2.,
            )
            .into_path(ARC_TOL),
        ),
    };

    bez_path.map(|path| Path {
        data: path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: options.outline_width,
        }),
    })
}

pub fn step(key: &key::Key, options: &Options) -> Option<Path> {
    matches!(key.shape, key::Shape::SteppedCaps).then(|| {
        let profile = &options.profile;

        // Take average dimensions of top and bottom
        let rect = {
            let top = profile.top_with_size((1., 1.));
            let btm = profile.bottom_with_size((1., 1.));
            RoundRect::from_origin_size(
                ((top.origin().to_vec2() + btm.origin().to_vec2()) / 2.).to_point(),
                (top.size() + btm.size()) / 2.,
                (top.radii() + btm.radii()) / 2.,
            )
        };

        Path {
            data: step_path(rect),
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
    let (rect125, radii) = {
        let rect = profile.bottom_with_size((1.25, 2.));
        (
            rect.with_origin(rect.origin() + (250., 0.)).rect(),
            rect.radii(),
        )
    };

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
    let (rect125, radii) = {
        let rect = profile.top_with_size((1.25, 2.));
        (
            rect.with_origin(rect.origin() + (250., 0.)).rect(),
            rect.radii(),
        )
    };

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

    use super::*;

    #[test]
    fn test_top() {
        let options = Options::default();

        // Regular 1u key
        let key = key::Key::example();
        let path = top(&key, &options);
        let bounds = path.data.bounding_box();

        assert_eq!(path.fill.unwrap(), key.color);
        assert_eq!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_eq!(path.outline.unwrap().width, options.outline_width);
        let top_rect = options.profile.top_with_size((1., 1.));
        assert_approx_eq!(bounds.origin().x, top_rect.origin().x);
        assert_approx_eq!(bounds.origin().y, top_rect.origin().y);
        assert_approx_eq!(bounds.size().width, top_rect.size().width);
        assert_approx_eq!(bounds.size().height, top_rect.size().height);

        // Stepped caps
        let key = {
            let mut key = key::Key::example();
            key.shape = key::Shape::SteppedCaps;
            key
        };
        let path = top(&key, &options);
        let bounds = path.data.bounding_box();
        let top_rect = options.profile.top_with_size((1.25, 1.));
        assert_approx_eq!(bounds.origin().x, top_rect.origin().x);
        assert_approx_eq!(bounds.origin().y, top_rect.origin().y);
        assert_approx_eq!(bounds.size().width, top_rect.size().width);
        assert_approx_eq!(bounds.size().height, top_rect.size().height);

        // ISO enter
        let key = {
            let mut key = key::Key::example();
            key.shape = key::Shape::IsoVertical;
            key
        };
        let path = top(&key, &options);
        let bounds = path.data.bounding_box();
        let top_rect = options.profile.top_with_size((1.5, 2.));
        assert_approx_eq!(bounds.origin().x, top_rect.origin().x);
        assert_approx_eq!(bounds.origin().y, top_rect.origin().y);
        assert_approx_eq!(bounds.size().width, top_rect.size().width);
        assert_approx_eq!(bounds.size().height, top_rect.size().height);
    }

    #[test]
    fn test_bottom() {
        let key = key::Key::example();
        let options = Options::default();

        let path = bottom(&key, &options);
        let bounds = path.data.bounding_box();

        assert_eq!(path.fill.unwrap(), key.color);
        assert_eq!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_eq!(path.outline.unwrap().width, options.outline_width);
        let bottom_rect = options.profile.bottom_with_size((1., 1.));
        assert_approx_eq!(bounds.origin().x, bottom_rect.origin().x);
        assert_approx_eq!(bounds.origin().y, bottom_rect.origin().y);
        assert_approx_eq!(bounds.size().width, bottom_rect.size().width);
        assert_approx_eq!(bounds.size().height, bottom_rect.size().height);

        // Stepped caps
        let key = {
            let mut key = key::Key::example();
            key.shape = key::Shape::SteppedCaps;
            key
        };
        let path = bottom(&key, &options);
        let bounds = path.data.bounding_box();
        let bottom_rect = options.profile.bottom_with_size((1.75, 1.));
        assert_approx_eq!(bounds.origin().x, bottom_rect.origin().x);
        assert_approx_eq!(bounds.origin().y, bottom_rect.origin().y);
        assert_approx_eq!(bounds.size().width, bottom_rect.size().width);
        assert_approx_eq!(bounds.size().height, bottom_rect.size().height);

        // ISO enter
        let key = {
            let mut key = key::Key::example();
            key.shape = key::Shape::IsoVertical;
            key
        };
        let path = bottom(&key, &options);
        let bounds = path.data.bounding_box();
        let bottom_rect = options.profile.bottom_with_size((1.5, 2.));
        assert_approx_eq!(bounds.origin().x, bottom_rect.origin().x);
        assert_approx_eq!(bounds.origin().y, bottom_rect.origin().y);
        assert_approx_eq!(bounds.size().width, bottom_rect.size().width);
        assert_approx_eq!(bounds.size().height, bottom_rect.size().height);
    }

    #[test]
    fn test_homing() {
        let options = Options::default();

        // Scoop
        let scoop = {
            let mut key = key::Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Scoop));
            key
        };

        let path = homing(&scoop, &options);
        assert!(path.is_none()); // Top is already scooped; no additional feature to draw

        // Bar
        let bar = {
            let mut key = key::Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Bar));
            key
        };

        let path = homing(&bar, &options);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.data.bounding_box();

        assert_eq!(path.fill.unwrap(), bar.color);
        assert_eq!(path.outline.unwrap().color, bar.color.highlight(0.15));
        assert_eq!(path.outline.unwrap().width, options.outline_width);
        let center = options.profile.top_with_size((1., 1.)).center()
            + (0., options.profile.homing.bar.y_offset);
        assert_approx_eq!(bounds.center().x, center.x);
        assert_approx_eq!(bounds.center().y, center.y);
        assert_approx_eq!(bounds.size().width, options.profile.homing.bar.size.width);
        assert_approx_eq!(bounds.size().height, options.profile.homing.bar.size.height);

        // Bump
        let bump = {
            let mut key = key::Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Bump));
            key
        };

        let path = homing(&bump, &options);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.data.bounding_box();

        assert_eq!(path.fill.unwrap(), bump.color);
        assert_eq!(path.outline.unwrap().color, bump.color.highlight(0.15));
        assert_eq!(path.outline.unwrap().width, options.outline_width);
        let center = options.profile.top_with_size((1., 1.)).center()
            + (0., options.profile.homing.bump.y_offset);
        assert_approx_eq!(bounds.center().x, center.x);
        assert_approx_eq!(bounds.center().y, center.y);
        assert_approx_eq!(bounds.width(), options.profile.homing.bump.diameter);
        assert_approx_eq!(bounds.height(), options.profile.homing.bump.diameter);

        // Non-homing key
        let none = key::Key::example();

        let path = homing(&none, &options);
        assert!(path.is_none()); // No additional feature to draw
    }

    #[test]
    fn test_step() {
        let key = {
            let mut key = key::Key::example();
            key.shape = key::Shape::SteppedCaps;
            key
        };
        let options = Options::default();

        let path = step(&key, &options);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.data.bounding_box();

        assert_eq!(path.fill.unwrap(), key.color);
        assert_eq!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_eq!(path.outline.unwrap().width, options.outline_width);

        let rect = RoundRect::from_origin_size(
            options
                .profile
                .top_with_size((1., 1.))
                .origin()
                .midpoint(options.profile.bottom_with_size((1., 1.)).origin()),
            (options.profile.top_with_size((1., 1.)).size()
                + options.profile.bottom_with_size((1., 1.)).size())
                / 2.,
            (options.profile.top_with_size((1., 1.)).radii()
                + options.profile.bottom_with_size((1., 1.)).radii())
                / 2.,
        );
        let rect = rect
            .with_origin((1250. - rect.origin().x - rect.radii().x, rect.origin().y))
            .with_size((500. + rect.radii().x, rect.size().height));

        assert_approx_eq!(bounds.origin().x, rect.origin().x);
        assert_approx_eq!(bounds.origin().y, rect.origin().y);
        assert_approx_eq!(bounds.size().width, rect.size().width);
        assert_approx_eq!(bounds.size().height, rect.size().height);
    }
}
