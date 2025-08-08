//! This crate contains the profile struct and types used for describing
//! profiles used by [keyset]. It also contains utility functions for loading
//! profiles from file
//!
//! [keyset]: https://crates.io/crates/keyset

#![cfg_attr(coverage, expect(unstable_features))]
#![cfg_attr(coverage, feature(coverage_attribute))]

#[cfg(feature = "serde")]
mod de;

use std::collections::HashMap;
use std::{array, fmt};

use interp::{interp_array, InterpMode};

use geom::{
    ConvertFrom as _, ConvertInto as _, Dot, Inch, KeyUnit, Mm, OffsetRect, Point, Rect, RoundRect,
    Unit as _, Vector,
};
use key::Homing;

/// The type of a profile
#[derive(Debug, Clone, Copy)]
pub enum Type {
    /// A cylindrical profile, e.g. Cherry or OEM
    Cylindrical {
        /// The depth of the key's dish
        depth: Dot,
    },
    /// A cylindrical profile, e.g. SA or KAT
    Spherical {
        /// The depth of the key's dish
        depth: Dot,
    },
    /// A flat profile, e.g. G20 or chiclet
    Flat,
}

impl Type {
    /// Returns the depth of a key's dish. This is zero for [`Type::Flat`]
    #[inline]
    #[must_use]
    pub const fn depth(self) -> Dot {
        match self {
            Self::Cylindrical { depth } | Self::Spherical { depth } => depth,
            Self::Flat => Dot(0.0),
        }
    }
}

impl Default for Type {
    #[inline]
    fn default() -> Self {
        Self::Cylindrical {
            // 1.0mm is approx the depth of OEM profile
            depth: Mm(1.0).convert_into(),
        }
    }
}

/// Scooped (a.k.a. deep dish) homing key properties
#[derive(Debug, Clone, Copy)]
pub struct ScoopProps {
    /// The depth of the scooped dish
    pub depth: Dot,
}

/// Homing bar properties
#[derive(Debug, Clone, Copy)]
pub struct BarProps {
    /// The size of the bar
    pub size: Vector<Dot>,
    /// The length of the bar from the center of the key top
    pub y_offset: Dot,
}

/// Homing bump (a.k.a. nub or nipple) properties
#[derive(Debug, Clone, Copy)]
pub struct BumpProps {
    /// The diameter of the bump
    pub diameter: Dot,
    /// The length of the bump from the center of the key top
    pub y_offset: Dot,
}

/// Struct used to deserialize [`key::Homing`]
#[cfg(feature = "serde")]
#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(remote = "Homing", rename_all = "kebab-case")]
enum HomingDef {
    #[serde(alias = "deep-dish", alias = "dish")]
    Scoop,
    #[serde(alias = "line")]
    Bar,
    #[serde(alias = "nub", alias = "dot", alias = "nipple")]
    Bump,
}

/// Homing key properties
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct HomingProps {
    /// The default type of homing key for this profile
    #[cfg_attr(feature = "serde", serde(with = "HomingDef"))]
    pub default: Homing,
    /// Properties for scooped homing keys
    pub scoop: ScoopProps,
    /// Properties for barred homing keys
    pub bar: BarProps,
    /// Properties for keys with a homing bump
    pub bump: BumpProps,
}

impl Default for HomingProps {
    #[inline]
    fn default() -> Self {
        Self {
            default: Homing::Bar,
            scoop: ScoopProps {
                depth: Type::default().depth() * 2.0, // 2x the regular depth
            },
            bar: BarProps {
                size: Vector::new(Inch(0.15), Inch(0.02)).convert_into(),
                y_offset: Inch(0.25).convert_into(),
            },
            bump: BumpProps {
                diameter: Inch(0.02).convert_into(),
                y_offset: Inch(0.0).convert_into(),
            },
        }
    }
}

/// Text height mapping. This maps a [`usize`] index (used by KLE for example)
/// to a length for the height of uppercase letter
#[derive(Debug, Clone, Copy)]
pub struct TextHeight([Dot; Self::NUM_HEIGHTS]);

impl TextHeight {
    const NUM_HEIGHTS: usize = 10;

