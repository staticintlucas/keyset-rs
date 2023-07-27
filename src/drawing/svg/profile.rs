use std::f64::consts::{FRAC_PI_2, PI};

use itertools::Itertools;
use kurbo::{Arc, BezPath, Circle, Point, Rect, Shape, Size, Vec2};
use svg::node::element::Path as SvgPath;

use crate::key::{self, Key};
use crate::profile::Profile;
use crate::utils::Color;

pub trait Draw {
    fn draw_key(&self, key: &Key) -> Vec<SvgPath>;
    fn draw_margin(&self, key: &Key) -> Vec<SvgPath>;
}

impl Draw for Profile {
    fn draw_key(&self, key: &Key) -> Vec<SvgPath> {
        if matches!(key.typ, key::Type::None) {
            // Nothing to draw
            return vec![];
        }

        let homing = if let key::Type::Homing(homing) = key.typ {
            homing.or(Some(self.homing.default))
        } else {
            None
        };
        let depth = if let Some(key::Homing::Scoop) = homing {
            self.homing.scoop.depth
        } else {
            self.profile_type.depth()
        };
        let typ = key.typ;
        let color = key.color;

        let mut paths = match key.shape {
            key::Shape::Normal(size) => {
                vec![
                    self.draw_key_bottom(size * 1e3, color),
                    self.draw_key_top(typ, size * 1e3, color, depth),
                ]
            }
            key::Shape::SteppedCaps => {
                vec![
                    self.draw_key_bottom(Size::new(1.75e3, 1e3), color),
                    self.draw_key_top(typ, Size::new(1.25e3, 1e3), color, depth),
                    self.draw_step(color),
                ]
            }
            key::Shape::IsoHorizontal | key::Shape::IsoVertical => {
                vec![
                    self.draw_iso_bottom(color),
                    self.draw_iso_top(typ, color, depth),
                ]
            }
        };

        match homing {
            Some(key::Homing::Bar) => {
                paths.push(self.draw_homing_bar(key.shape.size() * 1e3, color));
            }
            Some(key::Homing::Bump) => {
                paths.push(self.draw_homing_bump(key.shape.size() * 1e3, color));
            }
            Some(key::Homing::Scoop) | None => {}
        }

        paths
    }

    fn draw_margin(&self, key: &Key) -> Vec<SvgPath> {
        let rects = key
            .legends
            .iter()
            .flat_map(|s| s.iter())
            .filter_map(|l| l.as_ref().map(|l| l.size))
            .unique()
            .map(|s| self.text_margin.get(s));

        let (pos_off, size_off) = match key.shape {
            key::Shape::IsoHorizontal => (Vec2::ZERO, Size::new(500., 0.)),
            key::Shape::IsoVertical => (Vec2::new(250., 0.), Size::new(250., 1000.)),
            key::Shape::SteppedCaps => (Vec2::ZERO, Size::new(250., 0.)),
            key::Shape::Normal(size) => (Vec2::ZERO, (size * 1e3 - Size::new(1e3, 1e3))),
        };

        let (pos_off, size_off) = if matches!(key.typ, key::Type::None) {
            (
                pos_off + (self.bottom_rect.origin() - self.top_rect.origin()),
                size_off + (self.bottom_rect.rect().size() - self.top_rect.rect().size()),
            )
        } else {
            (pos_off, size_off)
        };

        rects
            .map(|rect| {
                let path = Rect::from_origin_size(rect.origin() + pos_off, rect.size() + size_off)
                    .into_path(1.);
                SvgPath::new()
                    .set("d", path.to_svg())
                    .set("fill", "none")
                    .set("stroke", Color::new(255, 0, 0).to_string())
                    .set("stroke-width", "5")
            })
            .collect()
    }
}

impl Profile {
    fn draw_key_top(&self, _typ: key::Type, size: Size, color: Color, _depth: f64) -> SvgPath {
        let radii = self.top_rect.radii();
        let rect = self.top_rect.rect();
        let rect = rect
            .with_size(rect.size() + size - Size::new(1e3, 1e3))
            .to_rounded_rect(radii);
        let path = rect.into_path(1.);

        // let curve = (depth / 19.05 * 1e3) * 0.381;
        // match (self.profile_type, typ) {
        //     (ProfileType::Flat, _) => {}
        //     (_, key::Type::Space) => {
        //         // TODO inset left & right sides
        //     }
        //     (ProfileType::Cylindrical { .. }, _) => {
        //         // TODO stretched curve on bottom
        //     }
        //     (ProfileType::Spherical { .. }, _) => {
        //         // TODO curve-line-curve on all sides
        //     }
        // };

        SvgPath::new()
            .set("d", path.to_svg())
            .set("fill", color.to_string())
            .set("stroke", color.highlight(0.15).to_string())
            .set("stroke-width", "10")
    }

