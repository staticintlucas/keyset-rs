use std::ops::{Index, IndexMut};

use color::Color;

/// A single legend
#[derive(Debug, Clone, PartialEq)]
pub struct Legend {
    /// The legend text
    pub text: String,
    /// The legend size
    pub size_idx: usize,
    /// The legend colour
    pub color: Color,
}

impl Legend {
    /// Create a new [`Legend`]
    pub fn new(text: impl Into<String>, size_idx: usize, color: Color) -> Self {
        Self {
            text: text.into(),
            size_idx,
            color,
        }
    }
}

/// A set of legends for a key
#[derive(Debug, Clone, Default)]
pub struct Legends([Option<Legend>; 9]);

impl Legends {
    /// An example non-blank set of legends
    #[must_use]
    pub fn example() -> Self {
        Self([
            Some(Legend::new("!", 4, Color::new(0.0, 0.0, 0.0))),
            None,
            Some(Legend::new("¹", 4, Color::new(0.0, 0.0, 0.0))),
            None,
            None,
            None,
            Some(Legend::new("1", 4, Color::new(0.0, 0.0, 0.0))),
            None,
            Some(Legend::new("¡", 4, Color::new(0.0, 0.0, 0.0))),
        ])
    }

    /// Creates an iterator in a left-to-right, top-to-bottom order
    pub fn iter(&self) -> std::slice::Iter<Option<Legend>> {
        self.0.iter()
    }
}

impl From<[Option<Legend>; 9]> for Legends {
    /// Converts from an array in left-to-right, top-to-bottom order
    fn from(value: [Option<Legend>; 9]) -> Self {
        Self(value)
    }
}

impl From<[[Option<Legend>; 3]; 3]> for Legends {
    /// Converts from an array of arrays in row-major order
    fn from(mut value: [[Option<Legend>; 3]; 3]) -> Self {
        let mut arr = <[Option<Legend>; 9]>::default();
        arr[0..3].swap_with_slice(&mut value[0]);
        arr[3..6].swap_with_slice(&mut value[1]);
        arr[6..9].swap_with_slice(&mut value[2]);
        Self(arr)
    }
}

impl IntoIterator for Legends {
    type Item = Option<Legend>;
    type IntoIter = <[Option<Legend>; 9] as IntoIterator>::IntoIter;

