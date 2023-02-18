use std::collections::HashMap;

use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};

use crate::profile::{HomingProps, ProfileType, TextHeight, TextRect};
use crate::utils::{Rect, RoundRect};

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
                width: props.width * (1000. / 19.05),
                height: props.height * (1000. / 19.05),
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
            radius: f32,
            y_offset: f32,
        }

        RawBumpProps::deserialize(deserializer).map(|props| {
            // Convert mm to milli units
            BumpProps {
                radius: props.radius * (1000. / 19.05),
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
        let w = rect.width * (1000. / 19.05);
        let h = rect.height * (1000. / 19.05);

        let x = 0.5 * (1000. - w);
        let y = 0.5 * (1000. - h);

        Rect::new(x, y, w, h)
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
        let w = rect.width * (1000. / 19.05);
        let h = rect.height * (1000. / 19.05);

        let rx = rect.radius * (1000. / 19.05);
        let ry = rx;

        let x = 0.5 * (1000. - w);
        let y = 0.5 * (1000. - h);

        RoundRect::new(x, y, w, h, rx, ry)
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
        let w = rect.width * (1000. / 19.05);
        let h = rect.height * (1000. / 19.05);
        let offset = rect.y_offset * (1000. / 19.05);

        let x = 0.5 * (1000. - w);
        let y = 0.5 * (1000. - h) + offset;

        Rect::new(x, y, w, h)
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
        let w = rect.width * (1000. / 19.05);
        let h = rect.height * (1000. / 19.05);
        let offset = rect.y_offset * (1000. / 19.05);

        let rx = rect.radius * (1000. / 19.05);
        let ry = rx;

        let x = 0.5 * (1000. - w);
        let y = 0.5 * (1000. - h) + offset;

        RoundRect::new(x, y, w, h, rx, ry)
    })
}

