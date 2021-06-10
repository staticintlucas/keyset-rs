mod utils;

use std::fmt;

use itertools::Itertools;
use serde::de::{value, Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

use self::utils::de_nl_delimited_colors;
use crate::layout::{HomingType, Key, KeyType};
use crate::types::{Color, Rect};

// The number of legends on a key adn number of alignment settings from KLE
const NUM_LEGENDS: u8 = 12;
const NUM_ALIGNMENTS: u8 = 8;

// This map is stolen straight from the kle-serial source code. Note the blanks are also filled in,
// so keyset-rs is slightly more permissive than KLE with not-strictly-valid KLE input.
const KLE_2_ORD: [[usize; NUM_LEGENDS as usize]; NUM_ALIGNMENTS as usize] = [
    [0, 6, 2, 8, 9, 11, 3, 5, 1, 4, 7, 10], // 0 = no centering
    [1, 7, 0, 2, 9, 11, 4, 3, 5, 6, 8, 10], // 1 = center x
    [3, 0, 5, 1, 9, 11, 2, 6, 4, 7, 8, 10], // 2 = center y
    [4, 0, 1, 2, 9, 11, 3, 5, 6, 7, 8, 10], // 3 = center x & y
    [0, 6, 2, 8, 10, 9, 3, 5, 1, 4, 7, 11], // 4 = center front (default)
    [1, 7, 0, 2, 10, 3, 4, 5, 6, 8, 9, 11], // 5 = center front & x
    [3, 0, 5, 1, 10, 2, 6, 7, 4, 8, 9, 11], // 6 = center front & y
    [4, 0, 1, 2, 10, 3, 5, 6, 7, 8, 9, 11], // 7 = center front & x & y
];

#[derive(Deserialize)]
struct RawKleProps {
    #[serde(default)]
    x: Option<f32>,
    #[serde(default)]
    y: Option<f32>,
    #[serde(default)]
    w: Option<f32>,
    #[serde(default)]
    h: Option<f32>,
    #[serde(default)]
    x2: Option<f32>,
    #[serde(default)]
    y2: Option<f32>,
    #[serde(default)]
    w2: Option<f32>,
    #[serde(default)]
    h2: Option<f32>,
    #[serde(default)]
    l: Option<bool>,
    #[serde(default)]
    n: Option<bool>,
    #[serde(default)]
    d: Option<bool>,
    #[serde(default)]
    c: Option<Color>,
    #[serde(default)]
    #[serde(deserialize_with = "de_nl_delimited_colors")]
    t: Option<Vec<Option<Color>>>,
    #[serde(default)]
    a: Option<u8>,
    #[serde(default)]
    p: Option<String>,
    #[serde(default)]
    f: Option<u8>,
    #[serde(default)]
    f2: Option<u8>,
    #[serde(default)]
    fa: Option<Vec<u8>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawKleRowItem {
    Object(Box<RawKleProps>),
    String(String),
}

struct RawKleFile {
    _props: Map<String, Value>,
    rows: Vec<Vec<RawKleRowItem>>,
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
    f2: u8,             // secondary font size
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
            c: Color::new(0.8, 0.8, 0.8),
            t: Color::new(0., 0., 0.),
            ta: vec![Color::new(0., 0., 0.); NUM_LEGENDS as usize],
            a: LegendAlignment::default(),
            p: "".to_string(),
            f: DEFAULT_FONT_SIZE,
            f2: DEFAULT_FONT_SIZE,
            fa: vec![DEFAULT_FONT_SIZE; NUM_LEGENDS as usize],
        }
    }

    fn update(&mut self, props: &RawKleProps) {
        self.x = props.x.unwrap_or(self.x);
        self.y = props.y.unwrap_or(self.y);
        self.w = props.w.unwrap_or(1.);
        self.h = props.h.unwrap_or(1.);
        self.x2 = props.x2.unwrap_or(0.);
        self.y2 = props.y2.unwrap_or(0.);
        self.w2 = props.w2.unwrap_or(self.w);
        self.h2 = props.h2.unwrap_or(self.h);
        self.l = props.l.unwrap_or(false);
        self.n = props.n.unwrap_or(false);
        self.d = props.d.unwrap_or(false);
        self.c = props.c.unwrap_or(self.c);

        match &props.t {
            Some(ta) if !ta.is_empty() => {
                if let Some(t) = ta[0] {
                    self.t = t;
                }
                let mut ta: Vec<_> = ta.iter().map(|color| color.unwrap_or(self.t)).collect();
                ta.resize(self.ta.len(), self.t);
                self.ta.copy_from_slice(&ta);
            }
            _ => (),
        }

        self.a = props.a.map_or(self.a, LegendAlignment::new);

        if let Some(p) = &props.p {
            self.p = p.clone()
        };
        if let Some(f) = props.f {
            self.f = f;
            self.f2 = f;
            self.fa = vec![f; NUM_LEGENDS as usize];
        }
        if let Some(f2) = props.f2 {
            self.f2 = f2;
            self.fa = vec![f2; NUM_LEGENDS as usize];
            self.fa[0] = self.f;
        }
        if let Some(fa) = &props.fa {
            let mut fa: Vec<_> = fa
                .iter()
                .map(|&size| if size == 0 { self.f } else { size })
                .collect();
            fa.resize(self.fa.len(), self.f);
            self.fa.copy_from_slice(&fa);
        }
    }

    fn next_key(&mut self) {
        // Reset variables
        self.x += self.w;
        // self.y_pos += 0.;
        self.w = 1.;
        self.h = 1.;
        self.x2 = 0.;
        self.y2 = 0.;
        self.w2 = self.w;
        self.h2 = self.h;
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

    fn to_key(&self, legends: Vec<String>) -> Key {
        let position = Rect::new(self.x, self.y, self.w, self.h);

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

        Key::new(
            position,
            key_type,
            self.c,
            realign(legends, self.a),
            realign(self.fa.clone(), self.a),
            realign(self.ta.clone(), self.a),
        )
    }
}

pub struct Layout {
    pub keys: Vec<Key>,
}

impl<'de> Deserialize<'de> for Layout {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_kle_file = RawKleFile::deserialize(deserializer)?;

        let mut props = KeyProps::default();
        let mut keys = vec![];

        for row in raw_kle_file.rows {
            for data in row {
                match data {
                    RawKleRowItem::Object(raw_props) => {
                        props.update(&*raw_props);
                    }
                    RawKleRowItem::String(legends) => {
                        let legend_array = legends
                            .lines()
                            .map(String::from)
                            .chain(std::iter::repeat(String::new()))
                            .take(NUM_LEGENDS as usize)
                            .collect::<Vec<_>>();

                        keys.push(props.to_key(legend_array));
                        props.next_key();
                    }
                }
            }
            props.next_line();
        }

        Ok(Layout { keys })
    }
}

