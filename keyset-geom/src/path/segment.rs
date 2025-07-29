use std::ops;

use isclose::IsClose;

use crate::{Point, Unit, Vector};

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

impl<U> IsClose<f32> for PathSegment<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <f32 as IsClose>::ABS_TOL;
    const REL_TOL: f32 = <f32 as IsClose>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        use PathSegment::*;
        match (*self, *other) {
            (Move(ref s), Move(ref o)) => s.is_close_impl(o, rel_tol, abs_tol),
            (Line(ref s), Line(ref o)) => s.is_close_impl(o, rel_tol, abs_tol),
            (CubicBezier(ref s1, ref s2, ref s), CubicBezier(ref o1, ref o2, ref o)) => {
                s1.is_close_impl(o1, rel_tol, abs_tol)
                    && s2.is_close_impl(o2, rel_tol, abs_tol)
                    && s.is_close_impl(o, rel_tol, abs_tol)
            }
            (QuadraticBezier(ref s1, ref s), QuadraticBezier(ref o1, ref o)) => {
                s1.is_close_impl(o1, rel_tol, abs_tol) && s.is_close_impl(o, rel_tol, abs_tol)
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

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::Mm;

    use super::PathSegment::*;
    use super::*;

    #[test]
    fn path_seg_is_close() {
        let segs = [
            Move(Point::<Mm>::new(1.0, 1.0)),
            Line(Vector::new(1.0, 1.0)),
            CubicBezier(
                Vector::new(0.0, 0.5),
                Vector::new(0.5, 1.0),
                Vector::new(1.0, 1.0),
            ),
            QuadraticBezier(Vector::new(0.0, 1.0), Vector::new(1.0, 1.0)),
            Close,
        ];
        let segs2 = segs.map(|s| s * 3.0 / 3.0);

        for (seg, seg2) in segs.into_iter().zip(segs2) {
            assert!(seg.is_close(seg2));
        }

        let segs2 = {
            let mut tmp = segs2;
            tmp.rotate_right(1);
            tmp
        };

        for (seg, seg2) in segs.into_iter().zip(segs2) {
            assert!(!seg.is_close(seg2));
        }
    }

    #[test]
    fn path_seg_mul() {
        let input = vec![
            Move(Point::<Mm>::new(1.0, 1.0)),
            Line(Vector::new(1.0, 1.0)),
            CubicBezier(
                Vector::new(0.0, 0.5),
                Vector::new(0.5, 1.0),
                Vector::new(1.0, 1.0),
            ),
            QuadraticBezier(Vector::new(0.0, 1.0), Vector::new(1.0, 1.0)),
            Close,
        ];
        let expected = vec![
            Move(Point::<Mm>::new(2.0, 2.0)),
            Line(Vector::new(2.0, 2.0)),
            CubicBezier(
                Vector::new(0.0, 1.0),
                Vector::new(1.0, 2.0),
                Vector::new(2.0, 2.0),
            ),
            QuadraticBezier(Vector::new(0.0, 2.0), Vector::new(2.0, 2.0)),
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
    fn path_seg_div_scale() {
        let input = vec![
            Move(Point::<Mm>::new(1.0, 1.0)),
            Line(Vector::new(1.0, 1.0)),
            CubicBezier(
                Vector::new(0.0, 0.5),
                Vector::new(0.5, 1.0),
                Vector::new(1.0, 1.0),
            ),
            QuadraticBezier(Vector::new(0.0, 1.0), Vector::new(1.0, 1.0)),
            Close,
        ];
        let expected = vec![
            Move(Point::<Mm>::new(0.5, 0.5)),
            Line(Vector::new(0.5, 0.5)),
            CubicBezier(
                Vector::new(0.0, 0.25),
                Vector::new(0.25, 0.5),
                Vector::new(0.5, 0.5),
            ),
            QuadraticBezier(Vector::new(0.0, 0.5), Vector::new(0.5, 0.5)),
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
}
