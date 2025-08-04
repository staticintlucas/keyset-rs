use std::ops;

use isclose::IsClose;

use crate::{Conversion, Point, Rotate, Scale, Transform, Translate, Unit, Vector};

/// Enum representing a path segment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathSegment<U: Unit> {
    /// Move to a point
    Move(Point<U>),
    /// Line by a distance
    Line(Vector<U>),
    /// Cubic Bézier curve, with relative control and end points
    CubicBezier(Vector<U>, Vector<U>, Vector<U>),
    /// Quadratic Bézier curve, with relative control and end points
    QuadraticBezier(Vector<U>, Vector<U>),
    /// Close the path
    Close,
}

impl<U> IsClose for PathSegment<U>
where
    U: Unit,
{
    type Tolerance = f32;
    const ZERO_TOL: Self::Tolerance = 0.0;
    const ABS_TOL: Self::Tolerance = <Self::Tolerance as IsClose>::ABS_TOL;
    const REL_TOL: Self::Tolerance = <Self::Tolerance as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        use PathSegment::*;
        match (*self, *other) {
            (Move(ref s), Move(ref o)) => s.is_close_tol(o, rel_tol, abs_tol),
            (Line(ref s), Line(ref o)) => s.is_close_tol(o, rel_tol, abs_tol),
            (CubicBezier(ref s1, ref s2, ref s), CubicBezier(ref o1, ref o2, ref o)) => {
                s1.is_close_tol(o1, rel_tol, abs_tol)
                    && s2.is_close_tol(o2, rel_tol, abs_tol)
                    && s.is_close_tol(o, rel_tol, abs_tol)
            }
            (QuadraticBezier(ref s1, ref s), QuadraticBezier(ref o1, ref o)) => {
                s1.is_close_tol(o1, rel_tol, abs_tol) && s.is_close_tol(o, rel_tol, abs_tol)
            }
            (Close, Close) => true,
            _ => false,
        }
    }
}

impl<U> ops::Mul<f32> for PathSegment<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, scale: f32) -> Self::Output {
        use PathSegment::*;
        match self {
            Move(point) => Move(point * scale),
            Line(dist) => Line(dist * scale),
            CubicBezier(ctrl1, ctrl2, dist) => {
                CubicBezier(ctrl1 * scale, ctrl2 * scale, dist * scale)
            }
            QuadraticBezier(ctrl1, dist) => QuadraticBezier(ctrl1 * scale, dist * scale),
            Close => Close,
        }
    }
}

impl<U> ops::MulAssign<f32> for PathSegment<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, scale: f32) {
        use PathSegment::*;
        match *self {
            Move(ref mut point) => *point *= scale,
            Line(ref mut dist) => *dist *= scale,
            CubicBezier(ref mut ctrl1, ref mut ctrl2, ref mut dist) => {
                *ctrl1 *= scale;
                *ctrl2 *= scale;
                *dist *= scale;
            }
            QuadraticBezier(ref mut ctrl1, ref mut dist) => {
                *ctrl1 *= scale;
                *dist *= scale;
            }
            Close => (),
        }
    }
}

impl<U> ops::Div<f32> for PathSegment<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, scale: f32) -> Self::Output {
        use PathSegment::*;
        match self {
            Move(point) => Move(point / scale),
            Line(dist) => Line(dist / scale),
            CubicBezier(ctrl1, ctrl2, dist) => {
                CubicBezier(ctrl1 / scale, ctrl2 / scale, dist / scale)
            }
            QuadraticBezier(ctrl1, dist) => QuadraticBezier(ctrl1 / scale, dist / scale),
            Close => Close,
        }
    }
}

