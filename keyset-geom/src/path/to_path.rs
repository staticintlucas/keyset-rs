use crate::{Angle, Circle, ExtVec as _, Length, Path, Rect, RoundRect, Size, Unit, Vector};

/// Trait to allow conversion of primitive shapes to a [`Path`]
pub trait ToPath<U> {
    /// Convert a primitive shape to a [`Path`]
    fn to_path(self) -> Path<U>;
}

impl<U> ToPath<U> for Circle<U>
where
    U: Unit,
{
    #[inline]
    fn to_path(self) -> Path<U> {
        let radius = self.radius.get();

        let mut builder = Path::builder_with_capacity(4);
        builder.abs_move(self.center - Size::new(radius, 0.0));
        builder.rel_arc(
            Vector::splat(radius),
            Angle::ZERO,
            false,
            true,
            Vector::new(2.0 * radius, 0.0),
        );
        builder.rel_arc(
            Vector::splat(radius),
            Angle::ZERO,
            false,
            true,
            Vector::new(-2.0 * radius, 0.0),
        );
        builder.close();

        builder.build()
    }
}

impl<U> ToPath<U> for Rect<U>
where
    U: Unit,
{
    #[inline]
    fn to_path(self) -> Path<U> {
        let mut builder = Path::builder_with_capacity(5);
        builder.abs_move(self.min);
        builder.abs_horiz_line(Length::new(self.max.x));
        builder.abs_vert_line(Length::new(self.max.y));
        builder.abs_horiz_line(Length::new(self.min.x));
        builder.close();

        builder.build()
    }
}

impl<U> ToPath<U> for RoundRect<U>
where
    U: Unit,
{
    #[inline]
    fn to_path(self) -> Path<U> {
        let radius = self.radius.get();
        let radii = Vector::splat(radius);

        let mut builder = Path::builder_with_capacity(9);
        builder.abs_move(self.min + Size::new(0.0, radius));
        builder.rel_arc(radii, Angle::ZERO, false, true, radii.neg_y());
        builder.abs_horiz_line(Length::new(self.max.x - radius));
        builder.rel_arc(radii, Angle::ZERO, false, true, radii);
        builder.abs_vert_line(Length::new(self.max.y - radius));
        builder.rel_arc(radii, Angle::ZERO, false, true, radii.neg_x());
        builder.abs_horiz_line(Length::new(self.min.x + radius));
        builder.rel_arc(radii, Angle::ZERO, false, true, -radii);
        builder.close();

        builder.build()
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use super::*;
    use crate::{Mm, PathSegment, Point};

    #[test]
    fn circle_to_path() {
        let circle = Circle::<Mm>::new(Point::new(1.5, 2.0), Length::new(1.0));
        let path = circle.to_path();

        let a = (4.0 / 3.0) * Angle::degrees(90.0 / 4.0).tan();
        let exp = [
            PathSegment::<Mm>::Move(Point::new(0.5, 2.0)),
            PathSegment::CubicBezier(
                Vector::new(0.0, -a),
                Vector::new(1.0 - a, -1.0),
                Vector::new(1.0, -1.0),
            ),
            PathSegment::CubicBezier(
                Vector::new(a, 0.0),
                Vector::new(1.0, 1.0 - a),
                Vector::new(1.0, 1.0),
            ),
            PathSegment::CubicBezier(
                Vector::new(0.0, a),
                Vector::new(-(1.0 - a), 1.0),
                Vector::new(-1.0, 1.0),
            ),
            PathSegment::CubicBezier(
                Vector::new(-a, 0.0),
                Vector::new(-1.0, -(1.0 - a)),
                Vector::new(-1.0, -1.0),
            ),
            PathSegment::Close,
        ];
        let bounds = Rect::new(Point::new(0.5, 1.0), Point::new(2.5, 3.0));

        assert_eq!(path.data.len(), exp.len());
        assert_is_close!(path.bounds, bounds);
        for (el, ex) in path.data.iter().zip(exp) {
            assert_is_close!(el, ex);
        }
    }

    #[test]
    fn rect_to_path() {
        let rect = Rect::<Mm>::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        let path = rect.to_path();

        let exp = [
            PathSegment::<Mm>::Move(Point::new(1.0, 2.0)),
            PathSegment::Line(Vector::new(2.0, 0.0)),
            PathSegment::Line(Vector::new(0.0, 2.0)),
            PathSegment::Line(Vector::new(-2.0, 0.0)),
            PathSegment::Close,
        ];

        assert_eq!(path.data.len(), exp.len());
        assert_is_close!(path.bounds, rect);
        for (el, ex) in path.data.iter().zip(exp) {
            assert_is_close!(el, ex);
        }
    }

    #[test]
    fn round_rect_to_path() {
        let rect =
            RoundRect::<Mm>::new(Point::new(2.0, 4.0), Point::new(6.0, 8.0), Length::new(1.0));
        let path = rect.to_path();

        let a = (4.0 / 3.0) * Angle::degrees(90.0 / 4.0).tan();
        let exp = [
            PathSegment::<Mm>::Move(Point::new(2.0, 5.0)),
            PathSegment::CubicBezier(
                Vector::new(0.0, -a),
                Vector::new(1.0 - a, -1.0),
                Vector::new(1.0, -1.0),
            ),
            PathSegment::Line(Vector::new(2.0, 0.0)),
            PathSegment::CubicBezier(
                Vector::new(a, 0.0),
                Vector::new(1.0, 1.0 - a),
                Vector::new(1.0, 1.0),
            ),
            PathSegment::Line(Vector::new(0.0, 2.0)),
            PathSegment::CubicBezier(
                Vector::new(0.0, a),
                Vector::new(-(1.0 - a), 1.0),
                Vector::new(-1.0, 1.0),
            ),
            PathSegment::Line(Vector::new(-2.0, 0.0)),
            PathSegment::CubicBezier(
                Vector::new(-a, 0.0),
                Vector::new(-1.0, -(1.0 - a)),
                Vector::new(-1.0, -1.0),
            ),
            PathSegment::Close,
        ];

        assert_eq!(path.data.len(), exp.len());
        assert_is_close!(path.bounds, rect.rect());
        for (el, ex) in path.data.iter().zip(exp) {
            assert_is_close!(el, ex);
        }
    }
}
