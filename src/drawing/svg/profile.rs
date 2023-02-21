use svg::node::element::path::Data;
use svg::node::element::Path;

use crate::layout::{Key, KeySize, KeyType};
use crate::profile::{HomingType, Profile, ProfileType};
use crate::utils::{Color, RoundRect, Size};

use super::path::{radius, EdgeType, PathData};

pub trait DrawKey {
    fn draw_key(&self, key: &Key) -> Vec<Path>;
}

impl DrawKey for Profile {
    fn draw_key(&self, key: &Key) -> Vec<Path> {
        if key.key_type == KeyType::None {
            // Nothing to draw
            return vec![];
        }

        let homing = if let KeyType::Homing(homing) = key.key_type {
            homing.or(Some(self.homing.default))
        } else {
            None
        };
        let depth = if let Some(HomingType::Scoop) = homing {
            self.homing.scoop.depth
        } else {
            self.profile_type.depth()
        };
        let typ = key.key_type;
        let color = key.key_color;

        let mut paths = match key.size {
            KeySize::Normal(size) => {
                vec![
                    self.draw_key_bottom(size * 1e3, color),
                    self.draw_key_top(typ, size * 1e3, color, depth),
                ]
            }
            KeySize::SteppedCaps => {
                vec![
                    self.draw_key_bottom(Size::new(1.75e3, 1e3), color),
                    self.draw_key_top(typ, Size::new(1.25e3, 1e3), color, depth),
                    self.draw_step(color),
                ]
            }
            KeySize::IsoHorizontal | KeySize::IsoVertical => {
                vec![
                    self.draw_iso_bottom(color),
                    self.draw_iso_top(typ, color, depth),
                ]
            }
        };

        match homing {
            Some(HomingType::Bar) => {
                paths.push(self.draw_homing_bar(key.size.size() * 1e3, color));
            }
            Some(HomingType::Bump) => {
                paths.push(self.draw_homing_bump(key.size.size() * 1e3, color));
            }
            Some(HomingType::Scoop) | None => {}
        }

        paths
    }
}

