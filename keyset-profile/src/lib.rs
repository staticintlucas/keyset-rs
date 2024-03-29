#![warn(
    missing_docs,
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
#![allow(
    missing_docs, // TODO
    clippy::missing_errors_doc, // TODO
    clippy::missing_panics_doc, // TODO
    clippy::suboptimal_flops // Optimiser is pretty good, and mul_add is pretty ugly
)]

#[cfg(feature = "serde")]
mod de;

use std::collections::HashMap;
use std::{array, iter};

use geom::{Insets, Point, Rect, RoundRect, Size, Vec2};
use interp::interp_array;
use itertools::Itertools;
use key::Homing;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "kebab-case"))]
pub enum Type {
    Cylindrical { depth: f64 },
    Spherical { depth: f64 },
    Flat,
}

impl Type {
    // 1.0mm is approx the depth of OEM profile
    pub const DEFAULT: Self = Self::Cylindrical { depth: 1.0 };

    #[must_use]
    pub const fn depth(self) -> f64 {
        match self {
            Self::Cylindrical { depth } | Self::Spherical { depth } => depth,
            Self::Flat => 0.0,
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
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

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(remote = "Homing", rename_all = "kebab-case"))]
pub enum HomingDef {
    #[cfg_attr(feature = "serde", serde(alias = "deep-dish", alias = "dish"))]
    Scoop,
    #[cfg_attr(feature = "serde", serde(alias = "line"))]
    Bar,
    #[cfg_attr(
        feature = "serde",
        serde(alias = "nub", alias = "dot", alias = "nipple")
    )]
    Bump,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct HomingProps {
    #[cfg_attr(feature = "serde", serde(with = "HomingDef"))]
    pub default: Homing,
    pub scoop: ScoopProps,
    pub bar: BarProps,
    pub bump: BumpProps,
}

impl HomingProps {
    pub const DEFAULT: Self = Self {
        default: Homing::Bar,
        scoop: ScoopProps {
            depth: 2.0 * Type::DEFAULT.depth(), // 2x the regular depth
        },
        bar: BarProps {
            size: Size::new(3.81, 0.51), // = 0.15in, 0.02in
            y_offset: 6.35,              // = 0.25in
        },
        bump: BumpProps {
            diameter: 0.51, // = 0.02in
            y_offset: 0.0,
        },
    };
}

impl Default for HomingProps {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone)]
pub struct TextHeight([f64; Self::NUM_HEIGHTS]);

impl TextHeight {
    const NUM_HEIGHTS: usize = 10;

    // From: https://github.com/ijprest/keyboard-layout-editor/blob/d2945e5b0a9cdfc7cc9bb225839192298d82a66d/kb.css#L113
    // TODO (6.0 + 2.0 * (i as f64)) * (1e3 / 72.)
    pub const DEFAULT: Self = Self([
        83.333_333_333,
        111.111_111_111,
        138.888_888_889,
        166.666_666_667,
        194.444_444_444,
        222.222_222_222,
        250.0,
        277.777_777_778,
        305.555_555_556,
        333.333_333_333,
    ]);

    #[must_use]
    pub fn new(heights: &HashMap<usize, f64>) -> Self {
        if heights.is_empty() {
            Self::default()
        } else {
            let (index, height): (Vec<_>, Vec<_>) = {
                iter::once((0., 0.))
                    .chain(
                        #[allow(clippy::cast_precision_loss)]
                        heights
                            .iter()
                            .sorted_by_key(|(&i, _)| i)
                            .map(|(&i, &h)| (i as f64, h)),
                    )
                    .unzip()
            };
            #[allow(clippy::cast_precision_loss)]
            let all_indeces = array::from_fn(|i| i as f64);
            Self(interp_array(&index, &height, &all_indeces))
        }
    }

    #[must_use]
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
        Self::DEFAULT
    }
}

#[derive(Debug, Clone)]
pub struct TextMargin([Insets; Self::NUM_RECTS]);

impl TextMargin {
    const NUM_RECTS: usize = 10;

    pub const DEFAULT: Self = Self([Insets::uniform(-50.0); Self::NUM_RECTS]);

