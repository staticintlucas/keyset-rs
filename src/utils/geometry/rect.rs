use std::ops;

use super::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    position: Vec2,
    size: Vec2,
}

impl Rect {
    #[inline]
    pub fn new(position: Vec2, size: Vec2) -> Self {
        let (p1, p2) = (position, position + size);
        let (min, max) = (Vec2::min(p1, p2), Vec2::max(p1, p2));
        Self {
            position: min,
            size: max - min,
        }
    }

    #[inline]
    pub fn from_points(point1: Vec2, point2: Vec2) -> Self {
        let (min, max) = (Vec2::min(point1, point2), Vec2::max(point1, point2));
        Self {
            position: min,
            size: max - min,
        }
    }

    #[inline]
    pub const fn position(self) -> Vec2 {
        self.position
    }

    #[inline]
    pub const fn size(self) -> Vec2 {
        self.size
    }

    #[inline]
    pub fn center(self) -> Vec2 {
        self.position + (self.size * 0.5)
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.size == other.size
    }
}

impl<T: Copy> ops::Mul<T> for Rect
where
    Vec2: ops::Mul<T, Output = Vec2>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        // The call to new will ensure that rhs < 0 is handled correctly
        Self::Output::new(self.position * rhs, self.size * rhs)
    }
}

impl ops::Mul<Rect> for f64 {
    type Output = Rect;

    #[inline]
    fn mul(self, rhs: Rect) -> Self::Output {
        // The call to new will ensure that rhs < 0 is handled correctly
        Self::Output::new(self * rhs.position, self * rhs.size)
    }
}

impl<T: Copy> ops::MulAssign<T> for Rect
where
    Vec2: ops::MulAssign<T>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.position *= rhs;
        self.size *= rhs;

        // The call to new will ensure that rhs < 0 is handled correctly
        *self = Self::new(self.position, self.size);
    }
}

impl<T: Copy> ops::Div<T> for Rect
where
    Vec2: ops::Div<T, Output = Vec2>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        // The call to new will ensure that rhs < 0 is handled correctly
        Self::Output::new(self.position / rhs, self.size / rhs)
    }
}

impl<T: Copy> ops::DivAssign<T> for Rect
where
    Vec2: ops::DivAssign<T>,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.position /= rhs;
        self.size /= rhs;

        // The call to new will ensure that rhs < 0 is handled correctly
        *self = Self::new(self.position, self.size);
    }
}

impl Rect {}

impl From<Rect> for (f64, f64, f64, f64) {
    fn from(rect: Rect) -> (f64, f64, f64, f64) {
        (rect.position.x, rect.position.y, rect.size.x, rect.size.y)
    }
}

impl From<(f64, f64, f64, f64)> for Rect {
    fn from(tuple: (f64, f64, f64, f64)) -> Self {
        // The call to new will ensure that width/height < 0 is handled correctly
        Self::new((tuple.0, tuple.1).into(), (tuple.2, tuple.3).into())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RoundRect {
    position: Vec2,
    size: Vec2,
    radius: Vec2,
}

impl RoundRect {
    #[inline]
    pub fn new(position: Vec2, size: Vec2, radius: Vec2) -> Self {
        let (p1, p2) = (position, position + size);
        let (min, max) = (Vec2::min(p1, p2), Vec2::max(p1, p2));
        let abs_r = Vec2::new(radius.x.abs(), radius.y.abs()).min((max - min) / 2.);
        Self {
            position: min,
            size: max - min,
            radius: abs_r,
        }
    }

    #[inline]
    pub fn from_points(point1: Vec2, point2: Vec2, radius: Vec2) -> Self {
        let (min, max) = (Vec2::min(point1, point2), Vec2::max(point1, point2));
        let abs_r = Vec2::new(radius.x.abs(), radius.y.abs()).min((max - min) / 2.);
        Self {
            position: min,
            size: max - min,
            radius: abs_r,
        }
    }

    #[inline]
    pub const fn position(self) -> Vec2 {
        self.position
    }

    #[inline]
    pub const fn size(self) -> Vec2 {
        self.size
    }

    #[inline]
    pub const fn radius(self) -> Vec2 {
        self.radius
    }

    #[inline]
    pub fn center(self) -> Vec2 {
        self.position + (self.size / 2.)
    }
}

impl PartialEq for RoundRect {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.size == other.size && self.radius == other.radius
    }
}

impl<T: Copy> ops::Mul<T> for RoundRect
where
    Vec2: ops::Mul<T, Output = Vec2>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        // The call to new will ensure that rhs < 0 is handled correctly
        Self::Output::new(self.position * rhs, self.size * rhs, self.radius * rhs)
    }
}

