use crate::types::{Color, Rect};

#[derive(Debug, Clone)]
pub(crate) struct HorizontalAlign<T> {
    left: T,
    middle: T,
    right: T,
}

#[derive(Debug, Clone)]
pub(crate) struct VerticalAlign<T> {
    top: T,
    middle: T,
    bottom: T,
}

pub(crate) type LegendMap<T> = VerticalAlign<HorizontalAlign<T>>;

pub(crate) fn legend_map<T>(values: [T; 9]) -> LegendMap<T> {
    // Note: we have to assign to these temporary values from the array because the compiler won't
    // let me do it directly as I'm moving out of a non-copy array.
    let [top_lft, top_mid, top_rgt, mid_lft, mid_mid, mid_rgt, btm_lft, btm_mid, btm_rgt] = values;

    VerticalAlign {
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
    }
}

impl<T> From<LegendMap<T>> for Vec<T> {
    fn from(error: LegendMap<T>) -> Vec<T> {
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
        } = error;

        vec![
            top_lft, top_mid, top_rgt, mid_lft, mid_mid, mid_rgt, btm_lft, btm_mid, btm_rgt,
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HomingType {
    Default, // The default for this profile
    Scoop,   // Homing scoop a.k.a. deep dish
    Bar,     // Homing bar a.k.a. line
    Bump,    // Homing bump a.k.a. nub, dot, or nipple
}

#[derive(Debug, Clone, Copy)]
pub enum KeyType {
    None,   // i.e. decal in KLE
    Normal, // Just a regular ol' key
    Homing(HomingType),
    Space,
}

#[derive(Debug)]
pub struct Key {
    position: Rect,
    key_type: KeyType,
    key_color: Color,
    legend: LegendMap<String>,
    legend_size: LegendMap<u8>,
    legend_color: LegendMap<Color>,
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
            legend: legend_map(legend),
            legend_size: legend_map(legend_size),
            legend_color: legend_map(legend_color),
        }
    }
}