    #[must_use]
    pub fn new(insets: &HashMap<usize, Insets>) -> Self {
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

    #[must_use]
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
        Self::DEFAULT
    }
}

#[derive(Debug, Clone)]
pub struct TopSurface {
    pub size: Size,
    pub radius: f64,
    pub y_offset: f64,
}

impl TopSurface {
    pub const DEFAULT: Self = Self {
        size: Size::new(660.0, 735.0),
        radius: 65.0,
        y_offset: -77.5,
    };

    pub(crate) fn rect(&self) -> Rect {
        Rect::from_center_size(Point::new(500., 500. + self.y_offset), self.size)
    }

    pub(crate) fn round_rect(&self) -> RoundRect {
        RoundRect::from_rect(self.rect(), Vec2::new(self.radius, self.radius))
    }
}

impl Default for TopSurface {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone)]
pub struct BottomSurface {
    pub size: Size,
    pub radius: f64,
}

impl BottomSurface {
    pub const DEFAULT: Self = Self {
        size: Size::new(950.0, 950.0),
        radius: 65.0,
    };

    pub(crate) fn rect(&self) -> Rect {
        Rect::from_center_size(Point::new(500., 500.), self.size)
    }

    pub(crate) fn round_rect(&self) -> RoundRect {
        RoundRect::from_rect(self.rect(), Vec2::new(self.radius, self.radius))
    }
}

impl Default for BottomSurface {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub typ: Type,
    pub bottom: BottomSurface,
    pub top: TopSurface,
    pub text_margin: TextMargin,
    pub text_height: TextHeight,
    pub homing: HomingProps,
}

impl Profile {
    pub const DEFAULT: Self = Self {
        typ: Type::DEFAULT,
        bottom: BottomSurface::DEFAULT,
        top: TopSurface::DEFAULT,
        text_margin: TextMargin::DEFAULT,
        text_height: TextHeight::DEFAULT,
        homing: HomingProps::DEFAULT,
    };

    #[cfg(feature = "toml")]
    pub fn from_toml(s: &str) -> de::Result<Self> {
        toml::from_str(s).map_err(de::Error::from)
    }

    #[cfg(feature = "json")]
    pub fn from_json(s: &str) -> de::Result<Self> {
        serde_json::from_str(s).map_err(de::Error::from)
    }

    pub fn top_with_size(&self, size: impl Into<Size>) -> RoundRect {
        let top_rect = self.top.round_rect();
        top_rect.with_size(top_rect.size() + 1e3 * (size.into() - Size::new(1., 1.)))
    }

    pub fn top_with_rect(&self, rect: impl Into<Rect>) -> RoundRect {
        let rect = rect.into();
        let result = self.top_with_size(rect.size());
        result.with_origin(result.origin() + 1e3 * rect.origin().to_vec2())
    }

    pub fn bottom_with_size(&self, size: impl Into<Size>) -> RoundRect {
        let bottom_rect = self.bottom.round_rect();
        bottom_rect.with_size(bottom_rect.size() + 1e3 * (size.into() - Size::new(1., 1.)))
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;
    use unindent::unindent;

    use super::*;

    #[test]
    fn test_profile_type_depth() {
        assert_eq!(Type::Cylindrical { depth: 1. }.depth(), 1.);
        assert_eq!(Type::Spherical { depth: 0.5 }.depth(), 0.5);
        assert_eq!(Type::Flat.depth(), 0.);
    }

    #[test]
    fn test_profile_type_default() {
        assert_matches!(Type::default(), Type::Cylindrical { depth } if depth == 1.);
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
        let result = TextHeight::new(&HashMap::new()).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e, r);
        }

