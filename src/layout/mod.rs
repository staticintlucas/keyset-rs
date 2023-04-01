use std::fmt;

// pub use self::de::*;
use crate::error::Result;
use crate::profile::HomingType;
use crate::utils::{Color, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    None,   // i.e. decal in KLE
    Normal, // Just a regular ol' key
    Homing(Option<HomingType>),
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeySize {
    Normal(Vec2),
    SteppedCaps,
    IsoVertical,
    IsoHorizontal,
}

impl KeySize {
    pub fn new(w: f32, h: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> Result<Self> {
        #[inline]
        fn is_stepped_caps(w: f32, h: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
            [w, h, x2, y2, w2, h2]
                .iter()
                .zip([1.25, 1., 0., 0., 1.75, 1.])
                .all(|(a, b)| (b - a).abs() < 0.01)
        }
        #[inline]
        fn is_iso_vertical(w: f32, h: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
            [w, h, x2, y2, w2, h2]
                .iter()
                .zip([1.25, 2., -0.25, 0., 1.5, 1.])
                .all(|(a, b)| (b - a).abs() < 0.01)
        }
        #[inline]
        fn is_iso_horizontal(w: f32, h: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
            [w, h, x2, y2, w2, h2]
                .iter()
                .zip([1.5, 1., 0.25, 0., 1.25, 2.])
                .all(|(a, b)| (b - a).abs() < 0.01)
        }
        #[inline]
        fn is_normal_key(w: f32, h: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
            [x2, y2, w2, h2]
                .iter()
                .zip([0., 0., w, h])
                .all(|(a, b)| (b - a).abs() < 0.01)
        }

        if is_stepped_caps(w, h, x2, y2, w2, h2) {
            Ok(Self::SteppedCaps)
        } else if is_iso_vertical(w, h, x2, y2, w2, h2) {
            Ok(Self::IsoVertical)
        } else if is_iso_horizontal(w, h, x2, y2, w2, h2) {
            Ok(Self::IsoHorizontal)
        } else if is_normal_key(w, h, x2, y2, w2, h2) {
            Ok(Self::Normal(Vec2::new(w, h)))
        } else {
            Err(InvalidKeySize {
                message: format!(
                    "{} (w: {:.2}, h: {:.2}, x2: {:.2}, y2: {:.2}, w2: {:.2}, h2: {:.2}) {}",
                    "Unsupported non-standard key size",
                    w,
                    h,
                    x2,
                    y2,
                    w2,
                    h2,
                    "Note ISO enter and stepped caps are supported as special cases"
                ),
            }
            .into())
        }
    }

    pub fn size(&self) -> Vec2 {
        match self {
            Self::Normal(s) => *s,
            Self::IsoHorizontal | Self::IsoVertical => Vec2::new(1.5, 2.0),
            Self::SteppedCaps => Vec2::new(1.75, 1.0),
        }
    }
}

#[derive(Debug)]
pub(crate) struct InvalidKeySize {
    message: String,
}

impl fmt::Display for InvalidKeySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InvalidKeySize {}

#[derive(Debug, Clone)]
pub struct Key {
    pub position: Vec2,
    pub size: KeySize,
    pub key_type: KeyType,
    pub key_color: Color,
    pub(crate) legend: [[String; 3]; 3],
    pub(crate) legend_size: [[u8; 3]; 3],
    pub(crate) legend_color: [[Color; 3]; 3],
}

impl Key {
    pub fn new(
        position: Vec2,
        size: KeySize,
        key_type: KeyType,
        key_color: Color,
        legend: [[String; 3]; 3],
        legend_size: [[u8; 3]; 3],
        legend_color: [[Color; 3]; 3],
    ) -> Self {
        Self {
            position,
            size,
            key_type,
            key_color,
            legend,
            legend_size,
            legend_color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub size: Vec2,
    pub keys: Vec<Key>,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // Not an actual test, returns a test key for use in other tests
    pub fn test_key() -> Key {
        Key::new(
            Vec2::ZERO,
            KeySize::Normal(Vec2::new(1., 1.)),
            KeyType::Normal,
            Color::default_key(),
            [
                ["!".into(), "".into(), "¹".into()],
                ["".into(), "".into(), "".into()],
                ["1".into(), "".into(), "¡".into()],
            ],
            [[4; 3]; 3],
            [[Color::default_legend(); 3]; 3],
        )
    }

    #[test]
    fn test_key_size_new() {
        let regular_key = KeySize::new(2.25, 1., 0., 0., 2.25, 1.).unwrap();
        let iso_horiz = KeySize::new(1.5, 1., 0.25, 0., 1.25, 2.).unwrap();
        let iso_vert = KeySize::new(1.25, 2., -0.25, 0., 1.5, 1.).unwrap();
        let step_caps = KeySize::new(1.25, 1., 0., 0., 1.75, 1.).unwrap();

        assert_eq!(regular_key, KeySize::Normal(Vec2::new(2.25, 1.)));
        assert_eq!(iso_horiz, KeySize::IsoHorizontal);
        assert_eq!(iso_vert, KeySize::IsoVertical);
        assert_eq!(step_caps, KeySize::SteppedCaps);
    }

    #[test]
    fn test_key_size_size() {
        let regular_key = KeySize::new(2.25, 1., 0., 0., 2.25, 1.).unwrap();
        let iso_horiz = KeySize::new(1.5, 1., 0.25, 0., 1.25, 2.).unwrap();
        let iso_vert = KeySize::new(1.25, 2., -0.25, 0., 1.5, 1.).unwrap();
        let step_caps = KeySize::new(1.25, 1., 0., 0., 1.75, 1.).unwrap();

        assert_eq!(regular_key.size(), Vec2::new(2.25, 1.));
        assert_eq!(iso_horiz.size(), Vec2::new(1.5, 2.0));
        assert_eq!(iso_vert.size(), Vec2::new(1.5, 2.0));
        assert_eq!(step_caps.size(), Vec2::new(1.75, 1.0));
    }

    #[test]
    fn test_key_size_new_invalid() {
        let invalid = KeySize::new(1., 1., -0.25, 0., 1.5, 1.);

        assert!(invalid.is_err());
        assert_eq!(
            format!("{}", invalid.unwrap_err()),
            format!(concat!(
                "error parsing KLE layout: Unsupported non-standard key size (w: 1.00, h: 1.00, ",
                "x2: -0.25, y2: 0.00, w2: 1.50, h2: 1.00) Note ISO enter and stepped caps are ",
                "supported as special cases"
            ))
        );
    }

    #[test]
    fn test_key_new() {
        let position = Vec2::new(1.0, 2.0);
        let size = KeySize::new(1.25, 2.0, -0.25, 0., 1.5, 1.).unwrap();
        let key_type = KeyType::Normal;
        let key_color = Color::new(204, 102, 51);
        let legend = [
            ["A".into(), "B".into(), "C".into()],
            ["D".into(), "E".into(), "F".into()],
            ["G".into(), "H".into(), "I".into()],
        ];
        let legend_size = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let legend_color = [
            [
                Color::new(25, 25, 25),
                Color::new(51, 51, 51),
                Color::new(76, 76, 76),
            ],
            [
                Color::new(102, 102, 102),
                Color::new(127, 127, 127),
                Color::new(153, 153, 153),
            ],
            [
                Color::new(178, 178, 178),
                Color::new(204, 204, 204),
                Color::new(229, 229, 229),
            ],
        ];

        let key = Key::new(
            position,
            size,
            key_type,
            key_color,
            legend.clone(),
            legend_size,
            legend_color,
        );

        assert_eq!(key.position, position);
        assert_eq!(key.size, size);
        assert_eq!(key.key_type, key_type);
        assert_eq!(key.key_color, key_color);
        assert_eq!(Vec::from(key.legend), legend);
        assert_eq!(Vec::from(key.legend_size), legend_size);
        assert_eq!(Vec::from(key.legend_color), legend_color);
    }
}
