use crate::utils::{Color, Vec2};

#[derive(Debug, Clone, Copy)]
pub enum Homing {
    Scoop,
    Bar,
    Bump,
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    None,   // a.k.a. decal in KLE lingo
    Normal, // Just a regular ol' key
    Homing(Option<Homing>),
    Space,
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Normal(Vec2),
    SteppedCaps,
    IsoVertical,
    IsoHorizontal,
}

impl Shape {
    #[inline]
    pub fn size(self) -> Vec2 {
        match self {
            Self::Normal(s) => s,
            Self::IsoHorizontal | Self::IsoVertical => Vec2::new(1.5, 2.0),
            Self::SteppedCaps => Vec2::new(1.75, 1.0),
        }
    }
}

impl From<Vec2> for Shape {
    fn from(value: Vec2) -> Self {
        Self::Normal(value)
    }
}

#[derive(Debug, Clone)]
pub struct Legend {
    pub text: String,
    pub size: usize,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Key {
    pub position: Vec2,
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

    // Example non-blank key used in some of our tests
    pub(crate) fn example() -> Self {
        Self {
            legends: [
                [
                    Some(Legend {
                        text: "!".into(),
                        size: 4,
                        color: Color::new(0, 0, 0),
                    }),
                    None,
                    Some(Legend {
                        text: "¹".into(),
                        size: 4,
                        color: Color::new(0, 0, 0),
                    }),
                ],
                [None, None, None],
                [
                    Some(Legend {
                        text: "1".into(),
                        size: 4,
                        color: Color::new(0, 0, 0),
                    }),
                    None,
                    Some(Legend {
                        text: "¡".into(),
                        size: 4,
                        color: Color::new(0, 0, 0),
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
            position: Vec2::ZERO,
            shape: Shape::Normal(Vec2::from(1.)),
            typ: Type::Normal,
            color: Color::new(0xCC, 0xCC, 0xCC),
            legends: Default::default(), // [[None; 3]; 3] won't work since Option<Legend> : !Copy
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use assert_matches::assert_matches;

    #[test]
    fn test_shape_size() {
        assert_eq!(
            Shape::Normal(Vec2::new(2.25, 1.)).size(),
            Vec2::new(2.25, 1.)
        );
        assert_eq!(Shape::IsoVertical.size(), Vec2::new(1.5, 2.0));
        assert_eq!(Shape::IsoHorizontal.size(), Vec2::new(1.5, 2.0));
        assert_eq!(Shape::SteppedCaps.size(), Vec2::new(1.75, 1.0));
    }

    #[test]
    fn test_shape_from() {
        let shape = Shape::from(Vec2::new(1.75, 1.));
        assert_matches!(shape, Shape::Normal(x) if x == Vec2::new(1.75, 1.));
    }

    #[test]
    fn test_key_new() {
        let key = Key::new();

        assert_eq!(key.position, Vec2::new(0., 0.));
        assert_matches!(key.shape, Shape::Normal(size) if size == Vec2::new(1., 1.));
        assert_matches!(key.typ, Type::Normal);
        assert_eq!(key.color, Color::new(0xCC, 0xCC, 0xCC));
        for row in key.legends {
            for el in row {
                assert!(el.is_none());
            }
        }
    }
}
