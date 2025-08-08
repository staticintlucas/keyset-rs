use geom::{
    Angle, ConvertFrom as _, ConvertInto as _, Dot, Ellipse, KeyUnit, Path, Point, Rect, RoundRect,
    Translate, Unit as _, Vector,
};
use profile::Profile;

use super::{KeyPath, Outline};
use crate::Template;

pub fn top(key: &key::Key, template: &Template) -> KeyPath {
    let path = match key.shape {
        key::Shape::None(..) => Path::empty(),
        key::Shape::Normal(size) | key::Shape::Space(size) => {
            round_rect_with_size(template.profile.top_rect(), size).to_path()
        }
        key::Shape::Homing(..) => template.profile.top_rect().to_path(),
        key::Shape::SteppedCaps => round_rect_with_size(
            template.profile.top_rect(),
            Vector::new(KeyUnit(1.25), KeyUnit(1.0)),
        )
        .to_path(),
        key::Shape::IsoHorizontal | key::Shape::IsoVertical => iso_top_path(&template.profile),
    };

    KeyPath {
        data: path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: template.outline_width,
        }),
    }
}

pub fn bottom(key: &key::Key, template: &Template) -> KeyPath {
    let path = match key.shape {
        key::Shape::None(..) => Path::empty(),
        key::Shape::Normal(size) | key::Shape::Space(size) => {
            round_rect_with_size(template.profile.bottom_rect(), size).to_path()
        }
        key::Shape::Homing(..) => template.profile.bottom_rect().to_path(),
        key::Shape::SteppedCaps => round_rect_with_size(
            template.profile.bottom_rect(),
            Vector::new(KeyUnit(1.75), KeyUnit(1.0)),
        )
        .to_path(),
        key::Shape::IsoHorizontal | key::Shape::IsoVertical => iso_bottom_path(&template.profile),
    };

    KeyPath {
        data: path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: template.outline_width,
        }),
    }
}

pub fn homing(key: &key::Key, template: &Template) -> Option<KeyPath> {
    let profile = &template.profile;

    let key::Shape::Homing(homing) = key.shape else {
        return None;
    };
    let homing = homing.unwrap_or(profile.homing.default);

    let center =
        rect_with_size(profile.top_rect().to_rect(), key.shape.inner_rect().size()).center();

    let bez_path = match homing {
        key::Homing::Scoop => None,
        key::Homing::Bar => Some(
            Rect::from_center_and_size(
                center + Vector::new(Dot(0.0), profile.homing.bar.y_offset),
                profile.homing.bar.size,
            )
            .to_path(),
        ),
        key::Homing::Bump => Some(
            Ellipse::from_circle(
                center + Vector::new(Dot(0.0), profile.homing.bump.y_offset),
                profile.homing.bump.diameter / 2.0,
            )
            .to_path(),
        ),
    };

    bez_path.map(|path| KeyPath {
        data: path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: template.outline_width,
        }),
    })
}

pub fn step(key: &key::Key, template: &Template) -> Option<KeyPath> {
    matches!(key.shape, key::Shape::SteppedCaps).then(|| {
        let profile = &template.profile;

        // Take average dimensions of top and bottom
        let rect = {
            let frac = 0.5;
            let top = profile.top_rect();
            let btm = profile.bottom_rect();
            RoundRect::new(
                Point::lerp(top.min, btm.min, frac),
                Point::lerp(top.max, btm.max, frac),
                Vector::lerp(top.radii, btm.radii, frac),
            )
        };

        KeyPath {
            data: step_path(rect),
            fill: Some(key.color),
            outline: Some(Outline {
                color: key.color.highlight(0.15),
                width: template.outline_width,
            }),
        }
    })
}

fn iso_bottom_path(profile: &Profile) -> Path<Dot> {
    let bottom_rect = profile.bottom_rect().to_rect();
    let rect150 = rect_with_size(bottom_rect, Vector::new(KeyUnit(1.5), KeyUnit(1.0)));
    let rect125 = rect_with_size(bottom_rect, Vector::new(KeyUnit(1.25), KeyUnit(2.0)))
        * Translate::new(KeyUnit(0.25).convert_into(), Dot(0.0));
    let radii = Vector::splat(profile.bottom.radius);

    let mut path = Path::builder();
    path.abs_move(rect150.min + Vector::new(Dot::zero(), radii.x));
    path.rel_arc(radii, Angle::ZERO, false, true, radii.neg_y());
    path.abs_horiz_line(rect150.max.x - radii.x);
    path.rel_arc(radii, Angle::ZERO, false, true, radii);
    path.abs_vert_line(rect125.max.y - radii.y);
    path.rel_arc(radii, Angle::ZERO, false, true, radii.neg_x());
    path.abs_horiz_line(rect125.min.x + radii.x);
    path.rel_arc(radii, Angle::ZERO, false, true, -radii);
    path.abs_vert_line(rect150.max.y + radii.y);
    path.rel_arc(radii, Angle::ZERO, false, false, -radii);
    path.abs_horiz_line(rect150.min.x + radii.x);
    path.rel_arc(radii, Angle::ZERO, false, true, -radii);
    path.close();

    path.build()
}

