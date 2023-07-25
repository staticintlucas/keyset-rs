use std::collections::HashMap;

use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};

use crate::profile::{HomingProps, ProfileType, TextHeight, TextRect};
use crate::utils::{Rect, RoundRect, Vec2};

use super::{BarProps, BumpProps, Profile};

impl<'de> Deserialize<'de> for BarProps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
            // Convert mm to milli units
            BarProps {
                size: Vec2::new(props.width, props.height) * (1e3 / 19.05),
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
            diameter: f32,
            y_offset: f32,
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

fn deserialize_rect<'de, D>(deserializer: D) -> Result<Rect, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct RawRect {
        width: f32,
        height: f32,
    }

    RawRect::deserialize(deserializer).map(|rect| {
        // Convert mm to milli units
        let size = Vec2::new(rect.width, rect.height) * (1e3 / 19.05);
        let position = ((Vec2::from(1e3)) - size) / 2.;

        Rect::new(position, size)
    })
}

fn deserialize_round_rect<'de, D>(deserializer: D) -> Result<RoundRect, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct RawRect {
        width: f32,
        height: f32,
        radius: f32,
    }

    RawRect::deserialize(deserializer).map(|rect| {
        // Convert mm to milli units
        let size = Vec2::new(rect.width, rect.height) * (1e3 / 19.05);
        let position = (Vec2::from(1e3) - size) / 2.;
        let radius = Vec2::from(rect.radius * (1000. / 19.05));

        RoundRect::new(position, size, radius)
    })
}

fn deserialize_offset_rect<'de, D>(deserializer: D) -> Result<Rect, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct RawOffsetRect {
        width: f32,
        height: f32,
        #[serde(default)]
        y_offset: f32,
    }

    RawOffsetRect::deserialize(deserializer).map(|rect| {
        // Convert mm to milli units
        let size = Vec2::new(rect.width, rect.height) * (1e3 / 19.05);
        let offset = rect.y_offset * (1e3 / 19.05);
        let position = (Vec2::from(1e3) - size) / 2. + Vec2::new(0., offset);

        Rect::new(position, size)
    })
}

fn deserialize_offset_round_rect<'de, D>(deserializer: D) -> Result<RoundRect, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct RawOffsetRoundRect {
        width: f32,
        height: f32,
        radius: f32,
        #[serde(default)]
        y_offset: f32,
    }

    RawOffsetRoundRect::deserialize(deserializer).map(|rect| {
        // Convert mm to milli units
        let size = Vec2::new(rect.width, rect.height) * (1e3 / 19.05);
        let offset = rect.y_offset * (1e3 / 19.05);
        let position = (Vec2::from(1e3) - size) / 2. + Vec2::new(0., offset);
        let radius = Vec2::from(rect.radius * (1000. / 19.05));

        RoundRect::new(position, size, radius)
    })
}

