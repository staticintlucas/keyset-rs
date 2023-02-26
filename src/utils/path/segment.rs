use crate::utils::{Point, Scale, Size};

#[derive(Debug, Clone)]
pub enum PathSegment {
    Move(Point),
    Line(Point),
    CubicBezier(Point, Point, Point),
    QuadraticBezier(Point, Point),
    Close,
}

impl PathSegment {
    pub fn scale(self, scale: Scale) -> Self {
        match self {
            Self::Move(point) => Self::Move(point * scale),
            Self::Line(point) => Self::Line(point * scale),
            Self::CubicBezier(ctrl1, ctrl2, point) => {
                Self::CubicBezier(ctrl1 * scale, ctrl2 * scale, point * scale)
            }
            Self::QuadraticBezier(ctrl1, point) => {
                Self::QuadraticBezier(ctrl1 * scale, point * scale)
            }
            Self::Close => Self::Close,
        }
    }

    pub fn translate(self, dist: Size) -> Self {
        match self {
            Self::Move(point) => Self::Move(point + dist),
            Self::Line(point) => Self::Line(point + dist),
            Self::CubicBezier(ctrl1, ctrl2, point) => {
                Self::CubicBezier(ctrl1 + dist, ctrl2 + dist, point + dist)
            }
            Self::QuadraticBezier(ctrl1, point) => {
                Self::QuadraticBezier(ctrl1 + dist, point + dist)
            }
            Self::Close => Self::Close,
        }
    }

    pub fn rotate(self, angle: f32) -> Self {
        match self {
            Self::Move(point) => Self::Move(point.rotate(angle)),
            Self::Line(point) => Self::Line(point.rotate(angle)),
            Self::CubicBezier(c1, c2, p) => {
                let [c1, c2, p] = [c1, c2, p].map(|p| p.rotate(angle));
                Self::CubicBezier(c1, c2, p)
            }
            Self::QuadraticBezier(c1, p) => {
                let [c1, p] = [c1, p].map(|p| p.rotate(angle));
                Self::QuadraticBezier(c1, p)
            }
            Self::Close => Self::Close,
        }
    }

    pub fn skew_x(self, angle: f32) -> Self {
        let tan = angle.tan();
        match self {
            Self::Move(point) => Self::Move(Point {
                x: point.x - point.y * tan,
                y: point.y,
            }),
            Self::Line(point) => Self::Line(Point {
                x: point.x - point.y * tan,
                y: point.y,
            }),
            Self::CubicBezier(c1, c2, p) => {
                let [c1, c2, p] = [c1, c2, p].map(|p| Point {
                    x: p.x - p.y * tan,
                    y: p.y,
                });
                Self::CubicBezier(c1, c2, p)
            }
            Self::QuadraticBezier(c1, p) => {
                let [c1, p] = [c1, p].map(|p| Point {
                    x: p.x - p.y * tan,
                    y: p.y,
                });
                Self::QuadraticBezier(c1, p)
            }
            Self::Close => Self::Close,
        }
    }

