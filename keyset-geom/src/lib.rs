//! This crate contains the geometry types used internally in [keyset]. At the moment it mainly just
//! re-exports types from [euclid] with a few custom additions.
//!
//! [keyset]: https://crates.io/crates/keyset

#![cfg_attr(coverage, expect(unstable_features))]
#![cfg_attr(coverage, feature(coverage_attribute))]

mod angle;
mod circle;
mod length;
mod path;
mod point;
mod rect;
mod round_rect;
mod traits;
mod unit;
mod vector;

pub use self::angle::Angle;
pub use self::circle::Circle;
pub use self::length::Length;
pub use self::path::{Path, PathBuilder, PathSegment, ToPath};
pub use self::round_rect::RoundRect;
pub use self::traits::*;
pub use self::unit::{
    Dot, FromUnit, Inch, IntoUnit, KeyUnit, Mm, Unit, DOT_PER_INCH, DOT_PER_MM, DOT_PER_UNIT,
    INCH_PER_UNIT, MM_PER_UNIT,
};

/// Temporary re-exports for the new API
pub mod new_api {
    pub use crate::length::Length;
    pub use crate::point::Point;
    pub use crate::rect::{Rect, RoundRect};
    pub use crate::vector::Vector;
}

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
