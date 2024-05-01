mod error;

use std::collections::HashMap;

use geom::{
    Dot, ExtRect, Length, Mm, Point, Rect, SideOffsets, Size, Vector, DOT_PER_MM, DOT_PER_UNIT,
};
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
            Point::new(0.5, 0.5) * DOT_PER_UNIT
                + Vector::new(0.0, top_offset.get() + self.y_offset),
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
                size: p.size * DOT_PER_MM.0,
                width: p.width * DOT_PER_MM.0,
                height: p.height * DOT_PER_MM.0,
                y_offset: p.y_offset * DOT_PER_MM.0,
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
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn deserialize_bar_props() {
        let bar_props: BarProps =
            toml::from_str("width = 3.85\nheight = 0.4\ny-offset = 5.05").unwrap();

        assert_is_close!(bar_props.size, Size::<Mm>::new(3.85, 0.4));
        assert_is_close!(bar_props.y_offset, Length::<Mm>::new(5.05));
    }

    #[test]
    fn deserialize_bump_props() {
        let bar_props: BumpProps = toml::from_str("diameter = 0.4\ny-offset = -0.2").unwrap();

        assert_is_close!(bar_props.diameter, Length::<Mm>::new(0.4));
        assert_is_close!(bar_props.y_offset, Length::<Mm>::new(-0.2));
    }

    #[test]
    fn deserialize_top_surface() {
        let surf: TopSurface =
            toml::from_str("width = 11.81\nheight = 13.91\nradius = 1.52\ny-offset = -1.62")
                .unwrap();

        assert_is_close!(surf.size, Size::new(11.81, 13.91) * DOT_PER_MM);
        assert_is_close!(surf.radius, Length::new(1.52) * DOT_PER_MM);
        assert_is_close!(surf.y_offset, Length::new(-1.62) * DOT_PER_MM);
    }

    #[test]
    fn deserialize_bottom_surface() {
        let surf: BottomSurface =
            toml::from_str("width = 18.29\nheight = 18.29\nradius = 0.38").unwrap();

        assert_is_close!(surf.size, Size::splat(18.29) * DOT_PER_MM);
        assert_is_close!(surf.radius, Length::new(0.38) * DOT_PER_MM);
    }
}