impl<U> ops::DivAssign<f32> for PathSegment<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, scale: f32) {
        use PathSegment::*;
        match *self {
            Move(ref mut point) => *point /= scale,
            Line(ref mut dist) => *dist /= scale,
            CubicBezier(ref mut ctrl1, ref mut ctrl2, ref mut dist) => {
                *ctrl1 /= scale;
                *ctrl2 /= scale;
                *dist /= scale;
            }
            QuadraticBezier(ref mut ctrl1, ref mut dist) => {
                *ctrl1 /= scale;
                *dist /= scale;
            }
            Close => (),
        }
    }
}

impl<U> ops::Mul<Rotate> for PathSegment<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Rotate) -> Self::Output {
        match self {
            Self::Move(point) => Self::Move(point * rhs),
            Self::Line(dist) => Self::Line(dist * rhs),
            Self::CubicBezier(ctrl1, ctrl2, dist) => {
                Self::CubicBezier(ctrl1 * rhs, ctrl2 * rhs, dist * rhs)
            }
            Self::QuadraticBezier(ctrl, dist) => Self::QuadraticBezier(ctrl * rhs, dist * rhs),
            Self::Close => Self::Close,
        }
    }
}

impl<U> ops::MulAssign<Rotate> for PathSegment<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Rotate) {
        match *self {
            Self::Move(ref mut point) => *point *= rhs,
            Self::Line(ref mut dist) => *dist *= rhs,
            Self::CubicBezier(ref mut ctrl1, ref mut ctrl2, ref mut dist) => {
                *ctrl1 *= rhs;
                *ctrl2 *= rhs;
                *dist *= rhs;
            }
            Self::QuadraticBezier(ref mut ctrl, ref mut dist) => {
                *ctrl *= rhs;
                *dist *= rhs;
            }
            Self::Close => {}
        }
    }
}

impl<U> ops::Mul<Scale> for PathSegment<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Scale) -> Self::Output {
        match self {
            Self::Move(point) => Self::Move(point * rhs),
            Self::Line(dist) => Self::Line(dist * rhs),
            Self::CubicBezier(ctrl1, ctrl2, dist) => {
                Self::CubicBezier(ctrl1 * rhs, ctrl2 * rhs, dist * rhs)
            }
            Self::QuadraticBezier(ctrl, dist) => Self::QuadraticBezier(ctrl * rhs, dist * rhs),
            Self::Close => Self::Close,
        }
    }
}

impl<U> ops::MulAssign<Scale> for PathSegment<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Scale) {
        match *self {
            Self::Move(ref mut point) => *point *= rhs,
            Self::Line(ref mut dist) => *dist *= rhs,
            Self::CubicBezier(ref mut ctrl1, ref mut ctrl2, ref mut dist) => {
                *ctrl1 *= rhs;
                *ctrl2 *= rhs;
                *dist *= rhs;
            }
            Self::QuadraticBezier(ref mut ctrl, ref mut dist) => {
                *ctrl *= rhs;
                *dist *= rhs;
            }
            Self::Close => {}
        }
    }
}

impl<U> ops::Div<Scale> for PathSegment<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Scale) -> Self::Output {
        match self {
            Self::Move(point) => Self::Move(point / rhs),
            Self::Line(dist) => Self::Line(dist / rhs),
            Self::CubicBezier(ctrl1, ctrl2, dist) => {
                Self::CubicBezier(ctrl1 / rhs, ctrl2 / rhs, dist / rhs)
            }
            Self::QuadraticBezier(ctrl, dist) => Self::QuadraticBezier(ctrl / rhs, dist / rhs),
            Self::Close => Self::Close,
        }
    }
}

impl<U> ops::DivAssign<Scale> for PathSegment<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: Scale) {
        match *self {
            Self::Move(ref mut point) => *point /= rhs,
            Self::Line(ref mut dist) => *dist /= rhs,
            Self::CubicBezier(ref mut ctrl1, ref mut ctrl2, ref mut dist) => {
                *ctrl1 /= rhs;
                *ctrl2 /= rhs;
                *dist /= rhs;
            }
            Self::QuadraticBezier(ref mut ctrl, ref mut dist) => {
                *ctrl /= rhs;
                *dist /= rhs;
            }
            Self::Close => {}
        }
    }
}

