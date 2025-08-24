//! This crate contains the profile struct and types used for describing
//! profiles used by [keyset]. It also contains utility functions for loading
//! profiles from file
//!
//! [keyset]: https://crates.io/crates/keyset

#![cfg_attr(coverage, expect(unstable_features))]
#![cfg_attr(coverage, feature(coverage_attribute))]

#[cfg(feature = "serde")]
mod de;

use std::fmt;

use geom::{ConvertInto as _, Dot, Inch, KeyUnit, Mm, OffsetRect, Point, Rect, RoundRect, Vector};
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

/// The geometry of a legend.
#[derive(Debug, Clone, Copy)]
pub struct LegendGeom {
    /// The height of the legend text.
    ///
    /// <div class="warning">
    ///
    /// This field corresponds to the height of an uppercase letter (i.e. cap height),
    /// not to the font size.
    /// The actual font size is a more abstract measure that varies from one font to another.
    ///
    /// </div>
    pub height: Dot,
    /// The margin between the legend and the edge of the key top.
    pub margin: OffsetRect<Dot>,
}

/// The profile's legend geometry mapping.
///
/// When using a KLE layout, size 3 in KLE is mapped to `modifier`, 4 to `symbol`, and 5 to `alpha`.
/// For more complex use cases, the geometry can also be set on a per-key per-legend basis
/// which override the defaults set by the profile.
// TODO: insert link for above
/// Many profiles use the same the margins all legend sizes,
/// but some (e.g. OG/GMK Cherry profile) do vary slightly based on legend size.
#[derive(Debug, Clone, Copy)]
pub struct LegendGeomMap {
    /// The legend geometry for alpha and mono legends
    pub alpha: LegendGeom,
    /// The legend geometry for symbol and number row legends
    pub symbol: LegendGeom,
    /// The legend geometry for modifier legends
    pub modifier: LegendGeom,
}

impl LegendGeomMap {
    /// Get the legend geometry for the given KLE size
    #[inline]
    #[must_use]
    pub fn for_kle_size(&self, kle_size: usize) -> LegendGeom {
        LegendGeom {
            height: self.height_for_kle_size(kle_size),
            margin: self.margin_for_kle_size(kle_size),
        }
    }

    /// Get the legend height for the given KLE size
    #[inline]
    #[must_use]
    pub fn height_for_kle_size(&self, kle_size: usize) -> Dot {
        match kle_size {
            3 => self.modifier.height,
            4 => self.symbol.height,
            5 => self.alpha.height,
            _ => Self::kle_approx(kle_size),
        }
    }

    /// Get the legend margin for the given KLE size
    #[inline]
    #[must_use]
    pub const fn margin_for_kle_size(&self, kle_size: usize) -> OffsetRect<Dot> {
        match kle_size {
            ..=3 => self.modifier.margin,
            4 => self.symbol.margin,
            5.. => self.alpha.margin,
        }
    }

    // Approximates the legend height for KLE size
    fn kle_approx(kle_size: usize) -> Dot {
        // KLE uses (6 + 2 * size) px on a 72 px key, default to an approximation of that
        // Ref: https://github.com/ijprest/keyboard-layout-editor/blob/d2945e5/kb.css#L113
        #[expect(clippy::cast_precision_loss, reason = "usize <= 9")]
        KeyUnit((6.0 + 2.0 * usize::min(kle_size, 9) as f32) / 72.0).convert_into()
    }
}

