mod color;
mod geometry;

pub(crate) use color::{Color, InvalidColor};
#[allow(unused_imports)] // TODO disallow when all 5 are actually used
pub(crate) use geometry::{Point, Rect, RoundRect, Scale, Size};