fn deserialize_legend_map<'de, D>(deserializer: D) -> Result<HashMap<usize, (f32, Rect)>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct RawLegendProps {
        size: f32,
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
            legend: HashMap<usize, (f32, Rect)>,
            homing: HomingProps,
        }

        let raw_data: RawProfileData = RawProfileData::deserialize(deserializer)?;

        let top_offset = raw_data.top.center().y - 500.;
        let (heights, rects): (HashMap<_, _>, HashMap<_, _>) = raw_data
            .legend
            .into_iter()
            .map(|(i, (s, r))| {
                let r = Rect::new(r.position() + Vec2::new(0., top_offset), r.size());
                ((i, s), (i, r))
            })
            .unzip();

        Ok(Self {
            profile_type: raw_data.profile_type,
            bottom_rect: raw_data.bottom,
            top_rect: raw_data.top,
            text_margin: TextRect::new(&rects),
            text_height: TextHeight::new(&heights),
            homing: raw_data.homing,
        })
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use maplit::hashmap;

    use super::*;

    #[test]
    fn test_text_height_new() {
        let expected = vec![0., 2., 4., 6., 8., 10., 12., 14., 16., 18.];
        let result = TextHeight::new(&hashmap! {}).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e, r);
        }

        let expected = vec![0., 3., 6., 9., 9.5, 10.5, 11.5, 14., 16.5, 19.];
        let result = TextHeight::new(&hashmap! {
            1 => 3.,
            3 => 9.,
            4 => 9.5,
            6 => 11.5,
            9 => 19.
        })
        .0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e, r);
        }
    }

    #[test]
    fn test_text_height_get() {
        let heights = TextHeight::new(&hashmap! {
            1 => 3.,
            3 => 9.,
            4 => 9.5,
            6 => 11.5,
            9 => 19.
        });
        assert_approx_eq!(heights.get(5), 10.5);
        assert_approx_eq!(heights.get(23), 19.);
    }

    #[test]
    fn test_text_rect_new() {
        let expected = vec![Rect::new(Vec2::ZERO, Vec2::from(1e3)); 10];
        let result = TextRect::new(&hashmap! {}).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.position(), r.position());
            assert_approx_eq!(e.size(), r.size());
        }

        let expected = vec![
            Rect::new(Vec2::new(200., 200.), Vec2::new(600., 600.)),
            Rect::new(Vec2::new(200., 200.), Vec2::new(600., 600.)),
            Rect::new(Vec2::new(200., 200.), Vec2::new(600., 600.)),
            Rect::new(Vec2::new(250., 250.), Vec2::new(500., 500.)),
            Rect::new(Vec2::new(250., 250.), Vec2::new(500., 500.)),
            Rect::new(Vec2::new(250., 250.), Vec2::new(500., 500.)),
            Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
            Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
            Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
            Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
        ];
        let result = TextRect::new(&hashmap! {
            2 => Rect::new(Vec2::new(200., 200.), Vec2::new(600., 600.)),
            5 => Rect::new(Vec2::new(250., 250.), Vec2::new(500., 500.)),
            7 => Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
        })
        .0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.position(), r.position());
            assert_approx_eq!(e.size(), r.size());
        }
    }

    #[test]
    fn test_text_rect_get() {
        let rects = TextRect::new(&hashmap! {
            2 => Rect::new(Vec2::new(200., 200.), Vec2::new(600., 600.)),
            5 => Rect::new(Vec2::new(250., 250.), Vec2::new(500., 500.)),
            7 => Rect::new(Vec2::new(300., 300.), Vec2::new(400., 400.)),
        });

        let r = rects.get(2);
        assert_approx_eq!(r.position(), Vec2::from(200.));
        assert_approx_eq!(r.size(), Vec2::from(600.));

        let r = rects.get(62);
        assert_approx_eq!(r.position(), Vec2::from(300.));
        assert_approx_eq!(r.size(), Vec2::from(400.));
    }

    #[test]
    fn test_deserialize_rect() {
        use toml::Deserializer;

        let rect = deserialize_rect(Deserializer::new(
            r"
            width = 15.24
            height = 15.24
        ",
        ))
        .unwrap();

        assert_approx_eq!(rect.position(), Vec2::from(100.));
        assert_approx_eq!(rect.size(), Vec2::from(800.));
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

        assert_approx_eq!(rect.position(), Vec2::from(100.));
        assert_approx_eq!(rect.size(), Vec2::from(800.));
        assert_approx_eq!(rect.radius(), Vec2::from(100.));
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

        assert_approx_eq!(rect.position(), Vec2::new(100., 150.));
        assert_approx_eq!(rect.size(), Vec2::from(800.));
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

        assert_approx_eq!(rect.position(), Vec2::new(100., 150.));
        assert_approx_eq!(rect.size(), Vec2::from(800.));
        assert_approx_eq!(rect.radius(), Vec2::from(100.));
    }
}