    /// Create a new [`TextHeight`] mapping from a [`HashMap`]
    #[inline]
    #[must_use]
    pub fn new(heights: &HashMap<usize, Dot>) -> Self {
        if heights.is_empty() {
            Self::default()
        } else {
            // Get all the indices and heights in the hashmap
            let (indices, heights): (Vec<_>, Vec<_>) = {
                // TODO is unconditionally prepending (0, 0.0) here correct? Self::default() doesn't
                // use 0.0 font size for 0
                let mut vec = Vec::with_capacity(heights.len().saturating_add(1));
                vec.push((0, 0.0));
                vec.extend(heights.iter().map(|(&i, &h)| (i, h.get())));
                vec.sort_unstable_by_key(|&(i, _h)| i);
                #[allow(clippy::cast_precision_loss)] // i <= 9
                vec.into_iter().map(|(i, h)| (i as f32, h)).unzip()
            };

            #[allow(clippy::cast_precision_loss)] // i <= 9
            let all_indices = array::from_fn(|i| i as f32);
            Self(interp_array(&indices, &heights, &all_indices, &InterpMode::Extrapolate).map(Dot))
        }
    }

    /// Get the height of an uppercase letter for the given index
    #[inline]
    #[must_use]
    pub fn get(&self, size_index: usize) -> Dot {
        *self
            .0
            .get(size_index)
            .unwrap_or_else(|| self.0.last().unwrap_or_else(|| unreachable!()))
    }
}

impl Default for TextHeight {
    #[inline]
    fn default() -> Self {
        // From: https://github.com/ijprest/keyboard-layout-editor/blob/d2945e5/kb.css#L113
        #[allow(clippy::cast_precision_loss)] // i <= 9
        Self(array::from_fn(|i| 6.0 + 2.0 * (i as f32)).map(|sz| KeyUnit(sz / 72.0).convert_into()))
    }
}

/// Text margin mapping. This maps a [`usize`] index (used by KLE for example)
/// to a [`OffsetRect`] for the text alignment relative to the key top
#[derive(Debug, Clone, Copy)]
pub struct TextMargin([OffsetRect<Dot>; Self::NUM_RECTS]);

impl TextMargin {
    const NUM_RECTS: usize = 10;

    /// Create a new [`TextMargin`] mapping from a [`HashMap`]
    #[inline]
    #[must_use]
    pub fn new(offsets: &HashMap<usize, OffsetRect<Dot>>) -> Self {
        // Get an array of all the offsets
        let mut offsets = array::from_fn(|i| offsets.get(&i));

        let default_offset = Self::default()
            .0
            .last()
            .copied()
            .unwrap_or_else(|| unreachable!("Self::default().0 is non-empty"));
        // Find the offset for the max index
        let mut last_offset = offsets
            .into_iter()
            .flatten()
            .last()
            .unwrap_or(&default_offset);

        // Set all Nones to the next Some(..) value by iterating in reverse
        for opt in offsets.iter_mut().rev() {
            if let Some(off) = *opt {
                last_offset = off;
            } else {
                *opt = Some(last_offset);
            }
        }

        // Map all Some(&offset) to offset
        Self(offsets.map(|opt| {
            opt.copied()
                .unwrap_or_else(|| unreachable!("Set to Some(..) in for loop"))
        }))
    }

    /// Get the text alignment for the given index relative to the key top
    #[inline]
    #[must_use]
    pub const fn get(&self, size_index: usize) -> OffsetRect<Dot> {
        if size_index < self.0.len() {
            self.0[size_index]
        } else {
            self.0[self.0.len() - 1] // Can't use .last() in a const fn
        }
    }
}

impl Default for TextMargin {
    #[inline]
    fn default() -> Self {
        Self([OffsetRect::splat(KeyUnit(0.05)).convert_into(); Self::NUM_RECTS])
    }
}

/// A key top surface
#[derive(Debug, Clone, Copy)]
pub struct TopSurface {
    /// The size of the key top
    pub size: Vector<Dot>,
    /// The corner radius for the key top
    pub radius: Dot,
    /// The offset of the key top relative to the key bottom
    pub y_offset: Dot,
}

impl TopSurface {
    pub(crate) fn rect(&self) -> Rect<Dot> {
        Rect::from_center_and_size(
            Point::<Dot>::convert_from(Point::splat(KeyUnit(0.5)))
                + Vector::new(Dot(0.0), self.y_offset),
            self.size,
        )
    }

    pub(crate) fn round_rect(&self) -> RoundRect<Dot> {
        RoundRect::from_rect_and_radii(self.rect(), Vector::splat(self.radius))
    }
}

impl Default for TopSurface {
    #[inline]
    fn default() -> Self {
        Self {
            size: Vector::new(KeyUnit(0.660), KeyUnit(0.735)).convert_into(),
            radius: KeyUnit(0.065).convert_into(),
            y_offset: KeyUnit(-0.0775).convert_into(),
        }
    }
}

