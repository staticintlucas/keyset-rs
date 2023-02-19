mod color;
mod geometry;

use std::fmt::Display;

pub(crate) use color::{Color, InvalidColor};
pub(crate) use geometry::{Point, Rect, RoundRect, Scale, Size};

// Utility wrapper used to format floats with minimal number of characters
pub struct Trim(pub f32);

impl Display for Trim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!("{:.5}", self.0);
        write!(f, "{}", str.trim_end_matches('0').trim_end_matches('.'))
    }
}
