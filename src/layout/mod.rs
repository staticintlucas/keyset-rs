use crate::types::{Color, Rect};

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
    pub(crate) fn new(values: [T; 9]) -> Self {
        // Note: we have to assign to these temporary values from the array because the compiler won't
        // let me do it directly as I'm moving out of a non-copy array.
        let [top_lft, top_mid, top_rgt, mid_lft, mid_mid, mid_rgt, btm_lft, btm_mid, btm_rgt] =
            values;

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

#[derive(Debug)]
pub struct Key {
    pub(super) position: Rect,
    pub(super) key_type: KeyType,
    pub(super) key_color: Color,
    pub(super) legend: LegendMap<String>,
    pub(super) legend_size: LegendMap<u8>,
    pub(super) legend_color: LegendMap<Color>,
}

impl Key {
    pub(crate) fn new(
        position: Rect,
        key_type: KeyType,
        key_color: Color,
        legend: [String; 9],
        legend_size: [u8; 9],
        legend_color: [Color; 9],
    ) -> Self {
        Self {
            position,
            key_type,
            key_color,
            legend: LegendMap::new(legend),
            legend_size: LegendMap::new(legend_size),
            legend_color: LegendMap::new(legend_color),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    use super::*;

    #[test]
    fn test_legend_map_new() {
        let legends = ["A", "B", "C", "D", "E", "F", "G", "H", "I"];
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
        let legends = ["A", "B", "C", "D", "E", "F", "G", "H", "I"];
        let map = LegendMap::new(legends);

        assert_eq!(Vec::<_>::from(map), legends)
    }

    #[test]
    fn test_key_new() {
        let position = Rect::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        let key_type = KeyType::Normal;
        let key_color = Color::new(0.8, 0.4, 0.2);
        let legend = [
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
        let legend_size = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let legend_color = [
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
            key_type,
            key_color,
            legend.clone(),
            legend_size,
            legend_color,
        );

        assert_eq!(key.position, position);
        assert_eq!(key.key_type, key_type);
        assert_eq!(key.key_color, key_color);
        assert_eq!(Vec::from(key.legend), legend);
        assert_eq!(Vec::from(key.legend_size), legend_size);
        assert_eq!(Vec::from(key.legend_color), legend_color);
    }
}
