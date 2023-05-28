use itertools::Itertools;
use svg::node::element::Path as SvgPath;

use crate::key::{self, Key};
use crate::profile::{Profile, ProfileType};
use crate::utils::{Color, Path, Rect, RoundRect, Vec2};

use super::path::{EdgeType, KeyHelpers};

pub trait Draw {
    fn draw_key(&self, key: &Key) -> Vec<SvgPath>;
    fn draw_margin(&self, key: &Key) -> Vec<SvgPath>;
}

impl Draw for Profile {
    fn draw_key(&self, key: &Key) -> Vec<SvgPath> {
        if key.typ == key::Type::None {
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
                    self.draw_key_bottom(Vec2::new(1.75e3, 1e3), color),
                    self.draw_key_top(typ, Vec2::new(1.25e3, 1e3), color, depth),
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
            .legend
            .iter()
            .flat_map(|s| s.iter())
            .filter_map(|l| l.as_ref().map(|l| l.size))
            .unique()
            .map(|s| self.text_margin.get(s));

        let (pos_off, size_off) = match key.shape {
            key::Shape::IsoHorizontal => (Vec2::ZERO, Vec2::new(500., 0.)),
            key::Shape::IsoVertical => (Vec2::new(250., 0.), Vec2::new(250., 1000.)),
            key::Shape::SteppedCaps => (Vec2::ZERO, Vec2::new(250., 0.)),
            key::Shape::Normal(size) => (Vec2::ZERO, (size - Vec2::from(1.)) * 1e3),
        };

        let (pos_off, size_off) = if key.typ == key::Type::None {
            (
                pos_off + (self.bottom_rect.position() - self.top_rect.position()),
                size_off + (self.bottom_rect.size() - self.top_rect.size()),
            )
        } else {
            (pos_off, size_off)
        };

        rects
            .map(|r| Rect::new(r.position() + pos_off, r.size() + size_off))
            .map(|r| {
                let mut path = Path::new();
                path.abs_move(r.position());
                path.rel_horiz_line(r.size().x);
                path.rel_vert_line(r.size().y);
                path.rel_horiz_line(-r.size().x);
                path.rel_vert_line(-r.size().y);
                path.close();

                SvgPath::new()
                    .set("d", path)
                    .set("fill", "none")
                    .set("stroke", Color::new(255, 0, 0).to_hex())
                    .set("stroke-width", "5")
            })
            .collect()
    }
}

impl Profile {
    fn draw_key_top(&self, typ: key::Type, size: Vec2, color: Color, depth: f32) -> SvgPath {
        let rect = self.top_rect;
        let curve = (depth / 19.05 * 1e3) * 0.381;

        let (edge_t, edge_r, edge_b, edge_l) = match (self.profile_type, typ) {
            (ProfileType::Flat, _) => (
                EdgeType::Line,
                EdgeType::Line,
                EdgeType::Line,
                EdgeType::Line,
            ),
            (_, key::Type::Space) => (
                EdgeType::Line,
                EdgeType::InsetCurve,
                EdgeType::Line,
                EdgeType::InsetCurve,
            ),
            (ProfileType::Cylindrical { .. }, _) => (
                EdgeType::Line,
                EdgeType::Line,
                EdgeType::CurveStretch,
                EdgeType::Line,
            ),
            (ProfileType::Spherical { .. }, _) => (
                EdgeType::CurveLineCurve,
                EdgeType::CurveLineCurve,
                EdgeType::CurveLineCurve,
                EdgeType::CurveLineCurve,
            ),
        };

        let mut path = Path::start(rect);
        path.corner_top_left(rect);
        path.edge_top(rect, size, edge_t, curve);
        path.corner_top_right(rect);
        path.edge_right(rect, size, edge_r, curve);
        path.corner_bottom_right(rect);
        path.edge_bottom(rect, size, edge_b, curve);
        path.corner_bottom_left(rect);
        path.edge_left(rect, size, edge_l, curve);
        path.close();

        SvgPath::new()
            .set("d", path)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_key_bottom(&self, size: Vec2, color: Color) -> SvgPath {
        let rect = self.bottom_rect;

        let mut path = Path::start(rect);
        path.corner_top_left(rect);
        path.edge_top(rect, size, EdgeType::Line, 0.);
        path.corner_top_right(rect);
        path.edge_right(rect, size, EdgeType::Line, 0.);
        path.corner_bottom_right(rect);
        path.edge_bottom(rect, size, EdgeType::Line, 0.);
        path.corner_bottom_left(rect);
        path.edge_left(rect, size, EdgeType::Line, 0.);
        path.close();

        SvgPath::new()
            .set("d", path)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_step(&self, color: Color) -> SvgPath {
        // Take dimensions from average of top and bottom, adjusting only x and width
        let rect = RoundRect::new(
            Vec2::new(
                1250. - (self.top_rect.position().x + self.bottom_rect.position().x) / 2.,
                (self.top_rect.position().y + self.bottom_rect.position().y) / 2.,
            ),
            Vec2::new(
                500. + (self.top_rect.radius().x + self.bottom_rect.radius().x),
                (self.top_rect.size().y + self.bottom_rect.size().y) / 2.,
            ),
            (self.top_rect.radius() + self.bottom_rect.radius()) / 2.,
        );
        // Just set 1u as the size, with the dimensions above it will all line up properly
        let size = Vec2::from(1e3);

        let radius = rect.radius();
        let mut path = Path::start(rect);
        path.rel_arc(radius, 0., false, false, rect.radius() * -1.);
        path.edge_top(rect, size, EdgeType::Line, 0.);
        path.corner_top_right(rect);
        path.edge_right(rect, size, EdgeType::Line, 0.);
        path.corner_bottom_right(rect);
        path.edge_bottom(rect, size, EdgeType::Line, 0.);
        path.rel_arc(radius, 0., false, false, rect.radius() * Vec2::new(1., -1.));
        path.edge_left(rect, size, EdgeType::Line, 0.);
        path.close();

        SvgPath::new()
            .set("d", path)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_iso_top(&self, typ: key::Type, color: Color, depth: f32) -> SvgPath {
        let rect = self.top_rect;
        let top_size = Vec2::new(1.5e3, 1e3);
        let btm_size = Vec2::new(1.25e3, 2e3);
        let curve = (depth / 19.05 * 1e3) * 0.381;

        let (edge_t, edge_r, edge_b, edge_l) = match (self.profile_type, typ) {
            (ProfileType::Flat, _) => (
                EdgeType::Line,
                EdgeType::Line,
                EdgeType::Line,
                EdgeType::Line,
            ),
            (_, key::Type::Space) => (
                EdgeType::Line,
                EdgeType::InsetCurve,
                EdgeType::Line,
                EdgeType::InsetCurve,
            ),
            (ProfileType::Cylindrical { .. }, _) => (
                EdgeType::Line,
                EdgeType::Line,
                EdgeType::CurveStretch,
                EdgeType::Line,
            ),
            (ProfileType::Spherical { .. }, _) => (
                EdgeType::CurveLineCurve,
                EdgeType::CurveLineCurve,
                EdgeType::CurveLineCurve,
                EdgeType::CurveLineCurve,
            ),
        };

        let mut path = Path::start(rect);
        path.corner_top_left(rect);
        path.edge_top(rect, top_size, edge_t, curve);
        path.corner_top_right(rect);
        path.edge_right(rect, btm_size, edge_r, curve);
        path.corner_bottom_right(rect);
        path.edge_bottom(rect, btm_size, edge_b, curve);
        path.corner_bottom_left(rect);

        let h_line = 0.25e3 - 2. * rect.radius().x;
        let v_line = 1e3 - 2. * rect.radius().y;
        let h_curve = match edge_b {
            // Curve deflection of the horizontal line
            EdgeType::Line => 0.,
            // a third seems to give a nice result here
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => curve / 3.,
            EdgeType::InsetCurve => unreachable!(), // No horizontal insets currently possible
        };
        let v_curve = match edge_l {
            // Curve deflection of the vertical line
            EdgeType::Line => 0.,
            EdgeType::CurveLineCurve => curve,
            EdgeType::CurveStretch => unreachable!(), // No vertical stretches currently possible
            EdgeType::InsetCurve => -curve,
        };

        match edge_l {
            EdgeType::Line => path.rel_vert_line(-(v_line - h_curve)),
            EdgeType::CurveLineCurve => {
                let radius = Path::radius(curve, rect.size().y);
                path.rel_arc(
                    radius,
                    0.,
                    false,
                    true,
                    Vec2::new(-v_curve, -rect.size().y / 2.),
                );
                path.rel_vert_line(-(v_line - rect.size().y / 2. - h_curve));
            }
            EdgeType::CurveStretch => {
                // let radius = Path::radius(curve, 2. * v_line);
                // path.rel_arc(radius, 0., false, true, Vector::new(-v_curve, v_line))
                unreachable!() // No vertical stretches currently possible
            }
            EdgeType::InsetCurve => {
                let radius = Path::radius(curve, 2. * v_line);
                path.rel_arc(radius, 0., false, false, Vec2::new(-v_curve, v_line));
            }
        };

        let radius = rect.radius();
        path.rel_arc(radius, 0., false, false, rect.radius() * -1.);
        path.rel_line(Vec2::new(-(h_line - v_curve), -h_curve));
        path.corner_bottom_left(rect);
        path.edge_left(rect, top_size, edge_l, curve);
        path.close();

        SvgPath::new()
            .set("d", path)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_iso_bottom(&self, color: Color) -> SvgPath {
        let rect = self.bottom_rect;
        let top_size = Vec2::new(1.5e3, 1e3);
        let btm_size = Vec2::new(1.25e3, 2e3);

        let radius = rect.radius();
        let mut path = Path::start(rect);
        path.corner_top_left(rect);
        path.edge_top(rect, top_size, EdgeType::Line, 0.);
        path.corner_top_right(rect);
        path.edge_right(rect, btm_size, EdgeType::Line, 0.);
        path.corner_bottom_right(rect);
        path.edge_bottom(rect, btm_size, EdgeType::Line, 0.);
        path.corner_bottom_left(rect);
        path.rel_vert_line(-(1e3 - 2. * rect.radius().y));
        path.rel_arc(radius, 0., false, false, rect.radius() * -1.);
        path.rel_horiz_line(-(0.25e3 - 2. * rect.radius().x));
        path.corner_bottom_left(rect);
        path.edge_left(rect, top_size, EdgeType::Line, 0.);
        path.close();

        SvgPath::new()
            .set("d", path)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_homing_bar(&self, size: Vec2, color: Color) -> SvgPath {
        let center = self.top_rect.center() + (size - Vec2::from(1e3)) / 2.;
        let rect = RoundRect::new(
            center - self.homing.bar.size / 2. + Vec2::new(0., self.homing.bar.y_offset),
            self.homing.bar.size,
            Vec2::from(self.homing.bar.size.y / 2.),
        );

        let mut path = Path::new();
        path.abs_move(rect.position() + Vec2::new(rect.radius().x, 0.));
        path.rel_horiz_line(rect.size().x - 2. * rect.radius().x);
        path.rel_arc(
            rect.radius(),
            0.,
            false,
            true,
            Vec2::new(0., 2. * rect.radius().y),
        );
        path.rel_horiz_line(-(rect.size().x - 2. * rect.radius().x));
        path.rel_arc(
            rect.radius(),
            0.,
            false,
            true,
            Vec2::new(0., -2. * rect.radius().y),
        );
        path.close();

        SvgPath::new()
            .set("d", path)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.25).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_homing_bump(&self, size: Vec2, color: Color) -> SvgPath {
        let center = self.top_rect.center() + (size - Vec2::from(1e3)) / 2.;
        let r = self.homing.bump.diameter / 2.;

        let mut path = Path::new();
        path.abs_move(center + Vec2::new(0., -r));
        path.rel_arc(Vec2::from(r), 0., false, true, Vec2::new(0., 2. * r));
        path.rel_arc(Vec2::from(r), 0., false, true, Vec2::new(0., -2. * r));
        path.close();

        SvgPath::new()
            .set("d", path)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.25).to_hex())
            .set("stroke-width", "10")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_key() {
        let profile = Profile::default();
        let key = Key::default();
        let homing_scoop = key::Type::Homing(Some(key::Homing::Scoop));
        let homing_bar = key::Type::Homing(Some(key::Homing::Bar));
        let homing_bump = key::Type::Homing(Some(key::Homing::Bump));
        let test_config = vec![
            (key::Shape::Normal(Vec2::new(1., 1.)), key::Type::Normal, 2),
            (key::Shape::SteppedCaps, key::Type::Normal, 3),
            (key::Shape::IsoHorizontal, key::Type::Normal, 2),
            (key::Shape::IsoVertical, key::Type::Normal, 2),
            (key::Shape::Normal(Vec2::new(1., 1.)), homing_scoop, 2),
            (key::Shape::Normal(Vec2::new(1., 1.)), homing_bar, 3),
            (key::Shape::Normal(Vec2::new(1., 1.)), homing_bump, 3),
            (key::Shape::Normal(Vec2::new(1., 1.)), key::Type::None, 0),
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
            (key::Shape::Normal(Vec2::new(1., 1.)), key::Type::Normal),
            (key::Shape::SteppedCaps, key::Type::Normal),
            (key::Shape::IsoHorizontal, key::Type::Normal),
            (key::Shape::IsoVertical, key::Type::Normal),
            (key::Shape::Normal(Vec2::new(1., 1.)), key::Type::None),
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
        let size = Vec2::new(1., 1.);
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
                Color::default_key(),
                profile.profile_type.depth(),
            );

            assert!(path.get_attributes().contains_key("d"));
        }
    }

    #[test]
    fn test_draw_key_bottom() {
        let profile = Profile::default();
        let size = Vec2::new(1., 1.);

        let path = profile.draw_key_bottom(size, Color::default_key());

        assert!(path.get_attributes().contains_key("d"));
    }

    #[test]
    fn test_draw_step() {
        let profile = Profile::default();

        let path = profile.draw_step(Color::default_key());

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
            let path =
                profile.draw_iso_top(key.typ, Color::default_key(), profile.profile_type.depth());

            assert!(path.get_attributes().contains_key("d"));
        }
    }

    #[test]
    fn test_draw_iso_bottom() {
        let profile = Profile::default();

        let path = profile.draw_iso_bottom(Color::default_key());

        assert!(path.get_attributes().contains_key("d"));
    }

    #[test]
    fn test_draw_homing_bar() {
        let profile = Profile::default();
        let size = Vec2::new(1., 1.);

        let path = profile.draw_homing_bar(size, Color::default_key());

        assert!(path.get_attributes().contains_key("d"));
    }

    #[test]
    fn test_draw_homing_bump() {
        let profile = Profile::default();
        let size = Vec2::new(1., 1.);

        let path = profile.draw_homing_bump(size, Color::default_key());

        assert!(path.get_attributes().contains_key("d"));
    }
}
