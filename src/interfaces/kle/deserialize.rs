use result::prelude::*;
use serde::Deserialize;
use serde_json;

use crate::types::Color;
use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct RawKleProps {
    #[serde(default)]
    pub x: Option<f32>,
    #[serde(default)]
    pub y: Option<f32>,
    #[serde(default)]
    pub w: Option<f32>,
    #[serde(default)]
    pub h: Option<f32>,
    #[serde(default)]
    pub x2: Option<f32>,
    #[serde(default)]
    pub y2: Option<f32>,
    #[serde(default)]
    pub w2: Option<f32>,
    #[serde(default)]
    pub h2: Option<f32>,
    #[serde(default)]
    pub l: Option<bool>,
    #[serde(default)]
    pub n: Option<bool>,
    #[serde(default)]
    pub d: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "parse_color")]
    pub c: Option<Color>,
    #[serde(default)]
    #[serde(deserialize_with = "parse_color_vec")]
    pub t: Option<Vec<Option<Color>>>,
    #[serde(default)]
    pub a: Option<u8>,
    #[serde(default)]
    pub p: Option<String>,
    #[serde(default)]
    pub f: Option<u8>,
    #[serde(default)]
    pub f2: Option<u8>,
    #[serde(default)]
    pub fa: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RawKlePropsOrLegend {
    Object(RawKleProps),
    String(String),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RawKleMetaDataOrRow {
    Object(serde_json::Value),
    Array(Vec<RawKlePropsOrLegend>),
}

pub fn deserialize(json: &str) -> Result<Vec<RawKleMetaDataOrRow>> {

    let mut jd = serde_json::Deserializer::from_str(json);

    serde_ignored::deserialize(&mut jd, |path| {
        println!("Warning: unrecognized KLE key {}", path);
    }).map_err(|e| e.into())
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

fn parse_color_vec<'de, D>(deserializer: D) -> std::result::Result<Option<Vec<Option<Color>>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};
    Option::<String>::deserialize(deserializer)?
        .map(|string| {
            string
                .lines()
                .map(|hex| {
                    if hex.len() == 0 {
                        Ok(None)
                    } else {
                        Color::from_hex(&hex).map(|c| Some(c)).map_err(|_| {
                            D::Error::invalid_value(Unexpected::Str(&hex), &"a hex color code")
                        })
                    }
                })
                .collect::<std::result::Result<Vec<Option<Color>>, D::Error>>()
        })
        .invert()
}
