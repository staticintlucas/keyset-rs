use std::{array, fmt};

use kle_serial as kle;
use kurbo::{Point, Size};

use crate::error::{Error, Result};
use crate::key::{self, Key, Legend, Shape};

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
        #[allow(clippy::cast_possible_truncation)]
        Ok(Shape::Normal(Size::new(w, h)))
    } else {
        // TODO support all key shapes/sizes
        Err(InvalidKleLayout {
            message: format!(
                "Unsupported non-standard key size \
                (w: {w:.2}, h: {h:.2}, x2: {x2:.2}, y2: {y2:.2}, w2: {w2:.2}, h2: {h2:.2}). \
                Note only ISO enter and stepped caps are supported as special cases"
            ),
        })?
    }
}

fn key_type_from_kle(key: &kle::Key) -> key::Type {
    // TODO support ghosted keys?
    if key.profile.contains("scoop") || key.profile.contains("dish") {
        key::Type::Homing(Some(key::Homing::Scoop))
    } else if key.profile.contains("bar") || key.profile.contains("line") {
        key::Type::Homing(Some(key::Homing::Bar))
    } else if key.profile.contains("bump")
        || key.profile.contains("dot")
        || key.profile.contains("nub")
        || key.profile.contains("nipple")
    {
        key::Type::Homing(Some(key::Homing::Bump))
    } else if key.profile.contains("space") {
        key::Type::Space
    } else if key.homing {
        key::Type::Homing(None)
    } else if key.decal {
        key::Type::None
    } else {
        key::Type::Normal
    }
}

impl From<kle::Legend> for Legend {
    fn from(legend: kle::Legend) -> Self {
        let kle::Legend { text, size, color } = legend;
        let color = color.rgb().into();
        Self { text, size, color }
    }
}

impl TryFrom<kle::Key> for Key {
    type Error = Error;

    fn try_from(key: kle::Key) -> Result<Self> {
        #[allow(clippy::cast_possible_truncation)]
        Ok(Self {
            position: Point::new(key.x + key.x2.min(0.), key.y + key.y2.min(0.)),
            shape: key_shape_from_kle(&key)?,
            typ: key_type_from_kle(&key),
            color: key.color.rgb().into(),
            legends: array::from_fn(|col| {
                array::from_fn(|row| key.legends[col * 3 + row].clone().map(Legend::from))
            }),
        })
    }
}

pub fn from_json(json: &str) -> Result<impl Iterator<Item = Result<Key>>> {
    Ok(serde_json::from_str::<kle::KeyIterator>(json)?.map(Key::try_from))
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;
    use itertools::Itertools;

    #[test]
    fn test_kle_from_json() {
        let result1: Vec<_> = from_json(
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
        .try_collect()
        .unwrap();

        assert_eq!(result1.len(), 4);
        assert_approx_eq!(result1[0].position.x, 0.0);
        assert_approx_eq!(result1[1].position.x, 1.0);
        assert_approx_eq!(result1[2].position.x, 1.5);
        assert_approx_eq!(result1[3].position.x, 0.0);

        let result2: Vec<_> = from_json(
            r#"[
                [
                    "A"
                ]
            ]"#,
        )
        .unwrap()
        .try_collect()
        .unwrap();

        assert_eq!(result2.len(), 1);
    }
}
