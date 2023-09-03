#[cfg(feature = "kle")]
pub mod kle;

use kurbo::{Point, Rect, Size};

use color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Homing {
    Scoop,
    Bar,
    Bump,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    None,   // a.k.a. decal in KLE lingo
    Normal, // Just a regular ol' key
    Homing(Option<Homing>),
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Normal(Size),
    SteppedCaps,
    IsoVertical,
    IsoHorizontal,
}

impl From<Size> for Shape {
    fn from(value: Size) -> Self {
        Self::Normal(value)
    }
}

impl Shape {
    #[must_use]
    pub fn bounds(self) -> Rect {
        use Shape::*;
        match self {
            Normal(size) => Rect::from_origin_size((0.0, 0.0), size),
            IsoHorizontal | IsoVertical => Rect::from_origin_size((0.0, 0.0), (1.5, 2.0)),
            SteppedCaps => Rect::from_origin_size((0.0, 0.0), (1.75, 1.0)),
        }
    }

    #[must_use]
    pub fn margin(self) -> Rect {
        use Shape::*;
        match self {
            Normal(size) => Rect::from_origin_size((0.0, 0.0), size),
            SteppedCaps => Rect::from_origin_size((0.0, 0.0), (1.25, 1.0)),
            IsoVertical => Rect::from_origin_size((0.25, 0.0), (1.25, 2.0)),
            IsoHorizontal => Rect::from_origin_size((0.0, 0.0), (1.5, 1.0)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Legend {
    pub text: String,
    pub size: usize,
    pub color: Color,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Key {
    pub position: Point,
    pub shape: Shape,
    pub typ: Type,
    pub color: Color,
    pub legends: [[Option<Legend>; 3]; 3],
}

impl Key {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // Example non-blank key used in some of our tests. Set as cfg(test) to avoid dead code warnings
    pub fn example() -> Self {
        Self {
            legends: [
                [
                    Some(Legend {
                        text: "!".into(),
                        size: 4,
                        color: Color::new(0.0, 0.0, 0.0),
                    }),
                    None,
                    Some(Legend {
                        text: "¹".into(),
                        size: 4,
                        color: Color::new(0.0, 0.0, 0.0),
                    }),
                ],
                [None, None, None],
                [
                    Some(Legend {
                        text: "1".into(),
                        size: 4,
                        color: Color::new(0.0, 0.0, 0.0),
                    }),
                    None,
                    Some(Legend {
                        text: "¡".into(),
                        size: 4,
                        color: Color::new(0.0, 0.0, 0.0),
                    }),
                ],
            ],
            ..Self::default()
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            position: Point::ORIGIN,
            shape: Shape::Normal(Size::new(1., 1.)),
            typ: Type::Normal,
            color: Color::new(0.8, 0.8, 0.8),
            legends: Default::default(), // [[None; 3]; 3] won't work since Option<Legend> : !Copy
        }
    }
}

#[cfg(test)]
pub mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn shape_bounds() {
        assert_eq!(
            Shape::Normal(Size::new(2.25, 1.)).bounds(),
            Rect::new(0.0, 0.0, 2.25, 1.)
        );
        assert_eq!(Shape::IsoVertical.bounds(), Rect::new(0.0, 0.0, 1.5, 2.0));
        assert_eq!(Shape::IsoHorizontal.bounds(), Rect::new(0.0, 0.0, 1.5, 2.0));
        assert_eq!(Shape::SteppedCaps.bounds(), Rect::new(0.0, 0.0, 1.75, 1.0));
    }

    #[test]
    fn shape_margin() {
        assert_eq!(
            Shape::Normal(Size::new(2.25, 1.)).margin(),
            Rect::new(0.0, 0.0, 2.25, 1.)
        );
        assert_eq!(Shape::IsoVertical.margin(), Rect::new(0.25, 0.0, 1.5, 2.0));
        assert_eq!(Shape::IsoHorizontal.margin(), Rect::new(0.0, 0.0, 1.5, 1.0));
        assert_eq!(Shape::SteppedCaps.margin(), Rect::new(0.0, 0.0, 1.25, 1.0));
    }

    #[test]
    fn shape_from() {
        let shape = Shape::from(Size::new(1.75, 1.));
        assert_matches!(shape, Shape::Normal(x) if x == Size::new(1.75, 1.));
    }

    #[test]
    fn key_new() {
        let key = Key::new();

        assert_eq!(key.position, Point::new(0., 0.));
        assert_matches!(key.shape, Shape::Normal(size) if size == Size::new(1., 1.));
        assert_matches!(key.typ, Type::Normal);
        assert_eq!(key.color, Color::new(0.8, 0.8, 0.8));
        for row in key.legends {
            for el in row {
                assert!(el.is_none());
            }
        }
    }
}
