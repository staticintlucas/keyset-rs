use std::array;

use itertools::izip;

use crate::error::Result;
use crate::key::{self, Key, Legend};
use crate::utils::{Color, Vec2};

use super::de::KlePropsObject;
use super::utils::{key_shape_from_kle, realign_legends};

// The number of legends on a key and number of alignment settings from KLE
pub const NUM_LEGENDS: usize = 12;
pub const NUM_ALIGNMENTS: usize = 8;

pub const DEFAULT_ALIGNMENT: usize = 4; // This is the default used by KLE

#[derive(Debug)]
pub struct KleProps {
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
    c: Color,                 // color
    t: Color,                 // fallback legend color
    ta: [Color; NUM_LEGENDS], // legend color array
    a: usize,                 // alignment
    p: String,                // profile
    f: usize,                 // fallback font size
    fa: [usize; NUM_LEGENDS], // font size array
}

impl Default for KleProps {
    fn default() -> Self {
        const DEFAULT_FONT_SIZE: usize = 3; // The default font size
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
            a: DEFAULT_ALIGNMENT,
            p: String::new(),
            f: DEFAULT_FONT_SIZE,
            fa: [DEFAULT_FONT_SIZE; NUM_LEGENDS],
        }
    }
}

impl KleProps {
    pub fn update(&mut self, props: KlePropsObject) {
        let f = props.f.unwrap_or(self.f);
        let fa = if let Some(fa) = props.fa {
            array::from_fn(|i| match fa.get(i).copied() {
                Some(fa) if fa > 0 => fa,
                _ => f,
            })
        } else if let Some(f2) = props.f2 {
            array::from_fn(|i| if i == 0 { f } else { f2 })
        } else if let Some(f) = props.f {
            [f; NUM_LEGENDS]
        } else {
            self.fa
        };

        let t = (props.t.as_ref())
            .and_then(|v| v.first().copied().flatten())
            .unwrap_or(self.t);
        let ta = props.t.map_or(self.ta, |ta| {
            array::from_fn(|i| ta.get(i).copied().flatten().unwrap_or(t))
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
        self.a = props.a.unwrap_or(self.a);
        self.p = props.p.unwrap_or(self.p.clone());
        self.f = f;
        self.fa = fa;
    }

    #[inline]
    pub fn next_key(&mut self) {
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
    pub fn next_line(&mut self) {
        self.next_key();
        self.x = 0.;
        self.y += 1.;
    }

    pub fn build_key(&self, legends: [String; NUM_LEGENDS]) -> Result<Key> {
        // Use (x + x2) if (x2 < 0). Needed because we always measure position to the top left
        // corner of the key rather than just the primary rectangle
        let position = Vec2::new(self.x + self.x2.min(0.), self.y + self.y2.min(0.));
        let shape = key_shape_from_kle(self.w, self.h, self.x2, self.y2, self.w2, self.h2)?;

        let typ = if self.p.contains("scoop") || self.p.contains("dish") {
            key::Type::Homing(Some(key::Homing::Scoop))
        } else if self.p.contains("bar") || self.p.contains("line") {
            key::Type::Homing(Some(key::Homing::Bar))
        } else if self.p.contains("bump")
            || self.p.contains("dot")
            || self.p.contains("nub")
            || self.p.contains("nipple")
        {
            key::Type::Homing(Some(key::Homing::Bump))
        } else if self.p.contains("space") {
            key::Type::Space
        } else if self.n {
            key::Type::Homing(None)
        } else if self.d {
            key::Type::None
        } else {
            key::Type::Normal
        };

        let color = self.c;

        let legends = izip!(legends, self.fa, self.ta)
            .map(|(text, size, color)| (!text.is_empty()).then_some(Legend { text, size, color }));
        let legends = realign_legends(legends, self.a)?;

        Ok(Key {
            position,
            shape,
            typ,
            color,
            legends,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_kleprops_default() {
        let kleprops = KleProps::default();

        assert_approx_eq!(kleprops.x, 0.);
        assert_approx_eq!(kleprops.y, 0.);
        assert_approx_eq!(kleprops.w, 1.);
        assert_approx_eq!(kleprops.h, 1.);
        assert_approx_eq!(kleprops.x2, 0.);
        assert_approx_eq!(kleprops.y2, 0.);
        assert_approx_eq!(kleprops.w2, 1.);
        assert_approx_eq!(kleprops.h2, 1.);
        assert_eq!(kleprops.l, false);
        assert_eq!(kleprops.n, false);
        assert_eq!(kleprops.d, false);
        assert_eq!(kleprops.c, Color::default_key());
        assert_eq!(kleprops.t, Color::default_legend());
        assert_eq!(kleprops.ta, [Color::default_legend(); NUM_LEGENDS]);
        assert_eq!(kleprops.a, DEFAULT_ALIGNMENT);
        assert_eq!(kleprops.p, "");
        assert_eq!(kleprops.f, 3);
        assert_eq!(kleprops.fa, [3; NUM_LEGENDS]);
    }

    #[test]
    fn test_keyprops_update() {
        let rawprops1 = KlePropsObject {
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
        let mut keyprops = KleProps::default();
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
        assert_eq!(keyprops.a, DEFAULT_ALIGNMENT);
        assert_eq!(keyprops.p, "".to_string());
        assert_eq!(keyprops.f, 3);
        assert_eq!(keyprops.fa, [3; NUM_LEGENDS]);

        let rawprops2 = KlePropsObject {
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
        assert_eq!(keyprops.a, 5);
        assert_eq!(keyprops.p, "space".to_string());
        assert_eq!(keyprops.f, 4);
        assert_eq!(keyprops.fa, [4; NUM_LEGENDS]);

        let rawprops3 = KlePropsObject {
            f: Some(2),
            f2: Some(4),
            ..KlePropsObject::default()
        };
        keyprops.update(rawprops3);
        assert_eq!(keyprops.fa, [2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4]);

        let rawprops4 = KlePropsObject {
            f: Some(5),
            ..KlePropsObject::default()
        };
        keyprops.update(rawprops4);
        assert_eq!(keyprops.fa, [5; NUM_LEGENDS]);
    }

    #[test]
    fn test_keyprops_next_key() {
        let mut keyprops = KleProps {
            x: 2.0,
            w: 3.0,
            h: 1.5,
            ..KleProps::default()
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
        let mut keyprops = KleProps {
            x: 2.0,
            ..KleProps::default()
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
    fn test_keyprops_build_key() {
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
        let expected = [["A", "I", "C"], ["G", "J", "H"], ["B", "K", "D"]];

        let keyprops1 = KleProps::default();
        let key1 = keyprops1.build_key(legends.clone()).unwrap();

        assert_eq!(key1.position, Vec2::ZERO);
        assert_eq!(key1.shape, key::Shape::Normal(Vec2::from(1.)));
        assert_eq!(key1.typ, key::Type::Normal);
        assert_eq!(key1.color, Color::default_key());
        for (res, exp) in key1.legends.iter().zip(expected) {
            for (r, e) in res.iter().zip(exp) {
                let r = r.as_ref().unwrap();
                assert_eq!(r.text, e);
                assert_eq!(r.size, 3);
                assert_eq!(r.color, Color::default_legend());
            }
        }

        let keyprops2 = KleProps {
            d: true,
            ..keyprops1
        };
        let key2 = keyprops2.build_key(legends.clone()).unwrap();
        assert_eq!(key2.typ, key::Type::None);

        let keyprops3 = KleProps {
            n: true,
            ..keyprops2
        };
        let key3 = keyprops3.build_key(legends.clone()).unwrap();
        assert_eq!(key3.typ, key::Type::Homing(None));

        let keyprops4 = KleProps {
            p: "space".into(),
            ..keyprops3
        };
        let key4 = keyprops4.build_key(legends.clone()).unwrap();
        assert_eq!(key4.typ, key::Type::Space);

        let keyprops5 = KleProps {
            p: "scoop".into(),
            ..keyprops4
        };
        let key5 = keyprops5.build_key(legends.clone()).unwrap();
        assert_eq!(key5.typ, key::Type::Homing(Some(key::Homing::Scoop)));

        let keyprops6 = KleProps {
            p: "bar".into(),
            ..keyprops5
        };
        let key6 = keyprops6.build_key(legends.clone()).unwrap();
        assert_eq!(key6.typ, key::Type::Homing(Some(key::Homing::Bar)));

        let keyprops7 = KleProps {
            p: "bump".into(),
            ..keyprops6
        };
        let key7 = keyprops7.build_key(legends.clone()).unwrap();
        assert_eq!(key7.typ, key::Type::Homing(Some(key::Homing::Bump)));
    }
}
