mod error;

use std::result::Result as StdResult;

use serde::{Deserialize, Deserializer};

use geom::{ConvertInto as _, Mm, OffsetRect, Vector};

pub use self::error::{Error, Result};
use super::{BarProps, BumpProps, Profile, TopSurface};
use crate::{BottomSurface, HomingProps, LegendGeom, LegendGeomMap, ScoopProps, Type};

// Utility mod to deserialize Mm
mod mm {
    use serde::{Deserialize as _, Deserializer};

    use geom::Mm;

    #[inline]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Mm, D::Error>
    where
        D: Deserializer<'de>,
    {
        f32::deserialize(deserializer).map(Mm)
    }

    #[inline]
    pub fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<Mm>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize(deserializer).map(Some)
    }

    pub mod option {
        pub use super::deserialize_option as deserialize;
    }
}

impl<'de> Deserialize<'de> for Type {
    #[inline]
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
        enum RawType {
            Cylindrical {
                #[serde(with = "mm")]
                depth: Mm,
            },
            Spherical {
                #[serde(with = "mm")]
                depth: Mm,
            },
            #[serde(alias = "chiclet")]
            Flat,
        }

        RawType::deserialize(deserializer).map(|typ| {
            // Convert to Length
            match typ {
                RawType::Cylindrical { depth } => Self::Cylindrical {
                    depth: depth.convert_into(),
                },
                RawType::Spherical { depth } => Self::Spherical {
                    depth: depth.convert_into(),
                },
                RawType::Flat => Self::Flat,
            }
        })
    }
}

impl<'de> Deserialize<'de> for ScoopProps {
    #[inline]
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawScoopProps {
            #[serde(with = "mm")]
            depth: Mm,
        }

        RawScoopProps::deserialize(deserializer).map(|props| {
            // Convert to Dist
            Self {
                depth: props.depth.convert_into(),
            }
        })
    }
}

impl<'de> Deserialize<'de> for BarProps {
    #[inline]
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawBarProps {
            #[serde(with = "mm")]
            width: Mm,
            #[serde(with = "mm")]
            height: Mm,
            #[serde(with = "mm")]
            y_offset: Mm,
        }

        RawBarProps::deserialize(deserializer).map(|props| {
            // Convert to Length
            Self {
                size: Vector::new(props.width, props.height).convert_into(),
                y_offset: props.y_offset.convert_into(),
            }
        })
    }
}

impl<'de> Deserialize<'de> for BumpProps {
    #[inline]
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawBumpProps {
            #[serde(with = "mm")]
            diameter: Mm,
            #[serde(with = "mm")]
            y_offset: Mm,
        }

        RawBumpProps::deserialize(deserializer).map(|props| {
            // Convert to Length
            Self {
                diameter: props.diameter.convert_into(),
                y_offset: props.y_offset.convert_into(),
            }
        })
    }
}

impl<'de> Deserialize<'de> for TopSurface {
    #[inline]
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawTopSurface {
            #[serde(with = "mm")]
            width: Mm,
            #[serde(with = "mm")]
            height: Mm,
            #[serde(with = "mm")]
            radius: Mm,
            #[serde(with = "mm")]
            y_offset: Mm,
        }

        RawTopSurface::deserialize(deserializer).map(|surface| {
            // Convert to Length
            Self {
                size: Vector::new(surface.width, surface.height).convert_into(),
                radius: surface.radius.convert_into(),
                y_offset: surface.y_offset.convert_into(),
            }
        })
    }
}

impl<'de> Deserialize<'de> for BottomSurface {
    #[inline]
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawBottomSurface {
            #[serde(with = "mm")]
            width: Mm,
            #[serde(with = "mm")]
            height: Mm,
            #[serde(with = "mm")]
            radius: Mm,
        }

        RawBottomSurface::deserialize(deserializer).map(|surface| {
            // Convert to Length
            Self {
                size: Vector::new(surface.width, surface.height).convert_into(),
                radius: surface.radius.convert_into(),
            }
        })
    }
}