    fn draw_key_bottom(&self, size: Size, color: Color) -> SvgPath {
        let radii = self.bottom_rect.radii();
        let rect = self.bottom_rect.rect();
        let rect = rect
            .with_size(rect.size() + size - Size::new(1e3, 1e3))
            .to_rounded_rect(radii);
        let path = rect.into_path(1.);

        SvgPath::new()
            .set("d", path.to_svg())
            .set("fill", color.to_string())
            .set("stroke", color.highlight(0.15).to_string())
            .set("stroke-width", "10")
    }

    fn draw_step(&self, color: Color) -> SvgPath {
        // Take average dimensions of top and bottom, adjusting only x and width
        let rect = Rect::from_origin_size(
            Point::new(
                1250. - (self.top_rect.origin().x + self.bottom_rect.origin().x) / 2.,
                (self.top_rect.origin().y + self.bottom_rect.origin().y) / 2.,
            ),
            Size::new(
                500.,
                (self.top_rect.height() + self.bottom_rect.height()) / 2.,
            ),
        );
        let radius = (self.top_rect.radii().as_single_radius().unwrap()
            + self.bottom_rect.radii().as_single_radius().unwrap())
            / 2.;

        let mut path = BezPath::new();
        path.move_to(rect.origin() + (0., radius));
        path.extend(
            Arc::new(
                rect.origin() + (-radius, radius),
                (radius, radius),
                0.,
                -FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect.origin() + (rect.width() - radius, 0.));
        path.extend(
            Arc::new(
                rect.origin() + (rect.width() - radius, radius),
                (radius, radius),
                -FRAC_PI_2,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect.origin() + (rect.width(), rect.height() - radius));
        path.extend(
            Arc::new(
                rect.origin() + (rect.width() - radius, rect.height() - radius),
                (radius, radius),
                0.,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect.origin() + (-radius, rect.height()));
        path.extend(
            Arc::new(
                rect.origin() + (-radius, rect.height() - radius),
                (radius, radius),
                FRAC_PI_2,
                -FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect.origin() + (0., radius));
        path.close_path();

        SvgPath::new()
            .set("d", path.to_svg())
            .set("fill", color.to_string())
            .set("stroke", color.highlight(0.15).to_string())
            .set("stroke-width", "10")
    }

    fn draw_iso_top(&self, _typ: key::Type, color: Color, _depth: f64) -> SvgPath {
        let rect = self.top_rect.rect();
        let rect150 = rect.with_size(rect.size() + Size::new(500., 0.));
        let rect125 = rect
            .with_origin(rect.origin() + (250., 1000.))
            .with_size(rect.size() + Size::new(250., 0.));
        let radius = self.top_rect.radii().as_single_radius().unwrap();

        // match (self.profile_type, typ) {
        //     (ProfileType::Flat, _) => {}
        //     (_, key::Type::Space) => {
        //         // TODO inset left & right sides
        //     }
        //     (ProfileType::Cylindrical { .. }, _) => {
        //         // TODO stretched curve on bottom
        //     }
        //     (ProfileType::Spherical { .. }, _) => {
        //         // TODO curve-line-curve on all sides
        //     }
        // };

        let mut path = BezPath::new();
        path.move_to(rect150.origin() + (0., radius));
        path.extend(
            Arc::new(
                rect150.origin() + (radius, radius),
                (radius, radius),
                PI,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect150.origin() + (rect150.width() - radius, 0.));
        path.extend(
            Arc::new(
                rect150.origin() + (rect150.width() - radius, radius),
                (radius, radius),
                -FRAC_PI_2,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect125.origin() + (rect125.width(), rect125.height() - radius));
        path.extend(
            Arc::new(
                rect125.origin() + (rect125.width() - radius, rect125.height() - radius),
                (radius, radius),
                0.,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect125.origin() + (radius, rect125.height()));
        path.extend(
            Arc::new(
                rect125.origin() + (radius, rect125.height() - radius),
                (radius, radius),
                FRAC_PI_2,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );

        // let h_line = 0.25e3 - 2. * radius;
        // let v_line = 1e3 - 2. * radius;
        // TODO curvature on inner L edges of ISO enter

        path.line_to(
            Point::new(rect125.origin().x, rect150.origin().y) + (0., rect150.height() + radius),
        );
        path.extend(
            Arc::new(
                Point::new(rect125.origin().x, rect150.origin().y)
                    + (-radius, rect150.height() + radius),
                (radius, radius),
                0.,
                -FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect150.origin() + (radius, rect150.height()));
        path.extend(
            Arc::new(
                rect150.origin() + (radius, rect150.height() - radius),
                (radius, radius),
                FRAC_PI_2,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect150.origin() + (0., radius));
        path.close_path();

        SvgPath::new()
            .set("d", path.to_svg())
            .set("fill", color.to_string())
            .set("stroke", color.highlight(0.15).to_string())
            .set("stroke-width", "10")
    }

    fn draw_iso_bottom(&self, color: Color) -> SvgPath {
        let rect = self.bottom_rect.rect();
        let rect150 = rect.with_size(rect.size() + Size::new(500., 0.));
        let rect125 = rect
            .with_origin(rect.origin() + (250., 1000.))
            .with_size(rect.size() + Size::new(250., 0.));
        let radius = self.bottom_rect.radii().as_single_radius().unwrap();

        let mut path = BezPath::new();
        path.move_to(rect150.origin() + (0., radius));
        path.extend(
            Arc::new(
                rect150.origin() + (radius, radius),
                (radius, radius),
                PI,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect150.origin() + (rect150.width() - radius, 0.));
        path.extend(
            Arc::new(
                rect150.origin() + (rect150.width() - radius, radius),
                (radius, radius),
                -FRAC_PI_2,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect125.origin() + (rect125.width(), rect125.height() - radius));
        path.extend(
            Arc::new(
                rect125.origin() + (rect125.width() - radius, rect125.height() - radius),
                (radius, radius),
                0.,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect125.origin() + (radius, rect125.height()));
        path.extend(
            Arc::new(
                rect125.origin() + (radius, rect125.height() - radius),
                (radius, radius),
                FRAC_PI_2,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(
            Point::new(rect125.origin().x, rect150.origin().y) + (0., rect150.height() + radius),
        );
        path.extend(
            Arc::new(
                Point::new(rect125.origin().x, rect150.origin().y)
                    + (-radius, rect150.height() + radius),
                (radius, radius),
                0.,
                -FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect150.origin() + (radius, rect150.height()));
        path.extend(
            Arc::new(
                rect150.origin() + (radius, rect150.height() - radius),
                (radius, radius),
                FRAC_PI_2,
                FRAC_PI_2,
                0.,
            )
            .append_iter(1.),
        );
        path.line_to(rect150.origin() + (0., radius));
        path.close_path();

        SvgPath::new()
            .set("d", path.to_svg())
            .set("fill", color.to_string())
            .set("stroke", color.highlight(0.15).to_string())
            .set("stroke-width", "10")
    }

    fn draw_homing_bar(&self, size: Size, color: Color) -> SvgPath {
        let center = self.top_rect.center() + (size - Size::new(1e3, 1e3)).to_vec2() / 2.;
        let rect = Rect::from_center_size(center, self.homing.bar.size)
            .to_rounded_rect(self.homing.bar.size.height);

        SvgPath::new()
            .set("d", rect.to_path(1.).to_svg())
            .set("fill", color.to_string())
            .set("stroke", color.highlight(0.25).to_string())
            .set("stroke-width", "10")
    }

    fn draw_homing_bump(&self, size: Size, color: Color) -> SvgPath {
        let center = self.top_rect.center() + (size - Size::new(1e3, 1e3)).to_vec2() / 2.;
        let radius = self.homing.bump.diameter / 2.;
        let circle = Circle::new(center, radius);

        SvgPath::new()
            .set("d", circle.to_path(1.).to_svg())
            .set("fill", color.to_string())
            .set("stroke", color.highlight(0.25).to_string())
            .set("stroke-width", "10")
    }
}

#[cfg(test)]
mod tests {
    use crate::profile::ProfileType;

    use super::*;

    #[test]
    fn test_draw_key() {
        let profile = Profile::default();
        let key = Key::default();
        let homing_scoop = key::Type::Homing(Some(key::Homing::Scoop));
        let homing_bar = key::Type::Homing(Some(key::Homing::Bar));
        let homing_bump = key::Type::Homing(Some(key::Homing::Bump));
        let test_config = vec![
            (key::Shape::Normal(Size::new(1., 1.)), key::Type::Normal, 2),
            (key::Shape::SteppedCaps, key::Type::Normal, 3),
            (key::Shape::IsoHorizontal, key::Type::Normal, 2),
            (key::Shape::IsoVertical, key::Type::Normal, 2),
            (key::Shape::Normal(Size::new(1., 1.)), homing_scoop, 2),
            (key::Shape::Normal(Size::new(1., 1.)), homing_bar, 3),
            (key::Shape::Normal(Size::new(1., 1.)), homing_bump, 3),
            (key::Shape::Normal(Size::new(1., 1.)), key::Type::None, 0),
        ];

        for (shape, typ, len) in test_config {
            let key = Key {
                shape,
                typ,
                ..key.clone()
            };
            let path = profile.draw_key(&key);

            assert_eq!(path.len(), len);
            path.into_iter()
                .for_each(|p| assert!(p.get_attributes().contains_key("d")));
        }
    }

    #[test]
    fn test_draw_margin() {
        let profile = Profile::default();
        let key = Key::example();
        let test_config = vec![
            (key::Shape::Normal(Size::new(1., 1.)), key::Type::Normal),
            (key::Shape::SteppedCaps, key::Type::Normal),
            (key::Shape::IsoHorizontal, key::Type::Normal),
            (key::Shape::IsoVertical, key::Type::Normal),
            (key::Shape::Normal(Size::new(1., 1.)), key::Type::None),
        ];

        for (shape, typ) in test_config {
            let key = Key {
                shape,
                typ,
                ..key.clone()
            };
            let path = profile.draw_margin(&key);

            assert_eq!(path.len(), 1);
            assert!(path[0].get_attributes().contains_key("d"));
        }
    }

    #[test]
    fn test_draw_key_top() {
        let profile = Profile::default();
        let key = Key::default();
        let homing_scoop = key::Type::Homing(Some(key::Homing::Scoop));
        let size = Size::new(1., 1.);
        let profile_typs = vec![
            (ProfileType::Cylindrical { depth: 1.0 }, key::Type::Normal),
            (ProfileType::Cylindrical { depth: 1.0 }, key::Type::Space),
            (ProfileType::Cylindrical { depth: 1.0 }, homing_scoop),
            (ProfileType::Spherical { depth: 0.8 }, key::Type::Normal),
            (ProfileType::Spherical { depth: 0.8 }, homing_scoop),
            (ProfileType::Flat, key::Type::Normal),
        ];

        for (profile_type, typ) in profile_typs {
            let profile = Profile {
                profile_type,
                ..profile.clone()
            };
            let key = Key { typ, ..key.clone() };
            let path = profile.draw_key_top(
                key.typ,
                size,
                Color::new(0xCC, 0xCC, 0xCC),
                profile.profile_type.depth(),
            );

            assert!(path.get_attributes().contains_key("d"));
        }
    }

    #[test]
    fn test_draw_key_bottom() {
        let profile = Profile::default();
        let size = Size::new(1., 1.);

        let path = profile.draw_key_bottom(size, Color::new(0xCC, 0xCC, 0xCC));

        assert!(path.get_attributes().contains_key("d"));
    }

    #[test]
    fn test_draw_step() {
        let profile = Profile::default();

        let path = profile.draw_step(Color::new(0xCC, 0xCC, 0xCC));

        assert!(path.get_attributes().contains_key("d"));
    }

    #[test]
    fn test_draw_iso_top() {
        let profile = Profile::default();
        let key = Key::default();
        let homing_scoop = key::Type::Homing(Some(key::Homing::Scoop));
        let profile_typs = vec![
            (ProfileType::Cylindrical { depth: 1.0 }, key::Type::Normal),
            (ProfileType::Cylindrical { depth: 1.0 }, key::Type::Space),
            (ProfileType::Cylindrical { depth: 1.0 }, homing_scoop),
            (ProfileType::Spherical { depth: 0.8 }, key::Type::Normal),
            (ProfileType::Spherical { depth: 0.8 }, homing_scoop),
            (ProfileType::Flat, key::Type::Normal),
        ];

        for (profile_type, typ) in profile_typs {
            let profile = Profile {
                profile_type,
                ..profile.clone()
            };
            let key = Key { typ, ..key.clone() };
            let path = profile.draw_iso_top(
                key.typ,
                Color::new(0xCC, 0xCC, 0xCC),
                profile.profile_type.depth(),
            );

            assert!(path.get_attributes().contains_key("d"));
        }
    }

    #[test]
    fn test_draw_iso_bottom() {
        let profile = Profile::default();

        let path = profile.draw_iso_bottom(Color::new(0xCC, 0xCC, 0xCC));

        assert!(path.get_attributes().contains_key("d"));
    }

    #[test]
    fn test_draw_homing_bar() {
        let profile = Profile::default();
        let size = Size::new(1., 1.);

        let path = profile.draw_homing_bar(size, Color::new(0xCC, 0xCC, 0xCC));

        assert!(path.get_attributes().contains_key("d"));
    }

    #[test]
    fn test_draw_homing_bump() {
        let profile = Profile::default();
        let size = Size::new(1., 1.);

        let path = profile.draw_homing_bump(size, Color::new(0xCC, 0xCC, 0xCC));

        assert!(path.get_attributes().contains_key("d"));
    }
}
