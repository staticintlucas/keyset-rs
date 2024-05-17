use geom::{
    Angle, Circle, Dot, ExtRect, ExtVec, Length, Path, Point, Rect, RoundRect, Size, ToPath,
    Vector, DOT_PER_UNIT,
};
use profile::Profile;

use crate::Options;

use super::{KeyPath, Outline};

pub fn top(key: &key::Key, options: &Options<'_>) -> KeyPath {
    let path = match key.shape {
        key::Shape::None(..) => Path::empty(),
        key::Shape::Normal(size) | key::Shape::Space(size) => {
            options.profile.top_with_size(size).to_path()
        }
        key::Shape::Homing(..) => options.profile.top_with_size(Size::new(1.0, 1.0)).to_path(),
        key::Shape::SteppedCaps => options
            .profile
            .top_with_size(Size::new(1.25, 1.0))
            .to_path(),
        key::Shape::IsoHorizontal | key::Shape::IsoVertical => iso_top_path(options.profile),
    };

    KeyPath {
        data: path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: options.outline_width,
        }),
    }
}

pub fn bottom(key: &key::Key, options: &Options<'_>) -> KeyPath {
    let path = match key.shape {
        key::Shape::None(..) => Path::empty(),
        key::Shape::Normal(size) | key::Shape::Space(size) => {
            options.profile.bottom_with_size(size).to_path()
        }
        key::Shape::Homing(..) => options
            .profile
            .bottom_with_size(Size::new(1.0, 1.0))
            .to_path(),
        key::Shape::SteppedCaps => options
            .profile
            .bottom_with_size(Size::new(1.75, 1.0))
            .to_path(),
        key::Shape::IsoHorizontal | key::Shape::IsoVertical => iso_bottom_path(options.profile),
    };

    KeyPath {
        data: path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: options.outline_width,
        }),
    }
}

pub fn homing(key: &key::Key, options: &Options<'_>) -> Option<KeyPath> {
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
            Rect::from_center_and_size(
                center + Size::new(0.0, profile.homing.bar.y_offset.get()),
                profile.homing.bar.size,
            )
            .to_path(),
        ),
        key::Homing::Bump => Some(
            Circle::from_center_and_diameter(
                center + Size::new(0.0, profile.homing.bump.y_offset.get()),
                profile.homing.bump.diameter,
            )
            .to_path(),
        ),
    };

    bez_path.map(|path| KeyPath {
        data: path,
        fill: Some(key.color),
        outline: Some(Outline {
            color: key.color.highlight(0.15),
            width: options.outline_width,
        }),
    })
}

pub fn step(key: &key::Key, options: &Options<'_>) -> Option<KeyPath> {
    matches!(key.shape, key::Shape::SteppedCaps).then(|| {
        let profile = &options.profile;

        // Take average dimensions of top and bottom
        let rect = {
            let frac = 0.5; // TODO make this configurable?
            let top = profile.top_with_size(Size::new(1.0, 1.0));
            let btm = profile.bottom_with_size(Size::new(1.0, 1.0));
            RoundRect::new(
                Point::lerp(top.min, btm.min, frac),
                Point::lerp(top.max, btm.max, frac),
                Length::lerp(top.radius, btm.radius, frac),
            )
        };

        KeyPath {
            data: step_path(rect),
            fill: Some(key.color),
            outline: Some(Outline {
                color: key.color.highlight(0.15),
                width: options.outline_width,
            }),
        }
    })
}

fn iso_bottom_path(profile: &Profile) -> Path<Dot> {
    let rect150 = profile.bottom_with_size(Size::new(1.5, 1.0)).rect();
    let rect125 = profile
        .bottom_with_rect(Rect::new(Point::new(0.25, 0.0), Point::new(1.5, 2.0)))
        .rect();
    let radii = Vector::splat(profile.bottom.radius.get());

    let mut path = Path::builder();
    path.abs_move(rect150.min + Size::new(0.0, radii.x));
    path.rel_arc(radii, Angle::zero(), false, true, radii.neg_y());
    path.abs_horiz_line(Length::new(rect150.max.x - radii.x));
    path.rel_arc(radii, Angle::zero(), false, true, radii);
    path.abs_vert_line(Length::new(rect125.max.y - radii.y));
    path.rel_arc(radii, Angle::zero(), false, true, radii.neg_x());
    path.abs_horiz_line(Length::new(rect125.min.x + radii.x));
    path.rel_arc(radii, Angle::zero(), false, true, -radii);
    path.abs_vert_line(Length::new(rect150.max.y + radii.y));
    path.rel_arc(radii, Angle::zero(), false, false, -radii);
    path.abs_horiz_line(Length::new(rect150.min.x + radii.x));
    path.rel_arc(radii, Angle::zero(), false, true, -radii);
    path.close();

    path.build()
}

