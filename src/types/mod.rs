pub(crate) mod color;

pub type Color = color::Color;
pub(crate) use color::InvalidColor;

/// Default unit used by keyset-rs. 1u == size of an alpha key == 19.05mm == 0.75in
#[derive(Debug, Clone, Copy)]
pub struct Unit;

pub type Point = euclid::Point2D<f32, Unit>;
pub type Size = euclid::Size2D<f32, Unit>;
pub type Rect = euclid::Box2D<f32, Unit>;
pub type Length = euclid::Length<f32, Unit>;
