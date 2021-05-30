use std::fmt;

use serde::de::{value, Error, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

use super::RawKleFile;
use crate::Color;

pub(super) fn de_nl_delimited_colors<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<Option<Color>>>, D::Error>
where
    D: Deserializer<'de>,
{
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
            .collect::<Result<Vec<Option<Color>>, D::Error>>()
    });

    match result {
        Some(Ok(v)) => Ok(Some(v)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

impl<'de> Deserialize<'de> for RawKleFile {
    fn deserialize<D>(deserializer: D) -> Result<RawKleFile, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RawKleFileVisitor;

        impl<'de> Visitor<'de> for RawKleFileVisitor {
            type Value = RawKleFile;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let p = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;
                let r = Vec::<_>::deserialize(value::SeqAccessDeserializer::new(seq))?;
                Ok(RawKleFile { _props: p, rows: r })
            }
        }

        deserializer.deserialize_seq(RawKleFileVisitor {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interfaces::kle::RawKleRowItem;

    use serde_json::{Deserializer, Error};

    #[test]
    fn test_de_nl_delimited_colors() {
        let colors = de_nl_delimited_colors(&mut Deserializer::from_str(r##""#f00\n\n#ba9""##));
        assert!(matches!(colors, Ok(Some(v)) if v.len() == 3 && v[1].is_none()));

        let colors = de_nl_delimited_colors(&mut Deserializer::from_str(r##""#abc\\n#bad""##));
        assert!(matches!(colors, Err(Error { .. })));

        let colors = de_nl_delimited_colors(&mut Deserializer::from_str("null"));
        assert!(matches!(colors, Ok(None)));
    }

    #[test]
    fn test_deserialize_raw_kle_file() {
        let result: RawKleFile = serde_json::from_str(
            r#"[
                {
                    "meta": "data"
                },
                [
                    {
                        "a": 4,
                        "unknown": "key"
                    },
                    "A",
                    "B",
                    "C"
                ],
                [
                    "D"
                ]
            ]"#,
        )
        .unwrap();

        assert_eq!(result._props.len(), 1);
        assert_eq!(result._props["meta"], "data");

        assert_eq!(result.rows.len(), 2);
        assert_eq!(result.rows[0].len(), 4);
        assert!(matches!(result.rows[0][0], RawKleRowItem::Object(_)));
        assert!(matches!(result.rows[0][1], RawKleRowItem::String(_)));

        assert!(matches!(serde_json::from_str::<RawKleFile>("null"), Err(_)))
    }
}
