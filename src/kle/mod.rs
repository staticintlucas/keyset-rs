mod de;
mod props;
mod utils;

use std::fmt;
use std::vec::IntoIter;

use self::de::KleKeyOrProps;
use self::props::KleProps;
use crate::error::Result;
use crate::key::Key;

#[derive(Debug)]
pub(crate) struct InvalidKleLayout {
    message: String,
}

impl fmt::Display for InvalidKleLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InvalidKleLayout {}

pub struct KleLayout {
    data: de::KleFile,
}

impl KleLayout {
    pub fn new(kle: &str) -> Result<Self> {
        Ok(Self {
            data: serde_json::from_str(kle)?,
        })
    }

    pub fn into_keys_iter(self) -> impl Iterator<Item = Result<Key>> {
        let state = KleProps::default();
        let mut row_iter = self.data.rows.into_iter();
        let key_iter = row_iter.next().unwrap_or(vec![]).into_iter();
        KleLayoutIterator {
            state,
            row_iter,
            key_iter,
        }
    }

    pub fn into_keys_vec(self) -> Result<Vec<Key>> {
        self.into_keys_iter().collect()
    }
}

pub struct KleLayoutIterator {
    state: KleProps,
    row_iter: IntoIter<Vec<KleKeyOrProps>>,
    key_iter: IntoIter<KleKeyOrProps>,
}

impl Iterator for KleLayoutIterator {
    type Item = Result<Key>;

    fn next(&mut self) -> Option<Self::Item> {
        let legends = loop {
            let key = self.key_iter.next().or_else(|| {
                self.key_iter = self.row_iter.next()?.into_iter();
                self.state.next_line();
                self.key_iter.next()
            })?;
            match key {
                KleKeyOrProps::Object(props) => self.state.update(*props),
                KleKeyOrProps::String(str) => break str,
            }
        };

        let legends = array_init::from_iter(
            legends
                .lines()
                .chain(std::iter::repeat(""))
                .map(String::from),
        )
        .unwrap(); // Can't panic due to the iter::repeat

        let key = self.state.build_key(legends);
        self.state.next_key();

        Some(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_layout_new() {
        let result1 = KleLayout::new(
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
                    {
                        "x": -0.5,
                        "y": 0.25
                    },
                    "C"
                ]
            ]"#,
        );

        let result2 = KleLayout::new("gibberish");

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }

    #[test]
    fn test_layout_into_keys() {
        let result1 = KleLayout::new(
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
                    {
                        "x": -0.5,
                        "y": 0.25
                    },
                    "C"
                ],
                [
                    "D"
                ]
            ]"#,
        )
        .unwrap()
        .into_keys_vec()
        .unwrap();

        assert_eq!(result1.len(), 4);
        assert_approx_eq!(result1[0].position.x, 0.0);
        assert_approx_eq!(result1[1].position.x, 1.0);
        assert_approx_eq!(result1[2].position.x, 1.5);
        assert_approx_eq!(result1[3].position.x, 0.0);

        let result2 = KleLayout::new(
            r#"[
                [
                    "A"
                ]
            ]"#,
        )
        .unwrap()
        .into_keys_vec()
        .unwrap();

        assert_eq!(result2.len(), 1);
    }
}
