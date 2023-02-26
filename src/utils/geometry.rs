use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! vector_type {
    ($name:ident, $x:ident, $y:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            pub $x: f32,
            pub $y: f32,
        }

        impl $name {
            #[inline]
            pub const fn new($x: f32, $y: f32) -> Self {
                Self { $x, $y }
            }

            #[inline]
            pub fn min(&self, other: Self) -> Self {
                Self::new(self.$x.min(other.$x), self.$y.min(other.$y))
            }

            #[inline]
            pub fn max(&self, other: Self) -> Self {
                Self::new(self.$x.max(other.$x), self.$y.max(other.$y))
            }

            #[inline]
            pub fn abs(&self) -> f32 {
                (self.$x * self.$x + self.$y * self.$y).sqrt()
            }

            #[inline]
            pub fn arg(&self) -> f32 {
                f32::atan2(self.$y, self.$x)
            }

            pub fn rotate(self, angle: f32) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::new(self.$x * cos - self.$y * sin, self.$x * sin + self.$y * cos)
            }
        }

        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.$x == other.$x && self.$y == other.$y
            }
        }

        impl Neg for $name {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                Self::Output::new(-self.$x, -self.$y)
            }
        }

        impl From<$name> for (f32, f32) {
            fn from(vector: $name) -> Self {
                (vector.$x, vector.$y)
            }
        }

        impl From<(f32, f32)> for $name {
            fn from(tuple: (f32, f32)) -> Self {
                Self::new(tuple.0, tuple.1)
            }
        }
    };

    ($name:ident, $x:ident, $y:ident, diff_no_dup=($diff:ident, $dx:ident, $dy:ident)) => {
        vector_type!($name, $x, $y);

        impl Add<$diff> for $name {
            type Output = Self;

            #[inline]
            fn add(self, other: $diff) -> Self::Output {
                Self::Output::new(self.$x + other.$dx, self.$y + other.$dy)
            }
        }

        impl AddAssign<$diff> for $name {
            #[inline]
            fn add_assign(&mut self, other: $diff) {
                self.$x += other.$dx;
                self.$y += other.$dy;
            }
        }

        impl Sub<$diff> for $name {
            type Output = $name;

            #[inline]
            fn sub(self, other: $diff) -> Self::Output {
                Self::Output::new(self.$x - other.$dx, self.$y - other.$dy)
            }
        }

        impl SubAssign<$diff> for $name {
            #[inline]
            fn sub_assign(&mut self, other: $diff) {
                self.$x -= other.$dx;
                self.$y -= other.$dy;
            }
        }

        impl<T: Copy> Mul<T> for $name
        where
            f32: Mul<T, Output = f32>,
        {
            type Output = Self;

            #[inline]
            fn mul(self, scale: T) -> Self::Output {
                Self::Output::new(self.$x * scale, self.$y * scale)
            }
        }

        impl<T: Copy> MulAssign<T> for $name
        where
            f32: MulAssign<T>,
        {
            #[inline]
            fn mul_assign(&mut self, scale: T) {
                self.$x *= scale;
                self.$y *= scale;
            }
        }

        impl Mul<Scale> for $name {
            type Output = Self;

            #[inline]
            fn mul(self, scale: Scale) -> Self::Output {
                Self::Output::new(self.$x * scale.x, self.$y * scale.y)
            }
        }

        impl MulAssign<Scale> for $name {
            #[inline]
            fn mul_assign(&mut self, scale: Scale) {
                self.$x *= scale.x;
                self.$y *= scale.y;
            }
        }

        impl<T: Copy> Div<T> for $name
        where
            f32: Div<T, Output = f32>,
        {
            type Output = Self;

            #[inline]
            fn div(self, scale: T) -> Self::Output {
                Self::Output::new(self.$x / scale, self.$y / scale)
            }
        }

        impl<T: Copy> DivAssign<T> for $name
        where
            f32: DivAssign<T>,
        {
            #[inline]
            fn div_assign(&mut self, scale: T) {
                self.$x /= scale;
                self.$y /= scale;
            }
        }

        impl Div<Scale> for $name {
            type Output = Self;

            #[inline]
            fn div(self, scale: Scale) -> Self::Output {
                Self::Output::new(self.$x / scale.x, self.$y / scale.y)
            }
        }

        impl DivAssign<Scale> for $name {
            #[inline]
            fn div_assign(&mut self, scale: Scale) {
                self.$x /= scale.x;
                self.$y /= scale.y;
            }
        }

        impl Div<$name> for $name {
            type Output = Scale;

            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                Self::Output::new(self.$x / rhs.$x, self.$y / rhs.$y)
            }
        }
    };

    ($name:ident, $x:ident, $y:ident, diff=Self) => {
        vector_type!($name, $x, $y, diff_no_dup = ($name, $x, $y));
    };

    ($name:ident, $x:ident, $y:ident, diff=($diff:ident, $dx:ident, $dy:ident)) => {
        vector_type!($name, $x, $y, diff_no_dup = ($diff, $dx, $dy));

        impl Sub<$name> for $name {
            type Output = $diff;

            #[inline]
            fn sub(self, other: Self) -> Self::Output {
                Self::Output::new(self.$x - other.$x, self.$y - other.$y)
            }
        }
    };
}

