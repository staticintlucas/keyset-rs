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
use saturate::SaturatingFrom as _;

use geom::{
    Dist, Dot, ExtRect as _, Inch, IntoUnit as _, KeyUnit, Mm, Point, Rect, RoundRect, SideOffsets,
    Size, Vector, DOT_PER_INCH, DOT_PER_UNIT,
};
use key::Homing;

/// The type of a profile
#[derive(Debug, Clone, Copy)]
pub enum Type {
    /// A cylindrical profile, e.g. Cherry or OEM
    Cylindrical {
        /// The depth of the key's dish
        depth: Dist<Dot>,
    },
    /// A cylindrical profile, e.g. SA or KAT
    Spherical {
        /// The depth of the key's dish
        depth: Dist<Dot>,
    },
    /// A flat profile, e.g. G20 or chiclet
    Flat,
}

impl Type {
    /// Returns the depth of a key's dish. This is zero for [`Type::Flat`]
    #[inline]
    #[must_use]
    pub const fn depth(self) -> Dist<Dot> {
        match self {
            Self::Cylindrical { depth } | Self::Spherical { depth } => depth,
            Self::Flat => Dist::new(Dot(0.0)),
        }
    }
}

impl Default for Type {
    #[inline]
    fn default() -> Self {
        Self::Cylindrical {
            // 1.0mm is approx the depth of OEM profile
            depth: Dist::new(Mm(1.0)).into_unit(),
        }
    }
}

/// Scooped (a.k.a. deep dish) homing key properties
#[derive(Debug, Clone, Copy)]
pub struct ScoopProps {
    /// The depth of the scooped dish
    pub depth: Dist<Dot>,
}

/// Homing bar properties
#[derive(Debug, Clone, Copy)]
pub struct BarProps {
    /// The size of the bar
    pub size: Size<Dot>,
    /// The distance of the bar from the center of the key top
    pub y_offset: Dist<Dot>,
}

/// Homing bump (a.k.a. nub or nipple) properties
#[derive(Debug, Clone, Copy)]
pub struct BumpProps {
    /// The diameter of the bump
    pub diameter: Dist<Dot>,
    /// The distance of the bump from the center of the key top
    pub y_offset: Dist<Dot>,
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
                size: Size::<Inch>::new(0.15, 0.02) * DOT_PER_INCH,
                y_offset: Dist::new(Inch(0.25)).into_unit(),
            },
            bump: BumpProps {
                diameter: Dist::new(Inch(0.02)).into_unit(),
                y_offset: Dist::new(Inch(0.0)).into_unit(),
            },
        }
    }
}

/// Text height mapping. This maps a [`usize`] index (used by KLE for example)
/// to a [`Length`] for the height of uppercase letter
#[derive(Debug, Clone, Copy)]
pub struct TextHeight([Dist<Dot>; Self::NUM_HEIGHTS]);

impl TextHeight {
    const NUM_HEIGHTS: usize = 10;

    /// Create a new [`TextHeight`] mapping from a [`HashMap`]
    #[inline]
    #[must_use]
    pub fn new(heights: &HashMap<usize, Dist<Dot>>) -> Self {
        if heights.is_empty() {
            Self::default()
        } else {
            // Get all the indices and heights in the hashmap
            let (indices, heights): (Vec<_>, Vec<_>) = {
                // TODO is unconditionally prepending (0, 0.0) here correct? Self::default() doesn't
                // use 0.0 font size for 0
                let mut vec = Vec::with_capacity(heights.len().saturating_add(1));
                vec.push((0, 0.0));
                vec.extend(heights.iter().map(|(&i, &h)| (i, h.into())));
                vec.sort_unstable_by_key(|&(i, _h)| i);
                vec.into_iter()
                    .map(|(i, h)| (f32::saturating_from(i), h))
                    .unzip()
            };

            let all_indices = array::from_fn(f32::saturating_from);
            Self(
                interp_array(&indices, &heights, &all_indices, &InterpMode::Extrapolate)
                    .map(Dist::<Dot>::from),
            )
        }
    }

    /// Get the height of an uppercase letter for the given index
    #[inline]
    #[must_use]
    pub fn get(&self, size_index: usize) -> Dist<Dot> {
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
        Self(
            array::from_fn(|i| 6.0 + 2.0 * f32::saturating_from(i))
                .map(|sz| Dist::new(KeyUnit(sz / 72.0)).into_unit()),
        )
    }
}

