use std::result::Result as StdResult;
use std::{fmt, iter};

use itertools::Itertools;
use serde::de::{value, Error, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

use crate::error::Result;
use crate::layout::{HomingType, Key, KeySize, KeyType, Layout};
use crate::utils::{Color, Point, Size};

// The number of legends on a key and number of alignment settings from KLE
const NUM_LEGENDS: u8 = 12;
const NUM_ALIGNMENTS: u8 = 8;

// This map is stolen straight from the kle-serial source code. Note the blanks are also filled in,
// so keyset-rs is slightly more permissive than KLE with not-strictly-valid KLE input.
const KLE_2_ORD: [[u8; NUM_LEGENDS as usize]; NUM_ALIGNMENTS as usize] = [
    [0, 6, 2, 8, 9, 11, 3, 5, 1, 4, 7, 10], // 0 = no centering
    [1, 7, 0, 2, 9, 11, 4, 3, 5, 6, 8, 10], // 1 = center x
    [3, 0, 5, 1, 9, 11, 2, 6, 4, 7, 8, 10], // 2 = center y
    [4, 0, 1, 2, 9, 11, 3, 5, 6, 7, 8, 10], // 3 = center x & y
    [0, 6, 2, 8, 10, 9, 3, 5, 1, 4, 7, 11], // 4 = center front (default)
    [1, 7, 0, 2, 10, 3, 4, 5, 6, 8, 9, 11], // 5 = center front & x
    [3, 0, 5, 1, 10, 2, 6, 7, 4, 8, 9, 11], // 6 = center front & y
    [4, 0, 1, 2, 10, 3, 5, 6, 7, 8, 9, 11], // 7 = center front & x & y
];

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
struct RawKleProps {
    x: Option<f32>,
    y: Option<f32>,
    w: Option<f32>,
    h: Option<f32>,
    x2: Option<f32>,
    y2: Option<f32>,
    w2: Option<f32>,
    h2: Option<f32>,
    l: Option<bool>,
    n: Option<bool>,
    d: Option<bool>,
    c: Option<Color>,
    #[serde(deserialize_with = "de_nl_delimited_colors")]
    t: Option<Vec<Option<Color>>>,
    a: Option<u8>,
    p: Option<String>,
    f: Option<u8>,
    f2: Option<u8>,
    fa: Option<Vec<u8>>,
}

// Kle color arrays are just \n delimited strings, so we use this function to turn them into Vecs
fn de_nl_delimited_colors<'de, D>(
    deserializer: D,
) -> StdResult<Option<Vec<Option<Color>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let result = Option::<String>::deserialize(deserializer)?.map(|string| {
        string
            .lines()
            .map(|c| {
                if c.trim().is_empty() {
                    None
                } else {
                    Some(c.trim())
                }
            })
            .map(|c| {
                c.map(|c| {
                    Color::from_hex(c).map_err(|_| {
                        D::Error::invalid_value(Unexpected::Str(c), &"a hex color code")
                    })
                })
                .transpose()
            })
            .collect::<StdResult<Vec<Option<Color>>, D::Error>>()
    });

    match result {
        Some(Ok(v)) => Ok(Some(v)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

// Represents a row item, either a key or a JSON object containing properties for the next key(s)
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum RawKleRowItem {
    Object(Box<RawKleProps>),
    String(String),
}

#[derive(Debug, Clone)]
struct RawKleFile {
    // TODO global layout properties are unused at the moment
    props: Map<String, Value>,
    rows: Vec<Vec<RawKleRowItem>>,
}

impl<'de> Deserialize<'de> for RawKleFile {
    fn deserialize<D>(deserializer: D) -> StdResult<RawKleFile, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RawKleFileVisitor;

        impl<'de> Visitor<'de> for RawKleFileVisitor {
            type Value = RawKleFile;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                #[derive(Deserialize)]
                #[serde(untagged)]
                enum MapOrSeq {
                    Props(Map<String, Value>),
                    Row(Vec<RawKleRowItem>),
                }

                Ok(match seq.next_element::<MapOrSeq>()? {
                    None => Self::Value {
                        props: Map::new(),
                        rows: vec![],
                    },
                    Some(MapOrSeq::Props(props)) => Self::Value {
                        props,
                        rows: Vec::<_>::deserialize(value::SeqAccessDeserializer::new(seq))?,
                    },
                    Some(MapOrSeq::Row(row)) => Self::Value {
                        props: Map::new(),
                        rows: [
                            vec![row],
                            Vec::<_>::deserialize(value::SeqAccessDeserializer::new(seq))?,
                        ]
                        .concat(),
                    },
                })
            }
        }

        deserializer.deserialize_seq(RawKleFileVisitor {})
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct LegendAlignment(u8);

impl LegendAlignment {
    fn new(alignment: u8) -> Self {
        Self(alignment.max(0).min(NUM_ALIGNMENTS - 1))
    }

    fn default() -> Self {
        // The default alignment when none is specified in the KLE data
        Self(4)
    }
}

#[derive(Debug)]
struct KeyProps {
    // Per-key properties
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    x2: f32,
    y2: f32,
    w2: f32,
    h2: f32,
    l: bool, // stepped
    n: bool, // homing
    d: bool, // decal
    // Persistent properties
    c: Color, // color
    // Note: t stores the default color while ta stores the array, so slightly different from KLE
    t: Color,           // legend color
    ta: Vec<Color>,     // legend color array
    a: LegendAlignment, // alignment
    p: String,          // profile
    f: u8,              // font size
    fa: Vec<u8>,        // font size array
}

impl KeyProps {
    fn default() -> Self {
        // The default font size
        const DEFAULT_FONT_SIZE: u8 = 3;

        Self {
            x: 0.,
            y: 0.,
            w: 1.,
            h: 1.,
            x2: 0.,
            y2: 0.,
            w2: 1.,
            h2: 1.,
            l: false,
            n: false,
            d: false,
            c: Color::default_key(),
            t: Color::default_legend(),
            ta: vec![Color::default_legend(); NUM_LEGENDS as usize],
            a: LegendAlignment::default(),
            p: "".to_string(),
            f: DEFAULT_FONT_SIZE,
            fa: vec![DEFAULT_FONT_SIZE; NUM_LEGENDS as usize],
        }
    }

    fn update(&self, props: RawKleProps) -> Self {
        let f = props.f.unwrap_or(self.f);
        let fa = if let Some(fa) = props.fa {
            fa.iter()
                .map(|&fa| if fa == 0 { f } else { fa })
                .chain(iter::repeat(f))
                .take(NUM_LEGENDS as usize)
                .collect()
        } else if let Some(f2) = props.f2 {
            iter::once(f)
                .chain(iter::repeat(f2))
                .take(NUM_LEGENDS as usize)
                .collect()
        } else if let Some(f) = props.f {
            vec![f; NUM_LEGENDS as usize]
        } else {
            self.fa.clone()
        };

        let t = props.t.as_ref().and_then(|t| t[0]).unwrap_or(self.t);
        let ta = props.t.map_or(self.ta.clone(), |ta| {
            ta.iter()
                .map(|c| c.unwrap_or(t))
                .chain(iter::repeat(t))
                .take(NUM_LEGENDS as usize)
                .collect()
        });

        Self {
            // Per-key properties
            x: self.x + props.x.unwrap_or(0.0),
            y: self.y + props.y.unwrap_or(0.0),
            w: props.w.unwrap_or(1.),
            h: props.h.unwrap_or(1.),
            x2: props.x2.unwrap_or(0.),
            y2: props.y2.unwrap_or(0.),
            w2: props.w2.or(props.w).unwrap_or(1.),
            h2: props.h2.or(props.h).unwrap_or(1.),
            l: props.l.unwrap_or(false),
            n: props.n.unwrap_or(false),
            d: props.d.unwrap_or(false),
            // Persistent properties
            c: props.c.unwrap_or(self.c),
            t,
            ta,
            a: props.a.map_or(self.a, LegendAlignment::new),
            p: props.p.unwrap_or_else(|| self.p.clone()),
            f,
            fa,
        }
    }

    fn next_key(self) -> Self {
        Self {
            // Increment x
            x: self.x + self.w.max(self.x2 + self.w2),
            // Reset per-key properties
            w: 1.,
            h: 1.,
            x2: 0.,
            y2: 0.,
            w2: 1.,
            h2: 1.,
            l: false,
            n: false,
            d: false,
            ..self
        }
    }

    #[inline]
    fn next_line(self) -> Self {
        Self {
            x: 0.0,
            y: self.y + 1.0,
            ..self.next_key()
        }
    }

    fn to_key(&self, legends: Vec<String>) -> Result<Key> {
        // Use (x + x2) if (x2 < 0). Needed because we always measure position to the top left
        // corner of the key rather than just the primary rectangle
        let position = Point::new(self.x + self.x2.min(0.), self.y + self.y2.min(0.));
        let size = KeySize::new(self.w, self.h, self.x2, self.y2, self.w2, self.h2)?;

        let is_scooped = ["scoop", "deep", "dish"]
            .iter()
            .map(|pat| self.p.contains(pat))
            .any(|b| b);
        let is_barred = ["bar", "line"]
            .iter()
            .map(|pat| self.p.contains(pat))
            .any(|b| b);
        let is_bumped = ["bump", "dot", "nub", "nipple"]
            .iter()
            .map(|pat| self.p.contains(pat))
            .any(|b| b);

        let key_type = if is_scooped {
            KeyType::Homing(HomingType::Scoop)
        } else if is_barred {
            KeyType::Homing(HomingType::Bar)
        } else if is_bumped {
            KeyType::Homing(HomingType::Bump)
        } else if self.p.contains("space") {
            KeyType::Space
        } else if self.n {
            KeyType::Homing(HomingType::Default)
        } else if self.d {
            KeyType::None
        } else {
            KeyType::Normal
        };

        Ok(Key::new(
            position,
            size,
            key_type,
            self.c,
            realign(legends, self.a),
            realign(self.fa.clone(), self.a),
            realign(self.ta.clone(), self.a),
        ))
    }
}

pub trait FromKle {
    fn from_kle(kle: &str) -> Result<Self>
    where
        Self: Sized;
}

impl FromKle for Layout {
    fn from_kle(kle: &str) -> Result<Self> {
        let raw: RawKleFile = serde_json::from_str(kle)?;

        let mut props = KeyProps::default();
        let mut keys = vec![];
        let mut size = Size::new(0., 0.);

        for row in raw.rows {
            for data in row {
                match data {
                    RawKleRowItem::Object(raw_props) => {
                        props = props.update(*raw_props);
                    }
                    RawKleRowItem::String(legends) => {
                        let legend_array = legends
                            .lines()
                            .map(String::from)
                            .chain(std::iter::repeat(String::new()))
                            .take(NUM_LEGENDS as usize)
                            .collect::<Vec<_>>();

                        let key = props.to_key(legend_array)?;

                        // Need to subtract 0,0 to keeps types consistent (point - point = size)
                        size = size.max((key.position - Point::new(0., 0.)) + key.size.size());
                        keys.push(key);
                        props = props.next_key();
                    }
                }
            }
            props = props.next_line();
        }

        Ok(Layout { size, keys })
    }
}

fn realign<T: std::fmt::Debug + Clone>(values: Vec<T>, alignment: LegendAlignment) -> Vec<T> {
    let alignment = if (alignment.0 as usize) > KLE_2_ORD.len() {
        LegendAlignment::default() // This is the default used by KLE
    } else {
        alignment
    };

    assert_eq!(values.len(), NUM_LEGENDS as usize);

    values
        .into_iter()
        .zip(KLE_2_ORD[alignment.0 as usize].iter())
        .sorted_by_key(|(_v, &i)| i)
        .map(|(v, _i)| v)
        .take(9)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LegendMap;
    use assert_approx_eq::assert_approx_eq;

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
        let result1: RawKleFile = serde_json::from_str(
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

        assert_eq!(result1.props.len(), 1);
        assert_eq!(result1.props["meta"], "data");

        assert_eq!(result1.rows.len(), 2);
        assert_eq!(result1.rows[0].len(), 4);
        assert!(matches!(result1.rows[0][0], RawKleRowItem::Object(_)));
        assert!(matches!(result1.rows[0][1], RawKleRowItem::String(_)));

        let result2: RawKleFile = serde_json::from_str(r#"[["A"]]"#).unwrap();
        assert_eq!(result2.props.len(), 0);
        assert_eq!(result2.rows.len(), 1);

        let result3: RawKleFile = serde_json::from_str(r#"[{"k": "v"}]"#).unwrap();
        assert_eq!(result3.props.len(), 1);
        assert_eq!(result3.rows.len(), 0);

        let result4: RawKleFile = serde_json::from_str(r#"[]"#).unwrap();
        assert_eq!(result4.props.len(), 0);
        assert_eq!(result4.rows.len(), 0);

        assert!(matches!(serde_json::from_str::<RawKleFile>("null"), Err(_)))
    }

    #[test]
    fn test_keyprops_default() {
        let keyprops = KeyProps::default();

        assert_approx_eq!(keyprops.x, 0.);
        assert_approx_eq!(keyprops.y, 0.);
        assert_approx_eq!(keyprops.w, 1.);
        assert_approx_eq!(keyprops.h, 1.);
        assert_approx_eq!(keyprops.x2, 0.);
        assert_approx_eq!(keyprops.y2, 0.);
        assert_approx_eq!(keyprops.w2, 1.);
        assert_approx_eq!(keyprops.h2, 1.);
        assert_eq!(keyprops.l, false);
        assert_eq!(keyprops.n, false);
        assert_eq!(keyprops.d, false);
        assert_eq!(keyprops.c, Color::default_key());
        assert_eq!(keyprops.t, Color::default_legend());
        assert_eq!(keyprops.ta, [Color::default_legend(); NUM_LEGENDS as usize]);
        assert_eq!(keyprops.a, LegendAlignment::default());
        assert_eq!(keyprops.p, "".to_string());
        assert_eq!(keyprops.f, 3);
        assert_eq!(keyprops.fa, [3; NUM_LEGENDS as usize]);
    }

    #[test]
    fn test_keyprops_update() {
        let rawprops1 = RawKleProps {
            x: None,
            y: None,
            w: None,
            h: None,
            x2: None,
            y2: None,
            w2: None,
            h2: None,
            l: None,
            n: None,
            d: None,
            c: None,
            t: None,
            a: None,
            p: None,
            f: None,
            f2: None,
            fa: None,
        };
        let keyprops1 = KeyProps::default().update(rawprops1);

        assert_approx_eq!(keyprops1.x, 0.);
        assert_approx_eq!(keyprops1.y, 0.);
        assert_approx_eq!(keyprops1.w, 1.);
        assert_approx_eq!(keyprops1.h, 1.);
        assert_approx_eq!(keyprops1.x2, 0.);
        assert_approx_eq!(keyprops1.y2, 0.);
        assert_approx_eq!(keyprops1.w2, 1.);
        assert_approx_eq!(keyprops1.h2, 1.);
        assert_eq!(keyprops1.l, false);
        assert_eq!(keyprops1.n, false);
        assert_eq!(keyprops1.d, false);
        assert_eq!(keyprops1.c, Color::default_key());
        assert_eq!(keyprops1.t, Color::default_legend());
        assert_eq!(
            keyprops1.ta,
            [Color::default_legend(); NUM_LEGENDS as usize]
        );
        assert_eq!(keyprops1.a, LegendAlignment::default());
        assert_eq!(keyprops1.p, "".to_string());
        assert_eq!(keyprops1.f, 3);
        assert_eq!(keyprops1.fa, [3; NUM_LEGENDS as usize]);

        let rawprops2 = RawKleProps {
            x: Some(1.),
            y: Some(1.),
            w: Some(2.),
            h: Some(2.),
            x2: Some(1.5),
            y2: Some(1.5),
            w2: Some(2.5),
            h2: Some(2.5),
            l: Some(true),
            n: Some(true),
            d: Some(true),
            c: Some(Color::new(0.5, 0.2, 0.3)),
            t: Some(vec![
                Some(Color::new(0.1, 0.1, 0.1)),
                None,
                Some(Color::new(0.3, 0.15, 0.2)),
            ]),
            a: Some(5),
            p: Some("space".to_string()),
            f: Some(4),
            f2: Some(4),
            fa: Some(vec![4, 4, 4]),
        };
        let keyprops2 = keyprops1.update(rawprops2);

        assert_approx_eq!(keyprops2.x, 1.);
        assert_approx_eq!(keyprops2.y, 1.);
        assert_approx_eq!(keyprops2.w, 2.);
        assert_approx_eq!(keyprops2.h, 2.);
        assert_approx_eq!(keyprops2.x2, 1.5);
        assert_approx_eq!(keyprops2.y2, 1.5);
        assert_approx_eq!(keyprops2.w2, 2.5);
        assert_approx_eq!(keyprops2.h2, 2.5);
        assert_eq!(keyprops2.l, true);
        assert_eq!(keyprops2.n, true);
        assert_eq!(keyprops2.d, true);
        assert_eq!(keyprops2.c, Color::new(0.5, 0.2, 0.3));
        assert_eq!(keyprops2.t, Color::new(0.1, 0.1, 0.1));
        assert_eq!(
            keyprops2.ta,
            [
                Color::new(0.1, 0.1, 0.1),
                Color::new(0.1, 0.1, 0.1),
                Color::new(0.3, 0.15, 0.2),
                Color::new(0.1, 0.1, 0.1),
                Color::new(0.1, 0.1, 0.1),
                Color::new(0.1, 0.1, 0.1),
                Color::new(0.1, 0.1, 0.1),
                Color::new(0.1, 0.1, 0.1),
                Color::new(0.1, 0.1, 0.1),
                Color::new(0.1, 0.1, 0.1),
                Color::new(0.1, 0.1, 0.1),
                Color::new(0.1, 0.1, 0.1)
            ]
        );
        assert_eq!(keyprops2.a, LegendAlignment::new(5));
        assert_eq!(keyprops2.p, "space".to_string());
        assert_eq!(keyprops2.f, 4);
        assert_eq!(keyprops2.fa, [4; NUM_LEGENDS as usize]);

        let rawprops3 = RawKleProps {
            f: Some(2),
            f2: Some(4),
            ..RawKleProps::default()
        };
        let keyprops3 = keyprops2.update(rawprops3);
        assert_eq!(keyprops3.fa, [2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4]);

        let rawprops4 = RawKleProps {
            f: Some(5),
            ..RawKleProps::default()
        };
        let keyprops4 = keyprops3.update(rawprops4);
        assert_eq!(keyprops4.fa, [5; NUM_LEGENDS as usize]);
    }

    #[test]
    fn test_keyprops_next_key() {
        let keyprops1 = KeyProps {
            x: 2.0,
            w: 3.0,
            h: 1.5,
            ..KeyProps::default()
        };
        let keyprops2 = keyprops1.next_key();

        assert_approx_eq!(keyprops2.x, 5.);
        assert_approx_eq!(keyprops2.y, 0.);
        assert_approx_eq!(keyprops2.w, 1.);
        assert_approx_eq!(keyprops2.h, 1.);
        assert_approx_eq!(keyprops2.x2, 0.);
        assert_approx_eq!(keyprops2.y2, 0.);
        assert_approx_eq!(keyprops2.w2, 1.);
        assert_approx_eq!(keyprops2.h2, 1.);
        assert_eq!(keyprops2.l, false);
        assert_eq!(keyprops2.n, false);
        assert_eq!(keyprops2.d, false);
    }

    #[test]
    fn test_keyprops_next_line() {
        let keyprops1 = KeyProps {
            x: 2.0,
            ..KeyProps::default()
        };
        let keyprops2 = keyprops1.next_line();

        assert_approx_eq!(keyprops2.x, 0.);
        assert_approx_eq!(keyprops2.y, 1.);
        assert_approx_eq!(keyprops2.w, 1.);
        assert_approx_eq!(keyprops2.h, 1.);
        assert_approx_eq!(keyprops2.x2, 0.);
        assert_approx_eq!(keyprops2.y2, 0.);
        assert_approx_eq!(keyprops2.w2, 1.);
        assert_approx_eq!(keyprops2.h2, 1.);
        assert_eq!(keyprops2.l, false);
        assert_eq!(keyprops2.n, false);
        assert_eq!(keyprops2.d, false);
    }

    #[test]
    fn test_keyprops_to_key() {
        let legends = vec![
            "A".into(),
            "B".into(),
            "C".into(),
            "D".into(),
            "E".into(),
            "F".into(),
            "G".into(),
            "H".into(),
            "I".into(),
            "J".into(),
            "K".into(),
            "L".into(),
        ];
        let ordered = vec![
            "A".to_string(),
            "I".into(),
            "C".into(),
            "G".into(),
            "J".into(),
            "H".into(),
            "B".into(),
            "K".into(),
            "D".into(),
        ];

        let keyprops1 = KeyProps::default();
        let key1 = keyprops1.to_key(legends.clone()).unwrap();

        assert_eq!(key1.position, Point::new(0., 0.));
        assert_eq!(key1.size, KeySize::Normal(Size::new(1., 1.)));
        assert_eq!(key1.key_type, KeyType::Normal);
        assert_eq!(key1.key_color, Color::default_key());
        assert_eq!(key1.legend, LegendMap::new(ordered));
        assert_eq!(key1.legend_size, LegendMap::new(vec![3; 9]));
        assert_eq!(
            key1.legend_color,
            LegendMap::new(vec![Color::default_legend(); 9])
        );

        let keyprops2 = KeyProps {
            d: true,
            ..keyprops1
        };
        let key2 = keyprops2.to_key(legends.clone()).unwrap();
        assert_eq!(key2.key_type, KeyType::None);

        let keyprops3 = KeyProps {
            n: true,
            ..keyprops2
        };
        let key3 = keyprops3.to_key(legends.clone()).unwrap();
        assert_eq!(key3.key_type, KeyType::Homing(HomingType::Default));

        let keyprops4 = KeyProps {
            p: "space".into(),
            ..keyprops3
        };
        let key4 = keyprops4.to_key(legends.clone()).unwrap();
        assert_eq!(key4.key_type, KeyType::Space);

        let keyprops5 = KeyProps {
            p: "scoop".into(),
            ..keyprops4
        };
        let key5 = keyprops5.to_key(legends.clone()).unwrap();
        assert_eq!(key5.key_type, KeyType::Homing(HomingType::Scoop));

        let keyprops6 = KeyProps {
            p: "bar".into(),
            ..keyprops5
        };
        let key6 = keyprops6.to_key(legends.clone()).unwrap();
        assert_eq!(key6.key_type, KeyType::Homing(HomingType::Bar));

        let keyprops7 = KeyProps {
            p: "bump".into(),
            ..keyprops6
        };
        let key7 = keyprops7.to_key(legends.clone()).unwrap();
        assert_eq!(key7.key_type, KeyType::Homing(HomingType::Bump));
    }

    #[test]
    fn test_from_kle() {
        let result1 = Layout::from_kle(
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
        )
        .unwrap();

        assert_eq!(result1.size, Size::new(2.5, 1.25));
        assert_eq!(result1.keys.len(), 3);
        assert_approx_eq!(result1.keys[0].position.x, 0.0);
        assert_approx_eq!(result1.keys[1].position.x, 1.0);
        assert_approx_eq!(result1.keys[2].position.x, 1.5);

        let result2 = Layout::from_kle(
            r#"[
                [
                    "A"
                ]
            ]"#,
        )
        .unwrap();

        assert_eq!(result2.size, Size::new(1., 1.));
        assert_eq!(result2.keys.len(), 1);
    }

    #[test]
    fn test_realign() {
        let legends = vec![
            "A".to_string(),
            "B".into(),
            "C".into(),
            "D".into(),
            "E".into(),
            "F".into(),
            "G".into(),
            "H".into(),
            "I".into(),
            "J".into(),
            "K".into(),
            "L".into(),
        ];
        let result = vec!["A", "I", "C", "G", "J", "H", "B", "K", "D"];

        assert_eq!(realign(legends.clone(), LegendAlignment::default()), result);

        // Using an invalid alignment so it should fall back to the default
        assert_eq!(realign(legends, LegendAlignment(42)), result);
    }
}
