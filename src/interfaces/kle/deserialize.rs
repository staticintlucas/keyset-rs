use std::result::Result as StdResult;

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
    Array(Vec<RawKlePropsOrLegend>),
    Object(serde_json::Value),
}

pub(super) fn deserialize(json: &str) -> Result<Vec<RawKleMetaDataOrRow>> {
    serde_json::from_str(json).map_err(|e| e.into())
}

fn parse_color<'de, D>(deserializer: D) -> StdResult<Option<Color>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};

    let result = Option::<String>::deserialize(deserializer)?.map(|hex| {
        Color::from_hex(&hex)
            .map_err(|_| D::Error::invalid_value(Unexpected::Str(&hex), &"a hex color code"))
    });

    match result {
        Some(Ok(value)) => Ok(Some(value)),
        Some(Err(error)) => Err(error),
        None => Ok(None),
    }
}

fn parse_color_vec<'de, D>(deserializer: D) -> StdResult<Option<Vec<Option<Color>>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};

    let result = Option::<String>::deserialize(deserializer)?.map(|string| {
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
            .collect::<StdResult<Vec<Option<Color>>, D::Error>>()
    });

    match result {
        Some(Ok(value)) => Ok(Some(value)),
        Some(Err(error)) => Err(error),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::Error;

    #[test]
    fn test_deserialize() {
        let json = r#"[
            {
                "some": "metadata"
            },
            [
                "a",
                "b",
                "c"
            ],
            [
                {
                    "invalid": "key"
                }
            ]
        ]"#;

        let data = deserialize(json).unwrap();

        assert_eq!(data.len(), 3);
        assert!(matches!(&data[0], RawKleMetaDataOrRow::Object(_)));
        assert!(matches!(&data[1], RawKleMetaDataOrRow::Array(v) if v.len() == 3));
    }

    #[test]
    fn test_parse_color() {
        let color = parse_color(&mut serde_json::Deserializer::from_str(r##""#ff0000""##));
        assert!(matches!(color, Ok(Some(Color { .. }))));

        let color = parse_color(&mut serde_json::Deserializer::from_str(r#""invalid""#));
        assert!(matches!(color, Err(Error { .. })));

        let color = parse_color(&mut serde_json::Deserializer::from_str("null"));
        assert!(matches!(color, Ok(None)));
    }

    #[test]
    fn test_parse_color_vec() {
        let colors = parse_color_vec(&mut serde_json::Deserializer::from_str(
            r##""#f00\n\n#ba9""##,
        ));
        assert!(matches!(colors, Ok(Some(v)) if v.len() == 3 && v[1].is_none()));

        let colors = parse_color_vec(&mut serde_json::Deserializer::from_str(
            r##""#abc\\n#bad""##,
        ));
        assert!(matches!(colors, Err(Error { .. })));

        let colors = parse_color_vec(&mut serde_json::Deserializer::from_str("null"));
        assert!(matches!(colors, Ok(None)));
    }
}