    pub fn skew_y(self, angle: f32) -> Self {
        let tan = angle.tan();
        match self {
            Self::Move(point) => Self::Move(Point {
                x: point.x,
                y: point.y + point.x * tan,
            }),
            Self::Line(point) => Self::Line(Point {
                x: point.x,
                y: point.y + point.x * tan,
            }),
            Self::CubicBezier(c1, c2, p) => {
                let [c1, c2, p] = [c1, c2, p].map(|p| Point {
                    x: p.x,
                    y: p.y + p.x * tan,
                });
                Self::CubicBezier(c1, c2, p)
            }
            Self::QuadraticBezier(c1, p) => {
                let [c1, p] = [c1, p].map(|p| Point {
                    x: p.x,
                    y: p.y + p.x * tan,
                });
                Self::QuadraticBezier(c1, p)
            }
            Self::Close => Self::Close,
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
                (Self::Move(p1), Self::Move(p2)) => (p2 - p1).abs(),
                (Self::Line(p1), Self::Line(p2)) => (p2 - p1).abs(),
                (Self::CubicBezier(c11, c21, p1), Self::CubicBezier(c12, c22, p2)) => {
                    [(c12 - c11).abs(), (c22 - c21).abs(), (p2 - p1).abs()]
                        .into_iter()
                        .map(|x| x * x)
                        .sum()
                }
                (Self::QuadraticBezier(c1, p1), Self::QuadraticBezier(c2, p2)) => {
                    [(c2 - c1).abs(), (p2 - p1).abs()]
                        .into_iter()
                        .map(|x| x * x)
                        .sum()
                }
                (Self::Close, Self::Close) => 0.,
                (_, _) => unreachable!(), // Different variants
            }
        }
    }

    // Needed to implement assert_approx_eq!()
    impl Copy for PathSegment {}

    #[test]
    fn test_scale() {
        let input = vec![
            PathSegment::Move(Point::new(1., 1.)),
            PathSegment::Line(Point::new(1., 1.)),
            PathSegment::CubicBezier(Point::new(0., 0.5), Point::new(0.5, 1.), Point::new(1., 1.)),
            PathSegment::QuadraticBezier(Point::new(0., 1.), Point::new(1., 1.)),
            PathSegment::Close,
        ];
        let expected = vec![
            PathSegment::Move(Point::new(2., 2.)),
            PathSegment::Line(Point::new(2., 2.)),
            PathSegment::CubicBezier(Point::new(0., 1.), Point::new(1., 2.), Point::new(2., 2.)),
            PathSegment::QuadraticBezier(Point::new(0., 2.), Point::new(2., 2.)),
            PathSegment::Close,
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
            PathSegment::Move(Point::new(1., 1.)),
            PathSegment::Line(Point::new(1., 1.)),
            PathSegment::CubicBezier(Point::new(0., 0.5), Point::new(0.5, 1.), Point::new(1., 1.)),
            PathSegment::QuadraticBezier(Point::new(0., 1.), Point::new(1., 1.)),
            PathSegment::Close,
        ];
        let expected = vec![
            PathSegment::Move(Point::new(2., 2.)),
            PathSegment::Line(Point::new(2., 2.)),
            PathSegment::CubicBezier(Point::new(1., 1.5), Point::new(1.5, 2.), Point::new(2., 2.)),
            PathSegment::QuadraticBezier(Point::new(1., 2.), Point::new(2., 2.)),
            PathSegment::Close,
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
            PathSegment::Move(Point::new(1., 1.)),
            PathSegment::Line(Point::new(1., 1.)),
            PathSegment::CubicBezier(Point::new(0., 0.5), Point::new(0.5, 1.), Point::new(1., 1.)),
            PathSegment::QuadraticBezier(Point::new(0., 1.), Point::new(1., 1.)),
            PathSegment::Close,
        ];
        let expected = vec![
            PathSegment::Move(Point::new(-1., 1.)),
            PathSegment::Line(Point::new(-1., 1.)),
            PathSegment::CubicBezier(
                Point::new(-0.5, 0.),
                Point::new(-1., 0.5),
                Point::new(-1., 1.),
            ),
            PathSegment::QuadraticBezier(Point::new(-1., 0.), Point::new(-1., 1.)),
            PathSegment::Close,
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
            PathSegment::Move(Point::new(1., 1.)),
            PathSegment::Line(Point::new(1., 1.)),
            PathSegment::CubicBezier(Point::new(0., 0.5), Point::new(0.5, 1.), Point::new(1., 1.)),
            PathSegment::QuadraticBezier(Point::new(0., 1.), Point::new(1., 1.)),
            PathSegment::Close,
        ];
        let expected = vec![
            PathSegment::Move(Point::new(0., 1.)),
            PathSegment::Line(Point::new(0., 1.)),
            PathSegment::CubicBezier(
                Point::new(-0.5, 0.5),
                Point::new(-0.5, 1.),
                Point::new(0., 1.),
            ),
            PathSegment::QuadraticBezier(Point::new(-1., 1.), Point::new(0., 1.)),
            PathSegment::Close,
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
            PathSegment::Move(Point::new(1., 1.)),
            PathSegment::Line(Point::new(1., 1.)),
            PathSegment::CubicBezier(Point::new(0., 0.5), Point::new(0.5, 1.), Point::new(1., 1.)),
            PathSegment::QuadraticBezier(Point::new(0., 1.), Point::new(1., 1.)),
            PathSegment::Close,
        ];
        let expected = vec![
            PathSegment::Move(Point::new(1., 2.)),
            PathSegment::Line(Point::new(1., 2.)),
            PathSegment::CubicBezier(
                Point::new(0., 0.5),
                Point::new(0.5, 1.5),
                Point::new(1., 2.),
            ),
            PathSegment::QuadraticBezier(Point::new(0., 1.), Point::new(1., 2.)),
            PathSegment::Close,
        ];

        assert_eq!(input.len(), expected.len());
        for (inp, exp) in input.into_iter().zip(expected) {
            let res = inp.skew_y(FRAC_PI_4);
            assert_approx_eq!(res, exp);
        }
    }
}