impl Default for LegendGeomMap {
    #[inline]
    fn default() -> Self {
        let margin = OffsetRect::splat(KeyUnit(0.05)).convert_into();
        Self {
            alpha: LegendGeom {
                height: Self::kle_approx(5),
                margin,
            },
            symbol: LegendGeom {
                height: Self::kle_approx(4),
                margin,
            },
            modifier: LegendGeom {
                height: Self::kle_approx(3),
                margin,
            },
        }
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
    /// Get the key top as a rectangle
    #[inline]
    #[must_use]
    pub fn to_rect(&self) -> Rect<Dot> {
        Rect::from_center_and_size(
            Point::splat(KeyUnit(0.5).convert_into()) + Vector::new(Dot(0.0), self.y_offset),
            self.size,
        )
    }

    /// Get the key top as a rounded rectangle
    #[inline]
    #[must_use]
    pub fn to_round_rect(&self) -> RoundRect<Dot> {
        RoundRect::from_rect_and_radii(self.to_rect(), Vector::splat(self.radius))
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
    /// Get the key bottom as a rectangle
    #[inline]
    #[must_use]
    pub fn to_rect(&self) -> Rect<Dot> {
        Rect::from_center_and_size(
            Point::new(KeyUnit(0.5), KeyUnit(0.5)).convert_into(),
            self.size,
        )
    }

    /// Get the key bottom as a rounded rectangle
    #[inline]
    #[must_use]
    pub fn to_round_rect(&self) -> RoundRect<Dot> {
        RoundRect::from_rect_and_radii(self.to_rect(), Vector::splat(self.radius))
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
#[derive(Clone, Copy)]
pub struct Profile {
    /// The type of profile
    pub typ: Type,
    /// The shape of the bottom surface
    pub bottom: BottomSurface,
    /// The shape of the top surface
    pub top: TopSurface,
    /// The legend size and marging geometry mapping
    pub legend_geom: LegendGeomMap,
    /// Homing properties
    pub homing: HomingProps,
    #[expect(
        private_interfaces,
        reason = "enforces non-exhaustive struct while still allowing functional update syntax"
    )]
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
            .field("legend_geom", &self.legend_geom)
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
}

impl Default for Profile {
    #[inline]
    fn default() -> Self {
        Self {
            typ: Type::default(),
            bottom: BottomSurface::default(),
            top: TopSurface::default(),
            legend_geom: LegendGeomMap::default(),
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
    fn type_depth() {
        assert_is_close!(Type::Cylindrical { depth: Dot(1.0) }.depth(), Dot(1.0));
        assert_is_close!(Type::Spherical { depth: Dot(0.5) }.depth(), Dot(0.5));
        assert_is_close!(Type::Flat.depth(), Dot(0.0));
    }

    #[test]
    fn type_default() {
        assert_matches!(
            Type::default(),
            Type::Cylindrical { depth }
                if depth.is_close(&Mm(1.0))
        );
    }

    #[test]
    fn homing_props_default() {
        assert_matches!(HomingProps::default().default, Homing::Bar);
        assert_is_close!(HomingProps::default().scoop.depth, Mm(2.0));
        assert_is_close!(
            HomingProps::default().bar.size,
            Vector::new(Mm(3.81), Mm(0.508))
        );
        assert_is_close!(HomingProps::default().bar.y_offset, Mm(6.35));
        assert_is_close!(HomingProps::default().bump.diameter, Mm(0.508));
        assert_is_close!(HomingProps::default().bump.y_offset, Mm(0.0));
    }

    #[test]
    fn legend_geom_map_for_kle_size() {
        let geom_map = LegendGeomMap {
            alpha: LegendGeom {
                height: Dot(11.5),
                margin: OffsetRect::splat(Dot(-100.0)),
            },
            symbol: LegendGeom {
                height: Dot(9.5),
                margin: OffsetRect::splat(Dot(-50.0)),
            },
            modifier: LegendGeom {
                height: Dot(9.0),
                margin: OffsetRect::splat(Dot(0.0)),
            },
        };

        let geom = geom_map.for_kle_size(1);
        assert_is_close!(geom.height, KeyUnit(8.0 / 72.0));
        assert_is_close!(geom.margin, OffsetRect::<Dot>::zero());

        let geom = geom_map.for_kle_size(3);
        assert_is_close!(geom.height, Dot(9.0));
        assert_is_close!(geom.margin, OffsetRect::<Dot>::zero());

        let geom = geom_map.for_kle_size(4);
        assert_is_close!(geom.height, Dot(9.5));
        assert_is_close!(geom.margin, OffsetRect::splat(Dot(-50.0)));

        let geom = geom_map.for_kle_size(5);
        assert_is_close!(geom.height, Dot(11.5));
        assert_is_close!(geom.margin, OffsetRect::splat(Dot(-100.0)));

        let geom = geom_map.for_kle_size(9);
        assert_is_close!(geom.height, KeyUnit(24.0 / 72.0));
        assert_is_close!(geom.margin, OffsetRect::splat(Dot(-100.0)));

        let geom = geom_map.for_kle_size(23);
        assert_is_close!(geom.height, KeyUnit(24.0 / 72.0));
        assert_is_close!(geom.margin, OffsetRect::splat(Dot(-100.0)));
    }

    #[test]
    fn legend_geom_map_default() {
        let geom_map = LegendGeomMap::default();

        assert_is_close!(geom_map.alpha.height, KeyUnit(16.0 / 72.0));
        assert_is_close!(geom_map.alpha.margin, OffsetRect::splat(KeyUnit(0.05)));
        assert_is_close!(geom_map.symbol.height, KeyUnit(14.0 / 72.0));
        assert_is_close!(geom_map.symbol.margin, OffsetRect::splat(KeyUnit(0.05)));
        assert_is_close!(geom_map.modifier.height, KeyUnit(12.0 / 72.0));
        assert_is_close!(geom_map.modifier.margin, OffsetRect::splat(KeyUnit(0.05)));

        for i in 1..=9 {
            let geom = geom_map.for_kle_size(i);
            assert_is_close!(geom.height, KeyUnit(6.0 + 2.0 * i.to_f32().unwrap()) / 72.0);
            assert_is_close!(geom.margin, OffsetRect::splat(KeyUnit(0.05)));
        }
    }

    #[test]
    fn top_surface_to_rect() {
        let surf = TopSurface::default();
        assert_is_close!(
            surf.to_rect(),
            Rect::from_origin_and_size(
                Point::new(KeyUnit(0.170), KeyUnit(0.055)),
                Vector::new(KeyUnit(0.660), KeyUnit(0.735)),
            )
        );
    }

    #[test]
    fn top_surface_to_round_rect() {
        let surf = TopSurface::default();
        assert_is_close!(
            surf.to_round_rect(),
            RoundRect::new(
                Point::new(KeyUnit(0.170), KeyUnit(0.055)),
                Point::new(KeyUnit(0.830), KeyUnit(0.790)),
                Vector::splat(KeyUnit(0.065)),
            )
        );
    }

    #[test]
    fn top_surface_default() {
        let surf = TopSurface::default();
        assert_is_close!(surf.size, Vector::new(KeyUnit(0.660), KeyUnit(0.735)));
        assert_is_close!(surf.radius, KeyUnit(0.065));
        assert_is_close!(surf.y_offset, KeyUnit(-0.0775));
    }

    #[test]
    fn bottom_surface_to_rect() {
        let surf = BottomSurface::default();
        assert_is_close!(
            surf.to_rect(),
            Rect::new(
                Point::new(KeyUnit(0.025), KeyUnit(0.025)),
                Point::new(KeyUnit(0.975), KeyUnit(0.975)),
            )
        );
    }

    #[test]
    fn bottom_surface_to_round_rect() {
        let surf = BottomSurface::default();
        assert_is_close!(
            surf.to_round_rect(),
            RoundRect::new(
                Point::new(KeyUnit(0.025), KeyUnit(0.025)),
                Point::new(KeyUnit(0.975), KeyUnit(0.975)),
                Vector::splat(KeyUnit(0.065)),
            )
        );
    }

    #[test]
    fn bottom_surface_default() {
        let surf = BottomSurface::default();
        assert_is_close!(surf.size, Vector::new(KeyUnit(0.950), KeyUnit(0.950)));
        assert_is_close!(surf.radius, KeyUnit(0.065));
    }

    #[test]
    fn profile_debug() {
        let profile = Profile::default();

        assert_eq!(
            format!("{profile:?}"),
            format!(
                "Profile {{ typ: {:?}, bottom: {:?}, top: {:?}, legend_geom: {:?}, homing: {:?} }}",
                Type::default(),
                BottomSurface::default(),
                TopSurface::default(),
                LegendGeomMap::default(),
                HomingProps::default(),
            )
        );
    }

    #[cfg(feature = "toml")]
    #[test]
    fn profile_from_toml() {
        let toml = indoc!(
            r#"
            type = "cylindrical"
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

            [legend.alpha]
            size = 4.84
            margin.top-bottom = 1.185
            margin.left-right = 1.18

            [legend.symbol]
            size = 3.18
            margin.top = 2.575
            margin.bottom = 1.775
            margin.left-right = 1.14

            [legend.modifier]
            size = 2.28
            margin.top = 1.185
            margin.bottom = 1.425
            margin.left-right = 1.18

            [homing]
            default = "scoop"
            scoop = { depth = 1.5 }
            bar = { width = 3.85, height = 0.4, y-offset = 5.05 }
            bump = { diameter = 0.4, y-offset = -0.2 }
            "#
        );
        let profile = Profile::from_toml(toml).unwrap();

        assert_matches!(
            profile.typ,
            Type::Cylindrical { depth }
                if depth.is_close(&Mm(0.5))
        );

        assert_is_close!(profile.bottom.size, Vector::splat(Mm(18.29)));
        assert_is_close!(profile.bottom.radius, Mm(0.38));

        assert_is_close!(profile.top.size, Vector::new(Mm(11.81), Mm(13.91)));
        assert_is_close!(profile.top.radius, Mm(1.52));
        assert_is_close!(profile.top.y_offset, Mm(-1.62));

        assert_is_close!(profile.legend_geom.alpha.height, Mm(4.84));
        assert_is_close!(
            profile.legend_geom.alpha.margin,
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18))
        );
        assert_is_close!(profile.legend_geom.symbol.height, Mm(3.18));
        assert_is_close!(
            profile.legend_geom.symbol.margin,
            OffsetRect::new(Mm(2.575), Mm(1.14), Mm(1.775), Mm(1.14))
        );
        assert_is_close!(profile.legend_geom.modifier.height, Mm(2.28));
        assert_is_close!(
            profile.legend_geom.modifier.margin,
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.425), Mm(1.18))
        );

        assert_matches!(profile.homing.default, Homing::Scoop);
        assert_is_close!(profile.homing.scoop.depth, Mm(1.5));
        assert_is_close!(profile.homing.bar.size, Vector::new(Mm(3.85), Mm(0.4)));
        assert_is_close!(profile.homing.bar.y_offset, Mm(5.05));
        assert_is_close!(profile.homing.bump.diameter, Mm(0.4));
        assert_is_close!(profile.homing.bump.y_offset, Mm(-0.2));
    }

