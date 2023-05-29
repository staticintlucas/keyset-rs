use std::array;

use itertools::Itertools;

use crate::error::Result;
use crate::key::{Legend, Shape};
use crate::utils::Vec2;

use super::props::{NUM_ALIGNMENTS, NUM_LEGENDS};
use super::InvalidKleLayout;

// This map is the same as that of kle-serial. Note the blanks are also filled in, so we're slightly
// more permissive with not-strictly-valid KLE input.
const KLE_2_ORD: [[usize; NUM_LEGENDS]; NUM_ALIGNMENTS] = [
    [0, 6, 2, 8, 9, 11, 3, 5, 1, 4, 7, 10], // 0 = no centering
    [1, 7, 0, 2, 9, 11, 4, 3, 5, 6, 8, 10], // 1 = center x
    [3, 0, 5, 1, 9, 11, 2, 6, 4, 7, 8, 10], // 2 = center y
    [4, 0, 1, 2, 9, 11, 3, 5, 6, 7, 8, 10], // 3 = center x & y
    [0, 6, 2, 8, 10, 9, 3, 5, 1, 4, 7, 11], // 4 = center front (default)
    [1, 7, 0, 2, 10, 3, 4, 5, 6, 8, 9, 11], // 5 = center front & x
    [3, 0, 5, 1, 10, 2, 6, 7, 4, 8, 9, 11], // 6 = center front & y
    [4, 0, 1, 2, 10, 3, 5, 6, 7, 8, 9, 11], // 7 = center front & x & y
];

pub fn realign_legends<T>(values: T, alignment: usize) -> Result<[[Option<Legend>; 3]; 3]>
where
    T: IntoIterator<Item = Option<Legend>>,
{
    let mapping = KLE_2_ORD.get(alignment).ok_or(InvalidKleLayout {
        message: format!("Unsupported legend alignment ({alignment}). Expected <{NUM_ALIGNMENTS}"),
    })?;

    // Rearrange values based on the mapping and reshape into [[T; 3]; 3]
    let mut iter = mapping
        .iter()
        .zip(values)
        .sorted_by_key(|(&i, _v)| i)
        .map(|(_i, v)| v);
    Ok(array::from_fn(|_| {
        array::from_fn(|_| iter.next().unwrap_or(None))
    }))
}

pub fn key_shape_from_kle(w: f32, h: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> Result<Shape> {
    fn is_close<const N: usize>(a: &[f32; N], b: &[f32; N]) -> bool {
        a.iter().zip(b).all(|(a, b)| (b - a).abs() < 1e-2)
    }

    if is_close(&[w, h, x2, y2, w2, h2], &[1.25, 1., 0., 0., 1.75, 1.]) {
        Ok(Shape::SteppedCaps)
    } else if is_close(&[w, h, x2, y2, w2, h2], &[1.25, 2., -0.25, 0., 1.5, 1.]) {
        Ok(Shape::IsoVertical)
    } else if is_close(&[w, h, x2, y2, w2, h2], &[1.5, 1., 0.25, 0., 1.25, 2.]) {
        Ok(Shape::IsoHorizontal)
    } else if is_close(&[x2, y2, w2, h2], &[0., 0., w, h]) {
        Ok(Shape::Normal(Vec2::new(w, h)))
    } else {
        Err(InvalidKleLayout {
            message: format!(
                "Unsupported non-standard key size \
                (w: {w:.2}, h: {h:.2}, x2: {x2:.2}, y2: {y2:.2}, w2: {w2:.2}, h2: {h2:.2}). \
                Note ISO enter and stepped caps are supported as special cases"
            ),
        })?
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_key_shape_from_kle() {
        let regular_key = key_shape_from_kle(2.25, 1., 0., 0., 2.25, 1.).unwrap();
        let iso_horiz = key_shape_from_kle(1.5, 1., 0.25, 0., 1.25, 2.).unwrap();
        let iso_vert = key_shape_from_kle(1.25, 2., -0.25, 0., 1.5, 1.).unwrap();
        let step_caps = key_shape_from_kle(1.25, 1., 0., 0., 1.75, 1.).unwrap();

        assert_eq!(regular_key, Shape::Normal(Vec2::new(2.25, 1.)));
        assert_eq!(iso_horiz, Shape::IsoHorizontal);
        assert_eq!(iso_vert, Shape::IsoVertical);
        assert_eq!(step_caps, Shape::SteppedCaps);
    }

    #[test]
    fn test_key_shape_from_kle_invalid() {
        let invalid = key_shape_from_kle(1., 1., -0.25, 0., 1.5, 1.);

        assert!(invalid.is_err());
        assert_eq!(
            format!("{}", invalid.unwrap_err()),
            format!(concat!(
                "error parsing KLE layout: Unsupported non-standard key size (w: 1.00, h: 1.00, ",
                "x2: -0.25, y2: 0.00, w2: 1.50, h2: 1.00). Note ISO enter and stepped caps are ",
                "supported as special cases"
            ))
        );
    }
}
