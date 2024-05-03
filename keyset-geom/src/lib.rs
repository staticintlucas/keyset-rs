//! This crate contains the geometry types used internally in [keyset]. At the moment it mainly just
//! re-exports types from Kurbo with a few custom additions.
//!
//! [keyset]: https://crates.io/crates/keyset

mod circle;
mod path;
mod round_rect;
mod traits;
mod unit;

pub use circle::Circle;
pub use path::{Path, PathBuilder, PathSegment, ToPath};
pub use round_rect::RoundRect;
pub use traits::*;
pub use unit::{
    Dot, Inch, Mm, ToTransform, Unit, DOT_PER_INCH, DOT_PER_MM, DOT_PER_UNIT, INCH_PER_UNIT,
    MM_PER_UNIT,
};

/// An angle in radians
pub type Angle = euclid::Angle<f32>;

/// A one-dimensional distance with unit `U`
pub type Length<U> = euclid::Length<f32, U>;

/// A 2-dimensional point with unit `U`
pub type Point<U> = euclid::Point2D<f32, U>;

/// A 2-dimensional rectangle with unit `U`
pub type Rect<U> = euclid::Box2D<f32, U>;

/// A scale to convert between different units
pub type Scale<U, V> = euclid::Scale<f32, U, V>;

/// A set of 2-dimensional side offsets for top/right/bottom/left borders, padding, and margins
pub type SideOffsets<U> = euclid::SideOffsets2D<f32, U>;

/// A 2-dimensional size with unit `U`
pub type Size<U> = euclid::Size2D<f32, U>;

/// A 2-dimensional transformation with conversion from `U` to `V`
pub type Transform<U, V> = euclid::Transform2D<f32, U, V>;

/// A 2-dimensional vector with unit `U`
pub type Vector<U> = euclid::Vector2D<f32, U>;