impl ops::Mul<RoundRect> for f64 {
    type Output = RoundRect;

    #[inline]
    fn mul(self, rhs: RoundRect) -> Self::Output {
        // The call to new will ensure that rhs < 0 is handled correctly
        Self::Output::new(self * rhs.position, self * rhs.size, self * rhs.radius)
    }
}

impl<T: Copy> ops::MulAssign<T> for RoundRect
where
    Vec2: ops::MulAssign<T>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.position *= rhs;
        self.size *= rhs;
        self.radius *= rhs;

        // The call to new will ensure that rhs < 0 is handled correctly
        *self = Self::new(self.position, self.size, self.radius);
    }
}

impl<T: Copy> ops::Div<T> for RoundRect
where
    Vec2: ops::Div<T, Output = Vec2>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        // The call to new will ensure that rhs < 0 is handled correctly
        Self::Output::new(self.position / rhs, self.size / rhs, self.radius / rhs)
    }
}

impl<T: Copy> ops::DivAssign<T> for RoundRect
where
    Vec2: ops::DivAssign<T>,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.position /= rhs;
        self.size /= rhs;
        self.radius /= rhs;

        // The call to new will ensure that rhs < 0 is handled correctly
        *self = Self::new(self.position, self.size, self.radius);
    }
}

impl From<RoundRect> for (f64, f64, f64, f64, f64, f64) {
    fn from(rect: RoundRect) -> (f64, f64, f64, f64, f64, f64) {
        let ((x, y), (w, h), (rx, ry)) =
            (rect.position.into(), rect.size.into(), rect.radius.into());
        (x, y, w, h, rx, ry)
    }
}

