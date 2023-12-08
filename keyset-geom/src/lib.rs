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

mod round_rect;

pub use round_rect::RoundRect;

pub use kurbo::{Affine, Arc, BezPath, Circle, Insets, PathEl, Point, Rect, Shape, Size, Vec2};
