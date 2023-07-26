use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Self = Self::from(0.);

    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn from(value: f64) -> Self {
        Self { x: value, y: value }
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self {
            x: f64::min(self.x, other.x),
            y: f64::min(self.y, other.y),
        }
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self {
            x: f64::max(self.x, other.x),
            y: f64::max(self.y, other.y),
        }
    }

    #[inline]
    pub fn abs(self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    #[inline]
    pub fn arg(self) -> f64 {
        f64::atan2(self.y, self.x)
    }

    #[inline]
    pub fn rotate(self, angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }
}

impl PartialEq for Vec2 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<Vec2> for (f64, f64) {
    fn from(value: Vec2) -> Self {
        (value.x, value.y)
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from(value: (f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T: Copy> ops::Mul<T> for Vec2
where
    f64: ops::Mul<T, Output = f64>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl<T: Copy> ops::MulAssign<T> for Vec2
where
    f64: ops::MulAssign<T>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Copy> ops::Div<T> for Vec2
where
    f64: ops::Div<T, Output = f64>,
{
    type Output = Self;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Self::Output {
            x: self.x / scale,
            y: self.y / scale,
        }
    }
}

impl<T: Copy> ops::DivAssign<T> for Vec2
where
    f64: ops::DivAssign<T>,
{
    #[inline]
    fn div_assign(&mut self, scale: T) {
        self.x /= scale;
        self.y /= scale;
    }
}

impl ops::Add<Self> for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, other: Vec2) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign<Self> for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub<Self> for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Vec2) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign<Self> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::Mul<Self> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, scale: Vec2) -> Self::Output {
        Self::Output {
            x: self.x * scale.x,
            y: self.y * scale.y,
        }
    }
}

impl ops::MulAssign<Self> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, scale: Vec2) {
        self.x *= scale.x;
        self.y *= scale.y;
    }
}

