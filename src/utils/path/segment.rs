use crate::utils::{Point, Scale, Size};

use PathSegment::{Close, CubicBezier, Line, Move, QuadraticBezier};

#[derive(Debug, Clone)]
pub enum PathSegment {
    Move(Point),
    Line(Size),
    CubicBezier(Size, Size, Size),
    QuadraticBezier(Size, Size),
    Close,
}

impl PathSegment {
    pub fn scale(self, scale: Scale) -> Self {
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

    pub fn translate(self, dist: Size) -> Self {
        match self {
            Move(point) => Move(point + dist),
            Line(dist) => Line(dist + dist),
            CubicBezier(ctrl1, ctrl2, dist) => CubicBezier(ctrl1 + dist, ctrl2 + dist, dist + dist),
            QuadraticBezier(ctrl1, dist) => QuadraticBezier(ctrl1 + dist, dist + dist),
            Close => Close,
        }
    }

    pub fn rotate(self, angle: f32) -> Self {
        match self {
            Move(point) => Move(point.rotate(angle)),
            Line(dist) => Line(dist.rotate(angle)),
            CubicBezier(c1, c2, d) => {
                let [c1, c2, d] = [c1, c2, d].map(|d| d.rotate(angle));
                CubicBezier(c1, c2, d)
            }
            QuadraticBezier(c1, d) => {
                let [c1, d] = [c1, d].map(|d| d.rotate(angle));
                QuadraticBezier(c1, d)
            }
            Close => Close,
        }
    }

    pub fn skew_x(self, angle: f32) -> Self {
        let tan = angle.tan();
        match self {
            Move(point) => Move(Point {
                x: point.x - point.y * tan,
                y: point.y,
            }),
            Line(dist) => Line(Size {
                w: dist.w - dist.h * tan,
                h: dist.h,
            }),
            CubicBezier(c1, c2, d) => {
                let [c1, c2, d] = [c1, c2, d].map(|d| Size {
                    w: d.w - d.h * tan,
                    h: d.h,
                });
                CubicBezier(c1, c2, d)
            }
            QuadraticBezier(c1, d) => {
                let [c1, d] = [c1, d].map(|d| Size {
                    w: d.w - d.h * tan,
                    h: d.h,
                });
                QuadraticBezier(c1, d)
            }
            Close => Close,
        }
    }

    pub fn skew_y(self, angle: f32) -> Self {
        let tan = angle.tan();
        match self {
            Move(point) => Move(Point {
                x: point.x,
                y: point.y + point.x * tan,
            }),
            Line(dist) => Line(Size {
                w: dist.w,
                h: dist.h + dist.w * tan,
            }),
            CubicBezier(c1, c2, d) => {
                let [c1, c2, d] = [c1, c2, d].map(|d| Size {
                    w: d.w,
                    h: d.h + d.w * tan,
                });
                CubicBezier(c1, c2, d)
            }
            QuadraticBezier(c1, d) => {
                let [c1, d] = [c1, d].map(|d| Size {
                    w: d.w,
                    h: d.h + d.w * tan,
                });
                QuadraticBezier(c1, d)
            }
            Close => Close,
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
            Move(Point::new(1., 1.)),
            Line(Size::new(1., 1.)),
            CubicBezier(Size::new(0., 0.5), Size::new(0.5, 1.), Size::new(1., 1.)),
            QuadraticBezier(Size::new(0., 1.), Size::new(1., 1.)),
            Close,
        ];
        let expected = vec![
            Move(Point::new(2., 2.)),
            Line(Size::new(2., 2.)),
            CubicBezier(Size::new(0., 1.), Size::new(1., 2.), Size::new(2., 2.)),
            QuadraticBezier(Size::new(0., 2.), Size::new(2., 2.)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp.scale(Scale::new(2., 2.));
            assert_approx_eq!(res, exp);
        }
    }

    #[test]
    fn test_translate() {
        let input = vec![
            Move(Point::new(1., 1.)),
            Line(Size::new(1., 1.)),
            CubicBezier(Size::new(0., 0.5), Size::new(0.5, 1.), Size::new(1., 1.)),
            QuadraticBezier(Size::new(0., 1.), Size::new(1., 1.)),
            Close,
        ];
        let expected = vec![
            Move(Point::new(2., 2.)),
            Line(Size::new(2., 2.)),
            CubicBezier(Size::new(1., 1.5), Size::new(1.5, 2.), Size::new(2., 2.)),
            QuadraticBezier(Size::new(1., 2.), Size::new(2., 2.)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp.translate(Size::new(1., 1.));
            assert_approx_eq!(res, exp);
        }
    }

    #[test]
    fn test_rotate() {
        let input = vec![
            Move(Point::new(1., 1.)),
            Line(Size::new(1., 1.)),
            CubicBezier(Size::new(0., 0.5), Size::new(0.5, 1.), Size::new(1., 1.)),
            QuadraticBezier(Size::new(0., 1.), Size::new(1., 1.)),
            Close,
        ];
        let expected = vec![
            Move(Point::new(-1., 1.)),
            Line(Size::new(-1., 1.)),
            CubicBezier(Size::new(-0.5, 0.), Size::new(-1., 0.5), Size::new(-1., 1.)),
            QuadraticBezier(Size::new(-1., 0.), Size::new(-1., 1.)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp.rotate(FRAC_PI_2);
            assert_approx_eq!(res, exp);
        }
    }

    #[test]
    fn test_skew_x() {
        let input = vec![
            Move(Point::new(1., 1.)),
            Line(Size::new(1., 1.)),
            CubicBezier(Size::new(0., 0.5), Size::new(0.5, 1.), Size::new(1., 1.)),
            QuadraticBezier(Size::new(0., 1.), Size::new(1., 1.)),
            Close,
        ];
        let expected = vec![
            Move(Point::new(0., 1.)),
            Line(Size::new(0., 1.)),
            CubicBezier(Size::new(-0.5, 0.5), Size::new(-0.5, 1.), Size::new(0., 1.)),
            QuadraticBezier(Size::new(-1., 1.), Size::new(0., 1.)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp.skew_x(FRAC_PI_4);
            assert_approx_eq!(res, exp);
        }
    }

    #[test]
    fn test_skew_y() {
        let input = vec![
            Move(Point::new(1., 1.)),
            Line(Size::new(1., 1.)),
            CubicBezier(Size::new(0., 0.5), Size::new(0.5, 1.), Size::new(1., 1.)),
            QuadraticBezier(Size::new(0., 1.), Size::new(1., 1.)),
            Close,
        ];
        let expected = vec![
            Move(Point::new(1., 2.)),
            Line(Size::new(1., 2.)),
            CubicBezier(Size::new(0., 0.5), Size::new(0.5, 1.5), Size::new(1., 2.)),
            QuadraticBezier(Size::new(0., 1.), Size::new(1., 2.)),
            Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp.skew_y(FRAC_PI_4);
            assert_approx_eq!(res, exp);
        }
    }
}