macro_rules! rect_type {
    ($name:ident, $(($x:ident, $y:ident)),+) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            $(pub $x: f32, pub $y: f32, )+
        }

        impl $name {
            #[inline]
            pub const fn position(&self) -> Point {
                Point::new(self.x, self.y)
            }

            #[inline]
            pub const fn size(&self) -> Size {
                Size::new(self.w, self.h)
            }

            #[inline]
            pub fn center(&self) -> Point {
                Point::new(self.x + self.w / 2., self.y + self.h / 2.)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                $(self.$x == other.$x && self.$y == other.$y &&)+
                true
            }
        }

        impl<T: Copy> Mul<T> for $name
        where
            f32: Mul<T, Output = f32>,
        {
            type Output = Self;

            #[inline]
            fn mul(self, scale: T) -> Self::Output {
                // The call to new will ensure that scale < 0 is handled correctly
                Self::Output::new(
                    $(self.$x * scale, self.$y * scale, )+
                )
            }
        }

        impl<T: Copy> MulAssign<T> for $name
        where
            f32: MulAssign<T>,
        {
            #[inline]
            fn mul_assign(&mut self, scale: T) {
                $(self.$x *= scale; self.$y *= scale;)+

                // The call to new will ensure that scale < 0 is handled correctly
                *self = Self::new(
                    $(self.$x, self.$y, )+
                )
            }
        }

        impl Mul<Scale> for $name {
            type Output = Self;

            #[inline]
            fn mul(self, scale: Scale) -> Self::Output {
                // The call to new will ensure that scale < 0 is handled correctly
                Self::Output::new(
                    $(self.$x * scale.x, self.$y * scale.y, )+
                )
            }
        }

        impl MulAssign<Scale> for $name {
            #[inline]
            fn mul_assign(&mut self, scale: Scale) {
                // Use Mul so that it will ensure that scale < 0 is handled correctly
                *self = *self * scale;
            }
        }

        impl<T: Copy> Div<T> for $name
        where
            f32: Div<T, Output = f32>,
        {
            type Output = Self;

            #[inline]
            fn div(self, scale: T) -> Self::Output {
                // The call to new will ensure that scale < 0 is handled correctly
                Self::Output::new(
                    $(self.$x / scale, self.$y / scale, )+
                )
            }
        }

        impl<T: Copy> DivAssign<T> for $name
        where
            f32: DivAssign<T>,
        {
            #[inline]
            fn div_assign(&mut self, scale: T) {
                $(self.$x /= scale; self.$y /= scale;)+

                // The call to new will ensure that scale < 0 is handled correctly
                *self = Self::new(
                    $(self.$x, self.$y, )+
                )
            }
        }

        impl Div<Scale> for $name {
            type Output = Self;

            #[inline]
            fn div(self, scale: Scale) -> Self::Output {
                // The call to new will ensure that scale < 0 is handled correctly
                Self::Output::new(
                    $(self.$x / scale.x, self.$y / scale.y, )+
                )
            }
        }

        impl DivAssign<Scale> for $name {
            #[inline]
            fn div_assign(&mut self, scale: Scale) {
                // Use Mul so that it will ensure that scale < 0 is handled correctly
                *self = *self / scale;
            }
        }
    }
}

vector_type!(Point, x, y, diff = (Size, w, h));
vector_type!(Size, w, h, diff = Self);
vector_type!(Scale, x, y);

