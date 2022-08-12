mod de;

use std::collections::HashMap;
use std::iter;

use interp::interp_slice;
use itertools::Itertools;
use serde::Deserialize;

use crate::utils::{Rect, RoundRect};

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ProfileType {
    Cylindrical { depth: f32 },
    Spherical { depth: f32 },
    Flat,
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

#[derive(Debug, Clone, Copy, Deserialize)]
struct ScoopProps {
    depth: f32,
}

#[derive(Debug, Clone, Copy)]
struct BarProps {
    width: f32,
    height: f32,
    y_offset: f32,
}

#[derive(Debug, Clone, Copy)]
struct BumpProps {
    radius: f32,
    y_offset: f32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct HomingProps {
    default: HomingType,
    scoop: ScoopProps,
    bar: BarProps,
    bump: BumpProps,
}

#[derive(Debug, Clone)]
struct TextHeight(Vec<f32>);

impl TextHeight {
    const NUM_HEIGHTS: u8 = 10;
    const DEFAULT_MAX: f32 = 18.;

    fn new(heights: &HashMap<u8, f32>) -> Self {
        if heights.is_empty() {
            Self(
                (0..Self::NUM_HEIGHTS)
                    .map(|i| f32::from(i) * Self::DEFAULT_MAX / f32::from(Self::NUM_HEIGHTS - 1))
                    .collect(),
            )
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
        if self.0.is_empty() {
            // TODO make this case impossible?
            if font_usize < usize::from(Self::NUM_HEIGHTS) {
                Self::DEFAULT_MAX * f32::from(kle_font_size) / f32::from(Self::NUM_HEIGHTS - 1)
            } else {
                Self::DEFAULT_MAX
            }
        } else if font_usize < self.0.len() {
            self.0[font_usize]
        } else {
            self.0[self.0.len() - 1]
        }
    }
}

#[derive(Debug, Clone)]
struct TextRect(Vec<Rect>);

impl TextRect {
    const NUM_RECTS: u8 = 10;
    const DEFAULT_RECT: Rect = Rect {
        x: 0.,
        y: 0.,
        w: 1000.,
        h: 1000.,
    };

    fn new(rects: &HashMap<u8, Rect>) -> Self {
        if rects.is_empty() {
            Self(vec![Self::DEFAULT_RECT; usize::from(Self::NUM_RECTS)])
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
        if self.0.len() > usize::from(kle_font_size) {
            self.0[usize::from(kle_font_size)]
        } else if self.0.is_empty() {
            Self::DEFAULT_RECT
        } else {
            self.0[self.0.len() - 1]
        }
    }
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub profile_type: ProfileType,
    bottom_rect: RoundRect,
    top_rect: RoundRect,
    text_margin: TextRect,
    text_height: TextHeight,
    homing: HomingProps,
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use maplit::hashmap;

    use super::*;

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
