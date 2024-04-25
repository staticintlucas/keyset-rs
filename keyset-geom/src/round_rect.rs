use crate::{ExtRect, Length, Point, Rect, Size};

/// A rectangle with rounded corners. Unlike [`kurbo::RoundedRect`] this has one set of radii
/// shared between all corners, but it does support elliptical corners.
#[derive(Debug, PartialEq)]
pub struct RoundRect<U> {
    /// Minimum point
    pub min: Point<U>,
    /// Maximum point
    pub max: Point<U>,
    /// Radius size
    pub radius: Length<U>,
}

// Impl here rather than derive so we don't require U: Clone everywhere
impl<U> Clone for RoundRect<U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<U> Copy for RoundRect<U> {}

impl<U> RoundRect<U> {
    /// Create a new rounded rectangle from minimum and maximum coordinates.
    #[inline]
    #[must_use]
    pub const fn new(min: Point<U>, max: Point<U>, radius: Length<U>) -> Self {
        Self { min, max, radius }
    }

    /// Create a new rounded rectangle from a [`kurbo::Rect`] and its radii.
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

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_round_rect_new() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_approx_eq!(rect.min.x, 1.0);
        assert_approx_eq!(rect.min.y, 2.0);
        assert_approx_eq!(rect.max.x, 3.0);
        assert_approx_eq!(rect.max.y, 5.0);
        assert_approx_eq!(rect.radius.0, 0.5);
    }

    #[test]
    fn test_round_rect_from_rect() {
        let rect = RoundRect::<()>::from_rect(
            Rect::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0)),
            Length::new(0.5),
        );

        assert_approx_eq!(rect.min.x, 1.0);
        assert_approx_eq!(rect.min.y, 2.0);
        assert_approx_eq!(rect.max.x, 3.0);
        assert_approx_eq!(rect.max.y, 5.0);
        assert_approx_eq!(rect.radius.0, 0.5);
    }

    #[test]
    fn test_round_rect_from_origin_and_size() {
        let rect = RoundRect::<()>::from_origin_and_size(
            Point::new(1.0, 2.0),
            Size::new(2.0, 3.0),
            Length::new(0.5),
        );

        assert_approx_eq!(rect.min.x, 1.0);
        assert_approx_eq!(rect.min.y, 2.0);
        assert_approx_eq!(rect.max.x, 3.0);
        assert_approx_eq!(rect.max.y, 5.0);
        assert_approx_eq!(rect.radius.0, 0.5);
    }

    #[test]
    fn test_round_rect_from_center_and_size() {
        let rect = RoundRect::<()>::from_center_and_size(
            Point::new(2.0, 3.5),
            Size::new(2.0, 3.0),
            Length::new(0.5),
        );

        assert_approx_eq!(rect.min.x, 1.0);
        assert_approx_eq!(rect.min.y, 2.0);
        assert_approx_eq!(rect.max.x, 3.0);
        assert_approx_eq!(rect.max.y, 5.0);
        assert_approx_eq!(rect.radius.0, 0.5);
    }

    #[test]
    fn test_round_rect_width() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_approx_eq!(rect.width(), 2.0);
    }

    #[test]
    fn test_round_rect_height() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_approx_eq!(rect.height(), 3.0);
    }

    #[test]
    fn test_round_rect_radius() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_eq!(rect.radius(), Length::new(0.5));
    }

    #[test]
    fn test_round_rect_rect() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_eq!(
            rect.rect(),
            Rect::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0))
        );
    }

    #[test]
    fn test_round_rect_center() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_eq!(rect.center(), Point::new(2.0, 3.5));
    }

    #[test]
    fn test_round_rect_size() {
        let rect =
            RoundRect::<()>::new(Point::new(1.0, 2.0), Point::new(3.0, 5.0), Length::new(0.5));

        assert_eq!(rect.size(), Size::new(2.0, 3.0));
    }
}