    #[cfg(feature = "toml")]
    #[test]
    fn test_profile_from_invalid_toml() {
        let result = Profile::from_toml("null");
        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "expected = after key");
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_profile_from_json() {
        let json = indoc!(
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
                    "alpha": {
                        "size": 4.84,
                        "margin": {
                            "top-bottom": 1.185,
                            "left-right": 1.18
                        }
                    },
                    "symbol": {
                        "size": 3.18,
                        "margin": {
                            "top": 2.575,
                            "bottom": 1.775,
                            "left-right": 1.14
                        }
                    },
                    "modifier": {
                        "size": 2.28,
                        "margin": {
                            "top": 1.185,
                            "bottom": 1.425,
                            "left-right": 1.18
                        }
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
            "#
        );
        let profile = Profile::from_json(json).unwrap();

        assert_matches!(
            profile.typ,
            Type::Cylindrical { depth }
                if depth.is_close(&Mm(0.5))
        );

        assert_is_close!(profile.bottom.size, Vector::splat(Mm(18.29)));
        assert_is_close!(profile.bottom.radius, Mm(0.38));

        assert_is_close!(profile.top.size, Vector::new(Mm(11.81), Mm(13.91)));
        assert_is_close!(profile.top.radius, Mm(1.52));
        assert_is_close!(profile.top.y_offset, Mm(-1.62));

        assert_is_close!(profile.legend_geom.alpha.height, Mm(4.84));
        assert_is_close!(
            profile.legend_geom.alpha.margin,
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18))
        );
        assert_is_close!(profile.legend_geom.symbol.height, Mm(3.18));
        assert_is_close!(
            profile.legend_geom.symbol.margin,
            OffsetRect::new(Mm(2.575), Mm(1.14), Mm(1.775), Mm(1.14))
        );
        assert_is_close!(profile.legend_geom.modifier.height, Mm(2.28));
        assert_is_close!(
            profile.legend_geom.modifier.margin,
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.425), Mm(1.18))
        );

        assert_matches!(profile.homing.default, Homing::Scoop);
        assert_is_close!(profile.homing.scoop.depth, Mm(1.5));
        assert_is_close!(profile.homing.bar.size, Vector::new(Mm(3.85), Mm(0.4)));
        assert_is_close!(profile.homing.bar.y_offset, Mm(5.05));
        assert_is_close!(profile.homing.bump.diameter, Mm(0.4));
        assert_is_close!(profile.homing.bump.y_offset, Mm(-0.2));
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
    fn test_profile_default() {
        let profile = Profile::default();

        assert_matches!(
            profile.typ,
            Type::Cylindrical { depth }
                if depth.is_close(&Mm(1.0))
        );

        assert_is_close!(profile.bottom.size, Vector::splat(KeyUnit(0.950)));
        assert_is_close!(profile.bottom.radius, KeyUnit(0.065));

        assert_is_close!(
            profile.top.size,
            Vector::new(KeyUnit(0.660), KeyUnit(0.735))
        );
        assert_is_close!(profile.top.radius, KeyUnit(0.065));
        assert_is_close!(profile.top.y_offset, KeyUnit(-0.0775));

        assert_is_close!(profile.legend_geom.alpha.height, Mm(16.0 / 72.0 * 19.05));
        assert_is_close!(
            profile.legend_geom.alpha.margin,
            OffsetRect::splat(Mm(0.9525))
        );
        assert_is_close!(profile.legend_geom.symbol.height, Mm(14.0 / 72.0 * 19.05));
        assert_is_close!(
            profile.legend_geom.symbol.margin,
            OffsetRect::splat(Mm(0.9525))
        );
        assert_is_close!(profile.legend_geom.modifier.height, Mm(12.0 / 72.0 * 19.05));
        assert_is_close!(
            profile.legend_geom.modifier.margin,
            OffsetRect::splat(Mm(0.9525))
        );
    }
}
