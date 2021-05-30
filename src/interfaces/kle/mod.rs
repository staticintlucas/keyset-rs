mod utils;

use std::convert::TryInto;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::{Map, Value};

use self::utils::de_nl_delimited_colors;
use crate::error::Result;
use crate::layout::{HomingType, Key, KeyType};
use crate::types::{Color, Rect};

const LEGEND_MAP_LEN: usize = 12;

// This map is stolen straight from the kle-serial source code. Note the blanks are also filled in,
// so keyset-rs is slightly more permissive than KLE with not-strictly-valid KLE input.
const KLE_2_ORD: [[usize; LEGEND_MAP_LEN]; 8] = [
    [0, 6, 2, 8, 9, 11, 3, 5, 1, 4, 7, 10], // 0 = no centering
    [1, 7, 0, 2, 9, 11, 4, 3, 5, 6, 8, 10], // 1 = center x
    [3, 0, 5, 1, 9, 11, 2, 6, 4, 7, 8, 10], // 2 = center y
    [4, 0, 1, 2, 9, 11, 3, 5, 6, 7, 8, 10], // 3 = center x & y
    [0, 6, 2, 8, 10, 9, 3, 5, 1, 4, 7, 11], // 4 = center front (default)
    [1, 7, 0, 2, 10, 3, 4, 5, 6, 8, 9, 11], // 5 = center front & x
    [3, 0, 5, 1, 10, 2, 6, 7, 4, 8, 9, 11], // 6 = center front & y
    [4, 0, 1, 2, 10, 3, 5, 6, 7, 8, 9, 11], // 7 = center front & x & y
];

// The default alignment when none is specified in the KLE data (index in KLE_2_ORD)
const DEFAULT_ALIGNMENT: u8 = 4;

// The default font size
const DEFAULT_FONT_SIZE: u8 = 3;

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
    t: Color,                    // legend color
    ta: [Color; LEGEND_MAP_LEN], // legend color array
    a: u8,                       // alignment
    p: String,                   // profile
    f: u8,                       // font size
    f2: u8,                      // secondary font size
    fa: [u8; LEGEND_MAP_LEN],    // font size array
}

impl KeyProps {
    fn default() -> Self {
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
            ta: [Color::new(0., 0., 0.); LEGEND_MAP_LEN],
            a: DEFAULT_ALIGNMENT,
            p: "".to_string(),
            f: DEFAULT_FONT_SIZE,
            f2: DEFAULT_FONT_SIZE,
            fa: [DEFAULT_FONT_SIZE; LEGEND_MAP_LEN],
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

        self.a = props.a.unwrap_or(self.a);

        if let Some(p) = &props.p {
            self.p = p.clone()
        };
        if let Some(f) = props.f {
            self.f = f;
            self.f2 = f;
            self.fa = [f; LEGEND_MAP_LEN];
        }
        if let Some(f2) = props.f2 {
            self.f2 = f2;
            self.fa = [f2; LEGEND_MAP_LEN];
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

    fn to_key(&self, legends: [String; LEGEND_MAP_LEN]) -> Key {
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
            realign(self.fa, self.a),
            realign(self.ta, self.a),
        )
    }
}

pub fn parse(json: &str) -> Result<Vec<Key>> {
    let parsed = serde_json::from_str::<'_, RawKleFile>(json)?;

    let mut props = KeyProps::default();
    let mut keys = vec![];

    for row in parsed.rows {
        for data in row {
            match data {
                RawKleRowItem::Object(raw_props) => {
                    props.update(&raw_props);
                }
                RawKleRowItem::String(legends) => {
                    let legend_array: [String; LEGEND_MAP_LEN] = {
                        let mut line_vec = legends.lines().map(String::from).collect::<Vec<_>>();
                        line_vec.resize(LEGEND_MAP_LEN, String::new());
                        line_vec.try_into().unwrap()
                    };
                    keys.push(props.to_key(legend_array));
                    props.next_key();
                }
            }
        }
        props.next_line();
    }

    Ok(keys)
}

fn realign<T: std::fmt::Debug + Clone>(values: [T; LEGEND_MAP_LEN], alignment: u8) -> [T; 9] {
    let alignment = if (alignment as usize) > KLE_2_ORD.len() {
        DEFAULT_ALIGNMENT // This is the default used by KLE
    } else {
        alignment
    } as usize;

    Vec::from(values)
        .into_iter()
        .enumerate()
        .sorted_by_key(|(i, _v)| KLE_2_ORD[alignment][*i])
        .map(|(_i, v)| v)
        .take(9)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{layout::LegendMap, types::Rect};
    use assert_approx_eq::assert_approx_eq;

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
        assert_eq!(keyprops.ta, [Color::new(0., 0., 0.); LEGEND_MAP_LEN]);
        assert_eq!(keyprops.a, 4);
        assert_eq!(keyprops.p, "".to_string());
        assert_eq!(keyprops.f, 3);
        assert_eq!(keyprops.f2, 3);
        assert_eq!(keyprops.fa, [3; LEGEND_MAP_LEN]);
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
        assert_eq!(keyprops.ta, [Color::new(0., 0., 0.); LEGEND_MAP_LEN]);
        assert_eq!(keyprops.a, 4);
        assert_eq!(keyprops.p, "".to_string());
        assert_eq!(keyprops.f, 3);
        assert_eq!(keyprops.f2, 3);
        assert_eq!(keyprops.fa, [3; LEGEND_MAP_LEN]);

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
        assert_eq!(keyprops.a, 5);
        assert_eq!(keyprops.p, "space".to_string());
        assert_eq!(keyprops.f, 4);
        assert_eq!(keyprops.f2, 4);
        assert_eq!(keyprops.fa, [4; LEGEND_MAP_LEN]);
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
            "A".into(),
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
        assert_eq!(key.legend_size, LegendMap::new([3; 9]));
        assert_eq!(
            key.legend_color,
            LegendMap::new([Color::new(0., 0., 0.); 9])
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
    fn test_parse_json() {
        let result = parse(
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

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_realign() {
        let legends = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"];
        let result = ["A", "I", "C", "G", "J", "H", "B", "K", "D"];

        assert_eq!(realign(legends, DEFAULT_ALIGNMENT), result);

        // Using an invalid alignment so it should fall back to the default
        assert_eq!(realign(legends, 42), result);
    }
}
