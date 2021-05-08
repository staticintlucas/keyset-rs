use result::prelude::*;
use serde::Deserialize;

use crate::error::Result;
use crate::types::Color;

#[derive(Debug, Deserialize)]
pub(super) struct RawKleProps {
    #[serde(default)]
    pub(super) x: Option<f32>,
    #[serde(default)]
    pub(super) y: Option<f32>,
    #[serde(default)]
    pub(super) w: Option<f32>,
    #[serde(default)]
    pub(super) h: Option<f32>,
    #[serde(default)]
    pub(super) x2: Option<f32>,
    #[serde(default)]
    pub(super) y2: Option<f32>,
    #[serde(default)]
    pub(super) w2: Option<f32>,
    #[serde(default)]
    pub(super) h2: Option<f32>,
    #[serde(default)]
    pub(super) l: Option<bool>,
    #[serde(default)]
    pub(super) n: Option<bool>,
    #[serde(default)]
    pub(super) d: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "parse_color")]
    pub(super) c: Option<Color>,
    #[serde(default)]
    #[serde(deserialize_with = "parse_color_vec")]
    pub(super) t: Option<Vec<Option<Color>>>,
    #[serde(default)]
    pub(super) a: Option<u8>,
    #[serde(default)]
    pub(super) p: Option<String>,
    #[serde(default)]
    pub(super) f: Option<u8>,
    #[serde(default)]
    pub(super) f2: Option<u8>,
    #[serde(default)]
    pub(super) fa: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum RawKlePropsOrLegend {
    Object(Box<RawKleProps>),
    String(String),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum RawKleMetaDataOrRow {
    Object(serde_json::Value),
    Array(Vec<RawKlePropsOrLegend>),
}

pub(super) fn deserialize(json: &str) -> Result<Vec<RawKleMetaDataOrRow>> {
    let mut jd = serde_json::Deserializer::from_str(json);

    serde_ignored::deserialize(&mut jd, |path| {
        println!("Warning: unrecognized KLE key {}", path);
    })
    .map_err(|e| e.into())
}

fn parse_color<'de, D>(deserializer: D) -> std::result::Result<Option<Color>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};
    Option::<String>::deserialize(deserializer)?
        .map(|hex| {
            Color::from_hex(&hex)
                .map_err(|_| D::Error::invalid_value(Unexpected::Str(&hex), &"a hex color code"))
        })
        .invert()
}

fn parse_color_vec<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Vec<Option<Color>>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};
    Option::<String>::deserialize(deserializer)?
        .map(|string| {
            string
                .lines()
                .map(|hex| {
                    if hex.is_empty() {
                        Ok(None)
                    } else {
                        Color::from_hex(&hex).map(Some).map_err(|_| {
                            D::Error::invalid_value(Unexpected::Str(&hex), &"a hex color code")
                        })
                    }
                })
                .collect::<std::result::Result<Vec<Option<Color>>, D::Error>>()
        })
        .invert()
}