/// Text margin mapping. This maps a [`usize`] index (used by KLE for example)
/// to a [`SideOffsets`] for the text alignment relative to the key top
#[derive(Debug, Clone, Copy)]
pub struct TextMargin([SideOffsets<Dot>; Self::NUM_RECTS]);

impl TextMargin {
    const NUM_RECTS: usize = 10;

    /// Create a new [`TextMargin`] mapping from a [`HashMap`]
    #[inline]
    #[must_use]
    pub fn new(offsets: &HashMap<usize, SideOffsets<Dot>>) -> Self {
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
    pub const fn get(&self, size_index: usize) -> SideOffsets<Dot> {
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
        Self([SideOffsets::<KeyUnit>::new_all_same(0.05) * DOT_PER_UNIT; Self::NUM_RECTS])
    }
}

/// A key top surface
#[derive(Debug, Clone, Copy)]
pub struct TopSurface {
    /// The size of the key top
    pub size: Size<Dot>,
    /// The corner radius for the key top
    pub radius: Dist<Dot>,
    /// The offset of the key top relative to the key bottom
    pub y_offset: Dist<Dot>,
}

impl TopSurface {
    pub(crate) fn rect(&self) -> Rect<Dot> {
        Rect::from_center_and_size(
            (Point::new(0.5, 0.5) * DOT_PER_UNIT) + Vector::new(0.0, self.y_offset.into()),
            self.size,
        )
    }

    pub(crate) fn round_rect(&self) -> RoundRect<Dot> {
        RoundRect::from_rect(self.rect(), self.radius)
    }
}

impl Default for TopSurface {
    #[inline]
    fn default() -> Self {
        Self {
            size: Size::<KeyUnit>::new(0.660, 0.735) * DOT_PER_UNIT,
            radius: Dist::new(KeyUnit(0.065)).into_unit(),
            y_offset: Dist::new(KeyUnit(-0.0775)).into_unit(),
        }
    }
}

/// A key bottom surface
#[derive(Debug, Clone, Copy)]
pub struct BottomSurface {
    /// The size of the key bottom
    pub size: Size<Dot>,
    /// The corner radius of the key bottom
    pub radius: Dist<Dot>,
}

impl BottomSurface {
    pub(crate) fn rect(&self) -> Rect<Dot> {
        Rect::from_center_and_size(Point::<KeyUnit>::new(0.5, 0.5) * DOT_PER_UNIT, self.size)
    }

