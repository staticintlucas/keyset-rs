use std::collections::HashMap;

use kurbo::{Point, Rect, Size};
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};

use crate::profile::{BottomSurface, HomingProps, ProfileType, TextHeight, TextMargin};

use super::{BarProps, BumpProps, Profile, TopSurface};

impl<'de> Deserialize<'de> for BarProps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawBarProps {
            width: f64,
            height: f64,
            y_offset: f64,
        }

        RawBarProps::deserialize(deserializer).map(|props| {
            // Convert mm to milli units
            Self {
                size: Size::new(props.width, props.height) * (1e3 / 19.05),
                y_offset: props.y_offset * (1000. / 19.05),
            }
        })
    }
}

impl<'de> Deserialize<'de> for BumpProps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawBumpProps {
            diameter: f64,
            y_offset: f64,
        }

        RawBumpProps::deserialize(deserializer).map(|props| {
            // Convert mm to milli units
            Self {
                diameter: props.diameter * (1000. / 19.05),
                y_offset: props.y_offset * (1000. / 19.05),
            }
        })
    }
}

impl<'de> Deserialize<'de> for TopSurface {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawTopSurface {
            width: f64,
            height: f64,
            radius: f64,
            y_offset: f64,
        }

        RawTopSurface::deserialize(deserializer).map(|surface| {
            // Convert mm to milli units
            Self {
                size: Size::new(surface.width, surface.height) * (1000. / 19.05),
                radius: surface.radius * (1000. / 19.05),
                y_offset: surface.y_offset * (1000. / 19.05),
            }
        })
    }
}

impl<'de> Deserialize<'de> for BottomSurface {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawBottomSurface {
            width: f64,
            height: f64,
            radius: f64,
        }

        RawBottomSurface::deserialize(deserializer).map(|surface| {
            // Convert mm to milli units
            Self {
                size: Size::new(surface.width, surface.height) * (1000. / 19.05),
                radius: surface.radius * (1000. / 19.05),
            }
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LegendProps {
    size: f64,
    width: f64,
    height: f64,
    #[serde(default)]
    y_offset: f64,
}

impl LegendProps {
    fn rect(&self, top_offset: f64) -> Rect {
        Rect::from_center_size(
            Point::new(500., 500. + top_offset + self.y_offset),
            Size::new(self.width, self.height),
        )
    }
}

fn deserialize_legend_map<'de, D>(deserializer: D) -> Result<HashMap<usize, LegendProps>, D::Error>
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
        .collect::<Result<_, _>>()
}

impl<'de> Deserialize<'de> for Profile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawProfileData {
            #[serde(flatten)]
            profile_type: ProfileType,
            bottom: BottomSurface,
            top: TopSurface,
            #[serde(deserialize_with = "deserialize_legend_map")]
            legend: HashMap<usize, LegendProps>,
            homing: HomingProps,
        }

        let raw_data: RawProfileData = RawProfileData::deserialize(deserializer)?;

        let (heights, insets): (HashMap<_, _>, HashMap<_, _>) = raw_data
            .legend
            .into_iter()
            .map(|(i, props)| {
                let inset = props.rect(raw_data.top.y_offset) - raw_data.top.rect();
                ((i, props.size), (i, inset))
            })
            .unzip();

        Ok(Self {
            typ: raw_data.profile_type,
            bottom: raw_data.bottom,
            top: raw_data.top,
            text_margin: TextMargin::new(&insets),
            text_height: TextHeight::new(&heights),
            homing: raw_data.homing,
        })
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::utils::KurboAbs;

    use super::*;

    #[test]
    fn test_deserialize_bar_props() {
        let bar_props: BarProps =
            toml::from_str("width = 3.85\nheight = 0.4\ny-offset = 5.05").unwrap();

        assert_approx_eq!(bar_props.size, Size::new(202., 21.), 0.5);
        assert_approx_eq!(bar_props.y_offset, 265., 0.5);
    }

    #[test]
    fn test_deserialize_bump_props() {
        let bar_props: BumpProps = toml::from_str("diameter = 0.4\ny-offset = -0.2").unwrap();

        assert_approx_eq!(bar_props.diameter, 21., 0.5);
        assert_approx_eq!(bar_props.y_offset, -10.5, 0.5);
    }

    #[test]
    fn test_deserialize_top_surface() {
        let surf: TopSurface =
            toml::from_str("width = 11.81\nheight = 13.91\nradius = 1.52\ny-offset = -1.62")
                .unwrap();

        assert_approx_eq!(surf.size, Size::new(620., 730.), 0.5);
        assert_approx_eq!(surf.radius, 80., 0.5);
        assert_approx_eq!(surf.y_offset, -85., 0.5);
    }

    #[test]
    fn test_deserialize_bottom_surface() {
        let surf: BottomSurface =
            toml::from_str("width = 18.29\nheight = 18.29\nradius = 0.38").unwrap();

        assert_approx_eq!(surf.size, Size::new(960., 960.), 0.5);
        assert_approx_eq!(surf.radius, 20., 0.5);
    }
}
