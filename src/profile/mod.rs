mod de;

use std::collections::HashMap;
use std::{array, iter};

use interp::interp_array;
use itertools::Itertools;
use serde::Deserialize;

use crate::error::Result;
use crate::key;
use crate::utils::{Rect, RoundRect, Vec2};

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ProfileType {
    Cylindrical { depth: f32 },
    Spherical { depth: f32 },
    Flat,
}

impl ProfileType {
    pub(crate) fn depth(self) -> f32 {
        match self {
            Self::Cylindrical { depth } | Self::Spherical { depth } => depth,
            Self::Flat => 0.,
        }
    }
}

impl Default for ProfileType {
    fn default() -> Self {
        // 1.0mm is approx the depth of OEM profile
        Self::Cylindrical { depth: 1.0 }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ScoopProps {
    pub depth: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct BarProps {
    pub size: Vec2,
    pub y_offset: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct BumpProps {
    pub diameter: f32,
    pub y_offset: f32,
}

#[derive(Deserialize)]
#[serde(remote = "key::Homing", rename_all = "kebab-case")]
pub enum HomingDef {
    #[serde(alias = "deep-dish")]
    Scoop,
    #[serde(alias = "line")]
    Bar,
    #[serde(alias = "nub", alias = "dot", alias = "nipple")]
    Bump,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct HomingProps {
    #[serde(with = "HomingDef")]
    pub default: key::Homing,
    pub scoop: ScoopProps,
    pub bar: BarProps,
    pub bump: BumpProps,
}

impl Default for HomingProps {
    fn default() -> Self {
        Self {
            default: key::Homing::Bar,
            scoop: ScoopProps {
                depth: 2. * ProfileType::default().depth(), // 2x the regular depth
            },
            bar: BarProps {
                size: Vec2::new(3.81, 0.51), // = 0.15in, 0.02in
                y_offset: 6.35,              // = 0.25in
            },
            bump: BumpProps {
                diameter: 0.51, // = 0.02in
                y_offset: 0.,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextHeight([f32; Self::NUM_HEIGHTS]);

impl TextHeight {
    const NUM_HEIGHTS: usize = 10;

    fn new(heights: &HashMap<usize, f32>) -> Self {
        if heights.is_empty() {
            Self::default()
        } else {
            let (index, height): (Vec<_>, Vec<_>) = {
                iter::once((0., 0.))
                    .chain(
                        heights
                            .iter()
                            .sorted_by_key(|(&i, _)| i)
                            .map(|(&i, &h)| (i as f32, h)),
                    )
                    .unzip()
            };
            let all_indeces = array::from_fn(|i| i as f32);
            Self(interp_array(&index, &height, &all_indeces))
        }
    }

    pub fn get(&self, size_index: usize) -> f32 {
        if size_index < self.0.len() {
            self.0[size_index]
        } else {
            self.0[self.0.len() - 1]
        }
    }
}

impl Default for TextHeight {
    fn default() -> Self {
        const DEFAULT_MAX: f32 = 18.;
        Self(array::from_fn(|i| {
            (i as f32) * DEFAULT_MAX / (Self::NUM_HEIGHTS as f32 - 1.)
        }))
    }
}

#[derive(Debug, Clone)]
pub struct TextRect([Rect; Self::NUM_RECTS]);

impl TextRect {
    const NUM_RECTS: usize = 10;

    fn new(rects: &HashMap<usize, Rect>) -> Self {
        if rects.is_empty() {
            Self::default()
        } else {
            // Note this unwrap will not panic because we know rects is not empty at this stage
            let max_rect = rects[rects.keys().max().unwrap()];

            // TODO clean up this logic
            // For each font size where the alignment rectangle isn't set, the rectangle of the
            // next largest rect is used, so we need to scan in reverse to carry the back the next
            // largest rect.
            let rects: Vec<_> = {
                let tmp = (0..Self::NUM_RECTS)
                    .rev()
                    .scan(max_rect, |prev, i| {
                        if let Some(&value) = rects.get(&i) {
                            *prev = value;
                        }
                        Some(*prev)
                    })
                    .collect_vec();
                tmp.into_iter().rev().collect()
            };

            Self(rects.try_into().unwrap())
        }
    }

    pub fn get(&self, size_index: usize) -> Rect {
        if size_index < self.0.len() {
            self.0[size_index]
        } else {
            self.0[self.0.len() - 1]
        }
    }
}

impl Default for TextRect {
    fn default() -> Self {
        let rect = Rect::new(Vec2::ZERO, Vec2::from(1e3));
        Self([rect; Self::NUM_RECTS])
    }
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub profile_type: ProfileType,
    pub bottom_rect: RoundRect,
    pub top_rect: RoundRect,
    pub text_margin: TextRect,
    pub text_height: TextHeight,
    pub homing: HomingProps,
}

impl Profile {
    pub fn from_toml(s: &str) -> Result<Self> {
        Ok(toml::from_str(s)?)
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            profile_type: ProfileType::default(),
            bottom_rect: RoundRect::new(Vec2::from(25.), Vec2::from(950.), Vec2::from(65.)),
            top_rect: RoundRect::new(Vec2::new(170., 55.), Vec2::new(660., 735.), Vec2::from(65.)),
            text_margin: TextRect::default(),
            text_height: TextHeight::default(),
            homing: HomingProps::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;
    use maplit::hashmap;

    use super::*;

    #[test]
    fn test_profile_type_depth() {
        assert_eq!(ProfileType::Cylindrical { depth: 1. }.depth(), 1.);
        assert_eq!(ProfileType::Spherical { depth: 0.5 }.depth(), 0.5);
        assert_eq!(ProfileType::Flat.depth(), 0.);
    }

    #[test]
    fn test_profile_type_default() {
        assert_matches!(ProfileType::default(), ProfileType::Cylindrical { depth } if depth == 1.);
    }

    #[test]
    fn test_homing_props_default() {
        assert_matches!(HomingProps::default().default, key::Homing::Bar);
        assert_eq!(HomingProps::default().scoop.depth, 2.);
        assert_eq!(HomingProps::default().bar.size, Vec2::new(3.81, 0.51));
        assert_eq!(HomingProps::default().bar.y_offset, 6.35);
        assert_eq!(HomingProps::default().bump.diameter, 0.51);
        assert_eq!(HomingProps::default().bump.y_offset, 0.);
    }

    #[test]
    fn test_text_height_new() {
        let expected = vec![0., 2., 4., 6., 8., 10., 12., 14., 16., 18.];
        let result = TextHeight::new(&hashmap! {}).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e, r);
        }

        let expected = vec![0., 3., 6., 9., 9.5, 10.5, 11.5, 14., 16.5, 19.];
        let result = TextHeight::new(&hashmap! {
            1 => 3.,
            3 => 9.,
            4 => 9.5,
            6 => 11.5,
            9 => 19.
        })
        .0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e, r);
        }
    }

    #[test]
    fn test_text_height_get() {
        let heights = TextHeight::new(&hashmap! {
            1 => 3.,
            3 => 9.,
            4 => 9.5,
            6 => 11.5,
            9 => 19.
        });
        assert_approx_eq!(heights.get(5), 10.5);
        assert_approx_eq!(heights.get(23), 19.);
    }

    #[test]
    fn test_text_height_default() {
        let heights = TextHeight::default();

        for (i, h) in heights.0.into_iter().enumerate() {
            assert_approx_eq!(h, 2. * (i as f32));
        }
    }

    #[test]
    fn test_text_rect_new() {
        let expected = vec![Rect::new(Vec2::ZERO, Vec2::from(1e3)); 10];
        let result = TextRect::new(&hashmap! {}).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.position(), r.position());
            assert_approx_eq!(e.size(), r.size());
        }

        let expected = vec![
            Rect::new(Vec2::new(200., 200.), Vec2::new(600., 600.)),
            Rect::new(Vec2::new(200., 200.), Vec2::new(600., 600.)),
            Rect::new(Vec2::new(200., 200.), Vec2::new(600., 600.)),
            Rect::new(Vec2::new(250., 250.), Vec2::new(500., 500.)),
            Rect::new(Vec2::new(250., 250.), Vec2::new(500., 500.)),
            Rect::new(Vec2::new(250., 250.), Vec2::new(500., 500.)),
            Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
            Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
            Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
            Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
        ];
        let result = TextRect::new(&hashmap! {
            2 => Rect::new(Vec2::new(200., 200.), Vec2::new(600., 600.)),
            5 => Rect::new(Vec2::new(250., 250.), Vec2::new(500., 500.)),
            7 => Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
        })
        .0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.position(), r.position());
            assert_approx_eq!(e.size(), r.size());
        }
    }

    #[test]
    fn test_text_rect_get() {
        let rects = TextRect::new(&hashmap! {
            2 => Rect::new(Vec2::new(200., 200.), Vec2::new(600., 600.)),
            5 => Rect::new(Vec2::new(250., 250.), Vec2::new(500., 500.)),
            7 => Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
        });

        let r = rects.get(2);
        assert_approx_eq!(r.position(), Vec2::from(200.));
        assert_approx_eq!(r.size(), Vec2::from(600.));

        let r = rects.get(62);
        assert_approx_eq!(r.position(), Vec2::from(300.));
        assert_approx_eq!(r.size(), Vec2::from(400.));
    }

    #[test]
    fn test_text_rect_default() {
        let rects = TextRect::default();

        for r in rects.0.into_iter() {
            assert_approx_eq!(r.position(), Vec2::ZERO);
            assert_approx_eq!(r.size(), Vec2::from(1e3));
        }
    }

    #[test]
    fn test_profile_from_toml() {
        let profile = Profile::from_toml(
            r#"
            type = 'cylindrical'
            depth = 0.5

            [bottom]
            width = 18.29
            height = 18.29
            radius = 0.38

            [top]
            width = 11.81
            height = 13.91
            radius = 1.52
            y-offset = -1.62

            [legend.5]
            size = 4.84
            width = 9.45
            height = 11.54
            y-offset = 0

            [legend.4]
            size = 3.18
            width = 9.53
            height = 9.56
            y-offset = 0.40

            [legend.3]
            size = 2.28
            width = 9.45
            height = 11.30
            y-offset = -0.12

            [homing]
            default = 'scoop'
            scoop = { depth = 1.5 }
            bar = { width = 3.85, height = 0.4, y-offset = 5.05 }
            bump = { diameter = 0.4, y-offset = -0.2 }
        "#,
        )
        .unwrap();

        assert!(
            matches!(profile.profile_type, ProfileType::Cylindrical { depth } if f32::abs(depth - 0.5) < 1e-6)
        );

        assert_approx_eq!(profile.bottom_rect.position(), Vec2::from(20.), 0.5);
        assert_approx_eq!(profile.bottom_rect.size(), Vec2::from(960.), 0.5);
        assert_approx_eq!(profile.bottom_rect.radius(), Vec2::from(20.), 0.5);

        assert_approx_eq!(profile.top_rect.position(), Vec2::new(190., 50.), 0.5);
        assert_approx_eq!(profile.top_rect.size(), Vec2::new(620., 730.), 0.5);
        assert_approx_eq!(profile.top_rect.radius(), Vec2::new(80., 80.), 0.5);

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = vec![0., 40., 80., 120., 167., 254., 341., 428., 515., 603., 690.];
        for (e, r) in expected.iter().zip(profile.text_height.0.iter()) {
            assert_approx_eq!(e, r, 0.5);
        }

        assert_eq!(profile.text_margin.0.len(), 10);
        let expected = vec![
            Rect::new(Vec2::new(252., 112.), Vec2::new(496., 593.)),
            Rect::new(Vec2::new(252., 112.), Vec2::new(496., 593.)),
            Rect::new(Vec2::new(252., 112.), Vec2::new(496., 593.)),
            Rect::new(Vec2::new(252., 112.), Vec2::new(496., 593.)),
            Rect::new(Vec2::new(250., 185.), Vec2::new(500., 502.)),
            Rect::new(Vec2::new(252., 112.), Vec2::new(496., 606.)),
            Rect::new(Vec2::new(252., 112.), Vec2::new(496., 606.)),
            Rect::new(Vec2::new(252., 112.), Vec2::new(496., 606.)),
            Rect::new(Vec2::new(252., 112.), Vec2::new(496., 606.)),
            Rect::new(Vec2::new(252., 112.), Vec2::new(496., 606.)),
        ];
        for (e, r) in expected.iter().zip(profile.text_margin.0.iter()) {
            assert_approx_eq!(e.position(), r.position(), 0.5);
            assert_approx_eq!(e.size(), r.size(), 0.5);
        }

        assert_eq!(profile.homing.default, key::Homing::Scoop);
        assert_approx_eq!(profile.homing.scoop.depth, 1.5);
        assert_approx_eq!(profile.homing.bar.size, Vec2::new(202., 21.), 0.5);
        assert_approx_eq!(profile.homing.bar.y_offset, 265., 0.5);
        assert_approx_eq!(profile.homing.bump.diameter, 21., 0.5);
        assert_approx_eq!(profile.homing.bump.y_offset, -10., 0.5);

        let result = Profile::from_toml("null");
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            format!(
                r#"error parsing TOML: TOML parse error at line 1, column 5
  |
1 | null
  |     ^
expected `.`, `=`
"#
            ),
        )
    }

    #[test]
    fn test_profile_default() {
        let profile = Profile::default();

        assert_matches!(profile.profile_type, ProfileType::Cylindrical { depth } if depth == 1.);

        assert_approx_eq!(profile.bottom_rect.position(), Vec2::from(25.));
        assert_approx_eq!(profile.bottom_rect.size(), Vec2::from(950.));
        assert_approx_eq!(profile.bottom_rect.radius(), Vec2::from(65.));

        assert_approx_eq!(profile.top_rect.position(), Vec2::new(170., 55.));
        assert_approx_eq!(profile.top_rect.size(), Vec2::new(660., 735.));
        assert_approx_eq!(profile.top_rect.radius(), Vec2::from(65.));

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = TextHeight::default();
        for (e, r) in expected.0.iter().zip(profile.text_height.0.iter()) {
            assert_approx_eq!(e, r);
        }

        assert_eq!(profile.text_margin.0.len(), 10);
        let expected = TextRect::default();
        for (e, r) in expected.0.iter().zip(profile.text_margin.0.iter()) {
            assert_approx_eq!(e.position(), r.position(), 0.5);
            assert_approx_eq!(e.size(), r.size(), 0.5);
        }
    }
}