    /// Creates an iterator in a left-to-right, top-to-bottom order
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Legends {
    type Item = &'a Option<Legend>;
    type IntoIter = <&'a [Option<Legend>; 9] as IntoIterator>::IntoIter;

    /// Creates an iterator in a left-to-right, top-to-bottom order
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Index<usize> for Legends {
    type Output = Option<Legend>;

    /// Indexes the legends arranged in left-to-right, top-to-bottom order
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Legends {
    /// Mutably indexes the legends arranged in left-to-right, top-to-bottom order
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Index<(usize, usize)> for Legends {
    type Output = Option<Legend>;

    /// Indexes the legends using a `(column, row)` tuple
    fn index(&self, (column, row): (usize, usize)) -> &Self::Output {
        self.0.index(row * 3 + column)
    }
}

impl IndexMut<(usize, usize)> for Legends {
    /// Mutably indexes the legends using a `(column, row)` tuple
    fn index_mut(&mut self, (column, row): (usize, usize)) -> &mut Self::Output {
        self.0.index_mut(row * 3 + column)
    }
}

#[cfg(test)]
pub mod tests {
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn legend_new() {
        let legend = Legend::new("test", 4, Color::new(0.0, 0.2, 0.4));

        assert_eq!(legend.text, "test");
        assert_eq!(legend.size_idx, 4);
        assert_is_close!(legend.color, Color::new(0.0, 0.2, 0.4));
    }

    #[test]
    fn legends_example() {
        let legends = Legends::example();
        let legend_is_some = [true, false, true, false, false, false, true, false, true];

        for (legend, is_some) in legends.into_iter().zip(legend_is_some) {
            assert_eq!(legend.is_some(), is_some);
        }
    }

    #[test]
    fn legends_iter() {
        let legends = Legends::default();
        let mut iter = legends.iter();

        for _ in 0..9 {
            assert!(iter.next().is_some());
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn legends_from() {
        let legends: Legends = [
            Some(Legend::new("A", 3, Color::new(0.0, 0.2, 0.4))),
            None,
            None,
            None,
            Some(Legend::new("B", 4, Color::new(0.3, 0.5, 0.7))),
            None,
            None,
            None,
            Some(Legend::new("C", 5, Color::new(0.6, 0.8, 1.0))),
        ]
        .into();
        let legend_is_some = [true, false, false, false, true, false, false, false, true];

        for (legend, is_some) in legends.into_iter().zip(legend_is_some) {
            assert_eq!(legend.is_some(), is_some);
        }

        let legends: Legends = [
            [
                Some(Legend::new("A", 3, Color::new(0.0, 0.2, 0.4))),
                None,
                None,
            ],
            [
                None,
                Some(Legend::new("B", 4, Color::new(0.3, 0.5, 0.7))),
                None,
            ],
            [
                None,
                None,
                Some(Legend::new("C", 5, Color::new(0.6, 0.8, 1.0))),
            ],
        ]
        .into();

        for (legend, is_some) in legends.into_iter().zip(legend_is_some) {
            assert_eq!(legend.is_some(), is_some);
        }
    }

    #[test]
    fn legends_into_iter() {
        let mut iter = Legends::default().into_iter();

        for _ in 0..9 {
            assert!(iter.next().is_some());
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn legends_index() {
        let legends = Legends::example();

        assert_eq!(legends[0].as_ref().unwrap().text, "!");
        assert_eq!(legends[2].as_ref().unwrap().text, "¹");
        assert_eq!(legends[6].as_ref().unwrap().text, "1");
        assert_eq!(legends[8].as_ref().unwrap().text, "¡");

        assert_eq!(legends[(0, 0)].as_ref().unwrap().text, "!");
        assert_eq!(legends[(2, 0)].as_ref().unwrap().text, "¹");
        assert_eq!(legends[(0, 2)].as_ref().unwrap().text, "1");
        assert_eq!(legends[(2, 2)].as_ref().unwrap().text, "¡");
    }

    #[test]
    fn legends_index_mut() {
        let mut legends = Legends::default();

        legends[0] = Some(Legend::new("A", 4, Color::new(0.2, 0.4, 0.6)));
        legends[2] = Some(Legend::new("B", 4, Color::new(0.2, 0.4, 0.6)));
        legends[6] = Some(Legend::new("C", 4, Color::new(0.2, 0.4, 0.6)));
        legends[8] = Some(Legend::new("D", 4, Color::new(0.2, 0.4, 0.6)));
        assert_eq!(legends[0].as_ref().unwrap().text, "A");
        assert_eq!(legends[2].as_ref().unwrap().text, "B");
        assert_eq!(legends[6].as_ref().unwrap().text, "C");
        assert_eq!(legends[8].as_ref().unwrap().text, "D");

        legends[(0, 0)] = Some(Legend::new("A", 4, Color::new(0.2, 0.4, 0.6)));
        legends[(2, 0)] = Some(Legend::new("B", 4, Color::new(0.2, 0.4, 0.6)));
        legends[(0, 2)] = Some(Legend::new("C", 4, Color::new(0.2, 0.4, 0.6)));
        legends[(2, 2)] = Some(Legend::new("D", 4, Color::new(0.2, 0.4, 0.6)));
        assert_eq!(legends[(0, 0)].as_ref().unwrap().text, "A");
        assert_eq!(legends[(2, 0)].as_ref().unwrap().text, "B");
        assert_eq!(legends[(0, 2)].as_ref().unwrap().text, "C");
        assert_eq!(legends[(2, 2)].as_ref().unwrap().text, "D");
    }
}