fn iso_top_path(profile: &Profile) -> Path<Dot> {
    let top_rect = profile.top_rect().to_rect();
    let rect150 = rect_with_size(top_rect, Vector::new(KeyUnit(1.5), KeyUnit(1.0)));
    let rect125 = rect_with_size(top_rect, Vector::new(KeyUnit(1.25), KeyUnit(2.0)))
        * Translate::new(KeyUnit(0.25).convert_into(), Dot(0.0));
    let radii = Vector::splat(profile.top.radius);

    let mut path = Path::builder();
    path.abs_move(rect150.min + Vector::new(Dot::zero(), radii.x));
    path.rel_arc(radii, Angle::ZERO, false, true, radii.neg_y());
    path.abs_horiz_line(rect150.max.x - radii.x);
    path.rel_arc(radii, Angle::ZERO, false, true, radii);
    path.abs_vert_line(rect125.max.y - radii.y);
    path.rel_arc(radii, Angle::ZERO, false, true, radii.neg_x());
    path.abs_horiz_line(rect125.min.x + radii.x);
    path.rel_arc(radii, Angle::ZERO, false, true, -radii);
    path.abs_vert_line(rect150.max.y + radii.y);
    path.rel_arc(radii, Angle::ZERO, false, false, -radii);
    path.abs_horiz_line(rect150.min.x + radii.x);
    path.rel_arc(radii, Angle::ZERO, false, true, -radii);
    path.close();

    path.build()
}

fn step_path(rect: RoundRect<Dot>) -> Path<Dot> {
    let radii = rect.radii;
    let rect = Rect::from_origin_and_size(
        Point::new(Dot::convert_from(KeyUnit(1.25)) - rect.min.x, rect.min.y),
        Vector::new(KeyUnit(0.5).convert_into(), rect.height()),
    );

    let mut path = Path::builder();
    path.abs_move(rect.min + Vector::new(Dot::zero(), radii.y));
    path.rel_arc(radii, Angle::ZERO, false, false, -radii);
    path.abs_horiz_line(rect.max.x - radii.x);
    path.rel_arc(radii, Angle::ZERO, false, true, radii);
    path.abs_vert_line(rect.max.y - radii.y);
    path.rel_arc(radii, Angle::ZERO, false, true, radii.neg_x());
    path.abs_horiz_line(rect.min.x - radii.x);
    path.rel_arc(radii, Angle::ZERO, false, false, radii.neg_y());
    path.close();

    path.build()
}

fn rect_with_size(rect: Rect<Dot>, size: Vector<KeyUnit>) -> Rect<Dot> {
    let Rect { min, max } = rect;
    let dmax = size - Vector::splat(KeyUnit(1.0));
    Rect::new(min, max + dmax.convert_into())
}