fn realign<T: std::fmt::Debug + Clone>(values: Vec<T>, alignment: LegendAlignment) -> Vec<T> {
    let alignment = if alignment.0 as usize > KLE_2_ORD.len() {
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
    use crate::{layout::LegendMap, types::Rect};
    use assert_approx_eq::assert_approx_eq;

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
        assert_eq!(keyprops.c, Color::new(0.8, 0.8, 0.8));
        assert_eq!(keyprops.t, Color::new(0., 0., 0.));
        assert_eq!(keyprops.ta, [Color::new(0., 0., 0.); NUM_LEGENDS as usize]);
        assert_eq!(keyprops.a, LegendAlignment::default());
        assert_eq!(keyprops.p, "".to_string());
        assert_eq!(keyprops.f, 3);
        assert_eq!(keyprops.f2, 3);
        assert_eq!(keyprops.fa, [3; NUM_LEGENDS as usize]);
    }

    #[test]
    fn test_keyprops_update() {
        let mut keyprops = KeyProps::default();

        let rawprops = RawKleProps {
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
        keyprops.update(&rawprops);

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
        assert_eq!(keyprops.c, Color::new(0.8, 0.8, 0.8));
        assert_eq!(keyprops.t, Color::new(0., 0., 0.));
        assert_eq!(keyprops.ta, [Color::new(0., 0., 0.); NUM_LEGENDS as usize]);
        assert_eq!(keyprops.a, LegendAlignment::default());
        assert_eq!(keyprops.p, "".to_string());
        assert_eq!(keyprops.f, 3);
        assert_eq!(keyprops.f2, 3);
        assert_eq!(keyprops.fa, [3; NUM_LEGENDS as usize]);

        let rawprops = RawKleProps {
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
        keyprops.update(&rawprops);

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
        assert_eq!(keyprops.c, Color::new(0.5, 0.2, 0.3));
        assert_eq!(keyprops.t, Color::new(0.1, 0.1, 0.1));
        assert_eq!(
            keyprops.ta,
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
        assert_eq!(keyprops.a, LegendAlignment::new(5));
        assert_eq!(keyprops.p, "space".to_string());
        assert_eq!(keyprops.f, 4);
        assert_eq!(keyprops.f2, 4);
        assert_eq!(keyprops.fa, [4; NUM_LEGENDS as usize]);
    }

    #[test]
    fn test_keyprops_next_key() {
        let mut keyprops = KeyProps::default();
        keyprops.x = 2.;
        keyprops.w = 3.;
        keyprops.h = 1.5;

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
        let mut keyprops = KeyProps::default();
        keyprops.x = 2.;

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

        let mut keyprops = KeyProps::default();
        let key = keyprops.to_key(legends.clone());

        assert_eq!(key.position, Rect::new(0., 0., 1., 1.));
        assert_eq!(key.key_type, KeyType::Normal);
        assert_eq!(key.key_color, Color::new(0.8, 0.8, 0.8));
        assert_eq!(key.legend, LegendMap::new(ordered));
        assert_eq!(key.legend_size, LegendMap::new(vec![3; 9]));
        assert_eq!(
            key.legend_color,
            LegendMap::new(vec![Color::new(0., 0., 0.); 9])
        );

        keyprops.d = true;
        let key = keyprops.to_key(legends.clone());
        assert_eq!(key.key_type, KeyType::None);

        keyprops.n = true;
        let key = keyprops.to_key(legends.clone());
        assert_eq!(key.key_type, KeyType::Homing(HomingType::Default));

        keyprops.p = "space".into();
        let key = keyprops.to_key(legends.clone());
        assert_eq!(key.key_type, KeyType::Space);

        keyprops.p = "scoop".into();
        let key = keyprops.to_key(legends.clone());
        assert_eq!(key.key_type, KeyType::Homing(HomingType::Scoop));

        keyprops.p = "bar".into();
        let key = keyprops.to_key(legends.clone());
        assert_eq!(key.key_type, KeyType::Homing(HomingType::Bar));

        keyprops.p = "bump".into();
        let key = keyprops.to_key(legends.clone());
        assert_eq!(key.key_type, KeyType::Homing(HomingType::Bump));
    }

    #[test]
    fn test_deserialize() {
        let result = serde_json::from_str::<'_, Layout>(
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
                ]
            ]"#,
        )
        .unwrap();

        assert_eq!(result.keys.len(), 3);
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
