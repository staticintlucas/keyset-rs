//! This crate contains the geometry types used internally in [keyset]. At the moment it mainly just
//! re-exports types from Kurbo with a few custom additions.
//!
//! [keyset]: https://crates.io/crates/keyset

#![warn(
    missing_docs,
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
#![allow(
    clippy::suboptimal_flops // Optimiser is pretty good, and mul_add is pretty ugly
)]

mod circle;
mod path;
mod round_rect;
mod unit;

pub use unit::{
    Dot, Inch, Mm, ToTransform, Unit, DOT_PER_INCH, DOT_PER_MM, DOT_PER_UNIT, INCH_PER_UNIT,
    MM_PER_UNIT,
};

pub use circle::Circle;
pub use path::{Path, PathBuilder, PathSegment, ToPath};
pub use round_rect::RoundRect;

pub use euclid::approxeq::ApproxEq;

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

/// Trait to add additional constructor to `Rect`
pub trait ExtRect<U> {
    /// Create a new `Rect` given a center point and size
    fn from_center_and_size(center: Point<U>, size: Size<U>) -> Self;
}

impl<U> ExtRect<U> for Rect<U> {
    fn from_center_and_size(center: Point<U>, size: Size<U>) -> Self {
        Self::from_origin_and_size(center - size / 2.0, size)
    }
}

/// Trait to rotate a `Vector`
pub trait ExtVec<T, U> {
    /// Rotate the vector by the given angle
    #[must_use]
    fn rotate(self, angle: T) -> Self;

    /// Negate the x component of the vector
    #[must_use]
    fn neg_x(self) -> Self;

    /// Negate the y component of the vector
    #[must_use]
    fn neg_y(self) -> Self;
}

impl<U> ExtVec<Angle, U> for Vector<U> {
    fn rotate(self, angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    fn neg_x(self) -> Self {
        let (x, y) = self.to_tuple();
        Self::new(-x, y)
    }

    fn neg_y(self) -> Self {
        let (x, y) = self.to_tuple();
        Self::new(x, -y)
    }
}

impl<U> ExtVec<euclid::Angle<f64>, U> for euclid::Vector2D<f64, U> {
    fn rotate(self, angle: euclid::Angle<f64>) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    fn neg_x(self) -> Self {
        let (x, y) = self.to_tuple();
        Self::new(-x, y)
    }

    fn neg_y(self) -> Self {
        let (x, y) = self.to_tuple();
        Self::new(x, -y)
    }
}