mod legend_margin {
    use serde::de::{Error as _, MapAccess as _};
    use serde::Deserializer;
    use serde_untagged::de::{Error, Map};
    use serde_untagged::UntaggedEnumVisitor;

    use geom::{Mm, OffsetRect};

    #[inline]
    fn visit_map(mut map: Map<'_, '_>) -> Result<OffsetRect<Mm>, Error> {
        const KEYS: &[&str; 4] = &["top", "bottom", "left", "right"];

        let mut vars = KEYS.map(|side| (side, None));

        while let Some((keys, value)) = map.next_entry::<String, f32>()? {
            for key in keys.split('-') {
                let &mut (side, ref mut val) = vars
                    .iter_mut()
                    .find(|&&mut (side, _)| side == key)
                    .ok_or_else(|| Error::unknown_field(key, KEYS))?;

                match *val {
                    Some(_) => return Err(Error::duplicate_field(side)),
                    None => *val = Some(value),
                }
            }
        }

        // TODO use try_map when stabilized
        let [top, bottom, left, right] =
            vars.map(|(key, val)| val.map(Mm).ok_or_else(|| Error::missing_field(key)));

        Ok(OffsetRect::new(top?, right?, bottom?, left?))
    }

    #[inline]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetRect<Mm>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO is there a better way to do this?
        #[expect(
            clippy::cast_possible_truncation,
            reason = "if we use f32 directly here it will fail to deserialize (because f64 is the
                default float for deserialization) so we need to convert. Realistically any value
                that would truncate here would truncate anyway if we could directly use f32"
        )]
        UntaggedEnumVisitor::new()
            .f64(|value| Ok(OffsetRect::splat(Mm(value as _))))
            .map(visit_map)
            .deserialize(deserializer)
    }

    #[inline]
    pub fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<OffsetRect<Mm>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize(deserializer).map(Some)
    }

    pub mod option {
        pub use super::deserialize_option as deserialize;
    }
}

impl<'de> Deserialize<'de> for LegendGeomMap {
    #[inline]
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Default, Deserialize)]
        #[serde(deny_unknown_fields)]
        struct SizeAndMargin {
            #[serde(default, with = "mm::option")]
            size: Option<Mm>,
            #[serde(default, with = "legend_margin::option")]
            margin: Option<OffsetRect<Mm>>,
        }

        #[derive(Debug, Default, Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Margin {
            #[serde(default, with = "legend_margin::option")]
            margin: Option<OffsetRect<Mm>>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(untagged, deny_unknown_fields)]
        enum RawLegendGeomMap {
            PerSize {
                #[serde(default)]
                alpha: SizeAndMargin,
                #[serde(default)]
                symbol: SizeAndMargin,
                #[serde(default)]
                modifier: SizeAndMargin,
                // Default is used if margin is not set for any of the others
                #[serde(default)]
                default: Margin,
            },
            DefaultOnly {
                #[serde(with = "legend_margin")]
                margin: OffsetRect<Mm>,
            },
        }

        let default = Self::default();

        Ok(match RawLegendGeomMap::deserialize(deserializer)? {
            RawLegendGeomMap::PerSize {
                alpha,
                symbol,
                modifier,
                default: default_legend,
            } => Self {
                alpha: LegendGeom {
                    height: alpha.size.map_or(default.alpha.height, Mm::convert_into),
                    margin: alpha
                        .margin
                        .or(default_legend.margin)
                        .map_or(default.alpha.margin, OffsetRect::convert_into),
                },
                symbol: LegendGeom {
                    height: symbol.size.map_or(default.symbol.height, Mm::convert_into),
                    margin: symbol
                        .margin
                        .or(default_legend.margin)
                        .map_or(default.symbol.margin, OffsetRect::convert_into),
                },
                modifier: LegendGeom {
                    height: modifier
                        .size
                        .map_or(default.modifier.height, Mm::convert_into),
                    margin: modifier
                        .margin
                        .or(default_legend.margin)
                        .map_or(default.modifier.margin, OffsetRect::convert_into),
                },
            },
            RawLegendGeomMap::DefaultOnly { margin } => {
                let margin = margin.convert_into();
                Self {
                    alpha: LegendGeom {
                        height: default.alpha.height,
                        margin,
                    },
                    symbol: LegendGeom {
                        height: default.symbol.height,
                        margin,
                    },
                    modifier: LegendGeom {
                        height: default.modifier.height,
                        margin,
                    },
                }
            }
        })
    }
}

