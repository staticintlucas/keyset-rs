//! This crate contains the key and legend types used for describing layouts used internally by
//! [keyset]. It also contains utility functions for loading KLE layouts
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

mod legend;

#[cfg(feature = "kle")]
pub mod kle;

pub use legend::{Legend, Legends};

use geom::{Point, Rect, Size};

use color::Color;

/// The type of homing used on a homing key
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Homing {
    /// A scooped homing key, also known as a dished homing key
    Scoop,
    /// A key with a homing bar, sometimes called a line
    Bar,
    /// A key with a homing bump, also known as a nub, dot, or nipple
    Bump,
}

/// The shape of a key
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    /// Not a *key* per se, but only a legend. This is usually used for labels and is the same as a
    /// decal in KLE
    None(Size),
    /// A regular key of the given size
    Normal(Size),
    /// A spacebar of the given size
    Space(Size),
    /// A homing key with the given homing type. If the homing type is [`None`] the profile's
    /// default homing type is assumed to be used
    Homing(Option<Homing>),
    /// A stepped caps lock key, i.e. a 1.25u key with additional 0.5u step on the right
    SteppedCaps,
    /// A vertically-aligned ISO enter, i.e. an ISO enter where legends are aligned within the
    /// vertical 1.25u &times; 2.0u section of the key
    IsoVertical,
    /// A horizontally-aligned ISO enter, i.e. an ISO enter where legends are aligned within the
    /// horizontal 1.5u top section of the key
    IsoHorizontal,
}

impl Shape {
    /// The outer bounding rectangle of the key shape, i.e. the bounding box of the key shape. The
    /// inner and outer bounds are the same for regular-shaped keys, but are different for stepped
    /// keys, L-shaped keys, etc.
    #[must_use]
    pub fn outer_rect(self) -> Rect {
        match self {
            Self::None(size) | Self::Normal(size) | Self::Space(size) => {
                Rect::from_origin_size(Point::ORIGIN, size)
            }
            Self::Homing(..) => Rect::from_origin_size(Point::ORIGIN, (1.0, 1.0)),
            Self::SteppedCaps => Rect::from_origin_size(Point::ORIGIN, (1.75, 1.0)),
            Self::IsoVertical | Self::IsoHorizontal => {
                Rect::from_origin_size(Point::ORIGIN, (1.5, 2.0))
            }
        }
    }

    /// The inner bounding rectangle of the key shape, i.e. the bounds for the part of the key
    /// containing the legend. The inner and outer bounds are the same for regular-shaped keys, but
    /// are different for stepped keys, L-shaped keys, etc.
    #[must_use]
    pub fn inner_rect(self) -> Rect {
        match self {
            Self::None(size) | Self::Normal(size) | Self::Space(size) => {
                Rect::from_origin_size(Point::ORIGIN, size)
            }
            Self::Homing(..) => Rect::from_origin_size(Point::ORIGIN, (1.0, 1.0)),
            Self::SteppedCaps => Rect::from_origin_size(Point::ORIGIN, (1.25, 1.0)),
            Self::IsoVertical => Rect::from_origin_size((0.25, 0.0), (1.25, 2.0)),
            Self::IsoHorizontal => Rect::from_origin_size(Point::ORIGIN, (1.5, 1.0)),
        }
    }
}

/// A key
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Key {
    /// The position of the key
    pub position: Point,
    /// The key's shape
    pub shape: Shape,
    /// The key's colour
    pub color: Color,
    /// The key's legends
    pub legends: Legends,
}

impl Key {
    /// A new blank key
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// An example non-blank key
    #[must_use]
    pub fn example() -> Self {
        Self {
            legends: Legends::example(),
            ..Self::default()
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            position: Point::ORIGIN,
            shape: Shape::Normal(Size::new(1., 1.)),
            color: Color::new(0.8, 0.8, 0.8),
            legends: Legends::default(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn shape_outer_size() {
        assert_eq!(
            Shape::Normal(Size::new(2.25, 1.)).outer_rect(),
            Rect::new(0.0, 0.0, 2.25, 1.)
        );
        assert_eq!(
            Shape::IsoVertical.outer_rect(),
            Rect::new(0.0, 0.0, 1.5, 2.0)
        );
        assert_eq!(
            Shape::IsoHorizontal.outer_rect(),
            Rect::new(0.0, 0.0, 1.5, 2.0)
        );
        assert_eq!(
            Shape::SteppedCaps.outer_rect(),
            Rect::new(0.0, 0.0, 1.75, 1.0)
        );
    }

    #[test]
    fn shape_inner_size() {
        assert_eq!(
            Shape::Normal(Size::new(2.25, 1.)).inner_rect(),
            Rect::new(0.0, 0.0, 2.25, 1.)
        );
        assert_eq!(
            Shape::IsoVertical.inner_rect(),
            Rect::new(0.25, 0.0, 1.5, 2.0)
        );
        assert_eq!(
            Shape::IsoHorizontal.inner_rect(),
            Rect::new(0.0, 0.0, 1.5, 1.0)
        );
        assert_eq!(
            Shape::SteppedCaps.inner_rect(),
            Rect::new(0.0, 0.0, 1.25, 1.0)
        );
    }

    #[test]
    fn key_new() {
        let key = Key::new();

        assert_eq!(key.position, Point::new(0., 0.));
        assert_matches!(key.shape, Shape::Normal(size) if size == Size::new(1., 1.));
        assert_eq!(key.color, Color::new(0.8, 0.8, 0.8));
        for legend in key.legends {
            assert!(legend.is_none());
        }
    }

    #[test]
    fn key_example() {
        let key = Key::example();
        let legend_is_some = [true, false, true, false, false, false, true, false, true];

        assert_eq!(key.position, Point::new(0., 0.));
        assert_matches!(key.shape, Shape::Normal(size) if size == Size::new(1., 1.));
        assert_eq!(key.color, Color::new(0.8, 0.8, 0.8));
        for (legend, is_some) in key.legends.into_iter().zip(legend_is_some) {
            assert_eq!(legend.is_some(), is_some);
        }
    }
}
