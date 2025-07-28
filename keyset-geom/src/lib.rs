//! This crate contains the geometry types used internally in [keyset]. At the moment it mainly just
//! re-exports types from [euclid] with a few custom additions.
//!
//! [keyset]: https://crates.io/crates/keyset

#![cfg_attr(coverage, expect(unstable_features))]
#![cfg_attr(coverage, feature(coverage_attribute))]

mod angle;
mod ellipse;
mod length;
mod path;
mod point;
mod rect;
mod transform;
mod unit;
mod vector;

pub use self::angle::Angle;
pub use self::ellipse::Ellipse;
pub use self::length::Length;
pub use self::path::{Path, PathBuilder, PathSegment, ToPath};
pub use self::point::Point;
pub use self::rect::{OffsetRect, Rect, RoundRect};
pub use self::transform::{Rotate, Scale, Transform, Translate};
pub use self::unit::{Conversion, ConvertFrom, ConvertInto, Dot, Inch, KeyUnit, Mm, Unit};
pub use self::vector::Vector;
