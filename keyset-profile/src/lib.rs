#![allow(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)] // TODO

#[cfg(feature = "serde")]
mod de;

use std::collections::HashMap;
use std::{array, iter};

use geom::{
    Dot, ExtRect, Length, Mm, Point, Rect, RoundRect, SideOffsets, Size, Unit, Vector, DOT_PER_UNIT,
};
use interp::interp_array;
use itertools::Itertools;
use key::Homing;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "kebab-case"))]
pub enum Type {
    Cylindrical { depth: f32 },
    Spherical { depth: f32 },
    Flat,
}

impl Type {
    // 1.0mm is approx the depth of OEM profile
    pub const DEFAULT: Self = Self::Cylindrical { depth: 1.0 };

    #[must_use]
    pub const fn depth(self) -> f32 {
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
pub struct ScoopProps {
    pub depth: Length<Mm>,
}

#[derive(Debug, Clone, Copy)]
pub struct BarProps {
    pub size: Size<Mm>,
    pub y_offset: Length<Mm>,
}

#[derive(Debug, Clone, Copy)]
pub struct BumpProps {
    pub diameter: Length<Mm>,
    pub y_offset: Length<Mm>,
}

#[derive(Debug, Clone, Copy)]
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
            depth: Length::new(2.0 * Type::DEFAULT.depth()), // 2x the regular depth
        },
        bar: BarProps {
            size: Size::new(3.81, 0.51), // = 0.15in, 0.02in
            y_offset: Length::new(6.35), // = 0.25in
        },
        bump: BumpProps {
            diameter: Length::new(0.51), // = 0.02in
            y_offset: Length::new(0.0),
        },
    };
}

impl Default for HomingProps {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextHeight([f32; Self::NUM_HEIGHTS]);

impl TextHeight {
    const NUM_HEIGHTS: usize = 10;

    // From: https://github.com/ijprest/keyboard-layout-editor/blob/d2945e5b0a9cdfc7cc9bb225839192298d82a66d/kb.css#L113
    // TODO can't use array::from_fn here because const_fn_floating_point_arithmetic isn't stable
    #[allow(clippy::unnecessary_cast, clippy::cast_precision_loss)]
    pub const DEFAULT: Self = Self([
        ((6.0 + 2.0 * (0 as f32)) / 72.0) * DOT_PER_UNIT.0,
        ((6.0 + 2.0 * (1 as f32)) / 72.0) * DOT_PER_UNIT.0,
        ((6.0 + 2.0 * (2 as f32)) / 72.0) * DOT_PER_UNIT.0,
        ((6.0 + 2.0 * (3 as f32)) / 72.0) * DOT_PER_UNIT.0,
        ((6.0 + 2.0 * (4 as f32)) / 72.0) * DOT_PER_UNIT.0,
        ((6.0 + 2.0 * (5 as f32)) / 72.0) * DOT_PER_UNIT.0,
        ((6.0 + 2.0 * (6 as f32)) / 72.0) * DOT_PER_UNIT.0,
        ((6.0 + 2.0 * (7 as f32)) / 72.0) * DOT_PER_UNIT.0,
        ((6.0 + 2.0 * (8 as f32)) / 72.0) * DOT_PER_UNIT.0,
        ((6.0 + 2.0 * (9 as f32)) / 72.0) * DOT_PER_UNIT.0,
    ]);

    #[must_use]
    pub fn new(heights: &HashMap<usize, f32>) -> Self {
        if heights.is_empty() {
            Self::default()
        } else {
            let (index, height): (Vec<_>, Vec<_>) = {
                iter::once((0.0, 0.0))
                    .chain(
                        #[allow(clippy::cast_precision_loss)] // `i` will never be that big
                        heights
                            .iter()
                            .sorted_by_key(|(&i, _)| i)
                            .map(|(&i, &h)| (i as f32, h)),
                    )
                    .unzip()
            };
            #[allow(clippy::cast_precision_loss)] // `i` will never be that big
            let all_indeces = array::from_fn(|i| i as f32);
            Self(interp_array(&index, &height, &all_indeces))
        }
    }

