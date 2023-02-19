use svg::node::element::path::Data;
use svg::node::element::Path;

use crate::layout::{Key, KeyType};
use crate::profile::{HomingType, Profile, ProfileType};

use super::path::{EdgeType, PathData};

pub trait DrawKey {
    fn draw_key_top(&self, key: &Key) -> Option<Path>;
    fn draw_key_bottom(&self, key: &Key) -> Option<Path>;
}

impl DrawKey for Profile {
    fn draw_key_top(&self, key: &Key) -> Option<Path> {
        // Nothing to draw for none
        if key.key_type == KeyType::None {
            return None;
        }

        let size = key.size.size() * 1e3;
        let rect = self.top_rect;
        let depth = if matches!(
            key.key_type,
            KeyType::Homing(homing) if homing.unwrap_or(self.homing.default) == HomingType::Scoop
        ) {
            self.homing.scoop.depth
        } else {
            self.profile_type.depth()
        };
        let curve = (depth / 19.05 * 1e3) * 0.381;

        let (edge_t, edge_r, edge_b, edge_l) = match (self.profile_type, key.key_type) {
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

        let data: Data = PathData::new(rect)
            .corner_top_left(rect)
            .edge_top(rect, size, edge_t, curve)
            .corner_top_right(rect)
            .edge_right(rect, size, edge_r, curve)
            .corner_bottom_right(rect)
            .edge_bottom(rect, size, edge_b, curve)
            .corner_bottom_left(rect)
            .edge_left(rect, size, edge_l, curve)
            .into();

        let path = Path::new()
            .set("fill", key.key_color.to_hex())
            .set("stroke", key.key_color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
            .set("d", data);

        Some(path)
    }

    fn draw_key_bottom(&self, key: &Key) -> Option<Path> {
        if key.key_type == KeyType::None {
            None
        } else {
            let rect = self.bottom_rect;
            let size = key.size.size() * 1e3;

            let data: Data = PathData::new(rect)
                .corner_top_left(rect)
                .edge_top(rect, size, EdgeType::Line, 0.)
                .corner_top_right(rect)
                .edge_right(rect, size, EdgeType::Line, 0.)
                .corner_bottom_right(rect)
                .edge_bottom(rect, size, EdgeType::Line, 0.)
                .corner_bottom_left(rect)
                .edge_left(rect, size, EdgeType::Line, 0.)
                .into();

            Some(
                Path::new()
                    .set("d", data)
                    .set("fill", key.key_color.to_hex())
                    .set("stroke", key.key_color.highlight(0.15).to_hex())
                    .set("stroke-width", "10"),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::tests::test_key;

    #[test]
    fn test_draw_key_top() {
        let profile = Profile::default();
        let key = test_key();
        let profile_key_types = vec![
            (ProfileType::Cylindrical { depth: 1.0 }, KeyType::Normal),
            (ProfileType::Cylindrical { depth: 1.0 }, KeyType::Space),
            (
                ProfileType::Cylindrical { depth: 1.0 },
                KeyType::Homing(Some(HomingType::Scoop)),
            ),
            (ProfileType::Spherical { depth: 0.8 }, KeyType::Normal),
            (
                ProfileType::Spherical { depth: 0.8 },
                KeyType::Homing(Some(HomingType::Scoop)),
            ),
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
            let path = profile.draw_key_top(&key);

            assert!(path.is_some());
            assert!(path.unwrap().get_attributes().contains_key("d"));
        }

        let key = Key {
            key_type: KeyType::None,
            ..key
        };
        let path = profile.draw_key_top(&key);

        assert!(path.is_none());
    }

    #[test]
    fn test_draw_key_bottom() {
        let profile = Profile::default();
        let key = test_key();
        let path = profile.draw_key_top(&key);

        assert!(path.is_some());
        assert!(path.unwrap().get_attributes().contains_key("d"));

        let key = Key {
            key_type: KeyType::None,
            ..key
        };
        let path = profile.draw_key_bottom(&key);

        assert!(path.is_none());
    }
}
