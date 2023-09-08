use std::ops::{Index, IndexMut};

use color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Legend {
    pub text: String,
    pub size_idx: usize,
    pub color: Color,
}

impl Legend {
    pub fn new(text: impl Into<String>, size_idx: usize, color: Color) -> Self {
        Self {
            text: text.into(),
            size_idx,
            color,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Legends([Option<Legend>; 9]);

impl Legends {
    pub fn iter(&self) -> std::slice::Iter<Option<Legend>> {
        self.0.iter()
    }
}

impl From<[Option<Legend>; 9]> for Legends {
    fn from(value: [Option<Legend>; 9]) -> Self {
        Self(value)
    }
}

impl From<[[Option<Legend>; 3]; 3]> for Legends {
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

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Index<usize> for Legends {
    type Output = Option<Legend>;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Legends {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Index<(usize, usize)> for Legends {
    type Output = Option<Legend>;

    fn index(&self, (column, row): (usize, usize)) -> &Self::Output {
        self.0.index(row * 3 + column)
    }
}

impl IndexMut<(usize, usize)> for Legends {
    fn index_mut(&mut self, (column, row): (usize, usize)) -> &mut Self::Output {
        self.0.index_mut(row * 3 + column)
    }
}
