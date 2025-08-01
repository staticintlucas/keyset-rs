use std::ops;

use isclose::IsClose;

use crate::{
    ConvertFrom, ConvertInto as _, Path, PathSegment, Point, Rect, Rotate, Scale, Transform,
    Translate, Unit, Vector,
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

    /// Converts the ellipse to a [`Path`]
    #[inline]
    pub fn to_path(self) -> Path<U> {
        const A: f32 = (4.0 / 3.0) * (std::f32::consts::SQRT_2 - 1.0);

        let (cx, cy) = (self.center.x, self.center.y);
        let (rx, ry) = (self.radii.x, self.radii.y);

        Path {
            data: Box::new([
                PathSegment::Move(Point::from_units(cx - rx, cy)),
                PathSegment::CubicBezier(
                    Vector::from_units(U::zero(), -ry * A),
                    Vector::from_units(rx * (1.0 - A), -ry),
                    Vector::from_units(rx, -ry),
                ),
                PathSegment::CubicBezier(
                    Vector::from_units(rx * A, U::zero()),
                    Vector::from_units(rx, ry * (1.0 - A)),
                    Vector::from_units(rx, ry),
                ),
                PathSegment::CubicBezier(
                    Vector::from_units(U::zero(), ry * A),
                    Vector::from_units(-rx * (1.0 - A), ry),
                    Vector::from_units(-rx, ry),
                ),
                PathSegment::CubicBezier(
                    Vector::from_units(-rx * A, U::zero()),
                    Vector::from_units(-rx, -ry * (1.0 - A)),
                    Vector::from_units(-rx, -ry),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::from_center_and_size(self.center, self.radii * 2.0),
        }
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

impl<U> IsClose for Ellipse<U>
where
    U: Unit,
{
    type Tolerance = f32;
    const ZERO_TOL: Self::Tolerance = 0.0;
    const ABS_TOL: Self::Tolerance = <U as IsClose>::ABS_TOL;
    const REL_TOL: Self::Tolerance = <U as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.center.is_close_tol(&other.center, rel_tol, abs_tol)
            && self.radii.is_close_tol(&other.radii, rel_tol, abs_tol)
    }
}

impl<U> ops::Mul<Rotate> for Ellipse<U>
where
    U: Unit,
{
    type Output = Path<U>;

    #[inline]
    fn mul(self, rhs: Rotate) -> Self::Output {
        self.to_path() * rhs
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

impl<U> ops::Div<Scale> for Ellipse<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Scale) -> Self::Output {
        Self {
            center: self.center / rhs,
            radii: self.radii / rhs,
        }
    }
}

impl<U> ops::DivAssign<Scale> for Ellipse<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: Scale) {
        self.center /= rhs;
        self.radii /= rhs;
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
    fn mul(self, rhs: Transform<U>) -> Self::Output {
        self.to_path() * rhs
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{Angle, Inch, Mm, PathBuilder};

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
    fn ellipse_to_path() {
        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        let mut builder = PathBuilder::new();
        builder.abs_move(Point::new(0.5, 3.0));
        builder.rel_arc(
            Vector::new(1.0, 2.0),
            Angle::new(0.0),
            false,
            true,
            Vector::new(1.0, -2.0),
        );
        builder.rel_arc(
            Vector::new(1.0, 2.0),
            Angle::new(0.0),
            false,
            true,
            Vector::new(1.0, 2.0),
        );
        builder.rel_arc(
            Vector::new(1.0, 2.0),
            Angle::new(0.0),
            false,
            true,
            Vector::new(-1.0, 2.0),
        );
        builder.rel_arc(
            Vector::new(1.0, 2.0),
            Angle::new(0.0),
            false,
            true,
            Vector::new(-1.0, -2.0),
        );
        builder.close();
        let expected = builder.build();

        assert_eq!(ellipse.to_path().len(), expected.len());
        assert_is_close!(ellipse.to_path().bounds, expected.bounds);
        for (&res, &exp) in ellipse.to_path().iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
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
        .is_close(&Ellipse {
            center: Point::new(1.0, 2.0) * 1.5,
            radii: Vector::new(2.0, 4.0) / 2.0,
        }));
        assert!(!Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        }
        .is_close(&Ellipse {
            center: Point::new(1.1, 2.0) * 1.5,
            radii: Vector::new(2.0, 4.0) / 2.0,
        }));
        assert!(!Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        }
        .is_close(&Ellipse {
            center: Point::new(1.0, 2.1) * 1.5,
            radii: Vector::new(2.0, 4.0) / 2.0,
        }));
        assert!(!Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        }
        .is_close(&Ellipse {
            center: Point::new(1.0, 2.0) * 1.5,
            radii: Vector::new(2.1, 4.0) / 2.0,
        }));
        assert!(!Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        }
        .is_close(&Ellipse {
            center: Point::new(1.0, 2.0) * 1.5,
            radii: Vector::new(2.0, 4.1) / 2.0,
        }));
    }

    #[test]
    fn ellipse_rotate() {
        use std::f32::consts::SQRT_2;

        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        let rotate = Rotate::degrees(135.0);
        let path = ellipse * rotate;

        let mut exp_bldr = PathBuilder::<Mm>::new();
        exp_bldr.abs_move(Point::new(-1.75 * SQRT_2, -1.25 * SQRT_2));
        exp_bldr.rel_arc(
            Vector::new(1.0, 2.0),
            Angle::degrees(135.0),
            false,
            true,
            Vector::new(0.5 * SQRT_2, 1.5 * SQRT_2),
        );
        exp_bldr.rel_arc(
            Vector::new(1.0, 2.0),
            Angle::degrees(135.0),
            false,
            true,
            Vector::new(-1.5 * SQRT_2, -0.5 * SQRT_2),
        );
        exp_bldr.rel_arc(
            Vector::new(1.0, 2.0),
            Angle::degrees(135.0),
            false,
            true,
            Vector::new(-0.5 * SQRT_2, -1.5 * SQRT_2),
        );
        exp_bldr.rel_arc(
            Vector::new(1.0, 2.0),
            Angle::degrees(135.0),
            false,
            true,
            Vector::new(1.5 * SQRT_2, 0.5 * SQRT_2),
        );
        exp_bldr.close();
        let expected = exp_bldr.build();

        assert_eq!(path.len(), expected.len());
        assert_is_close!(path.bounds, expected.bounds);
        for (&res, &exp) in path.iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
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

        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        } / Scale::new(2.0, 0.5);

        assert_is_close!(ellipse.center, Point::new(0.75, 6.0));
        assert_is_close!(ellipse.radii, Vector::new(0.5, 4.0));

        let mut ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        ellipse /= Scale::new(2.0, 0.5);

        assert_is_close!(ellipse.center, Point::new(0.75, 6.0));
        assert_is_close!(ellipse.radii, Vector::new(0.5, 4.0));
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
    fn ellipse_transform() {
        const A: f32 = (4.0 / 3.0) * (std::f32::consts::SQRT_2 - 1.0);

        let ellipse = Ellipse::<Mm> {
            center: Point::new(1.5, 3.0),
            radii: Vector::new(1.0, 2.0),
        };
        let transform = Transform::new(1.0, 0.5, -1.0, -0.5, 1.5, 2.0);
        let path = ellipse * transform;

        let mut exp_bldr = PathBuilder::<Mm>::new();
        exp_bldr.abs_move(Point::new(1.0, 6.25));
        exp_bldr.rel_cubic_bezier(
            Vector::new(-A, -3.0 * A),
            Vector::new(-A, -3.5 + 0.5 * A),
            Vector::new(0.0, -3.5),
        );
        exp_bldr.rel_cubic_bezier(
            Vector::new(A, -0.5 * A),
            Vector::new(2.0 - A, 2.5 - 3.0 * A),
            Vector::new(2.0, 2.5),
        );
        exp_bldr.rel_cubic_bezier(
            Vector::new(A, 3.0 * A),
            Vector::new(A, 3.5 - 0.5 * A),
            Vector::new(0.0, 3.5),
        );
        exp_bldr.rel_cubic_bezier(
            Vector::new(-A, 0.5 * A),
            Vector::new(-2.0 + A, -2.5 + 3.0 * A),
            Vector::new(-2.0, -2.5),
        );
        exp_bldr.close();
        let expected = exp_bldr.build();

        assert_eq!(path.len(), expected.len());
        assert_is_close!(path.bounds, expected.bounds);
        for (&res, &exp) in path.iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
    }
}