impl<U> ops::Mul<Translate<U>> for PathSegment<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Translate<U>) -> Self::Output {
        match self {
            Self::Move(point) => Self::Move(point * rhs),
            Self::Line(dist) => Self::Line(dist * rhs),
            Self::CubicBezier(ctrl1, ctrl2, dist) => {
                Self::CubicBezier(ctrl1 * rhs, ctrl2 * rhs, dist * rhs)
            }
            Self::QuadraticBezier(ctrl, dist) => Self::QuadraticBezier(ctrl * rhs, dist * rhs),
            Self::Close => Self::Close,
        }
    }
}

impl<U> ops::MulAssign<Translate<U>> for PathSegment<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Translate<U>) {
        match *self {
            Self::Move(ref mut point) => *point *= rhs,
            Self::Line(ref mut dist) => *dist *= rhs,
            Self::CubicBezier(ref mut ctrl1, ref mut ctrl2, ref mut dist) => {
                *ctrl1 *= rhs;
                *ctrl2 *= rhs;
                *dist *= rhs;
            }
            Self::QuadraticBezier(ref mut ctrl, ref mut dist) => {
                *ctrl *= rhs;
                *dist *= rhs;
            }
            Self::Close => {}
        }
    }
}

impl<U> ops::Mul<Transform<U>> for PathSegment<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Transform<U>) -> Self::Output {
        match self {
            Self::Move(point) => Self::Move(point * rhs),
            Self::Line(dist) => Self::Line(dist * rhs),
            Self::CubicBezier(ctrl1, ctrl2, dist) => {
                Self::CubicBezier(ctrl1 * rhs, ctrl2 * rhs, dist * rhs)
            }
            Self::QuadraticBezier(ctrl, dist) => Self::QuadraticBezier(ctrl * rhs, dist * rhs),
            Self::Close => Self::Close,
        }
    }
}

impl<U> ops::MulAssign<Transform<U>> for PathSegment<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Transform<U>) {
        match *self {
            Self::Move(ref mut point) => *point *= rhs,
            Self::Line(ref mut dist) => *dist *= rhs,
            Self::CubicBezier(ref mut ctrl1, ref mut ctrl2, ref mut dist) => {
                *ctrl1 *= rhs;
                *ctrl2 *= rhs;
                *dist *= rhs;
            }
            Self::QuadraticBezier(ref mut ctrl, ref mut dist) => {
                *ctrl *= rhs;
                *dist *= rhs;
            }
            Self::Close => {}
        }
    }
}

