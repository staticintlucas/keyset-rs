mod error;

use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use geom::{ConvertInto as _, Dot, Mm, Rect, Vector};

pub use self::error::{Error, Result};
use super::{BarProps, BumpProps, Profile, TopSurface};
use crate::{BottomSurface, HomingProps, ScoopProps, TextHeight, TextMargin, Type};

#[derive(Deserialize)]
#[serde(remote = "Mm")]
struct MmAsF32(f32);

impl<'de> Deserialize<'de> for Type {
    #[inline]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(tag = "type", rename_all = "kebab-case")]
        enum RawType {
            Cylindrical {
                #[serde(with = "MmAsF32")]
                depth: Mm,
            },
            Spherical {
                #[serde(with = "MmAsF32")]
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
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawScoopProps {
            #[serde(with = "MmAsF32")]
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
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawBarProps {
            #[serde(with = "MmAsF32")]
            width: Mm,
            #[serde(with = "MmAsF32")]
            height: Mm,
            #[serde(with = "MmAsF32")]
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
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawBumpProps {
            #[serde(with = "MmAsF32")]
            diameter: Mm,
            #[serde(with = "MmAsF32")]
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
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawTopSurface {
            #[serde(with = "MmAsF32")]
            width: Mm,
            #[serde(with = "MmAsF32")]
            height: Mm,
            #[serde(with = "MmAsF32")]
            radius: Mm,
            #[serde(with = "MmAsF32")]
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
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawBottomSurface {
            #[serde(with = "MmAsF32")]
            width: Mm,
            #[serde(with = "MmAsF32")]
            height: Mm,
            #[serde(with = "MmAsF32")]
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

impl<'de> Deserialize<'de> for Profile {
    #[inline]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Clone, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawLegendProps {
            #[serde(with = "MmAsF32")]
            size: Mm,
            #[serde(with = "MmAsF32")]
            width: Mm,
            #[serde(with = "MmAsF32")]
            height: Mm,
            #[serde(default, with = "MmAsF32")]
            y_offset: Mm,
        }

        #[derive(Deserialize)]
        struct RawProfileData {
            #[serde(flatten)]
            typ: Type,
            bottom: BottomSurface,
            top: TopSurface,
            #[serde(default)]
            legend: HashMap<usize, RawLegendProps>,
            homing: HomingProps,
        }

        let raw_data: RawProfileData = RawProfileData::deserialize(deserializer)?;

        let (heights, offsets): (HashMap<_, _>, HashMap<_, _>) = raw_data
            .legend
            .into_iter()
            .map(|(i, props)| {
                let text_height = props.size.convert_into();

                let key_top = raw_data.top.rect();
                let text_rect = Rect::from_center_and_size(
                    key_top.center() + Vector::new(Dot(0.0), props.y_offset.convert_into()),
                    Vector::new(props.width, props.height).convert_into(),
                );
                let offset = key_top - text_rect;

                ((i, text_height), (i, offset))
            })
            .unzip();

        Ok(Self {
            typ: raw_data.typ,
            bottom: raw_data.bottom,
            top: raw_data.top,
            text_margin: TextMargin::new(&offsets),
            text_height: TextHeight::new(&heights),
            homing: raw_data.homing,
            __non_exhaustive: super::NonExhaustive,
        })
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use assert_matches::assert_matches;
    use isclose::{assert_is_close, IsClose as _};

    use super::*;

    #[test]
    fn deserialize_type() {
        let cyl: Type = serde_json::from_str(r#"{ "type": "cylindrical", "depth": 0.5 }"#).unwrap();
        let sph: Type = serde_json::from_str(r#"{ "type": "spherical", "depth": 0.8 }"#).unwrap();
        let chc: Type = serde_json::from_str(r#"{ "type": "chiclet" }"#).unwrap();
        let flt: Type = serde_json::from_str(r#"{ "type": "flat" }"#).unwrap();

        assert_matches!(
            cyl,
            Type::Cylindrical { depth }
                if depth.is_close(&Mm(0.5))
        );
        assert_matches!(
            sph,
            Type::Spherical { depth }
                if depth.is_close(&Mm(0.8))
        );
        assert_matches!(chc, Type::Flat);
        assert_matches!(flt, Type::Flat);
    }

    #[test]
    fn deserialize_scoop_props() {
        let scoop_props: ScoopProps = serde_json::from_str(r#"{ "depth": 0.8 }"#).unwrap();

        assert_is_close!(scoop_props.depth, Mm(0.8));
    }

    #[test]
    fn deserialize_bar_props() {
        let bar_props: BarProps =
            serde_json::from_str(r#"{ "width": 3.85, "height": 0.4, "y-offset": 5.05 }"#).unwrap();

        assert_is_close!(bar_props.size, Vector::new(Mm(3.85), Mm(0.4)));
        assert_is_close!(bar_props.y_offset, Mm(5.05));
    }

    #[test]
    fn deserialize_bump_props() {
        let bar_props: BumpProps =
            serde_json::from_str(r#"{ "diameter": 0.4, "y-offset": -0.2 }"#).unwrap();

        assert_is_close!(bar_props.diameter, Mm(0.4));
        assert_is_close!(bar_props.y_offset, Mm(-0.2));
    }

    #[test]
    fn deserialize_top_surface() {
        let surf: TopSurface = serde_json::from_str(
            r#"{ "width": 11.81, "height": 13.91, "radius": 1.52, "y-offset": -1.62 }"#,
        )
        .unwrap();

        assert_is_close!(surf.size, Vector::new(Mm(11.81), Mm(13.91)));
        assert_is_close!(surf.radius, Mm(1.52));
        assert_is_close!(surf.y_offset, Mm(-1.62));
    }

    #[test]
    fn deserialize_bottom_surface() {
        let surf: BottomSurface =
            serde_json::from_str(r#"{ "width": 18.29, "height": 18.29, "radius": 0.38 }"#).unwrap();

        assert_is_close!(surf.size, Vector::splat(Mm(18.29)));
        assert_is_close!(surf.radius, Mm(0.38));
    }
}
