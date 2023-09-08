mod error;

use kle_serial as kle;
use kurbo::{Point, Size};

use crate::{Homing, Key, Legend, Shape, Type};
pub use error::{Error, Result};

fn key_shape_from_kle(key: &kle::Key) -> Result<Shape> {
    fn is_close<const N: usize>(a: &[f64; N], b: &[f64; N]) -> bool {
        a.iter().zip(b).all(|(a, b)| (b - a).abs() < 1e-2)
    }

    let &kle::Key {
        width: w,
        height: h,
        x2,
        y2,
        width2: w2,
        height2: h2,
        ..
    } = key;

    if is_close(&[w, h, x2, y2, w2, h2], &[1.25, 1., 0., 0., 1.75, 1.]) {
        Ok(Shape::SteppedCaps)
    } else if is_close(&[w, h, x2, y2, w2, h2], &[1.25, 2., -0.25, 0., 1.5, 1.]) {
        Ok(Shape::IsoVertical)
    } else if is_close(&[w, h, x2, y2, w2, h2], &[1.5, 1., 0.25, 0., 1.25, 2.]) {
        Ok(Shape::IsoHorizontal)
    } else if is_close(&[x2, y2, w2, h2], &[0., 0., w, h]) {
        Ok(Shape::Normal(Size::new(w, h)))
    } else {
        // TODO support all key shapes/sizes
        Err(Error::UnsupportedKeySize {
            w,
            h,
            x2,
            y2,
            w2,
            h2,
        })
    }
}

fn key_type_from_kle(key: &kle::Key) -> Type {
    const SCOOP_KW: [&str; 2] = ["scoop", "dish"];
    const BAR_KW: [&str; 2] = ["bar", "line"];
    const BUMP_KW: [&str; 4] = ["bump", "dot", "nub", "nipple"];

    // TODO support ghosted keys?
    if SCOOP_KW.iter().any(|kw| key.profile.contains(kw)) {
        Type::Homing(Some(Homing::Scoop))
    } else if BAR_KW.iter().any(|kw| key.profile.contains(kw)) {
        Type::Homing(Some(Homing::Bar))
    } else if BUMP_KW.iter().any(|kw| key.profile.contains(kw)) {
        Type::Homing(Some(Homing::Bump))
    } else if key.profile.contains("space") {
        Type::Space
    } else if key.homing {
        Type::Homing(None)
    } else if key.decal {
        Type::None
    } else {
        Type::Normal
    }
}

impl From<kle::Legend> for Legend {
    fn from(legend: kle::Legend) -> Self {
        let kle::Legend { text, size, color } = legend;
        Self {
            text,
            size_idx: size,
            color: color.rgb().into(),
        }
    }
}

impl TryFrom<kle::Key> for Key {
    type Error = Error;

    fn try_from(mut key: kle::Key) -> Result<Self> {
        let position = Point::new(key.x + key.x2.min(0.), key.y + key.y2.min(0.));
        let shape = key_shape_from_kle(&key)?;
        let typ = key_type_from_kle(&key);
        let color = key.color.rgb().into();
        let legends = {
            let mut arr = <[Option<kle::Legend>; 9]>::default();
            arr.swap_with_slice(&mut key.legends[..9]);
            arr
        };
        let legends = legends.map(|l| l.map(Legend::from)).into();
        Ok(Self {
            position,
            shape,
            typ,
            color,
            legends,
        })
    }
}

pub fn from_json(json: &str) -> Result<Vec<Key>> {
    let key_iter: kle::KeyIterator = serde_json::from_str(json)?;
    key_iter.map(Key::try_from).collect()
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use unindent::unindent;

    use super::*;

    #[test]
    fn test_key_shape_from_kle() {
        let regular_key = key_shape_from_kle(&kle::Key {
            width: 2.25,
            height: 1.,
            x2: 0.,
            y2: 0.,
            width2: 2.25,
            height2: 1.,
            ..Default::default()
        })
        .unwrap();
        let iso_horiz = key_shape_from_kle(&kle::Key {
            width: 1.5,
            height: 1.,
            x2: 0.25,
            y2: 0.,
            width2: 1.25,
            height2: 2.,
            ..Default::default()
        })
        .unwrap();
        let iso_vert = key_shape_from_kle(&kle::Key {
            width: 1.25,
            height: 2.,
            x2: -0.25,
            y2: 0.,
            width2: 1.5,
            height2: 1.,
            ..Default::default()
        })
        .unwrap();
        let step_caps = key_shape_from_kle(&kle::Key {
            width: 1.25,
            height: 1.,
            x2: 0.,
            y2: 0.,
            width2: 1.75,
            height2: 1.,
            ..Default::default()
        })
        .unwrap();

        assert_matches!(regular_key, Shape::Normal(size) if size == Size::new(2.25, 1.));
        assert_matches!(iso_horiz, Shape::IsoHorizontal);
        assert_matches!(iso_vert, Shape::IsoVertical);
        assert_matches!(step_caps, Shape::SteppedCaps);
    }

    #[test]
    fn test_key_shape_from_kle_invalid() {
        let invalid = key_shape_from_kle(&kle::Key {
            width: 1.,
            height: 1.,
            x2: -0.25,
            y2: 0.,
            width2: 1.5,
            height2: 1.,
            ..Default::default()
        });

        assert!(invalid.is_err());
        assert_eq!(
            format!("{}", invalid.unwrap_err()),
            format!(concat!(
                "unsupported non-standard key size (w: 1.00, h: 1.00, ",
                "x2: -0.25, y2: 0.00, w2: 1.50, h2: 1.00). Note only ISO enter and stepped caps ",
                "are supported as special cases"
            ))
        );
    }

    #[test]
    fn test_key_type_from_kle() {
        let regular_key = key_type_from_kle(&kle::Key {
            ..Default::default()
        });
        let decal = key_type_from_kle(&kle::Key {
            decal: true,
            ..Default::default()
        });
        let space = key_type_from_kle(&kle::Key {
            profile: "space".into(),
            ..Default::default()
        });
        let homing_default = key_type_from_kle(&kle::Key {
            homing: true,
            ..Default::default()
        });
        let homing_scoop = key_type_from_kle(&kle::Key {
            profile: "scoop".into(),
            ..Default::default()
        });
        let homing_bar = key_type_from_kle(&kle::Key {
            profile: "bar".into(),
            ..Default::default()
        });
        let homing_bump = key_type_from_kle(&kle::Key {
            profile: "bump".into(),
            ..Default::default()
        });

        assert_matches!(regular_key, Type::Normal);
        assert_matches!(decal, Type::None);
        assert_matches!(space, Type::Space);
        assert_matches!(homing_default, Type::Homing(None));
        assert_matches!(homing_scoop, Type::Homing(Some(Homing::Scoop)));
        assert_matches!(homing_bar, Type::Homing(Some(Homing::Bar)));
        assert_matches!(homing_bump, Type::Homing(Some(Homing::Bump)));
    }

    #[test]
    fn test_kle_from_json() {
        let result1: Vec<_> = from_json(&unindent(
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
        ))
        .unwrap();

        assert_eq!(result1.len(), 4);
        assert_eq!(result1[0].position.x, 0.0);
        assert_eq!(result1[1].position.x, 1.0);
        assert_eq!(result1[2].position.x, 1.5);
        assert_eq!(result1[3].position.x, 0.0);

        let result2: Vec<_> = from_json(&unindent(
            r#"[
                [
                    "A"
                ]
            ]"#,
        ))
        .unwrap();

        assert_eq!(result2.len(), 1);
    }
}
