mod de;

use std::collections::HashMap;
use std::iter;

use interp::interp_slice;
use itertools::Itertools;
use serde::Deserialize;

use crate::error::Result;
use crate::utils::{Rect, RoundRect};

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

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HomingType {
    #[serde(alias = "deep-dish")]
    Scoop,
    #[serde(alias = "line")]
    Bar,
    #[serde(alias = "nub", alias = "dot", alias = "nipple")]
    Bump,
}

impl Default for HomingType {
    fn default() -> Self {
        Self::Bar
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ScoopProps {
    pub depth: f32,
}

impl Default for ScoopProps {
    fn default() -> Self {
        // I haven't come across a scooped OEM profile key, but I'd guess it
        // would be around 2x the regular depth
        Self { depth: 2.0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BarProps {
    pub width: f32,
    pub height: f32,
    pub y_offset: f32,
}

impl Default for BarProps {
    fn default() -> Self {
        Self {
            width: 3.81,    // = 0.15in
            height: 0.51,   // = 0.02in
            y_offset: 6.35, // = 0.25in
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BumpProps {
    pub diameter: f32,
    pub y_offset: f32,
}

impl Default for BumpProps {
    fn default() -> Self {
        Self {
            diameter: 0.51, // = 0.02in
            y_offset: 0.,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Default)]
pub struct HomingProps {
    pub default: HomingType,
    pub scoop: ScoopProps,
    pub bar: BarProps,
    pub bump: BumpProps,
}

#[derive(Debug, Clone)]
pub struct TextHeight(Vec<f32>);

impl TextHeight {
    const NUM_HEIGHTS: u8 = 10;

    fn new(heights: &HashMap<u8, f32>) -> Self {
        if heights.is_empty() {
            Self::default()
        } else {
            let (index, height): (Vec<_>, Vec<_>) = {
                iter::once((0., 0.))
                    .chain(
                        heights
                            .iter()
                            .sorted_by_key(|(&i, _)| i)
                            .map(|(&i, &h)| (f32::from(i), h)),
                    )
                    .unzip()
            };
            let all_indexes: Vec<_> = (0..Self::NUM_HEIGHTS).map(f32::from).collect();

            Self(interp_slice(&index, &height, &all_indexes))
        }
    }

    fn get(&self, kle_font_size: u8) -> f32 {
        let font_usize = usize::from(kle_font_size);
        // TODO make empty case impossible
        let default = Self::default();
        let sizes = if self.0.is_empty() {
            &default.0
        } else {
            &self.0
        };
        if font_usize < sizes.len() {
            sizes[font_usize]
        } else {
            sizes[sizes.len() - 1]
        }
    }
}

impl Default for TextHeight {
    fn default() -> Self {
        const DEFAULT_MAX: f32 = 18.;
        Self(
            (0..Self::NUM_HEIGHTS)
                .map(|i| f32::from(i) * DEFAULT_MAX / f32::from(Self::NUM_HEIGHTS - 1))
                .collect(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct TextRect(Vec<Rect>);

impl TextRect {
    const NUM_RECTS: u8 = 10;

    fn new(rects: &HashMap<u8, Rect>) -> Self {
        if rects.is_empty() {
            Self::default()
        } else {
            // Note this unwrap will not panic because we know rects is not empty at this stage
            let max_rect = rects[rects.keys().max().unwrap()];

            // For each font size where the alignment rectangle isn't set, the rectangle of the
            // next largest rect is used, so we need to scan in reverse to carry the back the next
            // largest rect.
            let rects = {
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

            Self(rects)
        }
    }

    fn get(&self, kle_font_size: u8) -> Rect {
        // TODO make empty case impossible
        let default = Self::default();
        let rects = if self.0.is_empty() {
            &default.0
        } else {
            &self.0
        };
        if usize::from(kle_font_size) < rects.len() {
            rects[usize::from(kle_font_size)]
        } else {
            rects[rects.len() - 1]
        }
    }
}

impl Default for TextRect {
    fn default() -> Self {
        let rect = Rect::new(0., 0., 1000., 1000.);
        Self(vec![rect; usize::from(Self::NUM_RECTS)])
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
            bottom_rect: RoundRect::new(0.5, 0.5, 18.05, 18.05, 1.2, 1.2),
            top_rect: RoundRect::new(3.2, 1.0, 12.7, 13.95, 1.2, 1.2),
            text_margin: TextRect::default(),
            text_height: TextHeight::default(),
            homing: HomingProps::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use maplit::hashmap;

    use super::*;

    #[test]
    fn test_profile_type_depth() {
        assert_eq!(ProfileType::Cylindrical { depth: 1. }.depth(), 1.);
        assert_eq!(ProfileType::Spherical { depth: 0.5 }.depth(), 0.5);
        assert_eq!(ProfileType::Flat.depth(), 0.);
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

        let heights = TextHeight(vec![]);
        assert_approx_eq!(heights.get(5), 10.);
        assert_approx_eq!(heights.get(200), 18.);
    }

    #[test]
    fn test_text_rect_new() {
        let expected = vec![Rect::new(0., 0., 1e3, 1e3); 10];
        let result = TextRect::new(&hashmap! {}).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.x, r.x);
            assert_approx_eq!(e.y, r.y);
            assert_approx_eq!(e.w, r.w);
            assert_approx_eq!(e.h, r.h);
        }

        let expected = vec![
            Rect::new(200., 200., 600., 600.),
            Rect::new(200., 200., 600., 600.),
            Rect::new(200., 200., 600., 600.),
            Rect::new(250., 250., 500., 500.),
            Rect::new(250., 250., 500., 500.),
            Rect::new(250., 250., 500., 500.),
            Rect::new(300., 300., 400., 400.),
            Rect::new(300., 300., 400., 400.),
            Rect::new(300., 300., 400., 400.),
            Rect::new(300., 300., 400., 400.),
        ];
        let result = TextRect::new(&hashmap! {
            2 => Rect::new(200., 200., 600., 600.),
            5 => Rect::new(250., 250., 500., 500.),
            7 => Rect::new(300., 300., 400., 400.),
        })
        .0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.x, r.x);
            assert_approx_eq!(e.y, r.y);
            assert_approx_eq!(e.w, r.w);
            assert_approx_eq!(e.h, r.h);
        }
    }

    #[test]
    fn test_text_rect_get() {
        let rects = TextRect::new(&hashmap! {
            2 => Rect::new(200., 200., 600., 600.),
            5 => Rect::new(250., 250., 500., 500.),
            7 => Rect::new(300., 300., 400., 400.),
        });

        let r = rects.get(2);
        assert_approx_eq!(r.x, 200.);
        assert_approx_eq!(r.y, 200.);
        assert_approx_eq!(r.w, 600.);
        assert_approx_eq!(r.h, 600.);

        let r = rects.get(62);
        assert_approx_eq!(r.x, 300.);
        assert_approx_eq!(r.y, 300.);
        assert_approx_eq!(r.w, 400.);
        assert_approx_eq!(r.h, 400.);

        let rects = TextRect(vec![]);
        let r = rects.get(2);
        assert_approx_eq!(r.x, 0.);
        assert_approx_eq!(r.y, 0.);
        assert_approx_eq!(r.w, 1e3);
        assert_approx_eq!(r.h, 1e3);
    }
}