rect_type!(Rect, (x, y), (w, h));
rect_type!(RoundRect, (x, y), (w, h), (rx, ry));

impl Rect {
    #[inline]
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        let (x, w) = if w < 0. { (x + w, -w) } else { (x, w) };
        let (y, h) = if h < 0. { (y + h, -h) } else { (y, h) };

        Self { x, y, w, h }
    }

    #[inline]
    pub fn from_point_and_size(point: Point, size: Size) -> Self {
        Self::new(point.x, point.y, size.w, size.h)
    }

    #[inline]
    pub fn from_points(point1: Point, point2: Point) -> Self {
        let (w, h) = (point2 - point1).into();
        Self::new(point1.x, point1.y, w, h)
    }
}

impl From<Rect> for (f32, f32, f32, f32) {
    fn from(rect: Rect) -> (f32, f32, f32, f32) {
        (rect.x, rect.y, rect.w, rect.h)
    }
}

impl From<(f32, f32, f32, f32)> for Rect {
    fn from(tuple: (f32, f32, f32, f32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl RoundRect {
    #[inline]
    pub fn new(x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32) -> Self {
        let (x, w) = if w < 0. { (x + w, -w) } else { (x, w) };
        let (y, h) = if h < 0. { (y + h, -h) } else { (y, h) };
        let (rx, ry) = (rx.abs().min(w / 2.), ry.abs().min(h / 2.));

        Self { x, y, w, h, rx, ry }
    }

    #[inline]
    pub fn from_point_and_size(point: Point, size: Size, radius: Size) -> Self {
        Self::new(point.x, point.y, size.w, size.h, radius.w, radius.h)
    }

    #[inline]
    pub fn from_points(point1: Point, point2: Point, radius: Size) -> Self {
        let (w, h) = (point2 - point1).into();
        Self::new(point1.x, point1.y, w, h, radius.w, radius.h)
    }
}

impl From<RoundRect> for (f32, f32, f32, f32, f32, f32) {
    fn from(rect: RoundRect) -> (f32, f32, f32, f32, f32, f32) {
        (rect.x, rect.y, rect.w, rect.h, rect.rx, rect.ry)
    }
}

impl From<(f32, f32, f32, f32, f32, f32)> for RoundRect {
    fn from(tuple: (f32, f32, f32, f32, f32, f32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    // this is required for assert_approx_eq to work
    impl Sub<Rect> for Rect {
        type Output = Size;

        fn sub(self, other: Self) -> Size {
            Size::new(
                ((self.x - other.x).powi(2) + (self.w - other.w).powi(2)).sqrt(),
                ((self.y - other.y).powi(2) + (self.h - other.h).powi(2)).sqrt(),
            )
        }
    }

    // this is required for assert_approx_eq to work
    impl Sub<RoundRect> for RoundRect {
        type Output = Size;

        fn sub(self, other: Self) -> Size {
            Size::new(
                ((self.x - other.x).powi(2)
                    + (self.w - other.w).powi(2)
                    + (self.rx - other.rx).powi(2))
                .sqrt(),
                ((self.y - other.y).powi(2)
                    + (self.h - other.h).powi(2)
                    + (self.ry - other.ry).powi(2))
                .sqrt(),
            )
        }
    }

    #[test]
    fn test_vector_new() {
        let point = Point::new(2., 4.5);

        assert_approx_eq!(point.x, 2.);
        assert_approx_eq!(point.y, 4.5);
    }

    #[test]
    fn test_vector_min() {
        let point1 = Point::new(2., 4.5);
        let point2 = Point::new(3., 0.2);

        let min = point1.min(point2);

        assert_approx_eq!(min.x, 2.);
        assert_approx_eq!(min.y, 0.2);
    }

    #[test]
    fn test_vector_max() {
        let point1 = Point::new(2., 4.5);
        let point2 = Point::new(3., 0.2);

        let min = point1.max(point2);

        assert_approx_eq!(min.x, 3.);
        assert_approx_eq!(min.y, 4.5);
    }

    #[test]
    fn test_vector_eq() {
        let point1 = Point::new(1., 3.);
        let point2 = Point::new(1., 0.2);
        let point3 = Point::new(1., 3.);

        assert_ne!(point1, point2);
        assert_eq!(point1, point3);
    }

    #[test]
    fn test_vector_neg() {
        let point = Point::new(1., 3.);

        assert_approx_eq!(-point, Point::new(-1., -3.));
    }

    #[test]
    fn test_vector_into() {
        let point: (f32, f32) = Point::new(1., 3.).into();

        assert_eq!(point, (1., 3.));
    }

    #[test]
    fn test_vector_from() {
        let point: Point = (1., 3.).into();

        assert_eq!(point, Point::new(1., 3.));
    }

    #[test]
    fn test_vector_add() {
        let point = Point::new(2., 4.5);
        let size = Size::new(3., 0.2);

        assert_approx_eq!(point + size, Point::new(5., 4.7));
    }

    #[test]
    fn test_vector_add_assign() {
        let mut point = Point::new(3., 2.3);
        let size = Size::new(1., 1.5);

        point += size;

        assert_approx_eq!(point, Point::new(4., 3.8));
    }

    #[test]
    fn test_vector_sub() {
        let point = Point::new(2., 4.5);
        let size = Size::new(3., 0.2);

        assert_approx_eq!(point - size, Point::new(-1., 4.3));
    }

    #[test]
    fn test_vector_sub_assign() {
        let mut point = Point::new(3., 2.3);
        let size = Size::new(1., 1.5);

        point -= size;

        assert_approx_eq!(point, Point::new(2., 0.8));
    }

    #[test]
    fn test_vector_mul_f32() {
        let point = Point::new(2., 4.5);

        assert_approx_eq!(point * 2., Point::new(4., 9.));
    }

    #[test]
    fn test_vector_mul_assign_f32() {
        let mut point = Point::new(3., 2.3);

        point *= 1.4;

        assert_approx_eq!(point, Point::new(4.2, 3.22));
    }

    #[test]
    fn test_vector_mul_scale() {
        let point = Point::new(2., 4.5);
        let scale = Scale::new(1.2, 1. / 9.);

        assert_approx_eq!(point * scale, Point::new(2.4, 0.5));
    }

    #[test]
    fn test_vector_mul_assign_scale() {
        let mut point = Point::new(3., 2.3);
        let scale = Scale::new(-1., 1. / 2.3);

        point *= scale;

        assert_approx_eq!(point, Point::new(-3., 1.));
    }

    #[test]
    fn test_vector_div_f32() {
        let point = Point::new(2., 4.5);

        assert_approx_eq!(point / 2., Point::new(1., 2.25));
    }

    #[test]
    fn test_vector_div_assign_f32() {
        let mut point = Point::new(3., 2.25);

        point /= 1.5;

        assert_approx_eq!(point, Point::new(2., 1.5));
    }

    #[test]
    fn test_vector_div_scale() {
        let point = Point::new(2., 4.5);
        let scale = Scale::new(1.2, 9.);

        assert_approx_eq!(point / scale, Point::new(5. / 3., 0.5));
    }

    #[test]
    fn test_vector_div_assign_scale() {
        let mut point = Point::new(3., 2.3);
        let scale = Scale::new(-1., 0.23);

        point /= scale;

        assert_approx_eq!(point, Point::new(-3., 10.));
    }

    #[test]
    fn test_rect_new() {
        let rect1 = Rect::new(1., 2., 3., 4.);
        let rect2 = RoundRect::new(1., 2., -2., -3., 0.1, 1.6);

        assert_approx_eq!(rect1.x, 1.);
        assert_approx_eq!(rect1.y, 2.);
        assert_approx_eq!(rect1.w, 3.);
        assert_approx_eq!(rect1.h, 4.);

        assert_approx_eq!(rect2.x, -1.);
        assert_approx_eq!(rect2.y, -1.);
        assert_approx_eq!(rect2.w, 2.);
        assert_approx_eq!(rect2.h, 3.);
        assert_approx_eq!(rect2.rx, 0.1);
        assert_approx_eq!(rect2.ry, 1.5);
    }

    #[test]
    fn test_rect_from_point_and_size() {
        let point = Point::new(1., 2.);
        let size = Size::new(3., 4.);
        let radius = Size::new(2., 0.5);

        let rect = Rect::from_point_and_size(point, size);
        assert_approx_eq!(rect, Rect::new(1., 2., 3., 4.));

        let rect = RoundRect::from_point_and_size(point, size, radius);
        assert_approx_eq!(rect, RoundRect::new(1., 2., 3., 4., 1.5, 0.5));
    }

    #[test]
    fn test_rect_from_points() {
        let point1 = Point::new(1., 2.);
        let point2 = Point::new(3., 4.);
        let radius = Size::new(2., 0.5);

        let rect = Rect::from_points(point1, point2);
        assert_approx_eq!(rect, Rect::new(1., 2., 2., 2.));

        let rect = RoundRect::from_points(point1, point2, radius);
        assert_approx_eq!(rect, RoundRect::new(1., 2., 2., 2., 1., 0.5));
    }

    #[test]
    fn test_rect_position() {
        let rect = Rect::new(1., 2., 3., 4.);

        assert_approx_eq!(rect.position(), Point::new(1., 2.));
    }

    #[test]
    fn test_rect_size() {
        let rect = Rect::new(1., 2., 3., 4.);

        assert_approx_eq!(rect.size(), Size::new(3., 4.));
    }

    #[test]
    fn test_rect_center() {
        let rect = Rect::new(1., 2., 3., 4.);

        assert_approx_eq!(rect.center(), Point::new(2.5, 4.));
    }

    #[test]
    fn test_rect_into() {
        let rect: (f32, f32, f32, f32) = Rect::new(1., 2., 3., 4.).into();
        assert_eq!(rect, (1., 2., 3., 4.));

        let rect: (f32, f32, f32, f32, f32, f32) = RoundRect::new(1., 2., 3., 4., 1., 0.5).into();
        assert_eq!(rect, (1., 2., 3., 4., 1., 0.5));
    }

    #[test]
    fn test_rect_from() {
        let rect: Rect = (1., 2., 3., 4.).into();
        assert_eq!(rect, Rect::new(1., 2., 3., 4.));

        let rect: RoundRect = (1., 2., 3., 4., 1., 0.5).into();
        assert_eq!(rect, RoundRect::new(1., 2., 3., 4., 1., 0.5));
    }

    #[test]
    fn test_rect_mul_f32() {
        let rect = Rect::new(2., 4.5, 1.5, 2.);

        assert_approx_eq!(rect * 2., Rect::new(4., 9., 3., 4.));
    }

    #[test]
    fn test_rect_mul_assign_f32() {
        let mut rect = Rect::new(3., 2.3, 1. / 1.4, 1.);

        rect *= 1.4;

        assert_approx_eq!(rect, Rect::new(4.2, 3.22, 1., 1.4));
    }

    #[test]
    fn test_rect_mul_scale() {
        let rect = Rect::new(2., 4.5, 0.5, 1.8);
        let scale = Scale::new(1.2, 1. / 9.);

        assert_approx_eq!(rect * scale, Rect::new(2.4, 0.5, 0.6, 0.2));
    }

    #[test]
    fn test_rect_mul_assign_scale() {
        let mut rect = Rect::new(3., 2.3, 0.5, 5.75);
        let scale = Scale::new(-1., 1. / 2.3);

        rect *= scale;

        assert_approx_eq!(rect, Rect::new(-3.5, 1., 0.5, 2.5));
    }

    #[test]
    fn test_rect_div_f32() {
        let rect = Rect::new(2., 4.5, 0.4, 9.);

        assert_approx_eq!(rect / 2., Rect::new(1., 2.25, 0.2, 4.5));
    }

    #[test]
    fn test_rect_div_assign_f32() {
        let mut rect = Rect::new(3., 2.25, 1.5, 1.);

        rect /= 1.5;

        assert_approx_eq!(rect, Rect::new(2., 1.5, 1., 2. / 3.));
    }

    #[test]
    fn test_rect_div_scale() {
        let rect = Rect::new(2., 4.5, 6., 3.6);
        let scale = Scale::new(1.2, 9.);

        assert_approx_eq!(rect / scale, Rect::new(5. / 3., 0.5, 5., 0.4));
    }

    #[test]
    fn test_rect_div_assign_scale() {
        let mut rect = Rect::new(3., 2.3, 2., 0.322);
        let scale = Scale::new(-1., 0.23);

        rect /= scale;

        assert_approx_eq!(rect, Rect::new(-5., 10., 2., 1.4));
    }
}
