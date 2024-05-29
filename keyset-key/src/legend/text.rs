use std::fmt::Display;

use lazy_regex::regex;

/// Struct representing a single legend's text. This can be made up of one or
/// more lines
#[derive(Clone, Debug)]
pub struct Text(Box<[String]>);

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join("\\n"))
    }
}

impl Text {
    /// Parse a string legend. This currently supports splitting lines using the
    /// HTML `<br>` tag, but other HTML tags such as `<b>bold</b>` or
    /// `<i>italic</i>` are ignored.
    pub fn parse_from(string: &str) -> Self {
        Self(
            regex!(r"<br[^>]*>")
                .split(string)
                .map(ToString::to_string)
                .collect(),
        )
    }

    /// Create an iterator over the lines of the legend text
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_display() {
        let lines = ["hello", "world"].map(ToString::to_string);
        let text = Text(Box::new(lines));

        assert_eq!(format!("{text}"), "hello\\nworld");
    }

    #[test]
    fn text_parse_from() {
        let text = [
            Text::parse_from("hello world"),
            Text::parse_from("hello<br>world"),
            Text::parse_from("hello<br />world"),
            Text::parse_from("hello<braiugho;\0n'oarenb\\>world"),
            Text::parse_from("hello<p>world"),
            Text::parse_from("hello<br >panu>bae>ahba>world"),
        ];
        let expected = [
            &["hello world"][..],
            &["hello", "world"][..],
            &["hello", "world"][..],
            &["hello", "world"][..],
            &["hello<p>world"][..],
            &["hello", "panu>bae>ahba>world"][..],
        ];

        for (t, e) in text.iter().zip(expected) {
            assert_eq!(t.0.as_ref(), e);
        }
    }

    #[test]
    fn text_lines() {
        let text = Text::parse_from("hello<br>world");
        let mut iter = text.lines();

        assert_eq!(iter.next(), Some("hello"));
        assert_eq!(iter.next(), Some("world"));
        assert_eq!(iter.next(), None);
    }
}
