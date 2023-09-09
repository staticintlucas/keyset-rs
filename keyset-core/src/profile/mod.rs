mod de;

use std::collections::HashMap;
use std::{array, iter};

use interp::interp_array;
use itertools::Itertools;
use key::Homing;
use kurbo::{Insets, Point, Rect, Size, Vec2};
use serde::Deserialize;

use crate::error::{Error, Result};
use crate::utils::RoundRect;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ProfileType {
    Cylindrical { depth: f64 },
    Spherical { depth: f64 },
    Flat,
}

impl ProfileType {
    pub(crate) const fn depth(self) -> f64 {
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
    pub depth: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct BarProps {
    pub size: Size,
    pub y_offset: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct BumpProps {
    pub diameter: f64,
    pub y_offset: f64,
}

#[derive(Deserialize)]
#[serde(remote = "Homing", rename_all = "kebab-case")]
pub enum HomingDef {
    #[serde(alias = "deep-dish", alias = "dish")]
    Scoop,
    #[serde(alias = "line")]
    Bar,
    #[serde(alias = "nub", alias = "dot", alias = "nipple")]
    Bump,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct HomingProps {
    #[serde(with = "HomingDef")]
    pub default: Homing,
    pub scoop: ScoopProps,
    pub bar: BarProps,
    pub bump: BumpProps,
}

impl Default for HomingProps {
    fn default() -> Self {
        Self {
            default: Homing::Bar,
            scoop: ScoopProps {
                depth: 2. * ProfileType::default().depth(), // 2x the regular depth
            },
            bar: BarProps {
                size: Size::new(3.81, 0.51), // = 0.15in, 0.02in
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
pub struct TextHeight([f64; Self::NUM_HEIGHTS]);

impl TextHeight {
    const NUM_HEIGHTS: usize = 10;

    fn new(heights: &HashMap<usize, f64>) -> Self {
        if heights.is_empty() {
            Self::default()
        } else {
            let (index, height): (Vec<_>, Vec<_>) = {
                iter::once((0., 0.))
                    .chain(
                        heights
                            .iter()
                            .sorted_by_key(|(&i, _)| i)
                            .map(|(&i, &h)| (i as f64, h)),
                    )
                    .unzip()
            };
            let all_indeces = array::from_fn(|i| i as f64);
            Self(interp_array(&index, &height, &all_indeces))
        }
    }

    pub const fn get(&self, size_index: usize) -> f64 {
        if size_index < self.0.len() {
            self.0[size_index]
        } else {
            self.0[self.0.len() - 1]
        }
    }
}

impl Default for TextHeight {
    fn default() -> Self {
        Self(array::from_fn(|i| {
            // From: https://github.com/ijprest/keyboard-layout-editor/blob/d2945e5b0a9cdfc7cc9bb225839192298d82a66d/kb.css#L113
            (6.0 + 2.0 * (i as f64)) * (1e3 / 72.)
        }))
    }
}

#[derive(Debug, Clone)]
pub struct TextMargin([Insets; Self::NUM_RECTS]);

impl TextMargin {
    const NUM_RECTS: usize = 10;

    fn new(insets: &HashMap<usize, Insets>) -> Self {
        if insets.is_empty() {
            Self::default()
        } else {
            // Note this unwrap will not panic because we know rects is not empty at this stage
            let max_rect = insets[insets.keys().max().unwrap()];

            // TODO clean up this logic
            // For each font size where the alignment rectangle isn't set, the rectangle of the
            // next largest rect is used, so we need to scan in reverse to carry the back the next
            // largest rect.
            let insets: Vec<_> = {
                let tmp = (0..Self::NUM_RECTS)
                    .rev()
                    .scan(max_rect, |prev, i| {
                        if let Some(&value) = insets.get(&i) {
                            *prev = value;
                        }
                        Some(*prev)
                    })
                    .collect_vec();
                tmp.into_iter().rev().collect()
            };

            Self(insets.try_into().unwrap())
        }
    }

    pub const fn get(&self, size_index: usize) -> Insets {
        if size_index < self.0.len() {
            self.0[size_index]
        } else {
            self.0[self.0.len() - 1]
        }
    }
}

impl Default for TextMargin {
    fn default() -> Self {
        let insets = Insets::uniform(-50.);
        Self([insets; Self::NUM_RECTS])
    }
}

#[derive(Debug, Clone)]
pub struct TopSurface {
    pub size: Size,
    pub radius: f64,
    pub y_offset: f64,
}

impl TopSurface {
    pub(crate) fn rect(&self) -> Rect {
        Rect::from_center_size(Point::new(500., 500. + self.y_offset), self.size)
    }

    pub(crate) fn round_rect(&self) -> RoundRect {
        RoundRect::from_rect(self.rect(), Vec2::new(self.radius, self.radius))
    }
}

impl Default for TopSurface {
    fn default() -> Self {
        Self {
            size: Size::new(660., 735.),
            radius: 65.,
            y_offset: -77.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BottomSurface {
    pub size: Size,
    pub radius: f64,
}

impl BottomSurface {
    pub(crate) fn rect(&self) -> Rect {
        Rect::from_center_size(Point::new(500., 500.), self.size)
    }

    pub(crate) fn round_rect(&self) -> RoundRect {
        RoundRect::from_rect(self.rect(), Vec2::new(self.radius, self.radius))
    }
}

impl Default for BottomSurface {
    fn default() -> Self {
        Self {
            size: Size::new(950., 950.),
            radius: 65.,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Profile {
    pub typ: ProfileType,
    pub bottom: BottomSurface,
    pub top: TopSurface,
    pub text_margin: TextMargin,
    pub text_height: TextHeight,
    pub homing: HomingProps,
}

impl Profile {
    pub fn from_toml(s: &str) -> Result<Self> {
        toml::from_str(s).map_err(Error::from)
    }

    pub fn top_with_size(&self, size: impl Into<Size>) -> RoundRect {
        let top_rect = self.top.round_rect();
        top_rect.with_size(top_rect.size() + 1e3 * (size.into() - Size::new(1., 1.)))
    }

    pub fn bottom_with_size(&self, size: impl Into<Size>) -> RoundRect {
        let bottom_rect = self.bottom.round_rect();
        bottom_rect.with_size(bottom_rect.size() + 1e3 * (size.into() - Size::new(1., 1.)))
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;
    use maplit::hashmap;
    use unindent::unindent;

    use crate::utils::KurboAbs;

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
        assert_matches!(HomingProps::default().default, Homing::Bar);
        assert_eq!(HomingProps::default().scoop.depth, 2.);
        assert_eq!(HomingProps::default().bar.size, Size::new(3.81, 0.51));
        assert_eq!(HomingProps::default().bar.y_offset, 6.35);
        assert_eq!(HomingProps::default().bump.diameter, 0.51);
        assert_eq!(HomingProps::default().bump.y_offset, 0.);
    }

    #[test]
    fn test_text_height_new() {
        let expected: [_; 10] = array::from_fn(|i| (6. + 2. * (i as f64)) * (1e3 / 72.));
        let result = TextHeight::new(&hashmap! {}).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e, r);
        }

        let expected = [0., 60., 120., 180., 190., 210., 230., 280., 330., 380.];
        let result = TextHeight::new(&hashmap! {
            1 => 60.,
            3 => 180.,
            4 => 190.,
            6 => 230.,
            9 => 380.
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
            assert_approx_eq!(h, (6. + 2. * (i as f64)) * (1e3 / 72.));
        }
    }

    #[test]
    fn test_text_margin_new() {
        let expected = vec![Insets::uniform(-50.); 10];
        let result = TextMargin::new(&hashmap! {}).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.size(), r.size());
        }

        let expected = vec![
            Insets::uniform(0.),
            Insets::uniform(0.),
            Insets::uniform(0.),
            Insets::uniform(-50.),
            Insets::uniform(-50.),
            Insets::uniform(-50.),
            Insets::uniform(-100.),
            Insets::uniform(-100.),
            Insets::uniform(-100.),
            Insets::uniform(-100.),
        ];
        let result = TextMargin::new(&hashmap! {
            2 => Insets::uniform(0.),
            5 => Insets::uniform(-50.),
            7 => Insets::uniform(-100.),
        })
        .0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.size(), r.size());
        }
    }

    #[test]
    fn test_text_margin_get() {
        let margin = TextMargin::new(&hashmap! {
            2 => Insets::uniform(0.),
            5 => Insets::uniform(-50.),
            7 => Insets::uniform(-100.),
        });

        let inset = margin.get(2);
        assert_approx_eq!(inset.size(), Size::new(0., 0.));

        let inset = margin.get(62);
        assert_approx_eq!(inset.size(), Size::new(-200., -200.));
    }

    #[test]
    fn test_text_margin_default() {
        let margin = TextMargin::default();

        for inset in margin.0.into_iter() {
            assert_approx_eq!(inset.size(), Size::new(-100., -100.));
        }
    }

    #[test]
    fn test_top_surface_rect() {
        let surf = TopSurface::default();
        assert_eq!(surf.rect().origin(), Point::new(170., 55.),);
        assert_eq!(surf.rect().size(), Size::new(660., 735.),);
    }

    #[test]
    fn test_top_surface_round_rect() {
        let surf = TopSurface::default();
        assert_eq!(surf.round_rect().origin(), Point::new(170., 55.),);
        assert_eq!(surf.round_rect().size(), Size::new(660., 735.),);
        assert_eq!(surf.round_rect().radii(), Vec2::new(65., 65.),);
    }

    #[test]
    fn test_top_surface_default() {
        let surf = TopSurface::default();
        assert_eq!(surf.size, Size::new(660., 735.));
        assert_eq!(surf.radius, 65.);
        assert_eq!(surf.y_offset, -77.5);
    }

    #[test]
    fn test_bottom_surface_rect() {
        let surf = BottomSurface::default();
        assert_eq!(surf.rect().origin(), Point::new(25., 25.),);
        assert_eq!(surf.rect().size(), Size::new(950., 950.),);
    }

    #[test]
    fn test_bottom_surface_round_rect() {
        let surf = BottomSurface::default();
        assert_eq!(surf.round_rect().origin(), Point::new(25., 25.),);
        assert_eq!(surf.round_rect().size(), Size::new(950., 950.),);
        assert_eq!(surf.round_rect().radii(), Vec2::new(65., 65.),);
    }

    #[test]
    fn test_bottom_surface_default() {
        let surf = BottomSurface::default();
        assert_eq!(surf.size, Size::new(950., 950.));
        assert_eq!(surf.radius, 65.);
    }

    #[test]
    fn test_profile_from_toml() {
        let profile = Profile::from_toml(&unindent(
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
        ))
        .unwrap();

        assert!(
            matches!(profile.typ, ProfileType::Cylindrical { depth } if f64::abs(depth - 0.5) < 1e-6)
        );

        assert_approx_eq!(profile.bottom.size, Size::new(960., 960.), 0.5);
        assert_approx_eq!(profile.bottom.radius, 20., 0.5);

        assert_approx_eq!(profile.top.size, Size::new(620., 730.), 0.5);
        assert_approx_eq!(profile.top.radius, 80., 0.5);

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = vec![0., 40., 80., 120., 167., 254., 341., 428., 515., 603., 690.];
        for (e, r) in expected.iter().zip(profile.text_height.0.iter()) {
            assert_approx_eq!(e, r, 0.5);
        }

        assert_eq!(profile.text_margin.0.len(), 10);
        let expected = vec![
            Insets::new(-62., -62., -62., -75.),
            Insets::new(-62., -62., -62., -75.),
            Insets::new(-62., -62., -62., -75.),
            Insets::new(-62., -62., -62., -75.),
            Insets::new(-60., -135., -60., -93.),
            Insets::new(-62., -62., -62., -62.),
            Insets::new(-62., -62., -62., -62.),
            Insets::new(-62., -62., -62., -62.),
            Insets::new(-62., -62., -62., -62.),
            Insets::new(-62., -62., -62., -62.),
        ];
        for (e, r) in expected.iter().zip(profile.text_margin.0.iter()) {
            assert_approx_eq!(e.size(), r.size(), 0.5);
        }

        assert_matches!(profile.homing.default, Homing::Scoop);
        assert_approx_eq!(profile.homing.scoop.depth, 1.5);
        assert_approx_eq!(profile.homing.bar.size, Size::new(202., 21.), 0.5);
        assert_approx_eq!(profile.homing.bar.y_offset, 265., 0.5);
        assert_approx_eq!(profile.homing.bump.diameter, 21., 0.5);
        assert_approx_eq!(profile.homing.bump.y_offset, -10., 0.5);

        let result = Profile::from_toml("null");
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            unindent(
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
    fn test_profile_with_size() {
        let profile = Profile::default();

        assert_approx_eq!(
            profile.top_with_size((1., 1.)).origin(),
            Point::new(
                500. - profile.top.size.width / 2.,
                500. - profile.top.size.height / 2. + profile.top.y_offset,
            )
        );
        assert_approx_eq!(profile.top_with_size((1., 1.)).size(), profile.top.size);
        assert_approx_eq!(
            profile.bottom_with_size((1., 1.)).origin(),
            Point::new(
                500. - profile.bottom.size.width / 2.,
                500. - profile.bottom.size.height / 2.
            )
        );
        assert_approx_eq!(
            profile.bottom_with_size((1., 1.)).size(),
            profile.bottom.size
        );

        assert_approx_eq!(
            profile.top_with_size((3., 2.)).origin(),
            Point::new(
                500. - profile.top.size.width / 2.,
                500. - profile.top.size.height / 2. + profile.top.y_offset,
            )
        );
        assert_approx_eq!(
            profile.top_with_size((3., 2.)).size(),
            profile.top.size + Size::new(2e3, 1e3)
        );
        assert_approx_eq!(
            profile.bottom_with_size((3., 2.)).origin(),
            Point::new(
                500. - profile.bottom.size.width / 2.,
                500. - profile.bottom.size.height / 2.
            )
        );
        assert_approx_eq!(
            profile.bottom_with_size((3., 2.)).size(),
            profile.bottom.size + Size::new(2e3, 1e3)
        );
    }

    #[test]
    fn test_profile_default() {
        let profile = Profile::default();

        assert_matches!(profile.typ, ProfileType::Cylindrical { depth } if depth == 1.);

        assert_approx_eq!(profile.bottom.size, Size::new(950., 950.));
        assert_approx_eq!(profile.bottom.radius, 65.);

        assert_approx_eq!(profile.top.size, Size::new(660., 735.));
        assert_approx_eq!(profile.top.radius, 65.);
        assert_approx_eq!(profile.top.y_offset, -77.5);

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = TextHeight::default();
        for (e, r) in expected.0.iter().zip(profile.text_height.0.iter()) {
            assert_approx_eq!(e, r);
        }

        assert_eq!(profile.text_margin.0.len(), 10);
        let expected = TextMargin::default();
        for (e, r) in expected.0.iter().zip(profile.text_margin.0.iter()) {
            assert_approx_eq!(e.size(), r.size(), 0.5);
        }
    }
}