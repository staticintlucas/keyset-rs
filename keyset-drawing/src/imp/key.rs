use color::Color;
use geom::{
    Angle, ConvertFrom as _, ConvertInto as _, Dot, Ellipse, KeyUnit, Path, Point, Rect, RoundRect,
    Translate, Unit as _, Vector,
};
use profile::Profile;

use super::{KeyPath, Outline};
use crate::Template;

pub fn top(template: &Template, color: Color, size: Vector<KeyUnit>) -> KeyPath {
    let data = round_rect_with_size(template.profile.top_rect(), size).to_path();
    KeyPath {
        data,
        fill: Some(color),
        outline: Some(Outline {
            color: color.highlight(0.15),
            width: template.outline_width,
        }),
    }
}

pub fn iso_top(template: &Template, color: Color) -> KeyPath {
    let data = iso_top_path(&template.profile);
    KeyPath {
        data,
        fill: Some(color),
        outline: Some(Outline {
            color: color.highlight(0.15),
            width: template.outline_width,
        }),
    }
}

pub fn bottom(template: &Template, color: Color, size: Vector<KeyUnit>) -> KeyPath {
    let data = round_rect_with_size(template.profile.bottom_rect(), size).to_path();
    KeyPath {
        data,
        fill: Some(color),
        outline: Some(Outline {
            color: color.highlight(0.15),
            width: template.outline_width,
        }),
    }
}

pub fn iso_bottom(template: &Template, color: Color) -> KeyPath {
    let data = iso_bottom_path(&template.profile);
    KeyPath {
        data,
        fill: Some(color),
        outline: Some(Outline {
            color: color.highlight(0.15),
            width: template.outline_width,
        }),
    }
}

pub fn homing_bar(template: &Template, color: Color) -> KeyPath {
    let profile = &template.profile;

    let center = profile.top_rect().center();

    let data = Rect::from_center_and_size(
        center + Vector::new(Dot(0.0), profile.homing.bar.y_offset),
        profile.homing.bar.size,
    )
    .to_path();

    KeyPath {
        data,
        fill: Some(color),
        outline: Some(Outline {
            color: color.highlight(0.15),
            width: template.outline_width,
        }),
    }
}

pub fn homing_bump(template: &Template, color: Color) -> KeyPath {
    let profile = &template.profile;

    let center = profile.top_rect().center();

    let data = Ellipse::from_circle(
        center + Vector::new(Dot(0.0), profile.homing.bump.y_offset),
        profile.homing.bump.diameter / 2.0,
    )
    .to_path();

    KeyPath {
        data,
        fill: Some(color),
        outline: Some(Outline {
            color: color.highlight(0.15),
            width: template.outline_width,
        }),
    }
}

pub fn step(template: &Template, color: Color) -> KeyPath {
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
        fill: Some(color),
        outline: Some(Outline {
            color: color.highlight(0.15),
            width: template.outline_width,
        }),
    }
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
        let key = Key::example();

        let path = top(&template, key.color, key.shape.outer_rect().size());
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);

        let top_rect = template.profile.top_rect();
        assert_is_close!(bounds, top_rect.to_rect());
    }

    #[test]
    fn test_iso_top() {
        let template = Template::default();
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::IsoVertical;
            key
        };

        let path = iso_top(&template, key.color);
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);

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
        let key = Key::example();

        let path = bottom(&template, key.color, key.shape.outer_rect().size());
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);

        let bottom_rect = template.profile.bottom_rect();
        assert_is_close!(bounds, bottom_rect.to_rect());
    }

    #[test]
    fn test_iso_bottom() {
        let template = Template::default();
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::IsoVertical;
            key
        };

        let path = iso_bottom(&template, key.color);
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);

        let bottom_rect = {
            let RoundRect { min, max, radii } = template.profile.bottom_rect();
            let max = max + Vector::new(KeyUnit(0.5), KeyUnit(1.0)).convert_into();
            RoundRect::new(min, max, radii)
        };
        assert_is_close!(bounds, bottom_rect.to_rect());
    }

    #[test]
    fn test_homing_bar() {
        let template = Template::default();
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Bar));
            key
        };

        let path = homing_bar(&template, key.color);
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);

        let bar_rect = Rect::from_center_and_size(
            template.profile.top_rect().center()
                + Vector::new(Dot(0.0), template.profile.homing.bar.y_offset),
            template.profile.homing.bar.size,
        );
        assert_is_close!(bounds, bar_rect);
    }

    #[test]
    fn test_homing_bump() {
        let template = Template::default();
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Bump));
            key
        };

        let path = homing_bump(&template, key.color);
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);

        let bump_rect = Rect::from_center_and_size(
            template.profile.top_rect().center()
                + Vector::new(Dot(0.0), template.profile.homing.bump.y_offset),
            Vector::splat(template.profile.homing.bump.diameter),
        );
        assert_is_close!(bounds, bump_rect);
    }

    #[test]
    fn test_step() {
        let template = Template::default();
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::SteppedCaps;
            key
        };

        let path = step(&template, key.color);
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, template.outline_width);

        let step_rect = {
            let mid_rect = template
                .profile
                .top_rect()
                .lerp(template.profile.bottom_rect(), 0.5);
            Rect::new(
                Point::new(
                    Dot::convert_from(KeyUnit(1.25)) - mid_rect.min.x - mid_rect.radii.x,
                    mid_rect.min.y,
                ),
                Point::new(
                    Dot::convert_from(KeyUnit(1.75)) - mid_rect.min.x,
                    mid_rect.max.y,
                ),
            )
        };
        assert_is_close!(bounds, step_rect);
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