fn deserialize_legend_map<'de, D>(deserializer: D) -> Result<HashMap<u8, (f32, Rect)>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Clone, Copy, Deserialize)]
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
        #[derive(Debug, Clone, Deserialize)]
        struct RawProfileData {
            #[serde(flatten)]
            profile_type: ProfileType,
            #[serde(deserialize_with = "deserialize_round_rect")]
            bottom: RoundRect,
            #[serde(deserialize_with = "deserialize_offset_round_rect")]
            top: RoundRect,
            #[serde(deserialize_with = "deserialize_legend_map")]
            legend: HashMap<u8, (f32, Rect)>,
            homing: HomingProps,
        }

        let raw_data: RawProfileData = RawProfileData::deserialize(deserializer)?;

        let (heights, rects) = raw_data
            .legend
            .iter()
            .map(|(i, (s, r))| ((i, s), (i, r)))
            .unzip();

        // let (heights, rects) = process_results(
        //     raw_data.legend.iter().map(|(s, p)| {
        //         let i = s
        //             .parse::<u8>()
        //             .map_err(|_| D::Error::invalid_value(Unexpected::Str(s), &"an integer"))?;

        //         // Note: deserializing a rect already scales to milliunits, but the size still needs
        //         // to be scaled here
        //         Ok(((i, p.size * (1000. / 19.05)), (i, p.rect)))
        //     }),
        //     |i| i.unzip(),
        // )?;

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

    use crate::profile::HomingType;

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

        let heights = TextHeight(vec![]);
        assert_approx_eq!(heights.get(5), 10.);
        assert_approx_eq!(heights.get(200), 18.);
    }

    #[test]
    fn test_text_rect_new() {
        let expected = vec![Rect::new(0., 0., 1e3, 1e3); 10];
        let result = TextRect::new(&hashmap! {}).0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.x, r.x);
            assert_approx_eq!(e.y, r.y);
            assert_approx_eq!(e.w, r.w);
            assert_approx_eq!(e.h, r.h);
        }

        let expected = vec![
            Rect::new(200., 200., 600., 600.),
            Rect::new(200., 200., 600., 600.),
            Rect::new(200., 200., 600., 600.),
            Rect::new(250., 250., 500., 500.),
            Rect::new(250., 250., 500., 500.),
            Rect::new(250., 250., 500., 500.),
            Rect::new(300., 300., 400., 400.),
            Rect::new(300., 300., 400., 400.),
            Rect::new(300., 300., 400., 400.),
            Rect::new(300., 300., 400., 400.),
        ];
        let result = TextRect::new(&hashmap! {
            2 => Rect::new(200., 200., 600., 600.),
            5 => Rect::new(250., 250., 500., 500.),
            7 => Rect::new(300., 300., 400., 400.),
        })
        .0;

        assert_eq!(expected.len(), result.len());

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_approx_eq!(e.x, r.x);
            assert_approx_eq!(e.y, r.y);
            assert_approx_eq!(e.w, r.w);
            assert_approx_eq!(e.h, r.h);
        }
    }

    #[test]
    fn test_text_rect_get() {
        let rects = TextRect::new(&hashmap! {
            2 => Rect::new(200., 200., 600., 600.),
            5 => Rect::new(250., 250., 500., 500.),
            7 => Rect::new(300., 300., 400., 400.),
        });

        let r = rects.get(2);
        assert_approx_eq!(r.x, 200.);
        assert_approx_eq!(r.y, 200.);
        assert_approx_eq!(r.w, 600.);
        assert_approx_eq!(r.h, 600.);

        let r = rects.get(62);
        assert_approx_eq!(r.x, 300.);
        assert_approx_eq!(r.y, 300.);
        assert_approx_eq!(r.w, 400.);
        assert_approx_eq!(r.h, 400.);

        let rects = TextRect(vec![]);
        let r = rects.get(2);
        assert_approx_eq!(r.x, 0.);
        assert_approx_eq!(r.y, 0.);
        assert_approx_eq!(r.w, 1e3);
        assert_approx_eq!(r.h, 1e3);
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

        assert_approx_eq!(rect.x, 100.);
        assert_approx_eq!(rect.y, 100.);
        assert_approx_eq!(rect.w, 800.);
        assert_approx_eq!(rect.h, 800.);
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

        assert_approx_eq!(rect.x, 100.);
        assert_approx_eq!(rect.y, 100.);
        assert_approx_eq!(rect.w, 800.);
        assert_approx_eq!(rect.h, 800.);
        assert_approx_eq!(rect.rx, 100.);
        assert_approx_eq!(rect.ry, 100.);
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

        assert_approx_eq!(rect.x, 100.);
        assert_approx_eq!(rect.y, 150.);
        assert_approx_eq!(rect.w, 800.);
        assert_approx_eq!(rect.h, 800.);
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

        assert_approx_eq!(rect.x, 100.);
        assert_approx_eq!(rect.y, 150.);
        assert_approx_eq!(rect.w, 800.);
        assert_approx_eq!(rect.h, 800.);
        assert_approx_eq!(rect.rx, 100.);
        assert_approx_eq!(rect.ry, 100.);
    }

    #[test]
    fn test_deserialize_profile() {
        let profile: Profile = toml::from_str(
            r#"
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
            bump = { radius = 0.2, y-offset = -0.2 }
        "#,
        )
        .unwrap();

        assert!(
            matches!(profile.profile_type, ProfileType::Cylindrical { depth } if f32::abs(depth - 0.5) < 1e-6)
        );

        assert_approx_eq!(profile.bottom_rect.x, 20., 0.5);
        assert_approx_eq!(profile.bottom_rect.y, 20., 0.5);
        assert_approx_eq!(profile.bottom_rect.w, 960., 0.5);
        assert_approx_eq!(profile.bottom_rect.h, 960., 0.5);
        assert_approx_eq!(profile.bottom_rect.rx, 20., 0.5);
        assert_approx_eq!(profile.bottom_rect.ry, 20., 0.5);

        assert_approx_eq!(profile.top_rect.x, 190., 0.5);
        assert_approx_eq!(profile.top_rect.y, 50., 0.5);
        assert_approx_eq!(profile.top_rect.w, 620., 0.5);
        assert_approx_eq!(profile.top_rect.h, 730., 0.5);
        assert_approx_eq!(profile.top_rect.rx, 80., 0.5);
        assert_approx_eq!(profile.top_rect.ry, 80., 0.5);

        assert_eq!(profile.text_height.0.len(), 10);
        let expected = vec![0., 40., 80., 120., 167., 254., 341., 428., 515., 603., 690.];
        for (e, r) in expected.iter().zip(profile.text_height.0.iter()) {
            assert_approx_eq!(e, r, 0.5);
        }

        assert_eq!(profile.text_margin.0.len(), 10);
        let expected = vec![
            Rect::new(252., 197., 496., 593.),
            Rect::new(252., 197., 496., 593.),
            Rect::new(252., 197., 496., 593.),
            Rect::new(252., 197., 496., 593.),
            Rect::new(250., 270., 500., 502.),
            Rect::new(252., 197., 496., 606.),
            Rect::new(252., 197., 496., 606.),
            Rect::new(252., 197., 496., 606.),
            Rect::new(252., 197., 496., 606.),
            Rect::new(252., 197., 496., 606.),
        ];
        for (e, r) in expected.iter().zip(profile.text_margin.0.iter()) {
            assert_approx_eq!(e.x, r.x, 0.5);
            assert_approx_eq!(e.y, r.y, 0.5);
            assert_approx_eq!(e.w, r.w, 0.5);
            assert_approx_eq!(e.h, r.h, 0.5);
        }

        assert_eq!(profile.homing.default, HomingType::Scoop);
        assert_approx_eq!(profile.homing.scoop.depth, 1.5);
        assert_approx_eq!(profile.homing.bar.width, 202., 0.5);
        assert_approx_eq!(profile.homing.bar.height, 21., 0.5);
        assert_approx_eq!(profile.homing.bar.y_offset, 265., 0.5);
        assert_approx_eq!(profile.homing.bump.radius, 10., 0.5);
        assert_approx_eq!(profile.homing.bump.y_offset, -10., 0.5);
    }
}