impl<'de> Deserialize<'de> for Profile {
    #[inline]
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RawProfileData {
            #[serde(flatten)]
            typ: Type,
            bottom: BottomSurface,
            top: TopSurface,
            #[serde(default)]
            legend: LegendGeomMap,
            homing: HomingProps,
        }

        let raw_data: RawProfileData = RawProfileData::deserialize(deserializer)?;

        Ok(Self {
            typ: raw_data.typ,
            bottom: raw_data.bottom,
            top: raw_data.top,
            legend_geom: raw_data.legend,
            homing: raw_data.homing,
            __non_exhaustive: super::NonExhaustive,
        })
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use assert_matches::assert_matches;
    use indoc::indoc;
    use isclose::{assert_is_close, IsClose as _};
    use key::Homing;

    use super::*;

    #[test]
    fn deserialize_type() {
        let cyl: Type = soml::from_str(indoc! {r#"
            type = "cylindrical"
            depth = 0.5
        "#})
        .unwrap();
        assert_matches!(
            cyl,
            Type::Cylindrical { depth }
                if depth.is_close(&Mm(0.5))
        );

        let sph: Type = soml::from_str(indoc! {r#"
            type = "spherical"
            depth = 0.8
        "#})
        .unwrap();
        assert_matches!(
            sph,
            Type::Spherical { depth }
                if depth.is_close(&Mm(0.8))
        );

        let chc: Type = soml::from_str(indoc! {r#"
            type = "chiclet"
        "#})
        .unwrap();
        assert_matches!(chc, Type::Flat);

        let flt: Type = soml::from_str(indoc! {
            r#"
                type = "flat"
            "#
        })
        .unwrap();
        assert_matches!(flt, Type::Flat);
    }

    #[test]
    fn deserialize_scoop_props() {
        let scoop_props: ScoopProps = soml::from_str("depth = 0.8").unwrap();

        assert_is_close!(scoop_props.depth, Mm(0.8));
    }

    #[test]
    fn deserialize_bar_props() {
        let bar_props: BarProps = soml::from_str(indoc! {"
            width = 3.85
            height = 0.4
            y-offset = 5.05
        "})
        .unwrap();

        assert_is_close!(bar_props.size, Vector::new(Mm(3.85), Mm(0.4)));
        assert_is_close!(bar_props.y_offset, Mm(5.05));
    }

    #[test]
    fn deserialize_bump_props() {
        let bar_props: BumpProps = soml::from_str(indoc! {"
            diameter = 0.4
            y-offset = -0.2
        "})
        .unwrap();

        assert_is_close!(bar_props.diameter, Mm(0.4));
        assert_is_close!(bar_props.y_offset, Mm(-0.2));
    }

    #[test]
    fn deserialize_top_surface() {
        let surf: TopSurface = soml::from_str(indoc! {"
            width = 11.81
            height = 13.91
            radius = 1.52
            y-offset = -1.62
        "})
        .unwrap();

        assert_is_close!(surf.size, Vector::new(Mm(11.81), Mm(13.91)));
        assert_is_close!(surf.radius, Mm(1.52));
        assert_is_close!(surf.y_offset, Mm(-1.62));
    }

    #[test]
    fn deserialize_bottom_surface() {
        let surf: BottomSurface = soml::from_str(indoc! {"
            width = 18.29
            height = 18.29
            radius = 0.38
        "})
        .unwrap();

        assert_is_close!(surf.size, Vector::splat(Mm(18.29)));
        assert_is_close!(surf.radius, Mm(0.38));
    }

    #[test]
    fn deserialize_legend_margin() {
        #[derive(Debug, Deserialize)]
        struct Test {
            #[serde(with = "legend_margin")]
            margin: OffsetRect<Mm>,
        }

        let test: Test = soml::from_str(indoc! {"
            margin = 0.5
        "})
        .unwrap();
        assert_is_close!(test.margin, OffsetRect::splat(Mm(0.5)));

        let test: Test = soml::from_str(indoc! {"
            [margin]
            top-bottom-left-right = 0.5
        "})
        .unwrap();
        assert_is_close!(test.margin, OffsetRect::splat(Mm(0.5)));

        let test: Test = soml::from_str(indoc! {"
            [margin]
            top-bottom = 0.5
            left-right = 0.3
        "})
        .unwrap();
        assert_is_close!(
            test.margin,
            OffsetRect::new(Mm(0.5), Mm(0.3), Mm(0.5), Mm(0.3))
        );

        let test: Test = soml::from_str(indoc! {"
            [margin]
            top = 0.5
            bottom = 0.6
            left = 0.2
            right = 0.3
        "})
        .unwrap();
        assert_is_close!(
            test.margin,
            OffsetRect::new(Mm(0.5), Mm(0.3), Mm(0.6), Mm(0.2))
        );

        let test: StdResult<Test, _> = soml::from_str(indoc! {"
            [margin]
            top-right = 0.5
            bottom-right = 0.6
            left-right = 0.2
            right = 0.3
        "});
        assert!(test.is_err());

        let test: StdResult<Test, _> = soml::from_str(indoc! {"
            [margin]
            right = 0.3
        "});
        assert!(test.is_err());

        let test: StdResult<Test, _> = soml::from_str(indoc! {"
            [margin]
            top = 0.5
            bottom = 0.6
            left = 0.2
            right = 0.3
            foo = 0.4
        "});
        assert!(test.is_err());
    }

    #[test]
    fn deserialize_legend_margin_option() {
        #[derive(Debug, Deserialize)]
        struct Test {
            #[serde(default, with = "legend_margin::option")]
            margin: Option<OffsetRect<Mm>>,
        }

        let test: Test = soml::from_str(indoc! {"
            margin = 0.5
        "})
        .unwrap();
        assert!(test.margin.is_some());
        assert_is_close!(test.margin.unwrap(), OffsetRect::splat(Mm(0.5)));

        let test: Test = soml::from_str(indoc! {"
            [margin]
            top-bottom-left-right = 0.5
        "})
        .unwrap();
        assert!(test.margin.is_some());
        assert_is_close!(test.margin.unwrap(), OffsetRect::splat(Mm(0.5)));

        let test: Test = soml::from_str(indoc! {"
            [margin]
            top-bottom = 0.5
            left-right = 0.3
        "})
        .unwrap();
        assert!(test.margin.is_some());
        assert_is_close!(
            test.margin.unwrap(),
            OffsetRect::new(Mm(0.5), Mm(0.3), Mm(0.5), Mm(0.3))
        );

        let test: Test = soml::from_str(indoc! {"
            [margin]
            top = 0.5
            bottom = 0.6
            left = 0.2
            right = 0.3
        "})
        .unwrap();
        assert!(test.margin.is_some());
        assert_is_close!(
            test.margin.unwrap(),
            OffsetRect::new(Mm(0.5), Mm(0.3), Mm(0.6), Mm(0.2))
        );

        let test: Test = soml::from_str("").unwrap();
        assert!(test.margin.is_none());

        let test: StdResult<Test, _> = soml::from_str(indoc! {"
            [margin]
            top-right = 0.5
            bottom-right = 0.6
            left-right = 0.2
            right = 0.3
        "});
        assert!(test.is_err());

        let test: StdResult<Test, _> = soml::from_str(indoc! {"
            [margin]
            right = 0.3
        "});
        assert!(test.is_err());
    }

    #[test]
    fn deserialize_legend_mapping() {
        let map: LegendGeomMap = soml::from_str(indoc! {"
            margin = 0.5
        "})
        .unwrap();

        assert_is_close!(map.alpha.height, LegendGeomMap::default().alpha.height);
        assert_is_close!(map.alpha.margin, OffsetRect::splat(Mm(0.5)));
        assert_is_close!(map.symbol.height, LegendGeomMap::default().symbol.height);
        assert_is_close!(map.symbol.margin, OffsetRect::splat(Mm(0.5)));
        assert_is_close!(
            map.modifier.height,
            LegendGeomMap::default().modifier.height
        );
        assert_is_close!(map.modifier.margin, OffsetRect::splat(Mm(0.5)));

        let map: LegendGeomMap = soml::from_str(indoc! {"
            [default]
            margin = 0.5
        "})
        .unwrap();

        assert_is_close!(map.alpha.height, LegendGeomMap::default().alpha.height);
        assert_is_close!(map.alpha.margin, OffsetRect::splat(Mm(0.5)));
        assert_is_close!(map.symbol.height, LegendGeomMap::default().symbol.height);
        assert_is_close!(map.symbol.margin, OffsetRect::splat(Mm(0.5)));
        assert_is_close!(
            map.modifier.height,
            LegendGeomMap::default().modifier.height
        );
        assert_is_close!(map.modifier.margin, OffsetRect::splat(Mm(0.5)));

        let map: LegendGeomMap = soml::from_str(indoc! {"
            [default]
            margin = 0.5

            [alpha]
            size = 8
            margin = 0.6

            [symbol]
            size = 6
            margin = { top = 0.8, left-right = 0.5, bottom = 0.4 }

            [modifier]
            size = 4
        "})
        .unwrap();

        assert_is_close!(map.alpha.height, Mm(8.0));
        assert_is_close!(map.alpha.margin, OffsetRect::splat(Mm(0.6)));
        assert_is_close!(map.symbol.height, Mm(6.0));
        assert_is_close!(
            map.symbol.margin,
            OffsetRect::new(Mm(0.8), Mm(0.5), Mm(0.4), Mm(0.5))
        );
        assert_is_close!(map.modifier.height, Mm(4.0));
        assert_is_close!(map.modifier.margin, OffsetRect::splat(Mm(0.5)));

        let map: LegendGeomMap = soml::from_str(indoc! {"
            [alpha]
            size = 8
            margin = 0.6

            [symbol]
            size = 6
            margin = { top = 0.8, left-right = 0.5, bottom = 0.4 }

            [modifier]
            size = 4
            margin = 0.2
        "})
        .unwrap();

        assert_is_close!(map.alpha.height, Mm(8.0));
        assert_is_close!(map.alpha.margin, OffsetRect::splat(Mm(0.6)));
        assert_is_close!(map.symbol.height, Mm(6.0));
        assert_is_close!(
            map.symbol.margin,
            OffsetRect::new(Mm(0.8), Mm(0.5), Mm(0.4), Mm(0.5))
        );
        assert_is_close!(map.modifier.height, Mm(4.0));
        assert_is_close!(map.modifier.margin, OffsetRect::splat(Mm(0.2)));

        let map: LegendGeomMap = soml::from_str(indoc! {"
            [alpha]
            size = 8
            margin = 0.6

            [modifier]
            size = 4
            margin = 0.2
        "})
        .unwrap();

        assert_is_close!(map.alpha.height, Mm(8.0));
        assert_is_close!(map.alpha.margin, OffsetRect::splat(Mm(0.6)));
        assert_is_close!(map.symbol.height, LegendGeomMap::default().symbol.height);
        assert_is_close!(map.symbol.margin, LegendGeomMap::default().symbol.margin);
        assert_is_close!(map.modifier.height, Mm(4.0));
        assert_is_close!(map.modifier.margin, OffsetRect::splat(Mm(0.2)));
    }

    #[test]
    fn deserialize_profile() {
        let profile: Profile = soml::from_str(indoc! {
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
        })
        .unwrap();

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
        assert_is_close!(profile.legend_geom.symbol.height, Mm(3.18));
        assert_is_close!(profile.legend_geom.modifier.height, Mm(2.28));

        assert_is_close!(
            profile.legend_geom.alpha.margin,
            OffsetRect::new(Mm(1.185), Mm(1.18), Mm(1.185), Mm(1.18))
        );
        assert_is_close!(
            profile.legend_geom.symbol.margin,
            OffsetRect::new(Mm(2.575), Mm(1.14), Mm(1.775), Mm(1.14))
        );
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
}
