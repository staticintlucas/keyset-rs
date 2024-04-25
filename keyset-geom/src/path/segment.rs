use std::ops::{Div, DivAssign, Mul, MulAssign};

use PathSegment::{Close, CubicBezier, Line, Move, QuadraticBezier};

use crate::{ApproxEq, Point, Scale, Transform, Vector};

/// Enum representing a path segment
#[allow(clippy::module_name_repetitions)] // rust-lang/rust-clippy#8524
#[derive(Debug, PartialEq)]
pub enum PathSegment<U> {
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

// Impl here rather than derive so we don't require U: Clone everywhere
impl<U> Clone for PathSegment<U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<U> Copy for PathSegment<U> {}

impl<U> ApproxEq<Vector<U>> for PathSegment<U> {
    fn approx_epsilon() -> Vector<U> {
        Vector::<U>::approx_epsilon()
    }

    fn approx_eq_eps(&self, other: &Self, approx_epsilon: &Vector<U>) -> bool {
        match (self, other) {
            (Move(s), Move(o)) => s.approx_eq_eps(o, &approx_epsilon.to_point()),
            (Line(s), Line(o)) => s.approx_eq_eps(o, approx_epsilon),
            (CubicBezier(s1, s2, s), CubicBezier(o1, o2, o)) => {
                s1.approx_eq_eps(o1, approx_epsilon)
                    && s2.approx_eq_eps(o2, approx_epsilon)
                    && s.approx_eq_eps(o, approx_epsilon)
            }
            (QuadraticBezier(s1, s), QuadraticBezier(o1, o)) => {
                s1.approx_eq_eps(o1, approx_epsilon) && s.approx_eq_eps(o, approx_epsilon)
            }
            (Close, Close) => true,
            _ => false,
        }
    }
}

impl<U> PathSegment<U> {
    /// Translate the path segment
    #[must_use]
    pub fn translate(self, by: Vector<U>) -> Self {
        match self {
            Move(point) => Move(point + by),
            // Everything else is relative
            _ => self,
        }
    }

    /// Scale the path segment
    #[must_use]
    pub fn scale(self, x: f32, y: f32) -> Self {
        let scale = Vector::new(x, y);
        match self {
            Move(point) => Move(point.to_vector().component_mul(scale).to_point()),
            Line(dist) => Line(dist.component_mul(scale)),
            CubicBezier(ctrl1, ctrl2, dist) => CubicBezier(
                ctrl1.component_mul(scale),
                ctrl2.component_mul(scale),
                dist.component_mul(scale),
            ),
            QuadraticBezier(ctrl1, dist) => {
                QuadraticBezier(ctrl1.component_mul(scale), dist.component_mul(scale))
            }
            Close => Close,
        }
    }
}

impl<U, V> Mul<Scale<U, V>> for PathSegment<U> {
    type Output = PathSegment<V>;

    fn mul(self, scale: Scale<U, V>) -> Self::Output {
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

impl<U, V> Mul<Transform<U, V>> for PathSegment<U> {
    type Output = PathSegment<V>;

    fn mul(self, transform: Transform<U, V>) -> Self::Output {
        match self {
            Move(point) => Move(transform.transform_point(point)),
            Line(dist) => Line(transform.transform_vector(dist)),
            CubicBezier(ctrl1, ctrl2, dist) => CubicBezier(
                transform.transform_vector(ctrl1),
                transform.transform_vector(ctrl2),
                transform.transform_vector(dist),
            ),
            QuadraticBezier(ctrl1, dist) => QuadraticBezier(
                transform.transform_vector(ctrl1),
                transform.transform_vector(dist),
            ),
            Close => Close,
        }
    }
}

impl<U> MulAssign<Scale<U, U>> for PathSegment<U> {
    fn mul_assign(&mut self, scale: Scale<U, U>) {
        match self {
            Move(point) => *point *= scale,
            Line(dist) => *dist *= scale,
            CubicBezier(ctrl1, ctrl2, dist) => {
                *ctrl1 *= scale;
                *ctrl2 *= scale;
                *dist *= scale;
            }
            QuadraticBezier(ctrl1, dist) => {
                *ctrl1 *= scale;
                *dist *= scale;
            }
            Close => (),
        }
    }
}

impl<U> MulAssign<Transform<U, U>> for PathSegment<U> {
    fn mul_assign(&mut self, transform: Transform<U, U>) {
        match self {
            Move(point) => *point = transform.transform_point(*point),
            Line(dist) => *dist = transform.transform_vector(*dist),
            CubicBezier(ctrl1, ctrl2, dist) => {
                *ctrl1 = transform.transform_vector(*ctrl1);
                *ctrl2 = transform.transform_vector(*ctrl2);
                *dist = transform.transform_vector(*dist);
            }
            QuadraticBezier(ctrl1, dist) => {
                *ctrl1 = transform.transform_vector(*ctrl1);
                *dist = transform.transform_vector(*dist);
            }
            Close => (),
        }
    }
}

impl<U, V> Div<Scale<V, U>> for PathSegment<U> {
    type Output = PathSegment<V>;

    fn div(self, scale: Scale<V, U>) -> Self::Output {
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

impl<U> DivAssign<Scale<U, U>> for PathSegment<U> {
    fn div_assign(&mut self, scale: Scale<U, U>) {
        match self {
            Move(point) => *point /= scale,
            Line(dist) => *dist /= scale,
            CubicBezier(ctrl1, ctrl2, dist) => {
                *ctrl1 /= scale;
                *ctrl2 /= scale;
                *dist /= scale;
            }
            QuadraticBezier(ctrl1, dist) => {
                *ctrl1 /= scale;
                *dist /= scale;
            }
            Close => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        let input = vec![
            Move(Point::<()>::new(1.0, 1.0)),
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
            Move(Point::new(2.0, 2.0)),
            Line(Vector::new(1.0, 1.0)),
            CubicBezier(
                Vector::new(0.0, 0.5),
                Vector::new(0.5, 1.0),
                Vector::new(1.0, 1.0),
            ),
            QuadraticBezier(Vector::new(0.0, 1.0), Vector::new(1.0, 1.0)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp.translate(Vector::new(1.0, 1.0));
            assert!(res.approx_eq(&exp));
        }
    }
}