fn iso_top_path(profile: &Profile) -> Path<Dot> {
    let rect150 = profile.top_with_size(Size::new(1.5, 1.0)).rect();
    let rect125 = profile
        .top_with_rect(Rect::new(Point::new(0.25, 0.0), Point::new(1.5, 2.0)))
        .rect();
    let radii = Vector::splat(profile.top.radius.get());

    let mut path = Path::builder();
    path.abs_move(rect150.min + Size::new(0.0, radii.x));
    path.rel_arc(radii, Angle::zero(), false, true, radii.neg_y());
    path.abs_horiz_line(Length::new(rect150.max.x - radii.x));
    path.rel_arc(radii, Angle::zero(), false, true, radii);
    path.abs_vert_line(Length::new(rect125.max.y - radii.y));
    path.rel_arc(radii, Angle::zero(), false, true, radii.neg_x());
    path.abs_horiz_line(Length::new(rect125.min.x + radii.x));
    path.rel_arc(radii, Angle::zero(), false, true, -radii);
    path.abs_vert_line(Length::new(rect150.max.y + radii.y));
    path.rel_arc(radii, Angle::zero(), false, false, -radii);
    path.abs_horiz_line(Length::new(rect150.min.x + radii.x));
    path.rel_arc(radii, Angle::zero(), false, true, -radii);
    path.close();

    path.build()
}

fn step_path(rect: RoundRect<Dot>) -> Path<Dot> {
    let radii = Vector::splat(rect.radius.get());
    let rect = Rect::from_origin_and_size(
        Point::new(1.25 * DOT_PER_UNIT.get() - rect.min.x, rect.min.y),
        Size::new(0.5 * DOT_PER_UNIT.get(), rect.height()),
    );

    let mut path = Path::builder();
    path.abs_move(rect.min + Size::new(0.0, radii.y));
    path.rel_arc(radii, Angle::zero(), false, false, -radii);
    path.abs_horiz_line(Length::new(rect.max.x - radii.x));
    path.rel_arc(radii, Angle::zero(), false, true, radii);
    path.abs_vert_line(Length::new(rect.max.y - radii.y));
    path.rel_arc(radii, Angle::zero(), false, true, radii.neg_x());
    path.abs_horiz_line(Length::new(rect.min.x - radii.x));
    path.rel_arc(radii, Angle::zero(), false, false, radii.neg_y());
    path.close();

    path.build()
}

#[cfg(test)]
mod tests {
    use isclose::assert_is_close;
    use key::Key;

    use super::*;

    #[test]
    fn test_top() {
        let options = Options::default();

        // Regular 1u key
        let key = Key::example();
        let path = top(&key, &options);
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, options.outline_width);
        let top_rect = options.profile.top_with_size(Size::new(1.0, 1.0));
        assert_is_close!(bounds, top_rect.rect());