    #[must_use]
    pub const fn get(&self, size_index: usize) -> f32 {
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

#[derive(Debug, Clone, Copy)]
pub struct TextMargin([SideOffsets<Dot>; Self::NUM_RECTS]);

impl TextMargin {
    const NUM_RECTS: usize = 10;

    // Need to use SideOffsets::new() here because SideOffsets::new_all_same() isn't const
    pub const DEFAULT: Self = Self(
        [SideOffsets::new(
            0.05 * DOT_PER_UNIT.0,
            0.05 * DOT_PER_UNIT.0,
            0.05 * DOT_PER_UNIT.0,
            0.05 * DOT_PER_UNIT.0,
        ); Self::NUM_RECTS],
    );

    #[must_use]
    pub fn new(offsets: &HashMap<usize, SideOffsets<Dot>>) -> Self {
        if offsets.is_empty() {
            Self::default()
        } else {
            // Note this unwrap will not panic because we know rects is not empty at this stage
            // TODO there should be a better way to do this logic?
            let max_rect = offsets[offsets.keys().max().unwrap()];

            // TODO clean up this logic
            // For each font size where the alignment rectangle isn't set, the rectangle of the
            // next largest rect is used, so we need to scan in reverse to carry the back the next
            // largest rect.
            let offsets: Vec<_> = {
                let tmp = (0..Self::NUM_RECTS)
                    .rev()
                    .scan(max_rect, |prev, i| {
                        if let Some(&value) = offsets.get(&i) {
                            *prev = value;
                        }
                        Some(*prev)
                    })
                    .collect_vec();
                tmp.into_iter().rev().collect()
            };

            Self(offsets.try_into().unwrap())
        }
    }

    #[must_use]
    pub const fn get(&self, size_index: usize) -> SideOffsets<Dot> {
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

#[derive(Debug, Clone, Copy)]
pub struct TopSurface {
    pub size: Size<Dot>,
    pub radius: Length<Dot>,
    pub y_offset: Length<Dot>,
}

impl TopSurface {
    pub const DEFAULT: Self = Self {
        size: Size::new(0.660 * DOT_PER_UNIT.0, 0.735 * DOT_PER_UNIT.0),
        radius: Length::new(0.065 * DOT_PER_UNIT.0),
        y_offset: Length::new(-0.0775 * DOT_PER_UNIT.0),
    };

    pub(crate) fn rect(&self) -> Rect<Dot> {
        Rect::from_center_and_size(
            (Point::new(0.5, 0.5) * DOT_PER_UNIT) + Vector::new(0.0, self.y_offset.get()),
            self.size,
        )
    }

    pub(crate) fn round_rect(&self) -> RoundRect<Dot> {
        RoundRect::from_rect(self.rect(), self.radius)
    }
}

impl Default for TopSurface {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BottomSurface {
    pub size: Size<Dot>,
    pub radius: Length<Dot>,
}

impl BottomSurface {
    pub const DEFAULT: Self = Self {
        size: Size::new(0.95 * DOT_PER_UNIT.0, 0.95 * DOT_PER_UNIT.0),
        radius: Length::new(0.065 * DOT_PER_UNIT.0),
    };

    pub(crate) fn rect(&self) -> Rect<Dot> {
        Rect::from_center_and_size(Point::<Unit>::new(0.5, 0.5) * DOT_PER_UNIT, self.size)
    }

    pub(crate) fn round_rect(&self) -> RoundRect<Dot> {
        RoundRect::from_rect(self.rect(), self.radius)
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

    #[must_use]
    pub fn top_with_size(&self, size: Size<Unit>) -> RoundRect<Dot> {
        let RoundRect { min, max, radius } = self.top.round_rect();
        let max = max + (size - Size::splat(1.0)) * DOT_PER_UNIT;
        RoundRect::new(min, max, radius)
    }

    #[must_use]
    pub fn top_with_rect(&self, rect: Rect<Unit>) -> RoundRect<Dot> {
        let RoundRect { min, max, radius } = self.top.round_rect();
        let min = min + rect.min.to_vector() * DOT_PER_UNIT;
        let max = max + (rect.max.to_vector() - Vector::splat(1.0)) * DOT_PER_UNIT;
        RoundRect::new(min, max, radius)
    }

    #[must_use]
    pub fn bottom_with_size(&self, size: Size<Unit>) -> RoundRect<Dot> {
        let RoundRect { min, max, radius } = self.bottom.round_rect();
        let max = max + (size - Size::splat(1.0)) * DOT_PER_UNIT;
        RoundRect::new(min, max, radius)
    }

    #[must_use]
    pub fn bottom_with_rect(&self, rect: Rect<Unit>) -> RoundRect<Dot> {
        let RoundRect { min, max, radius } = self.bottom.round_rect();
        let min = min + rect.min.to_vector() * DOT_PER_UNIT;
        let max = max + (rect.max.to_vector() - Vector::splat(1.0)) * DOT_PER_UNIT;
        RoundRect::new(min, max, radius)
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use isclose::{assert_is_close, IsClose};
    use unindent::unindent;

    use super::*;

    #[test]
    fn test_profile_type_depth() {
        assert_is_close!(Type::Cylindrical { depth: 1.0 }.depth(), 1.0);
        assert_is_close!(Type::Spherical { depth: 0.5 }.depth(), 0.5);
        assert_is_close!(Type::Flat.depth(), 0.0);
    }

    #[test]
    fn test_profile_type_default() {
        assert_matches!(Type::default(), Type::Cylindrical { depth } if depth.is_close(1.0));
    }

    #[test]
    fn test_homing_props_default() {
        assert_matches!(HomingProps::default().default, Homing::Bar);
        assert_is_close!(HomingProps::default().scoop.depth, Length::new(2.0));
        assert_is_close!(HomingProps::default().bar.size, Size::new(3.81, 0.51));
        assert_is_close!(HomingProps::default().bar.y_offset, Length::new(6.35));
        assert_is_close!(HomingProps::default().bump.diameter, Length::new(0.51));
        assert_is_close!(HomingProps::default().bump.y_offset, Length::new(0.0));
    }

    #[test]
    fn test_text_height_new() {
        #[allow(clippy::cast_precision_loss)] // `i` will never be that big
        let expected: [_; 10] =
            array::from_fn(|i| (6.0 + 2.0 * (i as f32)) / 72.0 * DOT_PER_UNIT.get());
        let result = TextHeight::new(&HashMap::new()).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_is_close!(e, r);
        }

        let expected = [
            0.0, 60.0, 120.0, 180.0, 190.0, 210.0, 230.0, 280.0, 330.0, 380.0,
        ];
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
            assert_is_close!(e, r);
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
        assert_is_close!(heights.get(5), 10.5);
        assert_is_close!(heights.get(23), 19.0);
    }

    #[test]
    fn test_text_height_default() {
        let heights = TextHeight::default();

        #[allow(clippy::cast_precision_loss)] // `i` will never be that big
        for (i, h) in heights.0.into_iter().enumerate() {
            assert_is_close!(h, (6.0 + 2.0 * (i as f32)) / 72.0 * DOT_PER_UNIT.get());
        }
    }

    #[test]
    fn test_text_margin_new() {
        let expected = [SideOffsets::new_all_same(0.05) * DOT_PER_UNIT; 10];
        let result = TextMargin::new(&HashMap::new()).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_is_close!(e, r);
        }

        let expected = [
            SideOffsets::new_all_same(0.0),
            SideOffsets::new_all_same(0.0),
            SideOffsets::new_all_same(0.0),
            SideOffsets::new_all_same(-50.0),
            SideOffsets::new_all_same(-50.0),
            SideOffsets::new_all_same(-50.0),
            SideOffsets::new_all_same(-100.0),
            SideOffsets::new_all_same(-100.0),
            SideOffsets::new_all_same(-100.0),
            SideOffsets::new_all_same(-100.0),
        ];
        let result = TextMargin::new(&HashMap::from([
            (2, SideOffsets::new_all_same(0.0)),
            (5, SideOffsets::new_all_same(-50.0)),
            (7, SideOffsets::new_all_same(-100.0)),
        ]))
        .0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_is_close!(e, r);
        }
    }

    #[test]
    fn test_text_margin_get() {
        let margin = TextMargin::new(&HashMap::from([
            (2, SideOffsets::new_all_same(0.0)),
            (5, SideOffsets::new_all_same(-50.0)),
            (7, SideOffsets::new_all_same(-100.0)),
        ]));

        let offsets = margin.get(2);
        assert_is_close!(offsets, SideOffsets::zero());

        let offsets = margin.get(62);
        assert_is_close!(offsets, SideOffsets::new_all_same(-100.0));
    }

    #[test]
    fn test_text_margin_default() {
        let margin = TextMargin::default();

        for offsets in margin.0 {
            assert_is_close!(offsets, SideOffsets::new_all_same(0.05) * DOT_PER_UNIT);
        }
    }

    #[test]
    fn test_top_surface_rect() {
        let surf = TopSurface::default();
        assert_is_close!(
            surf.rect(),
            Rect::from_origin_and_size(Point::new(0.170, 0.055), Size::new(0.660, 0.735))
                * DOT_PER_UNIT
        );
    }

    #[test]
    fn test_top_surface_round_rect() {
        let surf = TopSurface::default();
        assert_is_close!(
            surf.round_rect(),
            RoundRect::new(
                Point::new(0.170, 0.055) * DOT_PER_UNIT,
                Point::new(0.830, 0.790) * DOT_PER_UNIT,
                Length::new(0.065) * DOT_PER_UNIT
            )
        );
    }

    #[test]
    fn test_top_surface_default() {
        let surf = TopSurface::default();
        assert_is_close!(surf.size, Size::new(0.660, 0.735) * DOT_PER_UNIT);
        assert_is_close!(surf.radius, Length::new(0.065) * DOT_PER_UNIT);
        assert_is_close!(surf.y_offset, Length::new(-0.0775) * DOT_PER_UNIT);
    }

    #[test]
    fn test_bottom_surface_rect() {
        let surf = BottomSurface::default();
        assert_is_close!(
            surf.rect(),
            Rect::new(Point::new(0.025, 0.025), Point::new(0.975, 0.975)) * DOT_PER_UNIT
        );
    }

    #[test]
    fn test_bottom_surface_round_rect() {
        let surf = BottomSurface::default();
        assert_is_close!(
            surf.round_rect(),
            RoundRect::new(
                Point::new(0.025, 0.025) * DOT_PER_UNIT,
                Point::new(0.975, 0.975) * DOT_PER_UNIT,
                Length::new(0.065) * DOT_PER_UNIT
            )
        );
    }

    #[test]
    fn test_bottom_surface_default() {
        let surf = BottomSurface::default();
        assert_is_close!(surf.size, Size::new(0.950, 0.950) * DOT_PER_UNIT);
        assert_is_close!(surf.radius, Length::new(0.065) * DOT_PER_UNIT);
    }

    #[cfg(feature = "toml")]
    #[test]
    fn test_profile_from_toml() {
        use geom::DOT_PER_MM;

        let profile = Profile::from_toml(&unindent(
            "
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
        ",
        ))
        .unwrap();

        assert!(matches!(profile.typ, Type::Cylindrical { depth } if depth.is_close(0.5)));

        assert_is_close!(profile.bottom.size, Size::splat(18.29) * DOT_PER_MM);
        assert_is_close!(profile.bottom.radius, Length::new(0.38) * DOT_PER_MM);

        assert_is_close!(profile.top.size, Size::new(11.81, 13.91) * DOT_PER_MM);
        assert_is_close!(profile.top.radius, Length::new(1.52) * DOT_PER_MM);
        assert_is_close!(profile.top.y_offset, Length::new(-1.62) * DOT_PER_MM);

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = [
            0.0, 0.76, 1.52, 2.28, 3.18, 4.84, 6.5, 8.16, 9.82, 11.48, 13.14,
        ]
        .map(|e| e * DOT_PER_MM.0);
        for (e, r) in expected.iter().zip(profile.text_height.0.iter()) {
            assert_is_close!(e, r);
        }

        assert_eq!(profile.text_margin.0.len(), 10);
        let expected = [
            SideOffsets::new(1.185, 1.18, 1.425, 1.18),
            SideOffsets::new(1.185, 1.18, 1.425, 1.18),
            SideOffsets::new(1.185, 1.18, 1.425, 1.18),
            SideOffsets::new(1.185, 1.18, 1.425, 1.18),
            SideOffsets::new(2.575, 1.14, 1.775, 1.14),
            SideOffsets::new(1.185, 1.18, 1.185, 1.18),
            SideOffsets::new(1.185, 1.18, 1.185, 1.18),
            SideOffsets::new(1.185, 1.18, 1.185, 1.18),
            SideOffsets::new(1.185, 1.18, 1.185, 1.18),
            SideOffsets::new(1.185, 1.18, 1.185, 1.18),
        ]
        .map(|e| e * DOT_PER_MM.0);
        for (e, r) in expected.iter().zip(profile.text_margin.0.iter()) {
            assert_is_close!(e, r);
        }

        assert_matches!(profile.homing.default, Homing::Scoop);
        assert_is_close!(profile.homing.scoop.depth, Length::<Mm>::new(1.5));
        assert_is_close!(profile.homing.bar.size, Size::<Mm>::new(3.85, 0.4));
        assert_is_close!(profile.homing.bar.y_offset, Length::<Mm>::new(5.05));
        assert_is_close!(profile.homing.bump.diameter, Length::<Mm>::new(0.4));
        assert_is_close!(profile.homing.bump.y_offset, Length::<Mm>::new(-0.2));
    }

    #[cfg(feature = "toml")]
    #[test]
    fn test_profile_from_invalid_toml() {
        let result = Profile::from_toml("null");
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            unindent(
                "TOML parse error at line 1, column 5
                  |
                1 | null
                  |     ^
                expected `.`, `=`
                "
            )
        );
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_profile_from_json() {
        use geom::DOT_PER_MM;

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

        assert!(matches!(profile.typ, Type::Cylindrical { depth } if depth.is_close(0.5)));

        assert_is_close!(profile.bottom.size, Size::splat(18.29) * DOT_PER_MM);
        assert_is_close!(profile.bottom.radius, Length::new(0.38) * DOT_PER_MM);

        assert_is_close!(profile.top.size, Size::new(11.81, 13.91) * DOT_PER_MM);
        assert_is_close!(profile.top.radius, Length::new(1.52) * DOT_PER_MM);
        assert_is_close!(profile.top.y_offset, Length::new(-1.62) * DOT_PER_MM);

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = [
            0.0, 0.76, 1.52, 2.28, 3.18, 4.84, 6.5, 8.16, 9.82, 11.48, 13.14,
        ]
        .map(|e| e * DOT_PER_MM.0);
        for (e, r) in expected.iter().zip(profile.text_height.0.iter()) {
            assert_is_close!(e, r);
        }

        assert_eq!(profile.text_margin.0.len(), 10);
        let expected = [
            SideOffsets::new(1.185, 1.18, 1.425, 1.18),
            SideOffsets::new(1.185, 1.18, 1.425, 1.18),
            SideOffsets::new(1.185, 1.18, 1.425, 1.18),
            SideOffsets::new(1.185, 1.18, 1.425, 1.18),
            SideOffsets::new(2.575, 1.14, 1.775, 1.14),
            SideOffsets::new(1.185, 1.18, 1.185, 1.18),
            SideOffsets::new(1.185, 1.18, 1.185, 1.18),
            SideOffsets::new(1.185, 1.18, 1.185, 1.18),
            SideOffsets::new(1.185, 1.18, 1.185, 1.18),
            SideOffsets::new(1.185, 1.18, 1.185, 1.18),
        ]
        .map(|e| e * DOT_PER_MM.0);
        for (e, r) in expected.iter().zip(profile.text_margin.0.iter()) {
            assert_is_close!(e, r);
        }

        assert_matches!(profile.homing.default, Homing::Scoop);
        assert_is_close!(profile.homing.scoop.depth, Length::<Mm>::new(1.5));
        assert_is_close!(profile.homing.bar.size, Size::<Mm>::new(3.85, 0.4));
        assert_is_close!(profile.homing.bar.y_offset, Length::<Mm>::new(5.05));
        assert_is_close!(profile.homing.bump.diameter, Length::<Mm>::new(0.4));
        assert_is_close!(profile.homing.bump.y_offset, Length::<Mm>::new(-0.2));
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_profile_from_invalid_json() {
        let result = Profile::from_json("null");
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            "invalid type: null, expected struct RawProfileData at line 1 column 4"
        );
    }

    #[test]
    fn test_profile_with_size() {
        let profile = Profile::default();

        let top = profile.top_with_size(Size::new(1.0, 1.0));
        let exp = RoundRect::from_center_and_size(
            Point::splat(0.5) * DOT_PER_UNIT + Vector::new(0.0, profile.top.y_offset.0),
            profile.top.size,
            profile.top.radius,
        );
        assert_is_close!(top, exp);

        let bottom = profile.bottom_with_size(Size::new(1.0, 1.0));
        let exp = RoundRect::from_center_and_size(
            Point::splat(0.5) * DOT_PER_UNIT,
            profile.bottom.size,
            profile.bottom.radius,
        );
        assert_is_close!(bottom, exp);

        let top = profile.top_with_size(Size::new(3.0, 2.0));
        let exp = RoundRect::from_center_and_size(
            Point::new(1.5, 1.0) * DOT_PER_UNIT + Vector::new(0.0, profile.top.y_offset.0),
            profile.top.size + Size::new(2.0, 1.0) * DOT_PER_UNIT,
            profile.top.radius,
        );
        assert_is_close!(top, exp);

        let bottom = profile.bottom_with_size(Size::new(3.0, 2.0));
        let exp = RoundRect::from_center_and_size(
            Point::new(1.5, 1.0) * DOT_PER_UNIT,
            profile.bottom.size + Size::new(2.0, 1.0) * DOT_PER_UNIT,
            profile.bottom.radius,
        );
        assert_is_close!(bottom, exp);
    }

    #[test]
    fn test_profile_default() {
        let profile = Profile::default();

        assert_matches!(profile.typ, Type::Cylindrical { depth } if depth.is_close(1.0));

        assert_is_close!(profile.bottom.size, Size::splat(0.950) * DOT_PER_UNIT);
        assert_is_close!(profile.bottom.radius, Length::new(0.065) * DOT_PER_UNIT);

        assert_is_close!(profile.top.size, Size::new(0.660, 0.735) * DOT_PER_UNIT);
        assert_is_close!(profile.top.radius, Length::new(0.065) * DOT_PER_UNIT);
        assert_is_close!(profile.top.y_offset, Length::new(-0.0775) * DOT_PER_UNIT);

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = TextHeight::default();
        for (e, r) in expected.0.iter().zip(profile.text_height.0.iter()) {
            assert_is_close!(e, r);
        }

        assert_eq!(profile.text_margin.0.len(), 10);
        let expected = TextMargin::default();
        for (e, r) in expected.0.iter().zip(profile.text_margin.0.iter()) {
            assert_is_close!(e, r);
        }
    }
}
