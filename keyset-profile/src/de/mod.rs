mod error;

use std::collections::HashMap;

use serde::de::{Error as _, Unexpected};
use serde::{Deserialize, Deserializer};

use geom::{
    ConvertFrom as _, ConvertInto as _, Dot, KeyUnit, Length, Mm, Point, Rect, Unit as _, Vector,
};

pub use self::error::{Error, Result};
use super::{BarProps, BumpProps, Profile, TopSurface};
use crate::{BottomSurface, HomingProps, ScoopProps, TextHeight, TextMargin, Type};

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
                depth: f32,
            },
            Spherical {
                depth: f32,
            },
            #[serde(alias = "chiclet")]
            Flat,
        }

        RawType::deserialize(deserializer).map(|typ| {
            // Convert to Length
            match typ {
                RawType::Cylindrical { depth } => Self::Cylindrical {
                    depth: Length::<Mm>::new(depth).convert_into(),
                },
                RawType::Spherical { depth } => Self::Spherical {
                    depth: Length::<Mm>::new(depth).convert_into(),
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
            depth: f32,
        }

        RawScoopProps::deserialize(deserializer).map(|props| {
            // Convert to Dist
            Self {
                depth: Length::<Mm>::new(props.depth).convert_into(),
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
            width: f32,
            height: f32,
            y_offset: f32,
        }

        RawBarProps::deserialize(deserializer).map(|props| {
            // Convert to Length
            Self {
                size: Vector::<Mm>::new(props.width, props.height).convert_into(),
                y_offset: Length::<Mm>::new(props.y_offset).convert_into(),
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
            diameter: f32,
            y_offset: f32,
        }

        RawBumpProps::deserialize(deserializer).map(|props| {
            // Convert to Length
            Self {
                diameter: Length::<Mm>::new(props.diameter).convert_into(),
                y_offset: Length::<Mm>::new(props.y_offset).convert_into(),
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
            width: f32,
            height: f32,
            radius: f32,
            y_offset: f32,
        }

        RawTopSurface::deserialize(deserializer).map(|surface| {
            // Convert to Length
            Self {
                size: Vector::<Mm>::new(surface.width, surface.height).convert_into(),
                radius: Length::<Mm>::new(surface.radius).convert_into(),
                y_offset: Length::<Mm>::new(surface.y_offset).convert_into(),
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
            width: f32,
            height: f32,
            radius: f32,
        }

        RawBottomSurface::deserialize(deserializer).map(|surface| {
            // Convert to Length
            Self {
                size: Vector::<Mm>::new(surface.width, surface.height).convert_into(),
                radius: Length::<Mm>::new(surface.radius).convert_into(),
            }
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LegendProps {
    size: f32,
    width: f32,
    height: f32,
    #[serde(default)]
    y_offset: f32,
}

impl LegendProps {
    fn rect(&self, top_offset: Length<Dot>) -> Rect<Dot> {
        Rect::from_center_and_size(
            Point::convert_from(Point::<KeyUnit>::new(0.5, 0.5))
                + Vector::new(0.0, top_offset.length.get() + self.y_offset),
            Vector::new(self.width, self.height),
        )
    }
}

fn deserialize_legend_map<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<usize, LegendProps>, D::Error>
where
    D: Deserializer<'de>,
{
    HashMap::<String, LegendProps>::deserialize(deserializer)?
        .into_iter()
        .map(|(s, p)| {
            let i = s
                .parse()
                .map_err(|_| D::Error::invalid_value(Unexpected::Str(&s), &"an integer"))?;
            let p = LegendProps {
                size: Dot::convert_from(Mm(p.size)).get(),
                width: Dot::convert_from(Mm(p.width)).get(),
                height: Dot::convert_from(Mm(p.height)).get(),
                y_offset: Dot::convert_from(Mm(p.y_offset)).get(),
            };
            Ok((i, p))
        })
        .collect()
}

impl<'de> Deserialize<'de> for Profile {
    #[inline]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawProfileData {
            #[serde(flatten)]
            typ: Type,
            bottom: BottomSurface,
            top: TopSurface,
            #[serde(deserialize_with = "deserialize_legend_map")]
            legend: HashMap<usize, LegendProps>,
            homing: HomingProps,
        }

        let raw_data: RawProfileData = RawProfileData::deserialize(deserializer)?;

        let (heights, offsets): (HashMap<_, _>, HashMap<_, _>) = raw_data
            .legend
            .into_iter()
            .map(|(i, props)| {
                let height = Length::new(props.size);
                let offset = raw_data.top.rect() - props.rect(raw_data.top.y_offset);
                ((i, height), (i, offset))
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
                if depth.is_close(&Length::from_unit(Mm(0.5).convert_into()))
        );
        assert_matches!(
            sph,
            Type::Spherical { depth }
                if depth.is_close(&Length::from_unit(Mm(0.8).convert_into()))
        );
        assert_matches!(chc, Type::Flat);
        assert_matches!(flt, Type::Flat);
    }

    #[test]
    fn deserialize_scoop_props() {
        let scoop_props: ScoopProps = serde_json::from_str(r#"{ "depth": 0.8 }"#).unwrap();

        assert_is_close!(scoop_props.depth, Length::from_unit(Mm(0.8).convert_into()));
    }

    #[test]
    fn deserialize_bar_props() {
        let bar_props: BarProps =
            serde_json::from_str(r#"{ "width": 3.85, "height": 0.4, "y-offset": 5.05 }"#).unwrap();

        assert_is_close!(
            bar_props.size,
            Vector::convert_from(Vector::<Mm>::new(3.85, 0.4))
        );
        assert_is_close!(
            bar_props.y_offset,
            Length::from_unit(Mm(5.05).convert_into())
        );
    }

    #[test]
    fn deserialize_bump_props() {
        let bar_props: BumpProps =
            serde_json::from_str(r#"{ "diameter": 0.4, "y-offset": -0.2 }"#).unwrap();

        assert_is_close!(
            bar_props.diameter,
            Length::from_unit(Mm(0.4).convert_into())
        );
        assert_is_close!(
            bar_props.y_offset,
            Length::from_unit(Mm(-0.2).convert_into())
        );
    }

    #[test]
    fn deserialize_top_surface() {
        let surf: TopSurface = serde_json::from_str(
            r#"{ "width": 11.81, "height": 13.91, "radius": 1.52, "y-offset": -1.62 }"#,
        )
        .unwrap();

        assert_is_close!(
            surf.size,
            Vector::convert_from(Vector::<Mm>::new(11.81, 13.91))
        );
        assert_is_close!(surf.radius, Length::from_unit(Mm(1.52).convert_into()));
        assert_is_close!(surf.y_offset, Length::from_unit(Mm(-1.62).convert_into()));
    }

    #[test]
    fn deserialize_bottom_surface() {
        let surf: BottomSurface =
            serde_json::from_str(r#"{ "width": 18.29, "height": 18.29, "radius": 0.38 }"#).unwrap();

        assert_is_close!(surf.size, Vector::convert_from(Vector::<Mm>::splat(18.29)));
        assert_is_close!(surf.radius, Length::from_unit(Mm(0.38).convert_into()));
    }
}
