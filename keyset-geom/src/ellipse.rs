use std::ops;

use isclose::IsClose;

use crate::{
    ConvertFrom, ConvertInto as _, Path, Point, Rotate, Scale, Transform, Translate, Unit, Vector,
};

/// An ellipse
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Ellipse<U: Unit> {
    /// The center point of the ellipse
    pub center: Point<U>,
    /// The radii of the ellipse
    pub radii: Vector<U>,
}

impl<U> Ellipse<U>
where
    U: Unit,
{
    /// Create a new ellipse with the given center and radii
    #[inline]
    pub const fn new(center: Point<U>, radii: Vector<U>) -> Self {
        Self { center, radii }
    }
    /// Create a new circle with the given center and radius
    #[inline]
    pub fn from_circle(center: Point<U>, radius: f32) -> Self {
        Self {
            center,
            radii: Vector::splat(radius),
        }
    }

    /// Returns the width of the ellipse
    #[inline]
    pub fn width(&self) -> U {
        self.radii.x * 2.0
    }

    /// Returns the height of the ellipse
    #[inline]
    pub fn height(&self) -> U {
        self.radii.y * 2.0
    }
}

impl<U, V> ConvertFrom<Ellipse<V>> for Ellipse<U>
where
    U: Unit + ConvertFrom<V>,
    V: Unit,
{
    #[inline]
    fn convert_from(value: Ellipse<V>) -> Self {
        Self {
            center: value.center.convert_into(),
            radii: value.radii.convert_into(),
        }
    }
}

impl<U> ops::Mul<f32> for Ellipse<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            center: self.center * rhs,
            radii: self.radii * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for Ellipse<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.center *= rhs;
        self.radii *= rhs;
    }
}

impl<U> ops::Div<f32> for Ellipse<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            center: self.center / rhs,
            radii: self.radii / rhs,
        }
    }
}

impl<U> ops::DivAssign<f32> for Ellipse<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.center /= rhs;
        self.radii /= rhs;
    }
}

impl<U> IsClose<f32> for Ellipse<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <U as IsClose<f32>>::ABS_TOL;
    const REL_TOL: f32 = <U as IsClose<f32>>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.center.is_close_impl(&other.center, rel_tol, abs_tol)
            && self.radii.is_close_impl(&other.radii, rel_tol, abs_tol)
    }
}

impl<U> ops::Mul<Rotate> for Ellipse<U>
where
    U: Unit,
{
    type Output = Path<U>;

    #[inline]
    fn mul(self, _rhs: Rotate) -> Self::Output {
        // self.to_path() * rhs
        todo!()
    }
}

impl<U> ops::Mul<Scale> for Ellipse<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Scale) -> Self::Output {
        Self {
            center: self.center * rhs,
            radii: self.radii * rhs,
        }
    }
}

impl<U> ops::MulAssign<Scale> for Ellipse<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Scale) {
        self.center *= rhs;
        self.radii *= rhs;
    }
}

impl<U> ops::Mul<Translate<U>> for Ellipse<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Translate<U>) -> Self::Output {
        Self {
            center: self.center * rhs,
            radii: self.radii * rhs,
        }
    }
}

impl<U> ops::MulAssign<Translate<U>> for Ellipse<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Translate<U>) {
        self.center *= rhs;
        self.radii *= rhs;
    }
}

