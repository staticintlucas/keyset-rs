use std::borrow::Borrow;
use std::fmt;

use isclose::IsClose;

use crate::{ExtRect as _, Length, Point, Rect, Size};

/// A rectangle with rounded corners
pub struct RoundRect<U> {
    /// Minimum point
    pub min: Point<U>,
    /// Maximum point
    pub max: Point<U>,
    /// Radius size
    pub radius: Length<U>,
}

// Impl here rather than derive so we don't require U: Clone
impl<U> Clone for RoundRect<U> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

// Impl here rather than derive so we don't require U: Copy
impl<U> Copy for RoundRect<U> {}

// Impl here rather than derive so we don't require U: PartialEq
impl<U> PartialEq for RoundRect<U> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.min.eq(&other.min) && self.max.eq(&other.max) && self.radius.eq(&other.radius)
    }
}

// Impl here rather than derive so we don't require U: Debug
impl<U> fmt::Debug for RoundRect<U> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoundRect")
            .field("min", &self.min)
            .field("max", &self.max)
            .field("radius", &self.radius)
            .finish()
    }
}

impl<U> RoundRect<U> {
    /// Create a new rounded rectangle from minimum and maximum coordinates.
    #[inline]
    #[must_use]
    pub const fn new(min: Point<U>, max: Point<U>, radius: Length<U>) -> Self {
        Self { min, max, radius }
    }

    /// Create a new rounded rectangle from a [`crate::Rect`] and its radii.
    #[inline]
    #[must_use]
    pub const fn from_rect(rect: Rect<U>, radius: Length<U>) -> Self {
        let Rect { min, max } = rect;
        Self { min, max, radius }
    }

    /// Create a new rounded rectangle from its origin point, size, and radii.
    #[inline]
    #[must_use]
    pub fn from_origin_and_size(origin: Point<U>, size: Size<U>, radius: Length<U>) -> Self {
        Self::from_rect(Rect::from_origin_and_size(origin, size), radius)
    }

    /// Create a new rounded rectangle from its center point, size, and radii.
    #[inline]
    #[must_use]
    pub fn from_center_and_size(origin: Point<U>, size: Size<U>, radius: Length<U>) -> Self {
        Self::from_rect(Rect::from_center_and_size(origin, size), radius)
    }

    /// Returns the width of the rounded rectangle
    #[inline]
    #[must_use]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Returns the height of the rounded rectangle
    #[inline]
    #[must_use]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Returns the radii of the rounded rectangle
    #[inline]
    #[must_use]
    pub const fn radius(&self) -> Length<U> {
        self.radius
    }

    /// Returns a rectangle with the same position and size as the rounded rectangle
    #[inline]
    #[must_use]
    pub const fn rect(&self) -> Rect<U> {
        let Self { min, max, .. } = *self;

        Rect { min, max }
    }

    /// Returns the center point of the rounded rectangle
    #[inline]
    #[must_use]
    pub fn center(&self) -> Point<U> {
        (self.min + self.max.to_vector()) / 2.0
    }

    /// Returns the size of the rounded rectangle
    #[inline]
    #[must_use]
    pub fn size(&self) -> Size<U> {
        self.rect().size()
    }
}

impl<U> IsClose<f32> for RoundRect<U> {
    const ABS_TOL: f32 = f32::ABS_TOL;
    const REL_TOL: f32 = f32::REL_TOL;

    #[inline]
    fn is_close_tol(
        &self,
        other: impl Borrow<Self>,
        rel_tol: impl Borrow<f32>,
        abs_tol: impl Borrow<f32>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &f32, &f32) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.min.is_close_tol(other.min, rel_tol, abs_tol)
            && self.max.is_close_tol(other.max, rel_tol, abs_tol)
            && self.radius.is_close_tol(other.radius, rel_tol, abs_tol)
    }
}

#[cfg(test)]
mod tests {
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn round_rect_clone() {
        struct NonCloneable;
        let rect = RoundRect::<NonCloneable> {
            min: Point::origin(),
            max: Point::new(1.0, 2.0),
            radius: Length::new(0.5),
        };

        #[allow(clippy::clone_on_copy)] // We want to test clone, not copy
        let rect2 = rect.clone();

        assert_is_close!(rect, rect2);
    }

    #[test]
    fn round_rect_partial_eq() {
        struct NonPartialEq;
        let rect = RoundRect::<NonPartialEq> {
            min: Point::origin(),
            max: Point::new(1.0, 2.0),
            radius: Length::new(0.5),
        };
        let rect2 = rect;

        assert_eq!(rect, rect2);
    }

    #[test]
    fn round_rect_debug() {
        struct NonDebug;
        let rect = RoundRect::<NonDebug>::new(
            Point::new(1.0, 2.0),
            Point::new(3.0, 5.0),
            Length::new(0.5),
        );
        let dbg = format!("{rect:?}");

        assert_eq!(
            dbg,
            "RoundRect { min: (1.0, 2.0), max: (3.0, 5.0), radius: 0.5 }"
        );
    }

    #[test]
    fn round_rect_new() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_is_close!(rect.min, Point::new(1.0, 2.0));
        assert_is_close!(rect.max, Point::new(3.0, 5.0));
        assert_is_close!(rect.radius, Length::new(0.5));
    }

    #[test]
    fn round_rect_from_rect() {
        let rect = RoundRect::<()>::from_rect(
            Rect::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0)),
            Length::new(0.5),
        );

        assert_is_close!(rect.min, Point::new(1.0, 2.0));
        assert_is_close!(rect.max, Point::new(3.0, 5.0));
        assert_is_close!(rect.radius, Length::new(0.5));
    }

    #[test]
    fn round_rect_from_origin_and_size() {
        let rect = RoundRect::<()>::from_origin_and_size(
            Point::new(1.0, 2.0),
            Size::new(2.0, 3.0),
            Length::new(0.5),
        );

        assert_is_close!(rect.min, Point::new(1.0, 2.0));
        assert_is_close!(rect.max, Point::new(3.0, 5.0));
        assert_is_close!(rect.radius, Length::new(0.5));
    }

    #[test]
    fn round_rect_from_center_and_size() {
        let rect = RoundRect::<()>::from_center_and_size(
            Point::new(2.0, 3.5),
            Size::new(2.0, 3.0),
            Length::new(0.5),
        );

        assert_is_close!(rect.min, Point::new(1.0, 2.0));
        assert_is_close!(rect.max, Point::new(3.0, 5.0));
        assert_is_close!(rect.radius, Length::new(0.5));
    }

    #[test]
    fn round_rect_width() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_is_close!(rect.width(), 2.0);
    }

    #[test]
    fn round_rect_height() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_is_close!(rect.height(), 3.0);
    }

    #[test]
    fn round_rect_radius() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_eq!(rect.radius(), Length::new(0.5));
    }

    #[test]
    fn round_rect_rect() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_eq!(
            rect.rect(),
            Rect::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0))
        );
    }

    #[test]
    fn round_rect_center() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_eq!(rect.center(), Point::new(2.0, 3.5));
    }

    #[test]
    fn round_rect_size() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_eq!(rect.size(), Size::new(2.0, 3.0));
    }
}
