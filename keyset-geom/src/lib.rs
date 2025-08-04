//! This crate contains the geometry types used internally in [keyset].
//!
//! [keyset]: https://crates.io/crates/keyset

#![cfg_attr(coverage, expect(unstable_features))]
#![cfg_attr(coverage, feature(coverage_attribute))]

#[doc(hidden)]
pub use isclose::IsClose as __IsClose;
#[doc(hidden)]
pub use paste::paste as __paste; // Reexport since it's used by declare_units! // Reexport since it's used by declare_units!

pub use self::angle::Angle;
pub use self::ellipse::Ellipse;
pub use self::path::{Path, PathBuilder, PathSegment};
pub use self::point::Point;
pub use self::rect::{OffsetRect, Rect, RoundRect};
pub use self::transform::{Rotate, Scale, Transform, Translate};
pub use self::unit::{Conversion, ConvertFrom, ConvertInto, Dot, Inch, KeyUnit, Mm, Unit};
pub use self::vector::Vector;

mod angle;
mod ellipse;
mod path;
mod point;
mod rect;
mod transform;
mod unit;
mod vector;
