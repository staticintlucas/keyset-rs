use std::ops;

use isclose::IsClose;

use crate::{FromUnit, IntoUnit as _, Unit};

/// A 2 dimensional vector
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Vector<U: Unit> {
    pub(crate) x: U,
    pub(crate) y: U,
}

impl<U> Vector<U>
where
    U: Unit,
{
    /// Create a new vector
    #[inline]
    pub const fn new(x: U, y: U) -> Self {
        Self { x, y }
    }

    /// Create a vector with the same value for the `x` and `y` coordinates
    #[inline]
    pub const fn splat(v: U) -> Self {
        Self { x: v, y: v }
    }

    /// Swap the `x` and `y` coordinates of the vector
    #[inline]
    pub const fn swap_xy(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    /// Linearly interpolate between two vectors
    #[inline]
    pub fn lerp(self, other: Self, factor: f32) -> Self {
        self + (other - self) * factor
    }
}

impl<U, V> FromUnit<Vector<V>> for Vector<U>
where
    U: Unit + FromUnit<V>,
    V: Unit,
{
    #[inline]
    fn from_unit(value: Vector<V>) -> Self {
        Self {
            x: value.x.into_unit(),
            y: value.y.into_unit(),
        }
    }
}

impl<U> From<[U; 2]> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn from([x, y]: [U; 2]) -> Self {
        Self { x, y }
    }
}

impl<U> From<(U, U)> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn from((x, y): (U, U)) -> Self {
        Self { x, y }
    }
}

impl<U> From<Vector<U>> for [U; 2]
where
    U: Unit,
{
    #[inline]
    fn from(value: Vector<U>) -> Self {
        [value.x, value.y]
    }
}

impl<U> From<Vector<U>> for (U, U)
where
    U: Unit,
{
    #[inline]
    fn from(value: Vector<U>) -> Self {
        (value.x, value.y)
    }
}

impl<U> ops::Add<Self> for Vector<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<U> ops::AddAssign<Self> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<U> ops::Sub<Self> for Vector<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<U> ops::SubAssign<Self> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<U> ops::Mul<f32> for Vector<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<U> ops::Div<f32> for Vector<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<U> ops::DivAssign<f32> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<U> ops::Neg for Vector<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<U> IsClose<f32> for Vector<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <U as IsClose<f32>>::ABS_TOL;
    const REL_TOL: f32 = <U as IsClose<f32>>::REL_TOL;

    #[inline]
    fn is_close_tol(
        &self,
        other: impl std::borrow::Borrow<Self>,
        rel_tol: impl std::borrow::Borrow<f32>,
        abs_tol: impl std::borrow::Borrow<f32>,
    ) -> bool {
        let (other, rel_tol, abs_tol) = (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.x.is_close_tol(other.x, rel_tol, abs_tol)
            && self.y.is_close_tol(other.y, rel_tol, abs_tol)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{Inch, Mm};

    use super::*;

    #[test]
    fn vector_new() {
        let vector = Vector::new(Mm(2.0), Mm(3.0));
        assert_is_close!(vector.x, Mm(2.0));
        assert_is_close!(vector.y, Mm(3.0));
    }

    #[test]
    fn vector_splat() {
        let vector = Vector::splat(Mm(2.0));
        assert_is_close!(vector.x, Mm(2.0));
        assert_is_close!(vector.y, Mm(2.0));
    }

    #[test]
    fn vector_swap_xy() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        }
        .swap_xy();
        assert_is_close!(vector.x, Mm(3.0));
        assert_is_close!(vector.y, Mm(2.0));
    }

    #[test]
    fn vector_lerp() {
        let start = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        let end = Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };

        assert_is_close!(start.lerp(end, 0.0).x, Mm(2.0));
        assert_is_close!(start.lerp(end, 0.0).y, Mm(3.0));

        assert_is_close!(start.lerp(end, 0.5).x, Mm(1.5));
        assert_is_close!(start.lerp(end, 0.5).y, Mm(1.75));

        assert_is_close!(start.lerp(end, 1.0).x, Mm(1.0));
        assert_is_close!(start.lerp(end, 1.0).y, Mm(0.5));
    }

    #[test]
    fn vector_from_unit() {
        let vector = Vector::<Mm>::from_unit(Vector {
            x: Inch(0.75),
            y: Inch(1.0),
        });
        assert_is_close!(vector.x, Mm(19.05));
        assert_is_close!(vector.y, Mm(25.4));
    }

    #[test]
    fn vector_from_array() {
        let vector = Vector::from([Mm(2.0), Mm(3.0)]);
        assert_is_close!(vector.x, Mm(2.0));
        assert_is_close!(vector.y, Mm(3.0));
    }

    #[test]
    fn vector_from_tuple() {
        let vector = Vector::from((Mm(2.0), Mm(3.0)));
        assert_is_close!(vector.x, Mm(2.0));
        assert_is_close!(vector.y, Mm(3.0));
    }

    #[test]
    fn vector_into_array() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        let [x, y] = vector.into();
        assert_is_close!(x, Mm(2.0));
        assert_is_close!(y, Mm(3.0));
    }

    #[test]
    fn vector_into_tuple() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        let (x, y) = vector.into();
        assert_is_close!(x, Mm(2.0));
        assert_is_close!(y, Mm(3.0));
    }

    #[test]
    fn vector_add() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } + Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(vector.x, Mm(3.0));
        assert_is_close!(vector.y, Mm(3.5));
    }

    #[test]
    fn vector_add_assign() {
        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector += Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(vector.x, Mm(3.0));
        assert_is_close!(vector.y, Mm(3.5));
    }

    #[test]
    fn vector_sub() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } - Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(vector.x, Mm(1.0));
        assert_is_close!(vector.y, Mm(2.5));
    }

    #[test]
    fn vector_sub_assign() {
        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector -= Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(vector.x, Mm(1.0));
        assert_is_close!(vector.y, Mm(2.5));
    }

    #[test]
    fn vector_mul_f32() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } * 1.5;
        assert_is_close!(vector.x, Mm(3.0));
        assert_is_close!(vector.y, Mm(4.5));

        // TODO: see comment by Unit
        // let vector = 1.5 * Vector{ x: Mm(2.0), y: Mm(3.0) };
        // assert_is_close!(vector.x, Mm(3.0));
        // assert_is_close!(vector.y, Mm(4.5));
    }

    #[test]
    fn vector_mul_assign_f32() {
        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector *= 1.5;
        assert_is_close!(vector.x, Mm(3.0));
        assert_is_close!(vector.y, Mm(4.5));
    }

    #[test]
    fn vector_div_f32() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } / 1.5;
        assert_is_close!(vector.x, Mm(4.0 / 3.0));
        assert_is_close!(vector.y, Mm(2.0));
    }

    #[test]
    fn vector_div_assign_f32() {
        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector /= 1.5;
        assert_is_close!(vector.x, Mm(4.0 / 3.0));
        assert_is_close!(vector.y, Mm(2.0));
    }

    #[test]
    fn vector_neg() {
        let vector = -Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        assert_is_close!(vector.x, -Mm(2.0));
        assert_is_close!(vector.y, -Mm(3.0));
    }

    #[test]
    fn vector_is_close() {
        assert!(Vector {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(Vector {
            x: Mm(4.0 * 0.5),
            y: Mm(2.0 * 1.5)
        }));
        assert!(!Vector {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(Vector {
            x: Mm(4.1 * 0.5),
            y: Mm(2.0 * 1.5)
        }));
        assert!(!Vector {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(Vector {
            x: Mm(4.0 * 0.5),
            y: Mm(2.1 * 1.5)
        }));
    }
}
