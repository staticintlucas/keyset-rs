mod error;

use std::collections::HashMap;

use geom::{Dot, ExtRect, Length, Mm, Point, Rect, SideOffsets, Size, DOT_PER_MM};
use serde::de::{Error as _, Unexpected};
use serde::{Deserialize, Deserializer};

use crate::{BottomSurface, HomingProps, ScoopProps, TextHeight, TextMargin, Type};

use super::{BarProps, BumpProps, Profile, TopSurface};

pub use error::{Error, Result};

impl<'de> Deserialize<'de> for ScoopProps {
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
            // Convert to Length
            Self {
                depth: Length::<Mm>::new(props.depth),
            }
        })
    }
}

impl<'de> Deserialize<'de> for BarProps {
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
                size: Size::<Mm>::new(props.width, props.height),
                y_offset: Length::<Mm>::new(props.y_offset),
            }
        })
    }
}

impl<'de> Deserialize<'de> for BumpProps {
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
                diameter: Length::<Mm>::new(props.diameter),
                y_offset: Length::<Mm>::new(props.y_offset),
            }
        })
    }
}

impl<'de> Deserialize<'de> for TopSurface {
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
                size: Size::<Mm>::new(surface.width, surface.height) * DOT_PER_MM,
                radius: Length::<Mm>::new(surface.radius) * DOT_PER_MM,
                y_offset: Length::<Mm>::new(surface.y_offset) * DOT_PER_MM,
            }
        })
    }
}

impl<'de> Deserialize<'de> for BottomSurface {
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
                size: Size::<Mm>::new(surface.width, surface.height) * DOT_PER_MM,
                radius: Length::<Mm>::new(surface.radius) * DOT_PER_MM,
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
            Point::new(500., 500. + top_offset.get() + self.y_offset),
            Size::new(self.width, self.height),
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
                size: p.size * (1000. / 19.05),
                width: p.width * (1000. / 19.05),
                height: p.height * (1000. / 19.05),
                y_offset: p.y_offset * (1000. / 19.05),
            };
            Ok((i, p))
        })
        .collect()
}

impl<'de> Deserialize<'de> for Profile {
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
                let Rect {
                    min: props_min,
                    max: props_max,
                } = props.rect(raw_data.top.y_offset);
                let Rect {
                    min: raw_min,
                    max: raw_max,
                } = raw_data.top.rect();
                let offsets =
                    SideOffsets::from_vectors_inner(props_min - raw_min, props_max - raw_max);
                ((i, props.size), (i, offsets))
            })
            .unzip();

        Ok(Self {
            typ: raw_data.typ,
            bottom: raw_data.bottom,
            top: raw_data.top,
            text_margin: TextMargin::new(&offsets),
            text_height: TextHeight::new(&heights),
            homing: raw_data.homing,
        })
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn deserialize_bar_props() {
        let bar_props: BarProps =
            toml::from_str("width = 3.85\nheight = 0.4\ny-offset = 5.05").unwrap();

        assert_approx_eq!(bar_props.size.width, 3.85);
        assert_approx_eq!(bar_props.size.height, 0.4);
        assert_approx_eq!(bar_props.y_offset.0, 5.05);
    }

    #[test]
    fn deserialize_bump_props() {
        let bar_props: BumpProps = toml::from_str("diameter = 0.4\ny-offset = -0.2").unwrap();

        assert_approx_eq!(bar_props.diameter.get(), 0.4);
        assert_approx_eq!(bar_props.y_offset.get(), -0.2, 0.5);
    }

    #[test]
    fn deserialize_top_surface() {
        let surf: TopSurface =
            toml::from_str("width = 11.81\nheight = 13.91\nradius = 1.52\ny-offset = -1.62")
                .unwrap();

        assert_approx_eq!(surf.size.width, 620.0, 0.5);
        assert_approx_eq!(surf.size.height, 730.0, 0.5);
        assert_approx_eq!(surf.radius.0, 80.0, 0.5);
        assert_approx_eq!(surf.y_offset.0, -85.0, 0.5);
    }

    #[test]
    fn deserialize_bottom_surface() {
        let surf: BottomSurface =
            toml::from_str("width = 18.29\nheight = 18.29\nradius = 0.38").unwrap();

        assert_approx_eq!(surf.size.width, 960.0, 0.5);
        assert_approx_eq!(surf.size.height, 960.0, 0.5);
        assert_approx_eq!(surf.radius.0, 20.0, 0.5);
    }
}