/// A key bottom surface
#[derive(Debug, Clone, Copy)]
pub struct BottomSurface {
    /// The size of the key bottom
    pub size: Vector<Dot>,
    /// The corner radius of the key bottom
    pub radius: Dot,
}

impl BottomSurface {
    pub(crate) fn rect(&self) -> Rect<Dot> {
        Rect::from_center_and_size(
            Point::new(KeyUnit(0.5), KeyUnit(0.5)).convert_into(),
            self.size,
        )
    }

    pub(crate) fn round_rect(&self) -> RoundRect<Dot> {
        RoundRect::from_rect_and_radii(self.rect(), Vector::splat(self.radius))
    }
}

impl Default for BottomSurface {
    #[inline]
    fn default() -> Self {
        Self {
            size: Vector::splat(KeyUnit(0.95)).convert_into(),
            radius: KeyUnit(0.065).convert_into(),
        }
    }
}

#[derive(Clone, Copy)]
struct NonExhaustive;

/// A keyboard profile
#[derive(Clone)]
pub struct Profile {
    /// The type of profile
    pub typ: Type,
    /// The shape of the bottom surface
    pub bottom: BottomSurface,
    /// The shape of the top surface
    pub top: TopSurface,
    /// The margin mapping for legend text alignment
    pub text_margin: TextMargin,
    /// The legend text size mapping
    pub text_height: TextHeight,
    /// Homing properties
    pub homing: HomingProps,
    /// Hidden field to enforce non-exhaustive struct while still allowing instantiation using
    /// `..Default::default()` functional update syntax
    #[allow(private_interfaces)]
    #[doc(hidden)]
    pub __non_exhaustive: NonExhaustive,
}

impl fmt::Debug for Profile {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("Profile");
        let _ = dbg
            .field("typ", &self.typ)
            .field("bottom", &self.bottom)
            .field("top", &self.top)
            .field("text_margin", &self.text_margin)
            .field("text_height", &self.text_height)
            .field("homing", &self.homing);

        #[cfg(clippy)] // Suppress clippy::missing_fields_in_debug but only for this one field
        let _ = dbg.field("__non_exhaustive", &"NonExhaustive");

        dbg.finish()
    }
}

impl Profile {
    /// Load a profile from a TOML configuration file
    ///
    /// # Errors
    ///
    /// If there was an error parsing the file
    #[cfg(feature = "toml")]
    #[inline]
    pub fn from_toml(s: &str) -> de::Result<Self> {
        soml::from_str(s).map_err(de::Error::from)
    }

    /// Load a profile from a JSON configuration file
    ///
    /// # Errors
    ///
    /// If there was an error parsing the file
    #[cfg(feature = "json")]
    #[inline]
    pub fn from_json(s: &str) -> de::Result<Self> {
        serde_json::from_str(s).map_err(de::Error::from)
    }

    /// Get the key top rectangle for the profile
    #[inline]
    #[must_use]
    pub fn top_rect(&self) -> RoundRect<Dot> {
        self.top.round_rect()
    }

    /// Get the key bottom rectangle for the profile
    #[inline]
    #[must_use]
    pub fn bottom_rect(&self) -> RoundRect<Dot> {
        self.bottom.round_rect()
    }
}

