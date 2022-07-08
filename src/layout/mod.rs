// mod de;

use itertools::Itertools;

use std::fmt;

// pub use self::de::*;
use crate::error::Result;
use crate::utils::{Color, Point, Size};

#[derive(Debug, Clone, PartialEq)]
struct HorizontalAlign<T> {
    left: T,
    middle: T,
    right: T,
}

#[derive(Debug, Clone, PartialEq)]
struct VerticalAlign<T> {
    top: T,
    middle: T,
    bottom: T,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LegendMap<T>(VerticalAlign<HorizontalAlign<T>>);

impl<T> LegendMap<T> {
    pub(crate) fn new(values: Vec<T>) -> Self {
        assert_eq!(values.len(), 9);

        // Note: we have to assign to these temporary values from the array because the compiler
        // won't let me do it directly as I'm moving out of a non-copy array. Also using tuples()
        // seems to be the only way to destructure the elements while taking ownership.
        let (top_lft, top_mid, top_rgt, mid_lft, mid_mid, mid_rgt, btm_lft, btm_mid, btm_rgt) =
            values.into_iter().tuples().next().unwrap();

        LegendMap(VerticalAlign {
            top: HorizontalAlign {
                left: top_lft,
                middle: top_mid,
                right: top_rgt,
            },
            middle: HorizontalAlign {
                left: mid_lft,
                middle: mid_mid,
                right: mid_rgt,
            },
            bottom: HorizontalAlign {
                left: btm_lft,
                middle: btm_mid,
                right: btm_rgt,
            },
        })
    }
}

impl<T> From<LegendMap<T>> for Vec<T> {
    fn from(map: LegendMap<T>) -> Vec<T> {
        let VerticalAlign {
            top:
                HorizontalAlign {
                    left: top_lft,
                    middle: top_mid,
                    right: top_rgt,
                },
            middle:
                HorizontalAlign {
                    left: mid_lft,
                    middle: mid_mid,
                    right: mid_rgt,
                },
            bottom:
                HorizontalAlign {
                    left: btm_lft,
                    middle: btm_mid,
                    right: btm_rgt,
                },
        } = map.0;

        vec![
            top_lft, top_mid, top_rgt, mid_lft, mid_mid, mid_rgt, btm_lft, btm_mid, btm_rgt,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomingType {
    Default, // The default for this profile
    Scoop,   // Homing scoop a.k.a. deep dish
    Bar,     // Homing bar a.k.a. line
    Bump,    // Homing bump a.k.a. nub, dot, or nipple
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    None,   // i.e. decal in KLE
    Normal, // Just a regular ol' key
    Homing(HomingType),
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeySize {
    Normal(Size),
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
            Ok(Self::Normal(Size::new(w, h)))
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

    pub fn size(&self) -> Size {
        match self {
            Self::Normal(s) => *s,
            Self::IsoHorizontal | Self::IsoVertical => Size::new(1.5, 2.0),
            Self::SteppedCaps => Size::new(1.75, 1.0),
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

#[derive(Debug)]
pub struct Key {
    pub position: Point,
    pub size: KeySize,
    pub key_type: KeyType,
    pub key_color: Color,
    pub(crate) legend: LegendMap<String>,
    pub(crate) legend_size: LegendMap<u8>,
    pub(crate) legend_color: LegendMap<Color>,
}

impl Key {
    pub fn new(
        position: Point,
        size: KeySize,
        key_type: KeyType,
        key_color: Color,
        legend: Vec<String>,
        legend_size: Vec<u8>,
        legend_color: Vec<Color>,
    ) -> Self {
        Self {
            position,
            size,
            key_type,
            key_color,
            legend: LegendMap::new(legend),
            legend_size: LegendMap::new(legend_size),
            legend_color: LegendMap::new(legend_color),
        }
    }
}

pub struct Layout {
    pub size: Size,
    pub keys: Vec<Key>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legend_map_new() {
        let legends = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];
        let map = LegendMap::new(legends);

        assert_eq!(
            map.0,
            VerticalAlign {
                top: HorizontalAlign {
                    left: "A",
                    middle: "B",
                    right: "C",
                },
                middle: HorizontalAlign {
                    left: "D",
                    middle: "E",
                    right: "F",
                },
                bottom: HorizontalAlign {
                    left: "G",
                    middle: "H",
                    right: "I",
                },
            }
        )
    }

    #[test]
    fn test_legend_map_into() {
        let legends = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];
        let map = LegendMap::new(legends.clone());

        assert_eq!(Vec::<_>::from(map), legends);
    }

    #[test]
    fn test_key_size_new() {
        let regular_key = KeySize::new(2.25, 1., 0., 0., 2.25, 1.).unwrap();
        let iso_horiz = KeySize::new(1.5, 1., 0.25, 0., 1.25, 2.).unwrap();
        let iso_vert = KeySize::new(1.25, 2., -0.25, 0., 1.5, 1.).unwrap();
        let step_caps = KeySize::new(1.25, 1., 0., 0., 1.75, 1.).unwrap();

        assert_eq!(regular_key, KeySize::Normal(Size::new(2.25, 1.)));
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

        assert_eq!(regular_key.size(), Size::new(2.25, 1.));
        assert_eq!(iso_horiz.size(), Size::new(1.5, 2.0));
        assert_eq!(iso_vert.size(), Size::new(1.5, 2.0));
        assert_eq!(step_caps.size(), Size::new(1.75, 1.0));
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
        let position = Point::new(1.0, 2.0);
        let size = KeySize::new(1.25, 2.0, -0.25, 0., 1.5, 1.).unwrap();
        let key_type = KeyType::Normal;
        let key_color = Color::new(0.8, 0.4, 0.2);
        let legend = vec![
            "A".into(),
            "B".into(),
            "C".into(),
            "D".into(),
            "E".into(),
            "F".into(),
            "G".into(),
            "H".into(),
            "I".into(),
        ];
        let legend_size = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let legend_color = vec![
            Color::new(0.1, 0.1, 0.1),
            Color::new(0.2, 0.2, 0.2),
            Color::new(0.3, 0.3, 0.3),
            Color::new(0.4, 0.4, 0.4),
            Color::new(0.5, 0.5, 0.5),
            Color::new(0.6, 0.6, 0.6),
            Color::new(0.7, 0.7, 0.7),
            Color::new(0.8, 0.8, 0.8),
            Color::new(0.9, 0.9, 0.9),
        ];

        let key = Key::new(
            position,
            size,
            key_type,
            key_color,
            legend.clone(),
            legend_size.clone(),
            legend_color.clone(),
        );

        assert_eq!(key.position, position);
        assert_eq!(key.size, size);
        assert_eq!(key.key_type, key_type);
        assert_eq!(key.key_color, key_color);
        assert_eq!(
            Vec::from(key.legend),
            legend.iter().map(String::clone).collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::from(key.legend_size),
            legend_size.iter().map(|&s| s).collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::from(key.legend_color),
            legend_color.iter().map(|&c| c).collect::<Vec<_>>()
        );
    }
}
