use std::collections::HashMap;

use kurbo::{Point, Rect, Size, Vec2};
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};

use crate::profile::{HomingProps, ProfileType, TextHeight, TextMargin};
use crate::utils::RoundRect;

use super::{BarProps, BumpProps, Profile};

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
            BarProps {
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
            BumpProps {
                diameter: props.diameter * (1000. / 19.05),
                y_offset: props.y_offset * (1000. / 19.05),
            }
        })
    }
}

fn deserialize_round_rect<'de, D>(deserializer: D) -> Result<RoundRect, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct RawRect {
        width: f64,
        height: f64,
        radius: f64,
    }

    RawRect::deserialize(deserializer).map(|rect| {
        // Convert mm to milli units
        let center = Point::new(500., 500.);
        let size = Size::new(rect.width, rect.height) * (1e3 / 19.05);
        let radii = Vec2::new(rect.radius, rect.radius) * (1000. / 19.05);

        RoundRect::from_center_size(center, size, radii)
    })
}

fn deserialize_offset_rect<'de, D>(deserializer: D) -> Result<Rect, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct RawOffsetRect {
        width: f64,
        height: f64,
        #[serde(default)]
        y_offset: f64,
    }

    RawOffsetRect::deserialize(deserializer).map(|rect| {
        // Convert mm to milli units
        let center = Point::new(500., 500.);
        let size = Size::new(rect.width, rect.height) * (1e3 / 19.05);
        let offset = rect.y_offset * (1e3 / 19.05);

        Rect::from_center_size(center + (0., offset), size)
    })
}

fn deserialize_offset_round_rect<'de, D>(deserializer: D) -> Result<RoundRect, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct RawOffsetRoundRect {
        width: f64,
        height: f64,
        radius: f64,
        #[serde(default)]
        y_offset: f64,
    }

    RawOffsetRoundRect::deserialize(deserializer).map(|rect| {
        // Convert mm to milli units
        let center = Point::new(500., 500.);
        let size = Size::new(rect.width, rect.height) * (1e3 / 19.05);
        let offset = rect.y_offset * (1e3 / 19.05);
        let radii = Vec2::new(rect.radius, rect.radius) * (1000. / 19.05);

        RoundRect::from_center_size(center + (0., offset), size, radii)
    })
}

fn deserialize_legend_map<'de, D>(deserializer: D) -> Result<HashMap<usize, (f64, Rect)>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct RawLegendProps {
        size: f64,
        #[serde(flatten, deserialize_with = "deserialize_offset_rect")]
        rect: Rect,
    }

    HashMap::<String, RawLegendProps>::deserialize(deserializer)?
        .iter()
        .map(|(s, p)| {
            let i = s
                .parse()
                .map_err(|_| D::Error::invalid_value(Unexpected::Str(s), &"an integer"))?;

            // Note: deserializing a rect already scales to milliunits, but the size still needs
            // to be scaled here
            Ok((i, (p.size * (1000. / 19.05), p.rect)))
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
            #[serde(deserialize_with = "deserialize_round_rect")]
            bottom: RoundRect,
            #[serde(deserialize_with = "deserialize_offset_round_rect")]
            top: RoundRect,
            #[serde(deserialize_with = "deserialize_legend_map")]
            legend: HashMap<usize, (f64, Rect)>,
            homing: HomingProps,
        }

        let raw_data: RawProfileData = RawProfileData::deserialize(deserializer)?;

        let top_offset = raw_data.top.center().y - 500.;
        let (heights, insets): (HashMap<_, _>, HashMap<_, _>) = raw_data
            .legend
            .into_iter()
            .map(|(i, (size, rect))| {
                let inset =
                    rect.with_origin(rect.origin() + (0., top_offset)) - raw_data.top.rect();
                ((i, size), (i, inset))
            })
            .unzip();

        Ok(Self {
            profile_type: raw_data.profile_type,
            bottom_rect: raw_data.bottom,
            top_rect: raw_data.top,
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
        let bar_props: BarProps = toml::from_str(
            r"
            width = 3.85
            height = 0.4
            y-offset = 5.05
        ",
        )
        .unwrap();

        assert_approx_eq!(bar_props.size, Size::new(202., 21.), 0.5);
        assert_approx_eq!(bar_props.y_offset, 265., 0.5);
    }

    #[test]
    fn test_deserialize_bump_props() {
        let bar_props: BumpProps = toml::from_str(
            r"
            diameter = 0.4
            y-offset = -0.2
        ",
        )
        .unwrap();

        assert_approx_eq!(bar_props.diameter, 21., 0.5);
        assert_approx_eq!(bar_props.y_offset, -10.5, 0.5);
    }

    #[test]
    fn test_deserialize_round_rect() {
        use toml::Deserializer;

        let rect = deserialize_round_rect(Deserializer::new(
            r"
            width = 15.24
            height = 15.24
            radius = 1.905
        ",
        ))
        .unwrap();

        assert_approx_eq!(rect.origin(), Point::new(100., 100.));
        assert_approx_eq!(rect.rect().size(), Size::new(800., 800.));
        assert_approx_eq!(rect.radii().x, 100.);
        assert_approx_eq!(rect.radii().y, 100.);
    }

    #[test]
    fn test_deserialize_offset_rect() {
        use toml::Deserializer;

        let rect = deserialize_offset_rect(Deserializer::new(
            r"
            width = 15.24
            height = 15.24
            y-offset = 0.9525
        ",
        ))
        .unwrap();

        assert_approx_eq!(rect.origin(), Point::new(100., 150.));
        assert_approx_eq!(rect.size(), Size::new(800., 800.));
    }

    #[test]
    fn test_deserialize_offset_round_rect() {
        use toml::Deserializer;

        let rect = deserialize_offset_round_rect(Deserializer::new(
            r"
            width = 15.24
            height = 15.24
            radius = 1.905
            y-offset = 0.9525
        ",
        ))
        .unwrap();

        assert_approx_eq!(rect.origin(), Point::new(100., 150.));
        assert_approx_eq!(rect.rect().size(), Size::new(800., 800.));
        assert_approx_eq!(rect.radii().x, 100.);
        assert_approx_eq!(rect.radii().y, 100.);
    }
}
