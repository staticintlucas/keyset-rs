mod text;

use color::Color;

pub use self::text::Text;

/// A single legend
#[derive(Debug, Clone)]
pub struct Legend {
    /// The legend text
    pub text: Text,
    /// The legend size
    pub size_idx: usize,
    /// The legend colour
    pub color: Color,
}

impl Legend {
    /// Create a new [`Legend`]
    #[inline]
    #[must_use]
    pub fn new(text: &str, size_idx: usize, color: Color) -> Self {
        Self {
            text: Text::parse_from(text),
            size_idx,
            color,
        }
    }

    /// An example non-blank set of legends
    #[inline]
    #[must_use]
    pub fn example_set() -> [Option<Box<Self>>; 9] {
        let size = 4;
        let color = Color::new(0.0, 0.0, 0.0);

        [
            Some(Box::new(Self::new("!", size, color))),
            None,
            Some(Box::new(Self::new("¹", size, color))),
            None,
            None,
            None,
            Some(Box::new(Self::new("1", size, color))),
            None,
            Some(Box::new(Self::new("¡", size, color))),
        ]
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn legend_new() {
        let legend = Legend::new("test", 4, Color::new(0.0, 0.2, 0.4));

        assert_eq!(legend.text.to_string(), "test");
        assert_eq!(legend.size_idx, 4);
        assert_is_close!(legend.color, Color::new(0.0, 0.2, 0.4));
    }

    #[test]
    fn legends_example() {
        let legends = Legend::example_set();
        let legend_is_some = [true, false, true, false, false, false, true, false, true];

        for (legend, is_some) in legends.into_iter().zip(legend_is_some) {
            assert_eq!(legend.is_some(), is_some);
        }
    }
}
