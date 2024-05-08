use crate::{Angle, Circle, ExtVec, Length, Path, Rect, RoundRect, Size, Vector};

/// Trait to allow conversion of primitive shapes to a [`Path`]
#[allow(clippy::module_name_repetitions)] // rust-lang/rust-clippy#8524
pub trait ToPath<U> {
    /// Convert a primitive shape to a [`Path`]
    fn to_path(self) -> Path<U>;
}

impl<U> ToPath<U> for Circle<U> {
    #[inline]
    fn to_path(self) -> Path<U> {
        let radius = self.radius.get();

        let mut builder = Path::builder_with_capacity(4);
        builder.abs_move(self.center - Size::new(radius, 0.0));
        builder.rel_arc(
            Vector::splat(radius),
            Angle::zero(),
            false,
            true,
            Vector::new(2.0 * radius, 0.0),
        );
        builder.rel_arc(
            Vector::splat(radius),
            Angle::zero(),
            false,
            true,
            Vector::new(-2.0 * radius, 0.0),
        );
        builder.close();

        builder.build()
    }
}

impl<U> ToPath<U> for Rect<U> {
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

impl<U> ToPath<U> for RoundRect<U> {
    #[inline]
    fn to_path(self) -> Path<U> {
        let radius = self.radius.get();
        let radii = Vector::splat(radius);

        let mut builder = Path::builder_with_capacity(9);
        builder.abs_move(self.min + Size::new(0.0, radius));
        builder.rel_arc(radii, Angle::zero(), false, true, radii.neg_y());
        builder.abs_horiz_line(Length::new(self.max.x - radius));
        builder.rel_arc(radii, Angle::zero(), false, true, radii);
        builder.abs_vert_line(Length::new(self.max.y - radius));
        builder.rel_arc(radii, Angle::zero(), false, true, radii.neg_x());
        builder.abs_horiz_line(Length::new(self.min.x + radius));
        builder.rel_arc(radii, Angle::zero(), false, true, -radii);
        builder.close();

        builder.build()
    }
}