        // None
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::None(Size::splat(1.0));
            key
        };
        let path = top(&key, &options);
        let bounds = path.data.bounds;
        assert_is_close!(bounds, Rect::zero());

        // Homing
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(None);
            key
        };
        let path = top(&key, &options);
        let bounds = path.data.bounds;
        let top_rect = options.profile.top_with_size(Size::splat(1.0));
        assert_is_close!(bounds, top_rect.rect());

        // Stepped caps
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::SteppedCaps;
            key
        };
        let path = top(&key, &options);
        let bounds = path.data.bounds;
        let top_rect = options.profile.top_with_size(Size::new(1.25, 1.0));
        assert_is_close!(bounds, top_rect.rect());

        // ISO enter
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::IsoVertical;
            key
        };
        let path = top(&key, &options);
        let bounds = path.data.bounds;
        let top_rect = options.profile.top_with_size(Size::new(1.5, 2.0));
        assert_is_close!(bounds, top_rect.rect());
    }

    #[test]
    fn test_bottom() {
        let options = Options::default();

        // Regular 1u key
        let key = Key::example();
        let path = bottom(&key, &options);
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, options.outline_width);
        let bottom_rect = options.profile.bottom_with_size(Size::new(1.0, 1.0));
        assert_is_close!(bounds, bottom_rect.rect());

        // None
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::None(Size::splat(1.0));
            key
        };
        let path = bottom(&key, &options);
        let bounds = path.data.bounds;
        assert_is_close!(bounds, Rect::zero());

        // Homing
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(None);
            key
        };
        let path = bottom(&key, &options);
        let bounds = path.data.bounds;
        let bottom_rect = options.profile.bottom_with_size(Size::splat(1.0));
        assert_is_close!(bounds, bottom_rect.rect());

        // Stepped caps
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::SteppedCaps;
            key
        };
        let path = bottom(&key, &options);
        let bounds = path.data.bounds;
        let bottom_rect = options.profile.bottom_with_size(Size::new(1.75, 1.0));
        assert_is_close!(bounds, bottom_rect.rect());

        // ISO enter
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::IsoVertical;
            key
        };
        let path = bottom(&key, &options);
        let bounds = path.data.bounds;
        let bottom_rect = options.profile.bottom_with_size(Size::new(1.5, 2.0));
        assert_is_close!(bounds, bottom_rect.rect());
    }

    #[test]
    fn test_homing() {
        let options = Options::default();

        // Scoop
        let scoop = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Scoop));
            key
        };

        let path = homing(&scoop, &options);
        assert!(path.is_none()); // Top is already scooped; no additional feature to draw

        // Bar
        let bar = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Bar));
            key
        };

        let path = homing(&bar, &options);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), bar.color);
        assert_is_close!(path.outline.unwrap().color, bar.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, options.outline_width);
        let expected = Rect::from_center_and_size(
            options.profile.top_with_size(Size::splat(1.0)).center(),
            options.profile.homing.bar.size,
        )
        .translate(Vector::new(0.0, options.profile.homing.bar.y_offset.get()));
        assert_is_close!(bounds, expected);

        // Bump
        let bump = {
            let mut key = Key::example();
            key.shape = key::Shape::Homing(Some(key::Homing::Bump));
            key
        };

        let path = homing(&bump, &options);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), bump.color);
        assert_is_close!(path.outline.unwrap().color, bump.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, options.outline_width);
        let expected = Rect::from_center_and_size(
            options.profile.top_with_size(Size::splat(1.0)).center(),
            Size::splat(options.profile.homing.bump.diameter.get()),
        )
        .translate(Vector::new(0.0, options.profile.homing.bump.y_offset.get()));
        assert_is_close!(bounds, expected);

        // Non-homing key
        let none = Key::example();

        let path = homing(&none, &options);
        assert!(path.is_none()); // No additional feature to draw
    }

    #[test]
    fn test_step() {
        let key = {
            let mut key = Key::example();
            key.shape = key::Shape::SteppedCaps;
            key
        };
        let options = Options::default();

        let path = step(&key, &options);
        assert!(path.is_some());
        let path = path.unwrap();
        let bounds = path.data.bounds;

        assert_is_close!(path.fill.unwrap(), key.color);
        assert_is_close!(path.outline.unwrap().color, key.color.highlight(0.15));
        assert_is_close!(path.outline.unwrap().width, options.outline_width);

        let top_rect = options.profile.top_with_size(Size::splat(1.0));
        let bottom_rect = options.profile.bottom_with_size(Size::splat(1.0));
        let rect = RoundRect::new(
            (top_rect.min + bottom_rect.min.to_vector()) / 2.0,
            (top_rect.max + bottom_rect.max.to_vector()) / 2.0,
            (top_rect.radius + bottom_rect.radius) / 2.0,
        );
        let rect = Rect::new(
            Point::new(
                1.25 * DOT_PER_UNIT.0 - rect.min.x - rect.radius.get(),
                rect.min.y,
            ),
            Point::new(1.75 * DOT_PER_UNIT.0 - rect.min.x, rect.max.y),
        );

        assert_is_close!(bounds, rect);
    }
}