impl<Dst, Src> ops::Mul<Conversion<Dst, Src>> for PathSegment<Src>
where
    Dst: Unit,
    Src: Unit,
{
    type Output = PathSegment<Dst>;

    #[inline]
    fn mul(self, rhs: Conversion<Dst, Src>) -> Self::Output {
        match self {
            Self::Move(point) => PathSegment::Move(point * rhs),
            Self::Line(dist) => PathSegment::Line(dist * rhs),
            Self::CubicBezier(ctrl1, ctrl2, dist) => {
                PathSegment::CubicBezier(ctrl1 * rhs, ctrl2 * rhs, dist * rhs)
            }
            Self::QuadraticBezier(ctrl, dist) => {
                PathSegment::QuadraticBezier(ctrl * rhs, dist * rhs)
            }
            Self::Close => PathSegment::Close,
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{declare_units, Mm};

    use super::PathSegment::*;
    use super::*;

    #[test]
    fn path_seg_is_close() {
        let segs = [
            Move(Point::new(Mm(1.0), Mm(1.0))),
            Line(Vector::new(Mm(1.0), Mm(1.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.5)),
                Vector::new(Mm(0.5), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(1.0)), Vector::new(Mm(1.0), Mm(1.0))),
            Close,
        ];
        let segs2 = segs.map(|s| s * 3.0 / 3.0);

        for (seg, seg2) in segs.into_iter().zip(segs2) {
            assert!(seg.is_close(&seg2));
        }

        let segs2 = {
            let mut tmp = segs2;
            tmp.rotate_right(1);
            tmp
        };

        for (seg, seg2) in segs.into_iter().zip(segs2) {
            assert!(!seg.is_close(&seg2));
        }
    }

    #[test]
    fn path_seg_mul() {
        let input = vec![
            Move(Point::new(Mm(1.0), Mm(1.0))),
            Line(Vector::new(Mm(1.0), Mm(1.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.5)),
                Vector::new(Mm(0.5), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(1.0)), Vector::new(Mm(1.0), Mm(1.0))),
            Close,
        ];
        let expected = vec![
            Move(Point::new(Mm(2.0), Mm(2.0))),
            Line(Vector::new(Mm(2.0), Mm(2.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(2.0)),
                Vector::new(Mm(2.0), Mm(2.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(2.0)), Vector::new(Mm(2.0), Mm(2.0))),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp * 2.0;
            assert_is_close!(res, exp);

            let mut res = inp;
            res *= 2.0;
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn path_seg_div() {
        let input = vec![
            Move(Point::new(Mm(1.0), Mm(1.0))),
            Line(Vector::new(Mm(1.0), Mm(1.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.5)),
                Vector::new(Mm(0.5), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(1.0)), Vector::new(Mm(1.0), Mm(1.0))),
            Close,
        ];
        let expected = vec![
            Move(Point::new(Mm(0.5), Mm(0.5))),
            Line(Vector::new(Mm(0.5), Mm(0.5))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.25)),
                Vector::new(Mm(0.25), Mm(0.5)),
                Vector::new(Mm(0.5), Mm(0.5)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(0.5)), Vector::new(Mm(0.5), Mm(0.5))),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp / 2.0;
            assert_is_close!(res, exp);

            let mut res = inp;
            res /= 2.0;
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn path_seg_rotate() {
        use std::f32::consts::SQRT_2;

        let rotate = Rotate::degrees(135.0);

        let input = vec![
            Move(Point::new(Mm(1.0), Mm(1.0))),
            Line(Vector::new(Mm(1.0), Mm(1.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.5)),
                Vector::new(Mm(0.5), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(1.0)), Vector::new(Mm(1.0), Mm(1.0))),
            Close,
        ];
        let expected = vec![
            Move(Point::new(Mm(-SQRT_2), Mm(0.0))),
            Line(Vector::new(Mm(-SQRT_2), Mm(0.0))),
            CubicBezier(
                Vector::new(Mm(-0.25 * SQRT_2), Mm(-0.25 * SQRT_2)),
                Vector::new(Mm(-0.75 * SQRT_2), Mm(-0.25 * SQRT_2)),
                Vector::new(Mm(-SQRT_2), Mm(0.0)),
            ),
            QuadraticBezier(
                Vector::new(Mm(-0.5 * SQRT_2), Mm(-0.5 * SQRT_2)),
                Vector::new(Mm(-SQRT_2), Mm(0.0)),
            ),
            Close,
        ];

        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp * rotate;
            assert_is_close!(res, exp);

            let mut res = inp;
            res *= rotate;
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn path_seg_scale() {
        let scale = Scale::new(2.0, 0.5);

        let input = vec![
            Move(Point::new(Mm(1.0), Mm(1.0))),
            Line(Vector::new(Mm(1.0), Mm(1.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.5)),
                Vector::new(Mm(0.5), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(1.0)), Vector::new(Mm(1.0), Mm(1.0))),
            Close,
        ];
        let expected = vec![
            Move(Point::new(Mm(2.0), Mm(0.5))),
            Line(Vector::new(Mm(2.0), Mm(0.5))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.25)),
                Vector::new(Mm(1.0), Mm(0.5)),
                Vector::new(Mm(2.0), Mm(0.5)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(0.5)), Vector::new(Mm(2.0), Mm(0.5))),
            Close,
        ];

        for (&inp, exp) in input.iter().zip(expected) {
            let res = inp * scale;
            assert_is_close!(res, exp);

            let mut res = inp;
            res *= scale;
            assert_is_close!(res, exp);
        }

        let expected = vec![
            Move(Point::new(Mm(0.5), Mm(2.0))),
            Line(Vector::new(Mm(0.5), Mm(2.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(1.0)),
                Vector::new(Mm(0.25), Mm(2.0)),
                Vector::new(Mm(0.5), Mm(2.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(2.0)), Vector::new(Mm(0.5), Mm(2.0))),
            Close,
        ];

        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp / scale;
            assert_is_close!(res, exp);

            let mut res = inp;
            res /= scale;
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn path_seg_translate() {
        let translate = Translate::new(Mm(2.0), Mm(-1.0));

        let input = vec![
            Move(Point::new(Mm(1.0), Mm(1.0))),
            Line(Vector::new(Mm(1.0), Mm(1.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.5)),
                Vector::new(Mm(0.5), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(1.0)), Vector::new(Mm(1.0), Mm(1.0))),
            Close,
        ];
        let expected = vec![
            Move(Point::new(Mm(3.0), Mm(0.0))),
            Line(Vector::new(Mm(1.0), Mm(1.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.5)),
                Vector::new(Mm(0.5), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(1.0)), Vector::new(Mm(1.0), Mm(1.0))),
            Close,
        ];

        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp * translate;
            assert_is_close!(res, exp);

            let mut res = inp;
            res *= translate;
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn path_seg_transform() {
        let transform = Transform::new(1.0, 0.5, Mm(-1.0), -0.5, 1.5, Mm(2.0));

        let input = vec![
            Move(Point::new(Mm(1.0), Mm(1.0))),
            Line(Vector::new(Mm(1.0), Mm(1.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.5)),
                Vector::new(Mm(0.5), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(1.0)), Vector::new(Mm(1.0), Mm(1.0))),
            Close,
        ];
        let expected = vec![
            Move(Point::new(Mm(0.5), Mm(3.0))),
            Line(Vector::new(Mm(1.5), Mm(1.0))),
            CubicBezier(
                Vector::new(Mm(0.25), Mm(0.75)),
                Vector::new(Mm(1.0), Mm(1.25)),
                Vector::new(Mm(1.5), Mm(1.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.5), Mm(1.5)), Vector::new(Mm(1.5), Mm(1.0))),
            Close,
        ];

        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp * transform;
            assert_is_close!(res, exp);

            let mut res = inp;
            res *= transform;
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn path_seg_convert() {
        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Test, Mm>::new(1.0, 0.5, -1.0, -0.5, 1.5, 2.0);

        let input = vec![
            Move(Point::new(Mm(1.0), Mm(1.0))),
            Line(Vector::new(Mm(1.0), Mm(1.0))),
            CubicBezier(
                Vector::new(Mm(0.0), Mm(0.5)),
                Vector::new(Mm(0.5), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
            ),
            QuadraticBezier(Vector::new(Mm(0.0), Mm(1.0)), Vector::new(Mm(1.0), Mm(1.0))),
            Close,
        ];
        let expected = vec![
            Move(Point::new(Test(0.5), Test(3.0))),
            Line(Vector::new(Test(1.5), Test(1.0))),
            CubicBezier(
                Vector::new(Test(0.25), Test(0.75)),
                Vector::new(Test(1.0), Test(1.25)),
                Vector::new(Test(1.5), Test(1.0)),
            ),
            QuadraticBezier(
                Vector::new(Test(0.5), Test(1.5)),
                Vector::new(Test(1.5), Test(1.0)),
            ),
            Close,
        ];

        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp * conv;
            assert_is_close!(res, exp);
        }
    }
}
