use crate::utils::Vec2;

use PathSegment::{Close, CubicBezier, Line, Move, QuadraticBezier};

// Move is absolute, others are all relative
#[derive(Debug, Clone)]
pub enum PathSegment {
    Move(Vec2),
    Line(Vec2),
    CubicBezier(Vec2, Vec2, Vec2),
    QuadraticBezier(Vec2, Vec2),
    Close,
}

impl PathSegment {
    pub fn scale(&mut self, scale: Vec2) {
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

    pub fn translate(&mut self, dist: Vec2) {
        // Everything else is relative distance
        if let Move(point) = self {
            *point += dist;
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        match self {
            Move(point) => *point = point.rotate(angle),
            Line(dist) => *dist = dist.rotate(angle),
            CubicBezier(c1, c2, d) => {
                *c1 = c1.rotate(angle);
                *c2 = c2.rotate(angle);
                *d = d.rotate(angle);
            }
            QuadraticBezier(c1, d) => {
                *c1 = c1.rotate(angle);
                *d = d.rotate(angle);
            }
            Close => (),
        }
    }

    pub fn skew_x(&mut self, angle: f32) {
        let tan = angle.tan();
        match self {
            Move(point) => point.x -= point.y * tan,
            Line(dist) => dist.x -= dist.y * tan,
            CubicBezier(c1, c2, d) => {
                c1.x -= c1.y * tan;
                c2.x -= c2.y * tan;
                d.x -= d.y * tan;
            }
            QuadraticBezier(c1, d) => {
                c1.x -= c1.y * tan;
                d.x -= d.y * tan;
            }
            Close => (),
        }
    }

    pub fn skew_y(&mut self, angle: f32) {
        let tan = angle.tan();
        match self {
            Move(point) => point.y += point.x * tan,
            Line(dist) => dist.y += dist.x * tan,
            CubicBezier(c1, c2, d) => {
                c1.y += c1.x * tan;
                c2.y += c2.x * tan;
                d.y += d.x * tan;
            }
            QuadraticBezier(c1, d) => {
                c1.y += c1.x * tan;
                d.y += d.x * tan;
            }
            Close => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
    use std::ops::Sub;

    use assert_approx_eq::assert_approx_eq;

    // Needed to implement assert_approx_eq!()
    impl Sub<PathSegment> for PathSegment {
        type Output = f32;
        fn sub(self, rhs: PathSegment) -> Self::Output {
            match (self, rhs) {
                (Move(p1), Move(p2)) => (p2 - p1).abs(),
                (Line(d1), Line(d2)) => (d2 - d1).abs(),
                (CubicBezier(c11, c21, d1), CubicBezier(c12, c22, d2)) => {
                    [(c12 - c11).abs(), (c22 - c21).abs(), (d2 - d1).abs()]
                        .into_iter()
                        .map(|x| x * x)
                        .sum()
                }
                (QuadraticBezier(c1, d1), QuadraticBezier(c2, d2)) => {
                    [(c2 - c1).abs(), (d2 - d1).abs()]
                        .into_iter()
                        .map(|x| x * x)
                        .sum()
                }
                (Close, Close) => 0.,
                (_, _) => unreachable!(), // Different variants
            }
        }
    }

    // Needed to implement assert_approx_eq!()
    impl Copy for PathSegment {}

    #[test]
    fn test_scale() {
        let input = vec![
            Move(Vec2::new(1., 1.)),
            Line(Vec2::new(1., 1.)),
            CubicBezier(Vec2::new(0., 0.5), Vec2::new(0.5, 1.), Vec2::new(1., 1.)),
            QuadraticBezier(Vec2::new(0., 1.), Vec2::new(1., 1.)),
            Close,
        ];
        let expected = vec![
            Move(Vec2::new(2., 2.)),
            Line(Vec2::new(2., 2.)),
            CubicBezier(Vec2::new(0., 1.), Vec2::new(1., 2.), Vec2::new(2., 2.)),
            QuadraticBezier(Vec2::new(0., 2.), Vec2::new(2., 2.)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let mut res = inp;
            res.scale(Vec2::new(2., 2.));
            assert_approx_eq!(res, exp);
        }
    }

    #[test]
    fn test_translate() {
        let input = vec![
            Move(Vec2::new(1., 1.)),
            Line(Vec2::new(1., 1.)),
            CubicBezier(Vec2::new(0., 0.5), Vec2::new(0.5, 1.), Vec2::new(1., 1.)),
            QuadraticBezier(Vec2::new(0., 1.), Vec2::new(1., 1.)),
            Close,
        ];
        let expected = vec![
            Move(Vec2::new(2., 2.)),
            Line(Vec2::new(1., 1.)),
            CubicBezier(Vec2::new(0., 0.5), Vec2::new(0.5, 1.), Vec2::new(1., 1.)),
            QuadraticBezier(Vec2::new(0., 1.), Vec2::new(1., 1.)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let mut res = inp;
            res.translate(Vec2::new(1., 1.));
            assert_approx_eq!(res, exp);
        }
    }

    #[test]
    fn test_rotate() {
        let input = vec![
            Move(Vec2::new(1., 1.)),
            Line(Vec2::new(1., 1.)),
            CubicBezier(Vec2::new(0., 0.5), Vec2::new(0.5, 1.), Vec2::new(1., 1.)),
            QuadraticBezier(Vec2::new(0., 1.), Vec2::new(1., 1.)),
            Close,
        ];
        let expected = vec![
            Move(Vec2::new(-1., 1.)),
            Line(Vec2::new(-1., 1.)),
            CubicBezier(Vec2::new(-0.5, 0.), Vec2::new(-1., 0.5), Vec2::new(-1., 1.)),
            QuadraticBezier(Vec2::new(-1., 0.), Vec2::new(-1., 1.)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let mut res = inp;
            res.rotate(FRAC_PI_2);
            assert_approx_eq!(res, exp);
        }
    }

    #[test]
    fn test_skew_x() {
        let input = vec![
            Move(Vec2::new(1., 1.)),
            Line(Vec2::new(1., 1.)),
            CubicBezier(Vec2::new(0., 0.5), Vec2::new(0.5, 1.), Vec2::new(1., 1.)),
            QuadraticBezier(Vec2::new(0., 1.), Vec2::new(1., 1.)),
            Close,
        ];
        let expected = vec![
            Move(Vec2::new(0., 1.)),
            Line(Vec2::new(0., 1.)),
            CubicBezier(Vec2::new(-0.5, 0.5), Vec2::new(-0.5, 1.), Vec2::new(0., 1.)),
            QuadraticBezier(Vec2::new(-1., 1.), Vec2::new(0., 1.)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let mut res = inp;
            res.skew_x(FRAC_PI_4);
            assert_approx_eq!(res, exp);
        }
    }

    #[test]
    fn test_skew_y() {
        let input = vec![
            Move(Vec2::new(1., 1.)),
            Line(Vec2::new(1., 1.)),
            CubicBezier(Vec2::new(0., 0.5), Vec2::new(0.5, 1.), Vec2::new(1., 1.)),
            QuadraticBezier(Vec2::new(0., 1.), Vec2::new(1., 1.)),
            Close,
        ];
        let expected = vec![
            Move(Vec2::new(1., 2.)),
            Line(Vec2::new(1., 2.)),
            CubicBezier(Vec2::new(0., 0.5), Vec2::new(0.5, 1.5), Vec2::new(1., 2.)),
            QuadraticBezier(Vec2::new(0., 1.), Vec2::new(1., 2.)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let mut res = inp;
            res.skew_y(FRAC_PI_4);
            assert_approx_eq!(res, exp);
        }
    }
}
