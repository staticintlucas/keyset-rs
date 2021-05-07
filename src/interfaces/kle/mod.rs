mod deserialize;

use crate::error::Result;
use crate::interfaces::kle::deserialize::deserialize;
use crate::layout::{HomingType, Key, KeyType};
use crate::types::{Rect, Point, Color};

use deserialize::{RawKleProps, RawKlePropsOrLegend, RawKleMetaDataOrRow};

const LEGEND_MAP_LEN: usize = 12;

// This map is stolen straight from the kle-serial source code, and inverted (swapped indeces with
// values) since we only want to deserialize the data. Note the blanks are also filled in, so that
// (as with a few other things) keyset-rs is slightly more permissive with invalid input than KLE.
const KLE_2_ORD: [[usize; LEGEND_MAP_LEN]; 8] = [
    [0, 8, 2, 6, 9, 7, 1, 10, 3, 4, 11, 5], // 0 = no centering
    [2, 0, 3, 7, 6, 8, 9, 1, 10, 4, 11, 5], // 1 = center x
    [1, 3, 6, 0, 8, 2, 7, 9, 10, 4, 11, 5], // 2 = center y
    [1, 2, 3, 6, 0, 7, 8, 9, 10, 4, 11, 5], // 3 = center x & y
    [0, 8, 2, 6, 9, 7, 1, 10, 3, 5, 4, 11], // 4 = center front (default)
    [2, 0, 3, 5, 6, 7, 8, 1, 9, 10, 4, 11], // 5 = center front & x
    [1, 3, 5, 0, 8, 2, 6, 7, 9, 10, 4, 11], // 6 = center front & y
    [1, 2, 3, 5, 0, 6, 7, 8, 9, 10, 4, 11], // 7 = center front & x & y
];


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
    t: Color, // legend color
    ta: [Color; LEGEND_MAP_LEN], // legend color array
    a: u8,         // alignment
    p: String,     // profile
    f: u8,         // font size
    f2: u8,        // secondary font size
    fa: [u8; LEGEND_MAP_LEN],  // font size array
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
            c: Color::new(0xCC, 0xCC, 0xCC),
            t: Color::new(0, 0, 0),
            ta: [Color::new(0, 0, 0); LEGEND_MAP_LEN],
            a: 4,
            p: "".to_string(),
            f: 3,
            f2: 3,
            fa: [3; LEGEND_MAP_LEN],
        }
    }

    fn update(&mut self, props: RawKleProps) {
        if let Some(x) = props.x { self.x = x };
        if let Some(y) = props.y { self.y = y };
        self.w = props.w.unwrap_or(1.);
        self.h = props.h.unwrap_or(1.);
        self.x2 = props.x2.unwrap_or(0.);
        self.y2 = props.y2.unwrap_or(0.);
        self.w2 = props.w2.unwrap_or(self.w);
        self.h2 = props.h2.unwrap_or(self.h);
        self.l = props.l.unwrap_or(false);
        self.n = props.n.unwrap_or(false);
        self.d = props.d.unwrap_or(false);

        if let Some(c) = props.c { self.c = c };
        match props.t {
            Some(ta) if ta.len() > 0 => {
                if let Some(t) = ta[0] {
                    self.t = t;
                }
                let ta: Vec<_> = ta.iter().map(|color| color.unwrap_or(self.t)).collect();
                let len = usize::min(ta.len(), self.ta.len());
                self.ta[0..len].copy_from_slice(&ta[0..len]);
            },
            _ => (),
        }
        if let Some(a) = props.a { self.a = a };
        if let Some(p) = props.p { self.p = p };
        if let Some(f) = props.f {
            self.f = f;
            self.f2 = f;
            self.fa = [f; LEGEND_MAP_LEN];
        }
        if let Some(f2) = props.f {
            self.f2 = f2;
            self.fa = [f2; LEGEND_MAP_LEN];
            self.fa[0] = self.f;
        }
        if let Some(fa) = props.fa {
            let fa: Vec<_> = fa.iter().map(|&size| if size != 0 { size } else { self.f }).collect();
            let len = usize::min(fa.len(), self.fa.len());
            self.fa[0..len].copy_from_slice(&fa[0..len]);
        }
    }

    #[inline]
    fn next_line(&mut self) {
        self.next_key();
        self.x = 0.;
        self.y += 1.;
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

    fn to_key(&self, legends: [String; LEGEND_MAP_LEN]) -> Key {

        let position = Rect::new(
            Point::new(self.x, self.y),
            Point::new(self.x + self.w, self.y + self.h),
        );

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

        // TODO implement default color similar to KLE-serial so that the default can be stored
        // as long as "t".split("\n")[0] is empty

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

    let parsed = deserialize(json)?;

    println!("{:?}", parsed);

    let mut props = KeyProps::default();
    let mut keys = vec![];

    for item in parsed {
        if let RawKleMetaDataOrRow::Array(row) = item {
            for data in row {
                match data {
                    RawKlePropsOrLegend::Object(raw_props) => {
                        props.update(raw_props);
                    }
                    RawKlePropsOrLegend::String(legends) => {
                        let legend_array = {
                            let mut line_vec = legends.lines().map(String::from).collect::<Vec<_>>();
                            line_vec.resize(LEGEND_MAP_LEN, String::new());
                            // Note re unsafe: This memory is overwritten in its entirety in the
                            // next line, using uninitialized memory avoids the need to require a
                            // Default trait bound on the type T.
                            let mut line_arr: [String; LEGEND_MAP_LEN] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
                            line_arr.clone_from_slice(&line_vec);
                            line_arr
                        };
                        keys.push(props.to_key(legend_array));
                        props.next_key();
                    }
                }
            }
        }
        props.next_line();
    }

    Ok(keys)
}

fn realign<T: Clone>(values: [T; LEGEND_MAP_LEN], alignment: u8) -> [T; 9] {

    let alignment = if (alignment as usize) > KLE_2_ORD.len() {
        0
    } else {
        alignment as usize
    };

    // Note re unsafe: This memory is overwritten in its entirety in the next line, using
    // uninitialized memory avoids the need to require a Default trait bound on the type T.
    let mut ordered: [T; 9] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    ordered.clone_from_slice(&KLE_2_ORD[alignment].iter().map(|&item| values[item].clone()).collect::<Vec<_>>()[0..9]);

    ordered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let keys = parse(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/test.json"
        )))
        .unwrap();

        println!("{:#?}", keys);
    }

    #[test]
    fn test_parse_json2() {
        let keys = parse(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/test2.json"
        )))
        .unwrap();

        println!("{:#?}", keys);
    }
}