impl Profile {
    fn draw_key_top(&self, key_type: KeyType, size: Size, color: Color, depth: f32) -> Path {
        let rect = self.top_rect;
        let curve = (depth / 19.05 * 1e3) * 0.381;

        let (edge_t, edge_r, edge_b, edge_l) = match (self.profile_type, key_type) {
            (ProfileType::Flat, _) => (
                EdgeType::Line,
                EdgeType::Line,
                EdgeType::Line,
                EdgeType::Line,
            ),
            (_, KeyType::Space) => (
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

        let data: Data = PathData::start(rect)
            .corner_top_left(rect)
            .edge_top(rect, size, edge_t, curve)
            .corner_top_right(rect)
            .edge_right(rect, size, edge_r, curve)
            .corner_bottom_right(rect)
            .edge_bottom(rect, size, edge_b, curve)
            .corner_bottom_left(rect)
            .edge_left(rect, size, edge_l, curve)
            .into();

        Path::new()
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
            .set("d", data)
    }

    fn draw_key_bottom(&self, size: Size, color: Color) -> Path {
        let rect = self.bottom_rect;

        let data: Data = PathData::start(rect)
            .corner_top_left(rect)
            .edge_top(rect, size, EdgeType::Line, 0.)
            .corner_top_right(rect)
            .edge_right(rect, size, EdgeType::Line, 0.)
            .corner_bottom_right(rect)
            .edge_bottom(rect, size, EdgeType::Line, 0.)
            .corner_bottom_left(rect)
            .edge_left(rect, size, EdgeType::Line, 0.)
            .into();

        Path::new()
            .set("d", data)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_step(&self, color: Color) -> Path {
        // Take dimensions from average of top and bottom, adjusting only x and width
        let rect = RoundRect {
            x: 1250. - (self.top_rect.x + self.bottom_rect.x) / 2.,
            y: (self.top_rect.y + self.bottom_rect.y) / 2.,
            w: 500. + (self.top_rect.rx + self.bottom_rect.rx),
            h: (self.top_rect.h + self.bottom_rect.h) / 2.,
            rx: (self.top_rect.rx + self.bottom_rect.rx) / 2.,
            ry: (self.top_rect.ry + self.bottom_rect.ry) / 2.,
        };
        // Just set 1u as the size, with the dimensions above it will all line up properly
        let size = Size::new(1000., 1000.);

        let data: Data = PathData::start(rect)
            .corner_inset(-rect.rx, -rect.ry)
            .edge_top(rect, size, EdgeType::Line, 0.)
            .corner_top_right(rect)
            .edge_right(rect, size, EdgeType::Line, 0.)
            .corner_bottom_right(rect)
            .edge_bottom(rect, size, EdgeType::Line, 0.)
            .corner_inset(rect.rx, -rect.ry)
            .edge_left(rect, size, EdgeType::Line, 0.)
            .into();

        Path::new()
            .set("d", data)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_iso_top(&self, key_type: KeyType, color: Color, depth: f32) -> Path {
        let rect = self.top_rect;
        let top_size = Size::new(1.5e3, 1e3);
        let btm_size = Size::new(1.25e3, 2e3);
        let curve = (depth / 19.05 * 1e3) * 0.381;

        let (edge_t, edge_r, edge_b, edge_l) = match (self.profile_type, key_type) {
            (ProfileType::Flat, _) => (
                EdgeType::Line,
                EdgeType::Line,
                EdgeType::Line,
                EdgeType::Line,
            ),
            (_, KeyType::Space) => (
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

        let path_data = PathData::start(rect)
            .corner_top_left(rect)
            .edge_top(rect, top_size, edge_t, curve)
            .corner_top_right(rect)
            .edge_right(rect, btm_size, edge_r, curve)
            .corner_bottom_right(rect)
            .edge_bottom(rect, btm_size, edge_b, curve)
            .corner_bottom_left(rect);

        let h_line = 0.25e3 - 2. * rect.rx;
        let v_line = 1e3 - 2. * rect.ry;
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

        let path_data = match edge_l {
            EdgeType::Line => path_data.v_line(-(v_line - h_curve)),
            EdgeType::CurveLineCurve => {
                let r = radius(curve, rect.h);
                path_data
                    .arc(r, r, -v_curve, -rect.h / 2.)
                    .v_line(-(v_line - rect.h / 2. - h_curve))
            }
            EdgeType::CurveStretch => {
                // let r = radius(curve, 2. * v_line);
                // path_data
                //     .arc(r, r, -v_curve, v_line)
                unreachable!() // No vertical stretches currently possible
            }
            EdgeType::InsetCurve => {
                let r = radius(curve, 2. * v_line);
                path_data.arc_inset(r, r, -v_curve, v_line)
            }
        };

        let data: Data = path_data
            .corner_inset(-rect.rx, -rect.ry)
            .line(-(h_line - v_curve), -h_curve)
            .corner_bottom_left(rect)
            .edge_left(rect, top_size, edge_l, curve)
            .into();

        Path::new()
            .set("d", data)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_iso_bottom(&self, color: Color) -> Path {
        let rect = self.bottom_rect;
        let top_size = Size::new(1.5e3, 1e3);
        let btm_size = Size::new(1.25e3, 2e3);

        let data: Data = PathData::start(rect)
            .corner_top_left(rect)
            .edge_top(rect, top_size, EdgeType::Line, 0.)
            .corner_top_right(rect)
            .edge_right(rect, btm_size, EdgeType::Line, 0.)
            .corner_bottom_right(rect)
            .edge_bottom(rect, btm_size, EdgeType::Line, 0.)
            .corner_bottom_left(rect)
            .v_line(-(1e3 - 2. * rect.ry))
            .corner_inset(-rect.rx, -rect.ry)
            .h_line(-(0.25e3 - 2. * rect.rx))
            .corner_bottom_left(rect)
            .edge_left(rect, top_size, EdgeType::Line, 0.)
            .into();

        Path::new()
            .set("d", data)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_homing_bar(&self, size: Size, color: Color) -> Path {
        let center = self.top_rect.center() + (size - Size::new(1e3, 1e3)) / 2.;
        let rect = RoundRect {
            x: center.x - self.homing.bar.width / 2.,
            y: center.y - self.homing.bar.height / 2. + self.homing.bar.y_offset,
            w: self.homing.bar.width,
            h: self.homing.bar.height,
            rx: self.homing.bar.height / 2.,
            ry: self.homing.bar.height / 2.,
        };

        let data: Data = PathData::new(rect.x, rect.y + rect.ry)
            .corner(rect.rx, -rect.ry)
            .h_line(rect.w - 2. * rect.rx)
            .corner(rect.rx, rect.ry)
            .corner(-rect.rx, rect.ry)
            .h_line(-(rect.w - 2. * rect.rx))
            .corner(-rect.rx, -rect.ry)
            .into();

        Path::new()
            .set("d", data)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.25).to_hex())
            .set("stroke-width", "10")
    }

    fn draw_homing_bump(&self, size: Size, color: Color) -> Path {
        let center = self.top_rect.center() + (size - Size::new(1e3, 1e3)) / 2.;
        let r = self.homing.bump.diameter / 2.;

        let data: Data = PathData::new(center.x - r, center.y)
            .corner(r, -r)
            .corner(r, r)
            .corner(-r, r)
            .corner(-r, -r)
            .into();

        Path::new()
            .set("d", data)
            .set("fill", color.to_hex())
            .set("stroke", color.highlight(0.25).to_hex())
            .set("stroke-width", "10")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::tests::test_key;

    #[test]
    fn test_draw_key() {
        let profile = Profile::default();
        let key = test_key();
        let homing_scoop = KeyType::Homing(Some(HomingType::Scoop));
        let homing_bar = KeyType::Homing(Some(HomingType::Bar));
        let homing_bump = KeyType::Homing(Some(HomingType::Bump));
        let test_config = vec![
            (KeySize::Normal(Size::new(1., 1.)), KeyType::Normal, 2),
            (KeySize::SteppedCaps, KeyType::Normal, 3),
            (KeySize::IsoHorizontal, KeyType::Normal, 2),
            (KeySize::IsoVertical, KeyType::Normal, 2),
            (KeySize::Normal(Size::new(1., 1.)), homing_scoop, 2),
            (KeySize::Normal(Size::new(1., 1.)), homing_bar, 3),
            (KeySize::Normal(Size::new(1., 1.)), homing_bump, 3),
            (KeySize::Normal(Size::new(1., 1.)), KeyType::None, 0),
        ];

        for (size, key_type, len) in test_config {
            let key = Key {
                size,
                key_type,
                ..key.clone()
            };
            let path = profile.draw_key(&key);

            assert_eq!(path.len(), len);
            path.into_iter()
                .for_each(|p| assert!(p.get_attributes().contains_key("d")));
        }
    }

    #[test]
    fn test_draw_key_top() {
        let profile = Profile::default();
        let key = test_key();
        let homing_scoop = KeyType::Homing(Some(HomingType::Scoop));
        let size = Size::new(1., 1.);
        let profile_key_types = vec![
            (ProfileType::Cylindrical { depth: 1.0 }, KeyType::Normal),
            (ProfileType::Cylindrical { depth: 1.0 }, KeyType::Space),
            (ProfileType::Cylindrical { depth: 1.0 }, homing_scoop),
            (ProfileType::Spherical { depth: 0.8 }, KeyType::Normal),
            (ProfileType::Spherical { depth: 0.8 }, homing_scoop),
            (ProfileType::Flat, KeyType::Normal),
        ];

        for (profile_type, key_type) in profile_key_types {
            let profile = Profile {
                profile_type,
                ..profile.clone()
            };
            let key = Key {
                key_type,
                ..key.clone()
            };
            let path = profile.draw_key_top(
                key.key_type,
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
        let size = Size::new(1., 1.);

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
        let key = test_key();
        let homing_scoop = KeyType::Homing(Some(HomingType::Scoop));
        let profile_key_types = vec![
            (ProfileType::Cylindrical { depth: 1.0 }, KeyType::Normal),
            (ProfileType::Cylindrical { depth: 1.0 }, KeyType::Space),
            (ProfileType::Cylindrical { depth: 1.0 }, homing_scoop),
            (ProfileType::Spherical { depth: 0.8 }, KeyType::Normal),
            (ProfileType::Spherical { depth: 0.8 }, homing_scoop),
            (ProfileType::Flat, KeyType::Normal),
        ];

        for (profile_type, key_type) in profile_key_types {
            let profile = Profile {
                profile_type,
                ..profile.clone()
            };
            let key = Key {
                key_type,
                ..key.clone()
            };
            let path = profile.draw_iso_top(
                key.key_type,
                Color::default_key(),
                profile.profile_type.depth(),
            );

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
        let size = Size::new(1., 1.);

        let path = profile.draw_homing_bar(size, Color::default_key());

        assert!(path.get_attributes().contains_key("d"));
    }

    #[test]
    fn test_draw_homing_bump() {
        let profile = Profile::default();
        let size = Size::new(1., 1.);

        let path = profile.draw_homing_bump(size, Color::default_key());

        assert!(path.get_attributes().contains_key("d"));
    }
}
