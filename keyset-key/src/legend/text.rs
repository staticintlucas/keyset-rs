use std::fmt::Display;

/// Struct representing a single legend's text. This can be made up of one or
/// more lines
#[derive(Clone, Debug)]
pub struct Text(Box<[String]>);

impl Display for Text {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join("\\n"))
    }
}

impl Text {
    /// Parse a string legend. This currently supports splitting lines using the
    /// HTML `<br>` tag, but other HTML tags such as `<b>bold</b>` or
    /// `<i>italic</i>` are ignored.
    #[inline]
    #[must_use]
    pub fn parse_from(mut string: &str) -> Self {
        // Vec of lines of text
        let mut result = Vec::new();

        // Find all <br> tags in string
        while let Some(start) = string.find("<br") {
            if let Some(len) = string[start..].find('>') {
                // Push string up to the tag
                result.push(string[..start].to_owned());
                string = &string[start + len + 1..];
            } else {
                // If we don't find a '>' this was not a valid tag
                break;
            }
        }
        // Push whatever's remaining
        if !string.is_empty() {
            result.push(string.to_owned());
        }

        Self(result.into_boxed_slice())
    }

    /// Create an iterator over the lines of the legend text
    #[inline]
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
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
            Text::parse_from("hello<br world"),
        ];
        let expected = [
            &["hello world"][..],
            &["hello", "world"][..],
            &["hello", "world"][..],
            &["hello", "world"][..],
            &["hello<p>world"][..],
            &["hello", "panu>bae>ahba>world"][..],
            &["hello<br world"][..],
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
