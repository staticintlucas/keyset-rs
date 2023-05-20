use std::{fmt, iter, result::Result as StdResult};

use itertools::Itertools;
use serde::{
    de::{value::SeqAccessDeserializer, Error, SeqAccess, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use serde_json::{Map, Value};

use crate::{
    error::Result,
    layout::{Key, KeySize, KeyType, Layout},
    profile::HomingType,
    utils::{Color, Vec2},
};

// The number of legends on a key and number of alignment settings from KLE
const NUM_LEGENDS: usize = 12;
const NUM_ALIGNMENTS: usize = 8;

// This map is the same as that of kle-serial except the indices and values are swapped. Note the
// blanks are also filled in, so we're slightly more permissive with not-strictly-valid KLE input.
const KLE_2_ORD: [[u8; NUM_LEGENDS]; NUM_ALIGNMENTS] = [
    [0, 8, 2, 6, 9, 7, 1, 10, 3, 4, 11, 5], // 0 = no centering
    [2, 0, 3, 7, 6, 8, 9, 1, 10, 4, 11, 5], // 1 = center x
    [1, 3, 6, 0, 8, 2, 7, 9, 10, 4, 11, 5], // 2 = center y
    [1, 2, 3, 6, 0, 7, 8, 9, 10, 4, 11, 5], // 3 = center x & y
    [0, 8, 2, 6, 9, 7, 1, 10, 3, 5, 4, 11], // 4 = center front (default)
    [2, 0, 3, 5, 6, 7, 8, 1, 9, 10, 4, 11], // 5 = center front & x
    [1, 3, 5, 0, 8, 2, 6, 7, 9, 10, 4, 11], // 6 = center front & y
    [1, 2, 3, 5, 0, 6, 7, 8, 9, 10, 4, 11], // 7 = center front & x & y
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
    fn invalid_color<'de, D: Deserializer<'de>>(c: &str) -> D::Error {
        D::Error::invalid_value(Unexpected::Str(c), &"a hex color code")
    }

    Option::<String>::deserialize(deserializer)?
        .map(|string| {
            string
                .lines()
                .map(str::trim)
                .map(|c| (!c.is_empty()).then_some(c))
                .map(|c| c.map(|c| Color::from_hex(c).map_err(|_| invalid_color::<D>(c))))
                .map(Option::transpose)
                .collect()
        })
        .transpose()
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
                        rows: Vec::<_>::deserialize(SeqAccessDeserializer::new(seq))?,
                    },
                    Some(MapOrSeq::Row(row)) => Self::Value {
                        props: Map::new(),
                        rows: [
                            vec![row],
                            Vec::<_>::deserialize(SeqAccessDeserializer::new(seq))?,
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
struct LegendAlignment(usize);

impl LegendAlignment {
    fn new(alignment: u8) -> Self {
        Self(usize::from(alignment).clamp(0, NUM_ALIGNMENTS - 1))
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
    t: Color,                 // legend color
    ta: [Color; NUM_LEGENDS], // legend color array
    a: LegendAlignment,       // alignment
    p: String,                // profile
    f: u8,                    // font size
    fa: [u8; NUM_LEGENDS],    // font size array
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
            ta: [Color::default_legend(); NUM_LEGENDS],
            a: LegendAlignment::default(),
            p: String::new(),
            f: DEFAULT_FONT_SIZE,
            fa: [DEFAULT_FONT_SIZE; NUM_LEGENDS],
        }
    }

    fn update(&mut self, props: RawKleProps) {
        let f = props.f.unwrap_or(self.f);
        let fa = if let Some(fa) = props.fa {
            array_init::from_iter(
                fa.into_iter()
                    .map(|fa| if fa == 0 { f } else { fa })
                    .chain(iter::repeat(f)),
            )
            .unwrap() // Can't panic due to the iter::repeat
        } else if let Some(f2) = props.f2 {
            array_init::array_init(|i| if i == 0 { f } else { f2 })
        } else if let Some(f) = props.f {
            [f; NUM_LEGENDS]
        } else {
            self.fa
        };

        let t = props
            .t
            .as_ref()
            .and_then(|v| v.get(0).copied().flatten())
            .unwrap_or(self.t);
        let ta = props.t.map_or(self.ta, |ta| {
            array_init::from_iter(
                ta.into_iter()
                    .map(|c| c.unwrap_or(t))
                    .chain(iter::repeat(t)),
            )
            .unwrap() // Can't panic due to the iter::repeat
        });

        // Per-key properties
        self.x += props.x.unwrap_or(0.0);
        self.y += props.y.unwrap_or(0.0);
        self.w = props.w.unwrap_or(1.);
        self.h = props.h.unwrap_or(1.);
        self.x2 = props.x2.unwrap_or(0.);
        self.y2 = props.y2.unwrap_or(0.);
        self.w2 = props.w2.or(props.w).unwrap_or(1.);
        self.h2 = props.h2.or(props.h).unwrap_or(1.);
        self.l = props.l.unwrap_or(false);
        self.n = props.n.unwrap_or(false);
        self.d = props.d.unwrap_or(false);
        // Persistent properties
        self.c = props.c.unwrap_or(self.c);
        self.t = t;
        self.ta = ta;
        self.a = props.a.map_or(self.a, LegendAlignment::new);
        self.p = props.p.unwrap_or(self.p.clone());
        self.f = f;
        self.fa = fa;
    }

    #[inline]
    fn next_key(&mut self) {
        // Increment x
        self.x += self.w.max(self.x2 + self.w2);
        // Reset per-key properties
        self.w = 1.;
        self.h = 1.;
        self.x2 = 0.;
        self.y2 = 0.;
        self.w2 = 1.;
        self.h2 = 1.;
        self.l = false;
        self.n = false;
        self.d = false;
    }

    #[inline]
    fn next_line(&mut self) {
        self.next_key();
        self.x = 0.;
        self.y += 1.;
    }

    fn build_key(&self, legends: &[String; NUM_LEGENDS]) -> Result<Key> {
        // Use (x + x2) if (x2 < 0). Needed because we always measure position to the top left
        // corner of the key rather than just the primary rectangle
        let position = Vec2::new(self.x + self.x2.min(0.), self.y + self.y2.min(0.));
        let size = KeySize::new(self.w, self.h, self.x2, self.y2, self.w2, self.h2)?;

        let is_scooped = ["scoop", "deep", "dish"]
            .map(|pat| self.p.contains(pat))
            .contains(&true);
        let is_barred = ["bar", "line"]
            .map(|pat| self.p.contains(pat))
            .contains(&true);
        let is_bumped = ["bump", "dot", "nub", "nipple"]
            .map(|pat| self.p.contains(pat))
            .contains(&true);

        let key_type = if is_scooped {
            KeyType::Homing(Some(HomingType::Scoop))
        } else if is_barred {
            KeyType::Homing(Some(HomingType::Bar))
        } else if is_bumped {
            KeyType::Homing(Some(HomingType::Bump))
        } else if self.p.contains("space") {
            KeyType::Space
        } else if self.n {
            KeyType::Homing(None)
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
            realign(&self.fa, self.a),
            realign(&self.ta, self.a),
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
        let mut size = Vec2::ZERO;

        for row in raw.rows {
            for data in row {
                match data {
                    RawKleRowItem::Object(raw_props) => {
                        props.update(*raw_props);
                    }
                    RawKleRowItem::String(legends) => {
                        let legend_array = array_init::from_iter(
                            legends
                                .lines()
                                .map(String::from)
                                .chain(std::iter::repeat(String::new())),
                        )
                        .unwrap(); // Can't panic due to the iter::repeat

                        let key = props.build_key(&legend_array)?;

                        size = size.max(key.position + key.size.size());
                        keys.push(key);
                        props.next_key();
                    }
                }
            }
            props.next_line();
        }

        Ok(Layout { size, keys })
    }
}

fn realign<T: Clone>(values: &[T; NUM_LEGENDS], alignment: LegendAlignment) -> [[T; 3]; 3] {
    let alignment = if alignment.0 > KLE_2_ORD.len() {
        LegendAlignment::default() // This is the default used by KLE
    } else {
        alignment
    };

    // Rearrange values based on the mapping
    let values = KLE_2_ORD[alignment.0].iter().map(|&i| &values[i as usize]);

    // Reshape into [[T; 3]; 3]
    array_init::from_iter(
        values
            .chunks(3)
            .into_iter()
            .map(|chunk| array_init::from_iter(chunk.cloned()).unwrap()),
    )
    .unwrap() // Guaranteed not to panic since input is longer than output
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;

    use serde_json::{Deserializer, Error};

    #[test]
    fn test_de_nl_delimited_colors() {
        let colors = de_nl_delimited_colors(&mut Deserializer::from_str(r##""#f00\n\n#ba9""##));
        assert_matches!(colors, Ok(Some(v)) if v.len() == 3 && v[1].is_none());

        let colors = de_nl_delimited_colors(&mut Deserializer::from_str(r##""#abc\\n#bad""##));
        assert_matches!(colors, Err(Error { .. }));

        let colors = de_nl_delimited_colors(&mut Deserializer::from_str("null"));
        assert_matches!(colors, Ok(None));
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
        assert_matches!(result1.rows[0][0], RawKleRowItem::Object(_));
        assert_matches!(result1.rows[0][1], RawKleRowItem::String(_));

        let result2: RawKleFile = serde_json::from_str(r#"[["A"]]"#).unwrap();
        assert_eq!(result2.props.len(), 0);
        assert_eq!(result2.rows.len(), 1);

        let result3: RawKleFile = serde_json::from_str(r#"[{"k": "v"}]"#).unwrap();
        assert_eq!(result3.props.len(), 1);
        assert_eq!(result3.rows.len(), 0);

        let result4: RawKleFile = serde_json::from_str(r#"[]"#).unwrap();
        assert_eq!(result4.props.len(), 0);
        assert_eq!(result4.rows.len(), 0);

        assert_matches!(serde_json::from_str::<RawKleFile>("null"), Err(_));
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
        assert_eq!(keyprops.ta, [Color::default_legend(); NUM_LEGENDS]);
        assert_eq!(keyprops.a, LegendAlignment::default());
        assert_eq!(keyprops.p, "".to_string());
        assert_eq!(keyprops.f, 3);
        assert_eq!(keyprops.fa, [3; NUM_LEGENDS]);
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
        let mut keyprops = KeyProps::default();
        keyprops.update(rawprops1);

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
        assert_eq!(keyprops.ta, [Color::default_legend(); NUM_LEGENDS]);
        assert_eq!(keyprops.a, LegendAlignment::default());
        assert_eq!(keyprops.p, "".to_string());
        assert_eq!(keyprops.f, 3);
        assert_eq!(keyprops.fa, [3; NUM_LEGENDS]);

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
            c: Some(Color::new(127, 51, 76)),
            t: Some(vec![
                Some(Color::new(25, 25, 25)),
                None,
                Some(Color::new(76, 38, 51)),
            ]),
            a: Some(5),
            p: Some("space".to_string()),
            f: Some(4),
            f2: Some(4),
            fa: Some(vec![4, 4, 4]),
        };
        keyprops.update(rawprops2);

        assert_approx_eq!(keyprops.x, 1.);
        assert_approx_eq!(keyprops.y, 1.);
        assert_approx_eq!(keyprops.w, 2.);
        assert_approx_eq!(keyprops.h, 2.);
        assert_approx_eq!(keyprops.x2, 1.5);
        assert_approx_eq!(keyprops.y2, 1.5);
        assert_approx_eq!(keyprops.w2, 2.5);
        assert_approx_eq!(keyprops.h2, 2.5);
        assert_eq!(keyprops.l, true);
        assert_eq!(keyprops.n, true);
        assert_eq!(keyprops.d, true);
        assert_eq!(keyprops.c, Color::new(127, 51, 76));
        assert_eq!(keyprops.t, Color::new(25, 25, 25));
        assert_eq!(
            keyprops.ta,
            [
                Color::new(25, 25, 25),
                Color::new(25, 25, 25),
                Color::new(76, 38, 51),
                Color::new(25, 25, 25),
                Color::new(25, 25, 25),
                Color::new(25, 25, 25),
                Color::new(25, 25, 25),
                Color::new(25, 25, 25),
                Color::new(25, 25, 25),
                Color::new(25, 25, 25),
                Color::new(25, 25, 25),
                Color::new(25, 25, 25)
            ]
        );
        assert_eq!(keyprops.a, LegendAlignment::new(5));
        assert_eq!(keyprops.p, "space".to_string());
        assert_eq!(keyprops.f, 4);
        assert_eq!(keyprops.fa, [4; NUM_LEGENDS]);

        let rawprops3 = RawKleProps {
            f: Some(2),
            f2: Some(4),
            ..RawKleProps::default()
        };
        keyprops.update(rawprops3);
        assert_eq!(keyprops.fa, [2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4]);

        let rawprops4 = RawKleProps {
            f: Some(5),
            ..RawKleProps::default()
        };
        keyprops.update(rawprops4);
        assert_eq!(keyprops.fa, [5; NUM_LEGENDS]);
    }

    #[test]
    fn test_keyprops_next_key() {
        let mut keyprops = KeyProps {
            x: 2.0,
            w: 3.0,
            h: 1.5,
            ..KeyProps::default()
        };
        keyprops.next_key();

        assert_approx_eq!(keyprops.x, 5.);
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
    }

    #[test]
    fn test_keyprops_next_line() {
        let mut keyprops = KeyProps {
            x: 2.0,
            ..KeyProps::default()
        };
        keyprops.next_line();

        assert_approx_eq!(keyprops.x, 0.);
        assert_approx_eq!(keyprops.y, 1.);
        assert_approx_eq!(keyprops.w, 1.);
        assert_approx_eq!(keyprops.h, 1.);
        assert_approx_eq!(keyprops.x2, 0.);
        assert_approx_eq!(keyprops.y2, 0.);
        assert_approx_eq!(keyprops.w2, 1.);
        assert_approx_eq!(keyprops.h2, 1.);
        assert_eq!(keyprops.l, false);
        assert_eq!(keyprops.n, false);
        assert_eq!(keyprops.d, false);
    }

    #[test]
    fn test_keyprops_to_key() {
        let legends = [
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
        let ordered = [
            ["A".to_string(), "I".into(), "C".into()],
            ["G".into(), "J".into(), "H".into()],
            ["B".into(), "K".into(), "D".into()],
        ];

        let keyprops1 = KeyProps::default();
        let key1 = keyprops1.build_key(&legends).unwrap();

        assert_eq!(key1.position, Vec2::ZERO);
        assert_eq!(key1.size, KeySize::Normal(Vec2::from(1.)));
        assert_eq!(key1.key_type, KeyType::Normal);
        assert_eq!(key1.key_color, Color::default_key());
        assert_eq!(key1.legend, ordered);
        assert_eq!(key1.legend_size, [[3; 3]; 3]);
        assert_eq!(key1.legend_color, [[Color::default_legend(); 3]; 3]);

        let keyprops2 = KeyProps {
            d: true,
            ..keyprops1
        };
        let key2 = keyprops2.build_key(&legends).unwrap();
        assert_eq!(key2.key_type, KeyType::None);

        let keyprops3 = KeyProps {
            n: true,
            ..keyprops2
        };
        let key3 = keyprops3.build_key(&legends).unwrap();
        assert_eq!(key3.key_type, KeyType::Homing(None));

        let keyprops4 = KeyProps {
            p: "space".into(),
            ..keyprops3
        };
        let key4 = keyprops4.build_key(&legends).unwrap();
        assert_eq!(key4.key_type, KeyType::Space);

        let keyprops5 = KeyProps {
            p: "scoop".into(),
            ..keyprops4
        };
        let key5 = keyprops5.build_key(&legends).unwrap();
        assert_eq!(key5.key_type, KeyType::Homing(Some(HomingType::Scoop)));

        let keyprops6 = KeyProps {
            p: "bar".into(),
            ..keyprops5
        };
        let key6 = keyprops6.build_key(&legends).unwrap();
        assert_eq!(key6.key_type, KeyType::Homing(Some(HomingType::Bar)));

        let keyprops7 = KeyProps {
            p: "bump".into(),
            ..keyprops6
        };
        let key7 = keyprops7.build_key(&legends).unwrap();
        assert_eq!(key7.key_type, KeyType::Homing(Some(HomingType::Bump)));
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

        assert_eq!(result1.size, Vec2::new(2.5, 1.25));
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

        assert_eq!(result2.size, Vec2::from(1.));
        assert_eq!(result2.keys.len(), 1);
    }

    #[test]
    fn test_realign() {
        let legends = [
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
        let result = [["A", "I", "C"], ["G", "J", "H"], ["B", "K", "D"]];

        assert_eq!(realign(&legends, LegendAlignment::default()), result);

        // Using an invalid alignment so it should fall back to the default
        assert_eq!(realign(&legends, LegendAlignment(42)), result);
    }
}