impl From<(f64, f64, f64, f64, f64, f64)> for RoundRect {
    fn from(tuple: (f64, f64, f64, f64, f64, f64)) -> Self {
        // The call to new will ensure that width/height < 0 is handled correctly
        Self::new(
            (tuple.0, tuple.1).into(),
            (tuple.2, tuple.3).into(),
            (tuple.4, tuple.5).into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    // this is required for assert_approx_eq to work
    impl ops::Sub<Rect> for Rect {
        type Output = f64;

        fn sub(self, other: Self) -> f64 {
            [
                (other.position() - self.position()),
                (other.size() - self.size()),
            ]
            .into_iter()
            .map(|v| v.abs().powi(2))
            .sum::<f64>()
            .sqrt()
        }
    }

    // this is required for assert_approx_eq to work
    impl ops::Sub<RoundRect> for RoundRect {
        type Output = f64;

        fn sub(self, other: Self) -> f64 {
            [
                (other.position - self.position),
                (other.size - self.size),
                (other.radius - self.radius),
            ]
            .into_iter()
            .map(|v| v.abs().powi(2))
            .sum::<f64>()
            .sqrt()
        }
    }

    #[test]
    fn test_rect_new() {
        let position = Vec2::new(1., 2.);
        let size = Vec2::new(3., 4.);
        let radius = Vec2::new(2., 0.5);

        let rect = Rect::new(position, size);
        assert_approx_eq!(rect, (1., 2., 3., 4.).into());

        let rect = RoundRect::new(position, size, radius);
        assert_approx_eq!(rect, (1., 2., 3., 4., 1.5, 0.5).into());
    }

    #[test]
    fn test_rect_from_points() {
        let point1 = Vec2::new(1., 2.);
        let point2 = Vec2::new(3., 4.);
        let radius = Vec2::new(2., 0.5);

        let rect = Rect::from_points(point1, point2);
        assert_approx_eq!(
            rect,
            Rect {
                position: Vec2::new(1., 2.),
                size: Vec2::new(2., 2.)
            }
        );

        let rect = RoundRect::from_points(point1, point2, radius);
        assert_approx_eq!(
            rect,
            RoundRect {
                position: Vec2::new(1., 2.),
                size: Vec2::new(2., 2.),
                radius: Vec2::new(1., 0.5)
            }
        );
    }

    #[test]
    fn test_rect_position() {
        let position = Vec2::new(1., 2.);
        let size = Vec2::new(3., 4.);
        let radius = Vec2::new(2., 0.5);

        let rect = Rect { position, size };
        assert_approx_eq!(rect.position(), Vec2::new(1., 2.));

        let rect = RoundRect {
            position,
            size,
            radius,
        };
        assert_approx_eq!(rect.position(), Vec2::new(1., 2.));
    }

    #[test]
    fn test_rect_size() {
        let position = Vec2::new(1., 2.);
        let size = Vec2::new(3., 4.);
        let radius = Vec2::new(2., 0.5);

        let rect = Rect { position, size };
        assert_approx_eq!(rect.size(), Vec2::new(3., 4.));

        let rect = RoundRect {
            position,
            size,
            radius,
        };
        assert_approx_eq!(rect.size(), Vec2::new(3., 4.));
    }

    #[test]
    fn test_rect_center() {
        let position = Vec2::new(1., 2.);
        let size = Vec2::new(3., 4.);
        let radius = Vec2::new(2., 0.5);

        let rect = Rect { position, size };
        assert_approx_eq!(rect.center(), Vec2::new(2.5, 4.));

        let rect = RoundRect {
            position,
            size,
            radius,
        };
        assert_approx_eq!(rect.center(), Vec2::new(2.5, 4.));
    }

    #[test]
    fn test_roundrect_radius() {
        let position = Vec2::new(1., 2.);
        let size = Vec2::new(3., 4.);
        let radius = Vec2::new(2., 0.5);

        let rect = RoundRect {
            position,
            size,
            radius,
        };
        assert_approx_eq!(rect.radius(), Vec2::new(2., 0.5));
    }

    #[test]
    fn test_rect_eq() {
        let position = Vec2::new(1., 2.);
        let size = Vec2::new(3., 4.);
        let radius = Vec2::new(2., 0.5);

        let rect = Rect { position, size };
        assert_eq!(rect, Rect { position, size });
        assert_ne!(
            rect,
            Rect {
                position: -position,
                size
            }
        );

        let rect = RoundRect {
            position,
            size,
            radius,
        };
        assert_eq!(
            rect,
            RoundRect {
                position,
                size,
                radius
            }
        );
        assert_ne!(
            rect,
            RoundRect {
                position: size,
                size: position,
                radius
            }
        );
    }

    #[test]
    fn test_rect_into() {
        let position = Vec2::new(1., 2.);
        let size = Vec2::new(3., 4.);
        let radius = Vec2::new(2., 0.5);

        let rect: (f64, f64, f64, f64) = Rect { position, size }.into();
        assert_eq!(rect, (1., 2., 3., 4.));

        let rect: (f64, f64, f64, f64, f64, f64) = RoundRect {
            position,
            size,
            radius,
        }
        .into();
        assert_eq!(rect, (1., 2., 3., 4., 2., 0.5));
    }

    #[test]
    fn test_rect_from() {
        let rect: Rect = (1., 2., 3., 4.).into();
        assert_approx_eq!(
            rect,
            Rect {
                position: Vec2::new(1., 2.),
                size: Vec2::new(3., 4.)
            }
        );

        let rect: RoundRect = (1., 2., 3., 4., 1., 0.5).into();
        assert_approx_eq!(
            rect,
            RoundRect {
                position: Vec2::new(1., 2.),
                size: Vec2::new(3., 4.),
                radius: Vec2::new(1., 0.5)
            }
        );
    }

    #[test]
    fn test_rect_mul_vector() {
        let position = Vec2::new(2., 4.5);
        let size = Vec2::new(0.5, 1.8);
        let radius = Vec2::new(1., 0.9);

        let scale = Vec2::new(1.2, 1. / 9.);

        let rect = Rect { position, size };
        assert_approx_eq!(
            rect * scale,
            Rect {
                position: Vec2::new(2.4, 0.5),
                size: Vec2::new(0.6, 0.2)
            }
        );

        let rect = RoundRect {
            position,
            size,
            radius,
        };
        assert_approx_eq!(
            rect * scale,
            RoundRect {
                position: Vec2::new(2.4, 0.5),
                size: Vec2::new(0.6, 0.2),
                radius: Vec2::new(0.3, 0.1)
            }
        );
    }

    #[test]
    fn test_rect_mul_assign_vector() {
        let position = Vec2::new(3., 2.3);
        let size = Vec2::new(0.5, 5.75);
        let radius = Vec2::new(2., 2.3);

        let scale = Vec2::new(-1., 1. / 2.3);

        let mut rect = Rect { position, size };
        rect *= scale;
        assert_approx_eq!(
            rect,
            Rect {
                position: Vec2::new(-3.5, 1.),
                size: Vec2::new(0.5, 2.5)
            }
        );

        let mut rect = RoundRect {
            position,
            size,
            radius,
        };
        rect *= scale;
        assert_approx_eq!(
            rect,
            RoundRect {
                position: Vec2::new(-3.5, 1.),
                size: Vec2::new(0.5, 2.5),
                radius: Vec2::new(0.25, 1.)
            }
        );
    }

    #[test]
    fn test_rect_mul_f64() {
        let position = Vec2::new(2., 4.5);
        let size = Vec2::new(1.5, 2.);
        let radius = Vec2::new(1., 0.9);

        let rect = Rect { position, size };
        assert_approx_eq!(
            rect * 2.,
            Rect {
                position: Vec2::new(4., 9.),
                size: Vec2::new(3., 4.)
            }
        );
        assert_approx_eq!(
            2. * rect,
            Rect {
                position: Vec2::new(4., 9.),
                size: Vec2::new(3., 4.)
            }
        );

        let rect = RoundRect {
            position,
            size,
            radius,
        };
        assert_approx_eq!(
            rect * 2.,
            RoundRect {
                position: Vec2::new(4., 9.),
                size: Vec2::new(3., 4.),
                radius: Vec2::new(1.5, 1.8)
            }
        );
        assert_approx_eq!(
            2. * rect,
            RoundRect {
                position: Vec2::new(4., 9.),
                size: Vec2::new(3., 4.),
                radius: Vec2::new(1.5, 1.8)
            }
        );
    }

    #[test]
    fn test_rect_mul_assign_f64() {
        let position = Vec2::new(3., 2.3);
        let size = Vec2::new(5. / 7., 1.);
        let radius = Vec2::new(2., 2.3);

        let mut rect = Rect { position, size };
        rect *= 1.4;
        assert_approx_eq!(
            rect,
            Rect {
                position: Vec2::new(4.2, 3.22),
                size: Vec2::new(1., 1.4)
            }
        );

        let mut rect = RoundRect {
            position,
            size,
            radius,
        };
        rect *= 1.4;
        assert_approx_eq!(
            rect,
            RoundRect {
                position: Vec2::new(4.2, 3.22),
                size: Vec2::new(1., 1.4),
                radius: Vec2::new(0.5, 0.7)
            }
        );
    }

    #[test]
    fn test_rect_div_vector() {
        let position = Vec2::new(2., 4.5);
        let size = Vec2::new(6., 3.6);
        let radius = Vec2::new(1., 0.9);

        let scale = Vec2::new(1.2, 9.);

        let rect = Rect { position, size };
        assert_approx_eq!(
            rect / scale,
            Rect {
                position: Vec2::new(5. / 3., 0.5),
                size: Vec2::new(5., 0.4)
            }
        );

        let rect = RoundRect {
            position,
            size,
            radius,
        };
        assert_approx_eq!(
            rect / scale,
            RoundRect {
                position: Vec2::new(5. / 3., 0.5),
                size: Vec2::new(5., 0.4),
                radius: Vec2::new(5. / 6., 0.1)
            }
        );
    }

    #[test]
    fn test_rect_div_assign_vector() {
        let position = Vec2::new(3., 2.3);
        let size = Vec2::new(2., 0.322);
        let radius = Vec2::new(1., 0.161);

        let scale = Vec2::new(-1., 0.23);

        let mut rect = Rect { position, size };
        rect /= scale;
        assert_approx_eq!(
            rect,
            Rect {
                position: Vec2::new(-5., 10.),
                size: Vec2::new(2., 1.4)
            }
        );

        let mut rect = RoundRect {
            position,
            size,
            radius,
        };
        rect /= scale;
        assert_approx_eq!(
            rect,
            RoundRect {
                position: Vec2::new(-5., 10.),
                size: Vec2::new(2., 1.4),
                radius: Vec2::new(1., 0.7)
            }
        );
    }

    #[test]
    fn test_rect_div_f64() {
        let position = Vec2::new(2., 4.5);
        let size = Vec2::new(0.4, 9.);
        let radius = Vec2::new(1., 0.6);

        let rect = Rect { position, size };
        assert_approx_eq!(
            rect / 2.,
            Rect {
                position: Vec2::new(1., 2.25),
                size: Vec2::new(0.2, 4.5)
            }
        );

        let rect = RoundRect {
            position,
            size,
            radius,
        };
        assert_approx_eq!(
            rect / 2.,
            RoundRect {
                position: Vec2::new(1., 2.25),
                size: Vec2::new(0.2, 4.5),
                radius: Vec2::new(0.1, 0.3)
            }
        );
    }

    #[test]
    fn test_rect_div_assign_f64() {
        let position = Vec2::new(3., 2.25);
        let size = Vec2::new(1.5, 1.);
        let radius = Vec2::new(0.6, 0.45);

        let mut rect = Rect { position, size };
        rect /= 1.5;
        assert_approx_eq!(
            rect,
            Rect {
                position: Vec2::new(2., 1.5),
                size: Vec2::new(1., 2. / 3.)
            }
        );

        let mut rect = RoundRect {
            position,
            size,
            radius,
        };
        rect /= 1.5;
        assert_approx_eq!(
            rect,
            RoundRect {
                position: Vec2::new(2., 1.5),
                size: Vec2::new(1., 2. / 3.),
                radius: Vec2::new(0.4, 0.3)
            }
        );
    }
}
