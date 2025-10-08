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
    pub fn parse_from(string: &str) -> Self {
        TextParser::new(string).parse()
    }

    /// Create an iterator over the lines of the legend text
    #[inline]
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }
}

struct TextParser<'a> {
    result: Vec<String>,
    line: Vec<u8>,
    input: &'a [u8],
}

impl<'a> TextParser<'a> {
    const fn new(input: &'a str) -> Self {
        Self {
            result: Vec::new(),
            line: Vec::new(),
            input: input.as_bytes(),
        }
    }

    fn parse(mut self) -> Text {
        loop {
            let Some((first, rest)) = self.input.split_first() else {
                break;
            };
            match *first {
                // Supported HTML tags
                b'<' => {
                    self.input = rest;
                    self.parse_tag();
                }
                b'\n' => {
                    self.result.push(
                        String::from_utf8(std::mem::take(&mut self.line))
                            .unwrap_or_else(|_| unreachable!("valid UTF-8")),
                    );
                    self.input = rest;
                }
                _ => {
                    self.line.push(*first);
                    self.input = rest;
                }
            }
        }

        if !self.line.is_empty() {
            self.result
                .push(String::from_utf8(self.line).unwrap_or_else(|_| unreachable!("valid UTF-8")));
        }

        Text(self.result.into_boxed_slice())
    }

    fn parse_tag(&mut self) {
        let Some(name_end) = self.input.iter().position(|&b| !b.is_ascii_alphanumeric()) else {
            self.line.push(b'<');
            self.line.extend_from_slice(self.input);
            self.input = &[];
            return;
        };
        let (name, rest) = self.input.split_at(name_end);

        let Some(tag_end) = rest.iter().position(|&b| b == b'>') else {
            self.line.push(b'<');
            self.line.extend_from_slice(name);
            self.input = rest;
            return;
        };
        let (tag_content, rest) = rest.split_at(tag_end + 1);
        self.input = rest;

        #[expect(clippy::single_match_else, reason = "easier to extend later")]
        match name {
            b"br" => {
                self.result.push(
                    String::from_utf8(std::mem::take(&mut self.line))
                        .unwrap_or_else(|_| unreachable!("valid UTF-8")),
                );
            }
            _ => {
                // Just pass through unsupported tags
                self.line.push(b'<');
                self.line.extend_from_slice(name);
                self.line.extend_from_slice(tag_content);
            }
        }
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
        let result = Text::parse_from("hello<br>world");
        assert_eq!(result.0.as_ref(), &["hello", "world"][..]);
    }

    #[test]
    fn text_lines() {
        let text = Text::parse_from("hello<br>world");
        let mut iter = text.lines();

        assert_eq!(iter.next(), Some("hello"));
        assert_eq!(iter.next(), Some("world"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn text_parser_new() {
        let text_parser = TextParser::new("hello world");
        assert_eq!(text_parser.input, b"hello world");
        assert!(text_parser.result.is_empty());
        assert!(text_parser.line.is_empty());
    }

    #[test]
    fn text_parser_parse() {
        let input = ["hello world", "hello<br>world", "hello\nworld", "hello\n"];
        let expected = [
            &["hello world"][..],
            &["hello", "world"][..],
            &["hello", "world"][..],
            &["hello"][..],
        ];

        assert!(input.len() == expected.len());
        for (inp, exp) in input.iter().zip(expected) {
            let parser = TextParser {
                result: Vec::new(),
                line: Vec::new(),
                input: inp.as_bytes(),
            };
            let result = parser.parse();
            assert_eq!(result.0.as_ref(), exp);
        }
    }

    #[test]
    fn text_parser_parse_tag() {
        let input = [
            "br>foo",
            "br attr=val>foo",
            "br aiugho;\0n'oarenb\\>foo",
            "bread>foo",
            "foo>bar",
            "foo bar>baz",
            "foo",
            "foo bar",
            // "hello<br>world",
            // "hello\nworld",
            // "hello\n",
        ];
        let expected = [
            TextParser {
                result: vec![String::new()],
                line: b"".to_vec(),
                input: b"foo",
            },
            TextParser {
                result: vec![String::new()],
                line: b"".to_vec(),
                input: b"foo",
            },
            TextParser {
                result: vec![String::new()],
                line: b"".to_vec(),
                input: b"foo",
            },
            TextParser {
                result: vec![],
                line: b"<bread>".to_vec(),
                input: b"foo",
            },
            TextParser {
                result: vec![],
                line: b"<foo>".to_vec(),
                input: b"bar",
            },
            TextParser {
                result: vec![],
                line: b"<foo bar>".to_vec(),
                input: b"baz",
            },
            TextParser {
                result: vec![],
                line: b"<foo".to_vec(),
                input: b"",
            },
            TextParser {
                result: vec![],
                line: b"<foo".to_vec(),
                input: b" bar",
            },
        ];

        assert!(input.len() == expected.len());
        for (inp, exp) in input.iter().zip(expected) {
            let mut parser = TextParser {
                result: Vec::new(),
                line: Vec::new(),
                input: inp.as_bytes(),
            };
            parser.parse_tag();

            assert_eq!(parser.result, exp.result);
            assert_eq!(parser.line, exp.line);
            assert_eq!(parser.input, exp.input);
        }
    }
}