impl<U> ops::Mul<Transform<U>> for Ellipse<U>
where
    U: Unit,
{
    type Output = Path<U>;

    #[inline]
    fn mul(self, _rhs: Transform<U>) -> Self::Output {
        // self.to_path() * rhs
        todo!()
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{Inch, Mm};

    use super::*;

    #[test]
    fn ellipse_new() {
        let ellipse = Ellipse::<Mm>::new(Point::new(1.5, 3.0), Vector::new(1.0, 2.0));
        assert_is_close!(ellipse.center, Point::new(1.5, 3.0));
        assert_is_close!(ellipse.radii, Vector::new(1.0, 2.0));
    }

    #[test]
    fn ellipse_from_circle() {
        let circle = Ellipse::<Mm>::from_circle(Point::new(1.5, 3.0), 1.5);
        assert_is_close!(circle.center, Point::new(1.5, 3.0));
        assert_is_close!(circle.radii, Vector::new(1.5, 1.5));
    }

    #[test]
    fn ellipse_width() {
        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        assert_is_close!(ellipse.width(), Mm(2.0));
    }

    #[test]
    fn ellipse_height() {
        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        assert_is_close!(ellipse.height(), Mm(4.0));
    }

    #[test]
    fn ellipse_from_unit() {
        let ellipse = Ellipse::<Mm>::convert_from(Ellipse::<Inch> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        });
        assert_is_close!(ellipse.center, Point::new(38.1, 76.2));
        assert_is_close!(ellipse.radii, Vector::new(25.4, 50.8));
    }

    #[test]
    fn ellipse_mul_f32() {
        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        } * 1.5;
        assert_is_close!(ellipse.center, Point::new(2.25, 4.5));
        assert_is_close!(ellipse.radii, Vector::new(1.5, 3.0));

        // TODO: see comment by Unit
        // let ellipse = 1.5 * Ellipse::<Mm> {
        //     center: Point::new(1.5, 3.0),
        //     radii: Vector::new(1.0, 2.0),
        // };
        // assert_is_close!(ellipse.center, Point::new(2.25, 4.5));
        // assert_is_close!(ellipse.radii, Vector::new(1.5, 3.0));
    }

    #[test]
    fn ellipse_mul_assign_f32() {
        let mut ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        ellipse *= 1.5;
        assert_is_close!(ellipse.center, Point::new(2.25, 4.5));
        assert_is_close!(ellipse.radii, Vector::new(1.5, 3.0));
    }

    #[test]
    fn ellipse_div_f32() {
        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        } / 1.5;
        assert_is_close!(ellipse.center, Point::new(1.0, 2.0));
        assert_is_close!(ellipse.radii, Vector::new(2.0 / 3.0, 4.0 / 3.0));
    }

    #[test]
    fn ellipse_div_assign_f32() {
        let mut ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        ellipse /= 1.5;
        assert_is_close!(ellipse.center, Point::new(1.0, 2.0));
        assert_is_close!(ellipse.radii, Vector::new(2.0 / 3.0, 4.0 / 3.0));
    }

    #[test]
    fn ellipse_is_close() {
        assert!(Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        }
        .is_close(Ellipse {
            center: Point::new(1.0, 2.0) * 1.5,
            radii: Vector::new(2.0, 4.0) / 2.0,
        }));
        assert!(!Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        }
        .is_close(Ellipse {
            center: Point::new(1.1, 2.0) * 1.5,
            radii: Vector::new(2.0, 4.0) / 2.0,
        }));
        assert!(!Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        }
        .is_close(Ellipse {
            center: Point::new(1.0, 2.1) * 1.5,
            radii: Vector::new(2.0, 4.0) / 2.0,
        }));
        assert!(!Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        }
        .is_close(Ellipse {
            center: Point::new(1.0, 2.0) * 1.5,
            radii: Vector::new(2.1, 4.0) / 2.0,
        }));
        assert!(!Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        }
        .is_close(Ellipse {
            center: Point::new(1.0, 2.0) * 1.5,
            radii: Vector::new(2.0, 4.1) / 2.0,
        }));
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn ellipse_rotate() {
        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        let rotate = Rotate::degrees(135.0);
        let path = ellipse * rotate;
        assert!(matches!(path, Path::<Mm> { .. }));
    }

    #[test]
    fn ellipse_scale() {
        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        } * Scale::new(2.0, 0.5);

        assert_is_close!(ellipse.center, Point::new(3.0, 1.5));
        assert_is_close!(ellipse.radii, Vector::new(2.0, 1.0));

        let mut ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        ellipse *= Scale::new(2.0, 0.5);

        assert_is_close!(ellipse.center, Point::new(3.0, 1.5));
        assert_is_close!(ellipse.radii, Vector::new(2.0, 1.0));
    }

    #[test]
    fn ellipse_translate() {
        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        } * Translate::new(2.0, -1.0);

        assert_is_close!(ellipse.center, Point::new(3.5, 2.0));
        assert_is_close!(ellipse.radii, Vector::new(1.0, 2.0));

        let mut ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        ellipse *= Translate::new(2.0, -1.0);

        assert_is_close!(ellipse.center, Point::new(3.5, 2.0));
        assert_is_close!(ellipse.radii, Vector::new(1.0, 2.0));
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn ellipse_transform() {
        let transform = Transform::new(1.0, 0.5, -1.0, -0.5, 1.5, 2.0);
        let path = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        } * transform;
        assert!(matches!(path, Path::<Mm> { .. }));
    }
}