        let expected = [0., 60., 120., 180., 190., 210., 230., 280., 330., 380.];
        let result = TextHeight::new(&HashMap::from([
            (1, 60.0),
            (3, 180.0),
            (4, 190.0),
            (6, 230.0),
            (9, 380.0),
        ]))
        .0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e, r);
        }
    }

    #[test]
    fn test_text_height_get() {
        let heights = TextHeight::new(&HashMap::from([
            (1, 3.0),
            (3, 9.0),
            (4, 9.5),
            (6, 11.5),
            (9, 19.0),
        ]));
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
        let result = TextMargin::new(&HashMap::new()).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.size().width, r.size().width);
            assert_approx_eq!(e.size().height, r.size().height);
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
        let result = TextMargin::new(&HashMap::from([
            (2, Insets::uniform(0.0)),
            (5, Insets::uniform(-50.0)),
            (7, Insets::uniform(-100.0)),
        ]))
        .0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.size().width, r.size().width);
            assert_approx_eq!(e.size().height, r.size().height);
        }
    }

    #[test]
    fn test_text_margin_get() {
        let margin = TextMargin::new(&HashMap::from([
            (2, Insets::uniform(0.0)),
            (5, Insets::uniform(-50.0)),
            (7, Insets::uniform(-100.0)),
        ]));

        let inset = margin.get(2);
        assert_approx_eq!(inset.size().width, 0.0);
        assert_approx_eq!(inset.size().height, 0.0);

        let inset = margin.get(62);
        assert_approx_eq!(inset.size().width, -200.0);
        assert_approx_eq!(inset.size().height, -200.0);
    }

    #[test]
    fn test_text_margin_default() {
        let margin = TextMargin::default();

        for inset in margin.0.into_iter() {
            assert_approx_eq!(inset.size().width, -100.0);
            assert_approx_eq!(inset.size().height, -100.0);
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

    #[cfg(feature = "toml")]
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

        assert!(matches!(profile.typ, Type::Cylindrical { depth } if f64::abs(depth - 0.5) < 1e-6));

        assert_approx_eq!(profile.bottom.size.width, 960.0, 0.5);
        assert_approx_eq!(profile.bottom.size.height, 960.0, 0.5);
        assert_approx_eq!(profile.bottom.radius, 20.0, 0.5);

        assert_approx_eq!(profile.top.size.width, 620.0, 0.5);
        assert_approx_eq!(profile.top.size.height, 730.0, 0.5);
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
            assert_approx_eq!(e.size().width, r.size().width, 0.5);
            assert_approx_eq!(e.size().height, r.size().height, 0.5);
        }

        assert_matches!(profile.homing.default, Homing::Scoop);
        assert_approx_eq!(profile.homing.scoop.depth, 1.5);
        assert_approx_eq!(profile.homing.bar.size.width, 202.0, 0.5);
        assert_approx_eq!(profile.homing.bar.size.height, 21.0, 0.5);
        assert_approx_eq!(profile.homing.bar.y_offset, 265., 0.5);
        assert_approx_eq!(profile.homing.bump.diameter, 21., 0.5);
        assert_approx_eq!(profile.homing.bump.y_offset, -10., 0.5);

        let result = Profile::from_toml("null");
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            unindent(
                r#"TOML parse error at line 1, column 5
                  |
                1 | null
                  |     ^
                expected `.`, `=`
                "#
            ),
        )
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_profile_from_json() {
        let profile = Profile::from_json(&unindent(
            r#"{
                "type": "cylindrical",
                "depth": 0.5,

                "bottom": {
                    "width": 18.29,
                    "height": 18.29,
                    "radius": 0.38
                },

                "top": {
                    "width": 11.81,
                    "height": 13.91,
                    "radius": 1.52,
                    "y-offset": -1.62
                },

                "legend": {
                    "5": {
                        "size": 4.84,
                        "width": 9.45,
                        "height": 11.54,
                        "y-offset": 0
                    },
                    "4": {
                        "size": 3.18,
                        "width": 9.53,
                        "height": 9.56,
                        "y-offset": 0.40
                    },
                    "3": {
                        "size": 2.28,
                        "width": 9.45,
                        "height": 11.30,
                        "y-offset": -0.12
                    }
                },

                "homing": {
                    "default": "scoop",
                    "scoop": {
                        "depth": 1.5
                    },
                    "bar": {
                        "width": 3.85,
                        "height": 0.4,
                        "y-offset": 5.05
                    },
                    "bump": {
                        "diameter": 0.4,
                        "y-offset": -0.2
                    }
                }
            }"#,
        ))
        .unwrap();

        assert!(matches!(profile.typ, Type::Cylindrical { depth } if f64::abs(depth - 0.5) < 1e-6));

        assert_approx_eq!(profile.bottom.size.width, 960.0, 0.5);
        assert_approx_eq!(profile.bottom.size.height, 960.0, 0.5);
        assert_approx_eq!(profile.bottom.radius, 20.0, 0.5);

        assert_approx_eq!(profile.top.size.width, 620.0, 0.5);
        assert_approx_eq!(profile.top.size.height, 730.0, 0.5);
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
            assert_approx_eq!(e.size().width, r.size().width, 0.5);
            assert_approx_eq!(e.size().height, r.size().height, 0.5);
        }

        assert_matches!(profile.homing.default, Homing::Scoop);
        assert_approx_eq!(profile.homing.scoop.depth, 1.5);
        assert_approx_eq!(profile.homing.bar.size.width, 202.0, 0.5);
        assert_approx_eq!(profile.homing.bar.size.height, 21.0, 0.5);
        assert_approx_eq!(profile.homing.bar.y_offset, 265., 0.5);
        assert_approx_eq!(profile.homing.bump.diameter, 21., 0.5);
        assert_approx_eq!(profile.homing.bump.y_offset, -10., 0.5);

        let result = Profile::from_json("null");
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            "invalid type: null, expected struct RawProfileData at line 1 column 4"
        )
    }

    #[test]
    fn test_profile_with_size() {
        let profile = Profile::default();

        let top = profile.top_with_size((1.0, 1.0));
        assert_approx_eq!(top.origin().x, 500.0 - profile.top.size.width / 2.0);
        assert_approx_eq!(
            top.origin().y,
            500.0 - profile.top.size.height / 2.0 + profile.top.y_offset
        );
        assert_approx_eq!(top.size().width, profile.top.size.width);
        assert_approx_eq!(top.size().height, profile.top.size.height);

        let bottom = profile.bottom_with_size((1.0, 1.0));
        assert_approx_eq!(bottom.origin().x, 500.0 - profile.bottom.size.width / 2.0);
        assert_approx_eq!(bottom.origin().y, 500.0 - profile.bottom.size.height / 2.0);
        assert_approx_eq!(bottom.size().width, profile.bottom.size.width);
        assert_approx_eq!(bottom.size().height, profile.bottom.size.height);

        let top = profile.top_with_size((3.0, 2.0));
        assert_approx_eq!(top.origin().x, 500.0 - profile.top.size.width / 2.0);
        assert_approx_eq!(
            top.origin().y,
            500.0 - profile.top.size.height / 2.0 + profile.top.y_offset
        );
        assert_approx_eq!(top.size().width, profile.top.size.width + 2e3);
        assert_approx_eq!(top.size().height, profile.top.size.height + 1e3);

        let bottom = profile.bottom_with_size((3.0, 2.0));
        assert_approx_eq!(bottom.origin().x, 500.0 - profile.bottom.size.width / 2.0);
        assert_approx_eq!(bottom.origin().y, 500.0 - profile.bottom.size.height / 2.0);
        assert_approx_eq!(bottom.size().width, profile.bottom.size.width + 2e3);
        assert_approx_eq!(bottom.size().height, profile.bottom.size.height + 1e3);
    }

    #[test]
    fn test_profile_default() {
        let profile = Profile::default();

        assert_matches!(profile.typ, Type::Cylindrical { depth } if depth == 1.);

        assert_approx_eq!(profile.bottom.size.width, 950.0);
        assert_approx_eq!(profile.bottom.size.height, 950.0);
        assert_approx_eq!(profile.bottom.radius, 65.);

        assert_approx_eq!(profile.top.size.width, 660.0);
        assert_approx_eq!(profile.top.size.height, 735.0);
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
            assert_approx_eq!(e.size().width, r.size().width, 0.5);
            assert_approx_eq!(e.size().height, r.size().height, 0.5);
        }
    }
}