    pub(crate) fn round_rect(&self) -> RoundRect<Dot> {
        RoundRect::from_rect(self.rect(), self.radius)
    }
}

impl Default for BottomSurface {
    #[inline]
    fn default() -> Self {
        Self {
            size: Size::<KeyUnit>::splat(0.95) * DOT_PER_UNIT,
            radius: Dist::new(KeyUnit(0.065)).into_unit(),
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

    /// Get the key top rectangle for a given key size
    #[inline]
    #[must_use]
    pub fn top_with_size(&self, size: Size<KeyUnit>) -> RoundRect<Dot> {
        let RoundRect { min, max, radius } = self.top.round_rect();
        let max = max + (size - Size::splat(1.0)) * DOT_PER_UNIT;
        RoundRect::new(min, max, radius)
    }

    /// Get the key top rectangle for a given key rect
    #[inline]
    #[must_use]
    pub fn top_with_rect(&self, rect: Rect<KeyUnit>) -> RoundRect<Dot> {
        let RoundRect { min, max, radius } = self.top.round_rect();
        let min = min + rect.min.to_vector() * DOT_PER_UNIT;
        let max = max + (rect.max.to_vector() - Vector::splat(1.0)) * DOT_PER_UNIT;
        RoundRect::new(min, max, radius)
    }

    /// Get the key bottom rectangle for a given key size
    #[inline]
    #[must_use]
    pub fn bottom_with_size(&self, size: Size<KeyUnit>) -> RoundRect<Dot> {
        let RoundRect { min, max, radius } = self.bottom.round_rect();
        let max = max + (size - Size::splat(1.0)) * DOT_PER_UNIT;
        RoundRect::new(min, max, radius)
    }

    /// Get the key bottom rectangle for a given key rectangle
    #[inline]
    #[must_use]
    pub fn bottom_with_rect(&self, rect: Rect<KeyUnit>) -> RoundRect<Dot> {
        let RoundRect { min, max, radius } = self.bottom.round_rect();
        let min = min + rect.min.to_vector() * DOT_PER_UNIT;
        let max = max + (rect.max.to_vector() - Vector::splat(1.0)) * DOT_PER_UNIT;
        RoundRect::new(min, max, radius)
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

    use geom::DOT_PER_MM;

    use super::*;

    #[test]
    fn test_profile_type_depth() {
        assert_is_close!(
            Type::Cylindrical {
                depth: Dist::from(1.0)
            }
            .depth(),
            Dist::from(1.0)
        );
        assert_is_close!(
            Type::Spherical {
                depth: Dist::from(0.5)
            }
            .depth(),
            Dist::from(0.5)
        );
        assert_is_close!(Type::Flat.depth(), Dist::from(0.0));
    }

    #[test]
    fn test_profile_type_default() {
        assert_matches!(Type::default(), Type::Cylindrical { depth } if depth.is_close(Dist::new(Mm(1.0).into_unit())));
    }

    #[test]
    fn test_homing_props_default() {
        assert_matches!(HomingProps::default().default, Homing::Bar);
        assert_is_close!(
            HomingProps::default().scoop.depth,
            Dist::new(Mm(2.0).into_unit())
        );
        assert_is_close!(
            HomingProps::default().bar.size,
            Size::<Mm>::new(3.81, 0.508) * DOT_PER_MM
        );
        assert_is_close!(
            HomingProps::default().bar.y_offset,
            Dist::new(Mm(6.35).into_unit())
        );
        assert_is_close!(
            HomingProps::default().bump.diameter,
            Dist::new(Mm(0.508).into_unit())
        );
        assert_is_close!(
            HomingProps::default().bump.y_offset,
            Dist::new(Mm(0.0).into_unit())
        );
    }

    #[test]
    fn test_text_height_new() {
        let expected: [_; 10] = array::from_fn(|i| {
            Dist::<Dot>::new(KeyUnit(6.0 + 2.0 * f32::saturating_from(i)).into_unit()) / 72.0
        });
        let result = TextHeight::new(&HashMap::new()).0;

        assert_eq!(expected.len(), result.len());
        for (e, r) in expected.iter().zip(result.iter()) {
            assert_is_close!(e, r);
        }

        let expected = [
            0.0, 60.0, 120.0, 180.0, 190.0, 210.0, 230.0, 280.0, 330.0, 380.0,
        ]
        .map(Dist::<Dot>::from);
        let result = TextHeight::new(&HashMap::from([
            (1, Dist::from(60.0)),
            (3, Dist::from(180.0)),
            (4, Dist::from(190.0)),
            (6, Dist::from(230.0)),
            (9, Dist::from(380.0)),
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
            (1, Dist::from(3.0)),
            (3, Dist::from(9.0)),
            (4, Dist::from(9.5)),
            (6, Dist::from(11.5)),
            (9, Dist::from(19.0)),
        ]));
        assert_is_close!(heights.get(5), Dist::from(10.5));
        assert_is_close!(heights.get(23), Dist::from(19.0));
    }

    #[test]
    fn test_text_height_default() {
        let heights = TextHeight::default();

        for (i, h) in heights.0.into_iter().enumerate() {
            assert_is_close!(
                h,
                Dist::<Dot>::new(KeyUnit(6.0 + 2.0 * f32::saturating_from(i)).into_unit()) / 72.0
            );
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
                Dist::new(KeyUnit(0.065)).into_unit()
            )
        );
    }

    #[test]
    fn test_top_surface_default() {
        let surf = TopSurface::default();
        assert_is_close!(surf.size, Size::new(0.660, 0.735) * DOT_PER_UNIT);
        assert_is_close!(surf.radius, Dist::new(KeyUnit(0.065).into_unit()));
        assert_is_close!(surf.y_offset, Dist::new(KeyUnit(-0.0775).into_unit()));
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
                Dist::new(KeyUnit(0.065)).into_unit()
            )
        );
    }

