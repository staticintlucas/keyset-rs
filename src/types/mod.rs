mod color;

use euclid;

pub use color::Color;
pub(crate) use color::InvalidColor;

/// Default unit used by keyset-rs. 1000 MilliUnit = 1 Unit (the size of an alpha key)
#[derive(Debug)]
pub struct MilliUnit;

impl MilliUnit {
    #[inline]
    fn from(number: f32) -> i32 {
        (number * 1000. + 0.5) as i32
    }
}

pub type Point = euclid::Point2D<i32, MilliUnit>;
pub type Size = euclid::Size2D<i32, MilliUnit>;
pub type Rect = euclid::Box2D<i32, MilliUnit>;
pub type Length = euclid::Length<i32, MilliUnit>;

#[inline]
pub fn point(x: f32, y: f32) -> Point {
    euclid::point2(MilliUnit::from(x), MilliUnit::from(y))
}

#[inline]
pub fn size(w: f32, h: f32) -> Size {
    euclid::size2(MilliUnit::from(w), MilliUnit::from(h))
}

#[inline]
pub fn rect(x1: f32, y1: f32, x2: f32, y2: f32) -> Rect {
    Rect::new(point(x1, y1), point(x2, y2))
}

#[inline]
pub fn length(x: f32) -> Length {
    Length::new(MilliUnit::from(x))
}