impl ops::Div<Self> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::DivAssign<Self> for Vec2 {
    #[inline]
    fn div_assign(&mut self, scale: Vec2) {
        self.x /= scale.x;
        self.y /= scale.y;
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_vector_new() {
        let vec = Vec2::new(2., 4.5);

        assert_approx_eq!(vec.x, 2.);
        assert_approx_eq!(vec.y, 4.5);
    }

    #[test]
    fn test_vector_from_number() {
        let vec = Vec2::from(3.2);

        assert_approx_eq!(vec.x, 3.2);
        assert_approx_eq!(vec.y, 3.2);
    }

    #[test]
    fn test_vector_min() {
        let vec1 = Vec2 { x: 2., y: 4.5 };
        let vec2 = Vec2 { x: 3., y: 0.2 };

        let min = Vec2::min(vec1, vec2);

        assert_approx_eq!(min.x, 2.);
        assert_approx_eq!(min.y, 0.2);
    }

    #[test]
    fn test_vector_max() {
        let vec1 = Vec2 { x: 2., y: 4.5 };
        let vec2 = Vec2 { x: 3., y: 0.2 };

        let min = Vec2::max(vec1, vec2);

        assert_approx_eq!(min.x, 3.);
        assert_approx_eq!(min.y, 4.5);
    }

    #[test]
    fn test_vector_abs() {
        let vec = Vec2 { x: 1.5, y: 2. };

        assert_approx_eq!(vec.abs(), 2.5);
    }

    #[test]
    fn test_vector_arg() {
        let vec = Vec2 { x: 1., y: 1. };

        assert_approx_eq!(vec.arg(), PI / 4.);
    }

    #[test]
    fn test_vector_rotate() {
        let vec = Vec2 { x: 1., y: 1. };

        assert_approx_eq!(vec.rotate(PI / 2.), Vec2 { x: -1., y: 1. });
    }

    #[test]
    fn test_vector_eq() {
        let point1 = Vec2 { x: 1., y: 3. };
        let point2 = Vec2 { x: 1., y: 0.2 };
        let point3 = Vec2 { x: 1., y: 3. };

        assert_ne!(point1, point2);
        assert_eq!(point1, point3);
    }

    #[test]
    fn test_vector_neg() {
        let point = Vec2 { x: 1., y: 3. };

        assert_approx_eq!(-point, Vec2 { x: -1., y: -3. });
    }

    #[test]
    fn test_vector_into() {
        let point: (f64, f64) = Vec2 { x: 1., y: 3. }.into();

        assert_eq!(point, (1., 3.));
    }

    #[test]
    fn test_vector_from_tuple() {
        let point: Vec2 = (1., 3.).into();

        assert_eq!(point, Vec2 { x: 1., y: 3. });
    }

    #[test]
    fn test_vector_add() {
        let point = Vec2 { x: 2., y: 4.5 };
        let size = Vec2 { x: 3., y: 0.2 };

        assert_approx_eq!(point + size, Vec2 { x: 5., y: 4.7 });
    }

    #[test]
    fn test_vector_add_assign() {
        let mut point = Vec2 { x: 3., y: 2.3 };
        let size = Vec2 { x: 1., y: 1.5 };

        point += size;

        assert_approx_eq!(point, Vec2 { x: 4., y: 3.8 });
    }

    #[test]
    fn test_vector_sub() {
        let point = Vec2 { x: 2., y: 4.5 };
        let size = Vec2 { x: 3., y: 0.2 };

        assert_approx_eq!(point - size, Vec2 { x: -1., y: 4.3 });
    }

    #[test]
    fn test_vector_sub_assign() {
        let mut point = Vec2 { x: 3., y: 2.3 };
        let size = Vec2 { x: 1., y: 1.5 };

        point -= size;

        assert_approx_eq!(point, Vec2 { x: 2., y: 0.8 });
    }

    #[test]
    fn test_vector_mul() {
        let point = Vec2 { x: 2., y: 4.5 };
        let scale = Vec2 { x: 1.2, y: 1. / 9. };

        assert_approx_eq!(point * scale, Vec2 { x: 2.4, y: 0.5 });
    }

    #[test]
    fn test_vector_mul_assign() {
        let mut point = Vec2 { x: 3., y: 2.3 };
        let scale = Vec2 {
            x: -1.,
            y: 1. / 2.3,
        };

        point *= scale;

        assert_approx_eq!(point, Vec2 { x: -3., y: 1. });
    }

    #[test]
    fn test_vector_mul_f64() {
        let point = Vec2 { x: 2., y: 4.5 };

        assert_approx_eq!(point * 2., Vec2 { x: 4., y: 9. });
        assert_approx_eq!(2. * point, Vec2 { x: 4., y: 9. });
    }

    #[test]
    fn test_vector_mul_assign_f64() {
        let mut point = Vec2 { x: 3., y: 2.3 };

        point *= 1.4;

        assert_approx_eq!(point, Vec2 { x: 4.2, y: 3.22 });
    }

    #[test]
    fn test_vector_div() {
        let point = Vec2 { x: 2., y: 4.5 };
        let scale = Vec2 { x: 1.2, y: 9. };

        assert_approx_eq!(point / scale, Vec2 { x: 5. / 3., y: 0.5 });
    }

    #[test]
    fn test_vector_div_assign() {
        let mut point = Vec2 { x: 3., y: 2.3 };
        let scale = Vec2 { x: -1., y: 0.23 };

        point /= scale;

        assert_approx_eq!(point, Vec2 { x: -3., y: 10. });
    }

    #[test]
    fn test_vector_div_f64() {
        let point = Vec2 { x: 2., y: 4.5 };

        assert_approx_eq!(point / 2., Vec2 { x: 1., y: 2.25 });
    }

    #[test]
    fn test_vector_div_assign_f64() {
        let mut point = Vec2 { x: 3., y: 2.25 };

        point /= 1.5;

        assert_approx_eq!(point, Vec2 { x: 2., y: 1.5 });
    }
}