    #[test]
    fn test_bottom_surface_default() {
        let surf = BottomSurface::default();
        assert_is_close!(surf.size, Size::new(0.950, 0.950) * DOT_PER_UNIT);
        assert_is_close!(surf.radius, Dist::new(KeyUnit(0.065).into_unit()));
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
        use geom::DOT_PER_MM;

        let profile = Profile::from_toml(PROFILE_TOML).unwrap();

        assert!(
            matches!(profile.typ, Type::Cylindrical { depth } if depth.is_close(Dist::new(Mm(0.5).into_unit())))
        );

        assert_is_close!(profile.bottom.size, Size::splat(18.29) * DOT_PER_MM);
        assert_is_close!(profile.bottom.radius, Dist::new(Mm(0.38).into_unit()));

        assert_is_close!(profile.top.size, Size::new(11.81, 13.91) * DOT_PER_MM);
        assert_is_close!(profile.top.radius, Dist::new(Mm(1.52).into_unit()));
        assert_is_close!(profile.top.y_offset, Dist::new(Mm(-1.62).into_unit()));

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = [
            0.0, 0.76, 1.52, 2.28, 3.18, 4.84, 6.5, 8.16, 9.82, 11.48, 13.14,
        ]
        .map(|e| Dist::new(Mm(e).into_unit()));
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
        assert_is_close!(profile.homing.scoop.depth, Dist::new(Mm(1.5).into_unit()));
        assert_is_close!(
            profile.homing.bar.size,
            Size::<Mm>::new(3.85, 0.4) * DOT_PER_MM
        );
        assert_is_close!(profile.homing.bar.y_offset, Dist::new(Mm(5.05).into_unit()));
        assert_is_close!(profile.homing.bump.diameter, Dist::new(Mm(0.4).into_unit()));
        assert_is_close!(
            profile.homing.bump.y_offset,
            Dist::new(Mm(-0.2).into_unit())
        );
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
        use geom::DOT_PER_MM;

        let profile = Profile::from_json(PROFILE_JSON).unwrap();

        assert_matches!(profile.typ, Type::Cylindrical { depth } if depth.is_close(Dist::new(Mm(0.5).into_unit())));

        assert_is_close!(profile.bottom.size, Size::splat(18.29) * DOT_PER_MM);
        assert_is_close!(profile.bottom.radius, Dist::new(Mm(0.38).into_unit()));

        assert_is_close!(profile.top.size, Size::new(11.81, 13.91) * DOT_PER_MM);
        assert_is_close!(profile.top.radius, Dist::new(Mm(1.52).into_unit()));
        assert_is_close!(profile.top.y_offset, Dist::new(Mm(-1.62).into_unit()));

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = [
            0.0, 0.76, 1.52, 2.28, 3.18, 4.84, 6.5, 8.16, 9.82, 11.48, 13.14,
        ]
        .map(|e| Dist::new(Mm(e).into_unit()));
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
        .map(|e| e * DOT_PER_MM);
        for (e, r) in expected.iter().zip(profile.text_margin.0.iter()) {
            assert_is_close!(e, r);
        }

        assert_matches!(profile.homing.default, Homing::Scoop);
        assert_is_close!(profile.homing.scoop.depth, Dist::new(Mm(1.5).into_unit()));
        assert_is_close!(
            profile.homing.bar.size,
            Size::<Mm>::new(3.85, 0.4) * DOT_PER_MM
        );
        assert_is_close!(profile.homing.bar.y_offset, Dist::new(Mm(5.05).into_unit()));
        assert_is_close!(profile.homing.bump.diameter, Dist::new(Mm(0.4).into_unit()));
        assert_is_close!(
            profile.homing.bump.y_offset,
            Dist::new(Mm(-0.2).into_unit())
        );
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
            Point::splat(0.5) * DOT_PER_UNIT + Vector::new(0.0, profile.top.y_offset.into()),
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
            Point::new(1.5, 1.0) * DOT_PER_UNIT + Vector::new(0.0, profile.top.y_offset.into()),
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

        assert_matches!(profile.typ, Type::Cylindrical { depth } if depth.is_close(Dist::new(Mm(1.0).into_unit())));

        assert_is_close!(profile.bottom.size, Size::splat(0.950) * DOT_PER_UNIT);
        assert_is_close!(profile.bottom.radius, Dist::new(KeyUnit(0.065).into_unit()));

        assert_is_close!(profile.top.size, Size::new(0.660, 0.735) * DOT_PER_UNIT);
        assert_is_close!(profile.top.radius, Dist::new(KeyUnit(0.065).into_unit()));
        assert_is_close!(
            profile.top.y_offset,
            Dist::new(KeyUnit(-0.0775).into_unit())
        );

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
