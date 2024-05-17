use crate::{Angle, Point, Rect, Scale, Size, Transform, Vector};

/// Trait to add additional constructor to `Rect`
pub trait ExtRect<U> {
    /// Create a new `Rect` given a center point and size
    fn from_center_and_size(center: Point<U>, size: Size<U>) -> Self;
}

impl<U> ExtRect<U> for Rect<U> {
    #[inline]
    fn from_center_and_size(center: Point<U>, size: Size<U>) -> Self {
        let half_size = size * 0.5;
        Self::new(center - half_size, center + half_size)
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
    #[inline]
    fn rotate(self, angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    #[inline]
    fn neg_x(self) -> Self {
        let (x, y) = self.to_tuple();
        Self::new(-x, y)
    }

    #[inline]
    fn neg_y(self) -> Self {
        let (x, y) = self.to_tuple();
        Self::new(x, -y)
    }
}

/// Trait to allow conversion from a [`Scale<U, V>`] to a [`Transform<U, V>`]
pub trait ToTransform<U, V> {
    /// Convert a [`Scale<U, V>`] to a [`Transform<U, V>`]
    fn to_transform(self) -> Transform<U, V>;
}

impl<U, V> ToTransform<U, V> for Scale<U, V> {
    #[inline]
    fn to_transform(self) -> Transform<U, V> {
        Transform::scale(self.get(), self.get())
    }
}

#[cfg(test)]
mod tests {
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn rect_from_center_and_size() {
        let rect = Rect::<()>::from_center_and_size(Point::new(1.0, 2.0), Size::new(3.0, 4.0));
        let exp = Rect::new(Point::new(-0.5, 0.0), Point::new(2.5, 4.0));

        assert_is_close!(rect, exp);
    }

    #[test]
    fn vector_rotate() {
        let vector = Vector::<()>::new(1.0, 0.0);
        let exp = Vector::splat(std::f32::consts::FRAC_1_SQRT_2);

        assert_is_close!(vector.rotate(Angle::degrees(45.0)), exp);
    }

    #[test]
    fn vector_neg_x() {
        let vector = Vector::<()>::new(1.0, 1.0);
        let exp = Vector::new(-1.0, 1.0);

        assert_is_close!(vector.neg_x(), exp);
    }

    #[test]
    fn vector_neg_y() {
        let vector = Vector::<()>::new(1.0, 1.0);
        let exp = Vector::new(1.0, -1.0);

        assert_is_close!(vector.neg_y(), exp);
    }

    #[test]
    fn scale_to_transform() {
        let scale = Scale::<(), ()>::new(2.0);
        let exp = Transform::new(2.0, 0.0, 0.0, 2.0, 0.0, 0.0);

        assert_is_close!(scale.to_transform(), exp);
    }
}