fn round_rect_with_size(rect: RoundRect<Dot>, size: Vector<KeyUnit>) -> RoundRect<Dot> {
    let RoundRect { min, max, radii } = rect;
    let dmax = size - Vector::splat(KeyUnit(1.0));
    RoundRect::new(min, max + dmax.convert_into(), radii)
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use key::Key;

    use super::*;

    #[test]
    fn test_top() {
        let template = Template::default();

        // Regular 1u key
        let key = Key::example();
        let path = top(&key, &template);
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);
        let top_rect = template.profile.top_rect();
        assert_is_close!(bounds, top_rect.to_rect());

        // None
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::None(Vector::splat(KeyUnit(1.0)));
            key
        };
        let path = top(&key, &template);
        let bounds = path.data.bounds;
        assert_is_close!(bounds, Rect::empty());

        // Homing
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(None);
            key
        };
        let path = top(&key, &template);
        let bounds = path.data.bounds;
        let top_rect = template.profile.top_rect();
        assert_is_close!(bounds, top_rect.to_rect());

        // Stepped caps
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::SteppedCaps;
            key
        };
        let path = top(&key, &template);
        let bounds = path.data.bounds;
        let top_rect = {
            let RoundRect { min, max, radii } = template.profile.top_rect();
            let max = max + Vector::new(KeyUnit(0.25), KeyUnit(0.0)).convert_into();
            RoundRect::new(min, max, radii)
        };
        assert_is_close!(bounds, top_rect.to_rect());

        // ISO enter
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::IsoVertical;
            key
        };
        let path = top(&key, &template);
        let bounds = path.data.bounds;
        let top_rect = {
            let RoundRect { min, max, radii } = template.profile.top_rect();
            let max = max + Vector::new(KeyUnit(0.5), KeyUnit(1.0)).convert_into();
            RoundRect::new(min, max, radii)
        };
        assert_is_close!(bounds, top_rect.to_rect());
    }

    #[test]
    fn test_bottom() {
        let template = Template::default();

        // Regular 1u key
        let key = Key::example();
        let path = bottom(&key, &template);
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);
        let bottom_rect = template.profile.bottom_rect();
        assert_is_close!(bounds, bottom_rect.to_rect());

        // None
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::None(Vector::splat(KeyUnit(1.0)));
            key
        };
        let path = bottom(&key, &template);
        let bounds = path.data.bounds;
        assert_is_close!(bounds, Rect::empty());

        // Homing
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(None);
            key
        };
        let path = bottom(&key, &template);
        let bounds = path.data.bounds;
        let bottom_rect = template.profile.bottom_rect();
        assert_is_close!(bounds, bottom_rect.to_rect());

        // Stepped caps
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::SteppedCaps;
            key
        };
        let path = bottom(&key, &template);
        let bounds = path.data.bounds;
        let bottom_rect = {
            let RoundRect { min, max, radii } = template.profile.bottom_rect();
            let max = max + Vector::new(KeyUnit(0.75), KeyUnit(0.0)).convert_into();
            RoundRect::new(min, max, radii)
        };
        assert_is_close!(bounds, bottom_rect.to_rect());

        // ISO enter
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::IsoVertical;
            key
        };
        let path = bottom(&key, &template);
        let bounds = path.data.bounds;
        let bottom_rect = {
            let RoundRect { min, max, radii } = template.profile.bottom_rect();
            let max = max + Vector::new(KeyUnit(0.5), KeyUnit(1.0)).convert_into();
            RoundRect::new(min, max, radii)
        };
        assert_is_close!(bounds, bottom_rect.to_rect());
    }

    #[test]
    fn test_homing() {
        let template = Template::default();

        // Scoop
        let scoop = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Scoop));
            key
        };

        let path = homing(&scoop, &template);
        assert!(path.is_none()); // Top is already scooped; no additional feature to draw

        // Bar
        let bar = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Bar));
            key
        };

        let path = homing(&bar, &template);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), bar.color);
        assert_is_close!(path.outline.unwrap().color, bar.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);
        let expected = Rect::from_center_and_size(
            template.profile.top_rect().center(),
            template.profile.homing.bar.size,
        ) * Translate::new(Dot(0.0), template.profile.homing.bar.y_offset);
        assert_is_close!(bounds, expected);

        // Bump
        let bump = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Bump));
            key
        };

        let path = homing(&bump, &template);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), bump.color);
        assert_is_close!(path.outline.unwrap().color, bump.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);
        let expected = Rect::from_center_and_size(
            template.profile.top_rect().center(),
            Vector::splat(template.profile.homing.bump.diameter),
        ) * Translate::new(Dot(0.0), template.profile.homing.bump.y_offset);
        assert_is_close!(bounds, expected);

        // Non-homing key
        let none = Key::example();

        let path = homing(&none, &template);
        assert!(path.is_none()); // No additional feature to draw
    }

    #[test]
    fn test_step() {
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::SteppedCaps;
            key
        };
        let template = Template::default();

        let path = step(&key, &template);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);

        let top_rect = template.profile.top_rect();
        let bottom_rect = template.profile.bottom_rect();
        let rect = RoundRect::new(
            top_rect.min.lerp(bottom_rect.min, 0.5),
            top_rect.max.lerp(bottom_rect.max, 0.5),
            (top_rect.radii + bottom_rect.radii) / 2.0,
        );
        let rect = Rect::new(
            Point::new(
                Dot::convert_from(KeyUnit(1.25)) - rect.min.x - rect.radii.x,
                rect.min.y,
            ),
            Point::new(Dot::convert_from(KeyUnit(1.75)) - rect.min.x, rect.max.y),
        );

        assert_is_close!(bounds, rect);
    }

    #[test]
    fn test_rect_with_size() {
        let rect = rect_with_size(
            Rect::from_center_and_size(Point::splat(Dot(500.0)), Vector::splat(Dot(920.0))),
            Vector::new(KeyUnit(1.5), KeyUnit(2.0)),
        );

        let exp = Rect::new(
            Point::splat(Dot(40.0)),
            Point::new(Dot(1460.0), Dot(1960.0)),
        );

        assert_is_close!(rect, exp);
    }

    #[test]
    fn test_round_rect_with_size() {
        let rect = round_rect_with_size(
            RoundRect::from_center_size_and_radii(
                Point::splat(Dot(500.0)),
                Vector::splat(Dot(920.0)),
                Vector::splat(Dot(80.0)),
            ),
            Vector::new(KeyUnit(1.5), KeyUnit(2.0)),
        );

        let exp = RoundRect::new(
            Point::splat(Dot(40.0)),
            Point::new(Dot(1460.0), Dot(1960.0)),
            Vector::splat(Dot(80.0)),
        );

        assert_is_close!(rect, exp);
    }
}
