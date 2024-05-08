//! Load KLE layouts from JSON files

mod error;

use geom::{Point, Size};
use kle_serial::f32 as kle;

use crate::{Homing, Key, Legend, Shape};
pub use error::{Error, Result};

fn shape_from_kle(key: &kle::Key) -> Result<Shape> {
    const STEP_CAPS: [f32; 6] = [1.25, 1.0, 0.0, 0.0, 1.75, 1.0];
    const ISO_VERT: [f32; 6] = [1.25, 2.0, -0.25, 0.0, 1.5, 1.0];
    const ISO_HORIZ: [f32; 6] = [1.5, 1.0, 0.25, 0.0, 1.25, 2.0];

    fn is_close<const N: usize>(a: &[f32; N], b: &[f32; N]) -> bool {
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

    let is_normal = is_close(&[x2, y2, w2, h2], &[0.0, 0.0, w, h]);
    let is_1u = is_normal && is_close(&[w, h], &[1.0, 1.0]);

    let dims = [w, h, x2, y2, w2, h2];

    if is_1u && (key.profile.contains("scoop") || key.profile.contains("dish")) {
        Ok(Shape::Homing(Some(Homing::Scoop)))
    } else if is_1u && key.profile.contains("bar") {
        Ok(Shape::Homing(Some(Homing::Bar)))
    } else if is_1u && (key.profile.contains("bump") || key.profile.contains("dot")) {
        Ok(Shape::Homing(Some(Homing::Bump)))
    } else if is_normal && key.profile.contains("space") {
        Ok(Shape::Space(Size::new(w, h)))
    } else if is_1u && key.homing {
        Ok(Shape::Homing(None))
    } else if key.decal {
        Ok(Shape::None(Size::new(w, h)))
    } else if is_normal {
        Ok(Shape::Normal(Size::new(w, h)))
    } else if is_close(&dims, &STEP_CAPS) {
        Ok(Shape::SteppedCaps)
    } else if is_close(&dims, &ISO_VERT) {
        Ok(Shape::IsoVertical)
    } else if is_close(&dims, &ISO_HORIZ) {
        Ok(Shape::IsoHorizontal)
    } else {
        // TODO arbitrary key shapes/sizes
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

impl From<kle::Legend> for Legend {
    #[inline]
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
        let position = Point::new(key.x + key.x2.min(0.0), key.y + key.y2.min(0.0));
        let shape = shape_from_kle(&key)?;
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
            color,
            legends,
        })
    }
}

/// Loads a KLE layout from a JSON string into a [`Vec<Key>`]
///
/// # Errors
///
/// If an invalid or unsupported JSON string is encountered, this will return an [`Error`]
#[inline]
pub fn from_json(json: &str) -> Result<Vec<Key>> {
    let key_iter: kle::KeyIterator = serde_json::from_str(json)?;
    key_iter.map(Key::try_from).collect()
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use isclose::{assert_is_close, IsClose};
    use unindent::unindent;

    use super::*;

    #[test]
    fn key_shape_from_kle() {
        let default_key = shape_from_kle(&kle::Key::default()).unwrap();
        let decal = shape_from_kle(&kle::Key {
            decal: true,
            ..Default::default()
        })
        .unwrap();
        let space = shape_from_kle(&kle::Key {
            profile: "space".into(),
            ..Default::default()
        })
        .unwrap();
        let homing_default = shape_from_kle(&kle::Key {
            homing: true,
            ..Default::default()
        })
        .unwrap();
        let homing_scoop = shape_from_kle(&kle::Key {
            profile: "scoop".into(),
            ..Default::default()
        })
        .unwrap();
        let homing_bar = shape_from_kle(&kle::Key {
            profile: "bar".into(),
            ..Default::default()
        })
        .unwrap();
        let homing_bump = shape_from_kle(&kle::Key {
            profile: "bump".into(),
            ..Default::default()
        })
        .unwrap();
        let regular_key = shape_from_kle(&kle::Key {
            width: 2.25,
            height: 1.0,
            x2: 0.0,
            y2: 0.0,
            width2: 2.25,
            height2: 1.0,
            ..Default::default()
        })
        .unwrap();
        let iso_horiz = shape_from_kle(&kle::Key {
            width: 1.5,
            height: 1.0,
            x2: 0.25,
            y2: 0.0,
            width2: 1.25,
            height2: 2.0,
            ..Default::default()
        })
        .unwrap();
        let iso_vert = shape_from_kle(&kle::Key {
            width: 1.25,
            height: 2.0,
            x2: -0.25,
            y2: 0.0,
            width2: 1.5,
            height2: 1.0,
            ..Default::default()
        })
        .unwrap();
        let step_caps = shape_from_kle(&kle::Key {
            width: 1.25,
            height: 1.0,
            x2: 0.0,
            y2: 0.0,
            width2: 1.75,
            height2: 1.0,
            ..Default::default()
        })
        .unwrap();

        assert_matches!(default_key, Shape::Normal(size) if size.is_close(Size::new(1.0, 1.0)));
        assert_matches!(regular_key, Shape::Normal(size) if size.is_close(Size::new(2.25, 1.0)));
        assert_matches!(decal, Shape::None(size) if size.is_close(Size::new(1.0, 1.0)));
        assert_matches!(space, Shape::Space(size) if size.is_close(Size::new(1.0, 1.0)));
        assert_matches!(homing_default, Shape::Homing(None));
        assert_matches!(homing_scoop, Shape::Homing(Some(Homing::Scoop)));
        assert_matches!(homing_bar, Shape::Homing(Some(Homing::Bar)));
        assert_matches!(homing_bump, Shape::Homing(Some(Homing::Bump)));
        assert_matches!(iso_horiz, Shape::IsoHorizontal);
        assert_matches!(iso_vert, Shape::IsoVertical);
        assert_matches!(step_caps, Shape::SteppedCaps);
    }

    #[test]
    fn key_shape_from_kle_invalid() {
        let invalid = shape_from_kle(&kle::Key {
            width: 1.0,
            height: 1.0,
            x2: -0.25,
            y2: 0.0,
            width2: 1.5,
            height2: 1.0,
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
    fn kle_from_json() {
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
        assert_is_close!(result1[0].position, Point::new(0.0, 0.0));
        assert_is_close!(result1[1].position, Point::new(1.0, 0.0));
        assert_is_close!(result1[2].position, Point::new(1.5, 0.25));
        assert_is_close!(result1[3].position, Point::new(0.0, 1.25));

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
