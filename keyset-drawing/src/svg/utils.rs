use std::fmt;

use geom::{Dot, Path, PathSegment, Point, Rect, Vector};

/// Format a value to an SVG representation
pub trait ToSvg {
    fn to_svg(&self) -> impl fmt::Display;
}

#[repr(transparent)]
struct FloatToSvg(f32);

impl fmt::Display for FloatToSvg {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{value}", value = (self.0 * 1e3).round() / 1e3)
    }
}

impl ToSvg for f32 {
    fn to_svg(&self) -> impl fmt::Display {
        FloatToSvg(*self)
    }
}

#[repr(transparent)]
struct DotToSvg(Dot);

impl fmt::Display for DotToSvg {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FloatToSvg(self.0 .0).fmt(f)
    }
}

impl ToSvg for Dot {
    fn to_svg(&self) -> impl fmt::Display {
        DotToSvg(*self)
    }
}

#[repr(transparent)]
struct PathSegmentToSvg(PathSegment<Dot>);

impl fmt::Display for PathSegmentToSvg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            PathSegment::Move(Point { x, y }) => {
                write!(f, "M{}{}", x.to_svg(), y.to_svg_spaced())
            }
            PathSegment::Line(Vector { x, y }) => {
                write!(f, "l{}{}", x.to_svg(), y.to_svg_spaced())
            }
            PathSegment::CubicBezier(
                Vector { x: x1, y: y1 },
                Vector { x: x2, y: y2 },
                Vector { x, y },
            ) => {
                write!(
                    f,
                    "c{}{}{}{}{}{}",
                    x1.to_svg(),
                    y1.to_svg_spaced(),
                    x2.to_svg_spaced(),
                    y2.to_svg_spaced(),
                    x.to_svg_spaced(),
                    y.to_svg_spaced()
                )
            }
            PathSegment::QuadraticBezier(Vector { x: x1, y: y1 }, Vector { x, y }) => {
                write!(
                    f,
                    "q{}{}{}{}",
                    x1.to_svg(),
                    y1.to_svg_spaced(),
                    x.to_svg_spaced(),
                    y.to_svg_spaced()
                )
            }
            PathSegment::Close => {
                write!(f, "z")
            }
        }
    }
}

impl ToSvg for PathSegment<Dot> {
    fn to_svg(&self) -> impl fmt::Display {
        PathSegmentToSvg(*self)
    }
}

#[repr(transparent)]
struct PathToSvg<'a>(&'a Path<Dot>);

impl fmt::Display for PathToSvg<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.data.iter().try_for_each(|seg| seg.to_svg().fmt(f))
    }
}

impl ToSvg for Path<Dot> {
    fn to_svg(&self) -> impl fmt::Display {
        PathToSvg(self)
    }
}

/// Similar to `ToSvg`, but formats with either a leading ' ' or '-' depending on the sign. This is
/// used for efficiently formatting path data
trait ToSvgSpaced {
    fn to_svg_spaced(&self) -> impl fmt::Display;
}

#[repr(transparent)]
struct DotToSvgSpaced(Dot);

impl fmt::Display for DotToSvgSpaced {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dot = self.0;
        write!(
            f,
            "{}{}",
            if dot.0.is_sign_positive() { " " } else { "" },
            DotToSvg(dot),
        )
    }
}

impl ToSvgSpaced for Dot {
    fn to_svg_spaced(&self) -> impl fmt::Display {
        DotToSvgSpaced(*self)
    }
}

/// Used to format the value of the viewBox attribute
pub trait ToViewBox {
    fn to_view_box(&self) -> impl fmt::Display;
}

#[repr(transparent)]
pub struct RectToViewBox(Rect<Dot>);

impl fmt::Display for RectToViewBox {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            DotToSvg(self.0.min.x),
            DotToSvgSpaced(self.0.min.y),
            DotToSvgSpaced(self.0.width()),
            DotToSvgSpaced(self.0.height()),
        )
    }
}

impl ToViewBox for Rect<Dot> {
    fn to_view_box(&self) -> impl fmt::Display {
        RectToViewBox(*self)
    }
}
