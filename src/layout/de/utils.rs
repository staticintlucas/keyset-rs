use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};

use crate::utils::Color;

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
                    Color::from_hex(hex).map(Some).map_err(|_| {
                        D::Error::invalid_value(Unexpected::Str(hex), &"a hex color code")
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