impl Default for Profile {
    #[inline]
    fn default() -> Self {
        Self {
            typ: Type::default(),
            bottom: BottomSurface::default(),
            top: TopSurface::default(),
            text_margin: TextMargin::default(),
            text_height: TextHeight::default(),
            homing: HomingProps::default(),
            __non_exhaustive: NonExhaustive,
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use assert_matches::assert_matches;
    use indoc::indoc;
    use isclose::{assert_is_close, IsClose as _};
    use num_traits::ToPrimitive as _;

    use super::*;

    #[test]
    fn test_profile_type_depth() {
        assert_is_close!(Type::Cylindrical { depth: Dot(1.0) }.depth(), Dot(1.0));
        assert_is_close!(Type::Spherical { depth: Dot(0.5) }.depth(), Dot(0.5));
        assert_is_close!(Type::Flat.depth(), Dot(0.0));
    }

    #[test]
    fn test_profile_type_default() {
        assert_matches!(
            Type::default(),
            Type::Cylindrical { depth }
                if depth.is_close(&Dot::convert_from(Mm(1.0)))
        );
    }

    #[test]
    fn test_homing_props_default() {
        assert_matches!(HomingProps::default().default, Homing::Bar);
        assert_is_close!(
            HomingProps::default().scoop.depth,
            Dot::convert_from(Mm(2.0))
        );
        assert_is_close!(
            HomingProps::default().bar.size,
            Vector::convert_from(Vector::new(Mm(3.81), Mm(0.508)))
        );
        assert_is_close!(
            HomingProps::default().bar.y_offset,
            Dot::convert_from(Mm(6.35))
        );
        assert_is_close!(
            HomingProps::default().bump.diameter,
            Dot::convert_from(Mm(0.508))
        );
        assert_is_close!(
            HomingProps::default().bump.y_offset,
            Dot::convert_from(Mm(0.0))
        );
    }

    #[test]
    fn test_text_height_new() {
        let expected: [_; 10] =
            array::from_fn(|i| Dot::convert_from(KeyUnit(6.0 + 2.0 * i.to_f32().unwrap()) / 72.0));
        let result = TextHeight::new(&HashMap::new()).0;

        assert_eq!(expected.len(), result.len());
        for (e, r) in expected.iter().zip(result.iter()) {
            assert_is_close!(e, r);
        }

        let expected = [
            0.0, 60.0, 120.0, 180.0, 190.0, 210.0, 230.0, 280.0, 330.0, 380.0,
        ]
        .map(Dot);
        let result = TextHeight::new(&HashMap::from([
            (1, Dot(60.0)),
            (3, Dot(180.0)),
            (4, Dot(190.0)),
            (6, Dot(230.0)),
            (9, Dot(380.0)),
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
            (1, Dot(3.0)),
            (3, Dot(9.0)),
            (4, Dot(9.5)),
            (6, Dot(11.5)),
            (9, Dot(19.0)),
        ]));
        assert_is_close!(heights.get(5), Dot(10.5));
        assert_is_close!(heights.get(23), Dot(19.0));
    }

    #[test]
    fn test_text_height_default() {
        let heights = TextHeight::default();

        for (i, h) in heights.0.into_iter().enumerate() {
            assert_is_close!(
                h,
                Dot::convert_from(KeyUnit(6.0 + 2.0 * i.to_f32().unwrap()) / 72.0)
            );
        }
    }

    #[test]
    fn test_text_margin_new() {
        let expected = [OffsetRect::convert_from(OffsetRect::splat(KeyUnit(0.05))); 10];
        let result = TextMargin::new(&HashMap::new()).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_is_close!(e, r);
        }

        let expected = [
            OffsetRect::splat(Dot(0.0)),
            OffsetRect::splat(Dot(0.0)),
            OffsetRect::splat(Dot(0.0)),
            OffsetRect::splat(Dot(-50.0)),
            OffsetRect::splat(Dot(-50.0)),
            OffsetRect::splat(Dot(-50.0)),
            OffsetRect::splat(Dot(-100.0)),
            OffsetRect::splat(Dot(-100.0)),
            OffsetRect::splat(Dot(-100.0)),
            OffsetRect::splat(Dot(-100.0)),
        ];
        let result = TextMargin::new(&HashMap::from([
            (2, OffsetRect::splat(Dot(0.0))),
            (5, OffsetRect::splat(Dot(-50.0))),
            (7, OffsetRect::splat(Dot(-100.0))),
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
            (2, OffsetRect::splat(Dot(0.0))),
            (5, OffsetRect::splat(Dot(-50.0))),
            (7, OffsetRect::splat(Dot(-100.0))),
        ]));

        let offsets = margin.get(2);
        assert_is_close!(offsets, OffsetRect::zero());

        let offsets = margin.get(62);
        assert_is_close!(offsets, OffsetRect::splat(Dot(-100.0)));
    }

    #[test]
    fn test_text_margin_default() {
        let margin = TextMargin::default();

        for offsets in margin.0 {
            assert_is_close!(
                offsets,
                OffsetRect::convert_from(OffsetRect::splat(KeyUnit(0.05)))
            );
        }
    }

    #[test]
    fn test_top_surface_rect() {
        let surf = TopSurface::default();
        assert_is_close!(
            surf.rect(),
            Rect::from_origin_and_size(
                Point::new(KeyUnit(0.170), KeyUnit(0.055)).convert_into(),
                Vector::new(KeyUnit(0.660), KeyUnit(0.735)).convert_into()
            )
        );
    }

    #[test]
    fn test_top_surface_round_rect() {
        let surf = TopSurface::default();
        assert_is_close!(
            surf.round_rect(),
            RoundRect::new(
                Point::new(KeyUnit(0.170), KeyUnit(0.055)).convert_into(),
                Point::new(KeyUnit(0.830), KeyUnit(0.790)).convert_into(),
                Vector::splat(KeyUnit(0.065)).convert_into(),
            )
        );
    }

    #[test]
    fn test_top_surface_default() {
        let surf = TopSurface::default();
        assert_is_close!(
            surf.size,
            Vector::convert_from(Vector::new(KeyUnit(0.660), KeyUnit(0.735)))
        );
        assert_is_close!(surf.radius, Dot::convert_from(KeyUnit(0.065)));
        assert_is_close!(surf.y_offset, Dot::convert_from(KeyUnit(-0.0775)));
    }

    #[test]
    fn test_bottom_surface_rect() {
        let surf = BottomSurface::default();
        assert_is_close!(
            surf.rect(),
            Rect::new(
                Point::new(KeyUnit(0.025), KeyUnit(0.025)).convert_into(),
                Point::new(KeyUnit(0.975), KeyUnit(0.975)).convert_into()
            )
        );
    }

    #[test]
    fn test_bottom_surface_round_rect() {
        let surf = BottomSurface::default();
        assert_is_close!(
            surf.round_rect(),
            RoundRect::new(
                Point::new(KeyUnit(0.025), KeyUnit(0.025)).convert_into(),
                Point::new(KeyUnit(0.975), KeyUnit(0.975)).convert_into(),
                Vector::<KeyUnit>::splat(KeyUnit(0.065)).convert_into()
            )
        );
    }

    #[test]
    fn test_bottom_surface_default() {
        let surf = BottomSurface::default();
        assert_is_close!(
            surf.size,
            Vector::convert_from(Vector::new(KeyUnit(0.950), KeyUnit(0.950)))
        );
        assert_is_close!(surf.radius, Dot::convert_from(KeyUnit(0.065)));
    }

    #[test]
    fn profile_debug() {
        let profile = Profile::default();

        assert_eq!(
            format!("{profile:?}"),
            format!(
                "Profile {{ typ: {:?}, bottom: {:?}, top: {:?}, text_margin: {:?}, \
                text_height: {:?}, homing: {:?} }}",
                Type::default(),
                BottomSurface::default(),
                TopSurface::default(),
                TextMargin::default(),
                TextHeight::default(),
                HomingProps::default(),
            )
        );
    }

    #[cfg(feature = "toml")]
    const PROFILE_TOML: &str = indoc!(
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
        "
    );

    #[cfg(feature = "toml")]
    #[test]
    fn test_profile_from_toml() {
        let profile = Profile::from_toml(PROFILE_TOML).unwrap();

        assert_matches!(
            profile.typ,
            Type::Cylindrical { depth }
                if depth.is_close(&Dot::convert_from(Mm(0.5)))
        );

        assert_is_close!(
            profile.bottom.size,
            Vector::convert_from(Vector::splat(Mm(18.29)))
        );
        assert_is_close!(profile.bottom.radius, Dot::convert_from(Mm(0.38)));

        assert_is_close!(
            profile.top.size,
            Vector::convert_from(Vector::new(Mm(11.81), Mm(13.91)))
        );
        assert_is_close!(profile.top.radius, Dot::convert_from(Mm(1.52)));
        assert_is_close!(profile.top.y_offset, Dot::convert_from(Mm(-1.62)));

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = [
            0.0, 0.76, 1.52, 2.28, 3.18, 4.84, 6.5, 8.16, 9.82, 11.48, 13.14,
        ]
        .map(Mm)
        .map(Dot::convert_from);
        for (e, r) in expected.iter().zip(profile.text_height.0.iter()) {
            assert_is_close!(e, r);
        }

        assert_eq!(profile.text_margin.0.len(), 10);
        let expected = [
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.425), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.425), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.425), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.425), Mm(1.18)),
            OffsetRect::new(Mm(2.575), Mm(1.14), Mm(1.775), Mm(1.14)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18)),
        ]
        .map(OffsetRect::convert_from);
        for (e, r) in expected.iter().zip(profile.text_margin.0.iter()) {
            assert_is_close!(e, r);
        }

        assert_matches!(profile.homing.default, Homing::Scoop);
        assert_is_close!(profile.homing.scoop.depth, Dot::convert_from(Mm(1.5)));
        assert_is_close!(
            profile.homing.bar.size,
            Vector::convert_from(Vector::new(Mm(3.85), Mm(0.4)))
        );
        assert_is_close!(profile.homing.bar.y_offset, Dot::convert_from(Mm(5.05)));
        assert_is_close!(profile.homing.bump.diameter, Dot::convert_from(Mm(0.4)));
        assert_is_close!(profile.homing.bump.y_offset, Dot::convert_from(Mm(-0.2)));
    }

    #[cfg(feature = "toml")]
    #[test]
    fn test_profile_from_invalid_toml() {
        let result = Profile::from_toml("null");
        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "expected = after key");
    }

    #[cfg(feature = "json")]
    const PROFILE_JSON: &str = indoc!(
        r#"
        {
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
        }
        "#,
    );

    #[cfg(feature = "json")]
    #[test]
    fn test_profile_from_json() {
        let profile = Profile::from_json(PROFILE_JSON).unwrap();

        assert_matches!(
            profile.typ,
            Type::Cylindrical { depth }
                if depth.is_close(&Dot::convert_from(Mm(0.5)))
        );

        assert_is_close!(
            profile.bottom.size,
            Vector::convert_from(Vector::splat(Mm(18.29)))
        );
        assert_is_close!(profile.bottom.radius, Dot::convert_from(Mm(0.38)));

        assert_is_close!(
            profile.top.size,
            Vector::convert_from(Vector::new(Mm(11.81), Mm(13.91)))
        );
        assert_is_close!(profile.top.radius, Dot::convert_from(Mm(1.52)));
        assert_is_close!(profile.top.y_offset, Dot::convert_from(Mm(-1.62)));

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = [
            0.0, 0.76, 1.52, 2.28, 3.18, 4.84, 6.5, 8.16, 9.82, 11.48, 13.14,
        ]
        .map(Mm)
        .map(Dot::convert_from);
        for (e, r) in expected.iter().zip(profile.text_height.0.iter()) {
            assert_is_close!(e, r);
        }

        assert_eq!(profile.text_margin.0.len(), 10);
        let expected = [
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.425), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.425), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.425), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.425), Mm(1.18)),
            OffsetRect::new(Mm(2.575), Mm(1.14), Mm(1.775), Mm(1.14)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18)),
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18)),
        ]
        .map(OffsetRect::convert_from);
        for (e, r) in expected.iter().zip(profile.text_margin.0.iter()) {
            assert_is_close!(e, r);
        }

        assert_matches!(profile.homing.default, Homing::Scoop);
        assert_is_close!(profile.homing.scoop.depth, Dot::convert_from(Mm(1.5)));
        assert_is_close!(
            profile.homing.bar.size,
            Vector::convert_from(Vector::new(Mm(3.85), Mm(0.4)))
        );
        assert_is_close!(profile.homing.bar.y_offset, Dot::convert_from(Mm(5.05)));
        assert_is_close!(profile.homing.bump.diameter, Dot::convert_from(Mm(0.4)));
        assert_is_close!(profile.homing.bump.y_offset, Dot::convert_from(Mm(-0.2)));
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
    fn test_profile_rect() {
        let profile = Profile::default();

        let top = profile.top_rect();
        let exp = RoundRect::from_center_size_and_radii(
            Point::convert_from(Point::splat(KeyUnit(0.5)))
                + Vector::new(Dot(0.0), profile.top.y_offset),
            profile.top.size,
            Vector::splat(profile.top.radius),
        );
        assert_is_close!(top, exp);

        let bottom = profile.bottom_rect();
        let exp = RoundRect::from_center_size_and_radii(
            Point::splat(KeyUnit(0.5)).convert_into(),
            profile.bottom.size,
            Vector::splat(profile.bottom.radius),
        );
        assert_is_close!(bottom, exp);
    }

    #[test]
    fn test_profile_default() {
        let profile = Profile::default();

        assert_matches!(
            profile.typ,
            Type::Cylindrical { depth }
                if depth.is_close(&Dot::convert_from(Mm(1.0)))
        );

        assert_is_close!(
            profile.bottom.size,
            Vector::convert_from(Vector::splat(KeyUnit(0.950)))
        );
        assert_is_close!(profile.bottom.radius, Dot::convert_from(KeyUnit(0.065)));

        assert_is_close!(
            profile.top.size,
            Vector::convert_from(Vector::new(KeyUnit(0.660), KeyUnit(0.735)))
        );
        assert_is_close!(profile.top.radius, Dot::convert_from(KeyUnit(0.065)));
        assert_is_close!(profile.top.y_offset, Dot::convert_from(KeyUnit(-0.0775)));

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
