use std::sync::OnceLock;

use geom::Length;
use ttf_parser::GlyphId;

use crate::{Font, FontUnit, Glyph};

const FONT_TTF: &[u8] = include_bytes!(env!("DEFAULT_TTF"));
static FONT: OnceLock<Font> = OnceLock::new();

pub fn font() -> &'static Font {
    FONT.get_or_init(|| {
        Font::from_ttf(FONT_TTF.to_owned())
            .unwrap_or_else(|_| unreachable!("default font is tested"))
    })
}

pub fn notdef() -> Glyph {
    Glyph::parse_from(&font().face, GlyphId(0))
        .unwrap_or_else(|| unreachable!("default font is tested"))
}

pub fn cap_height() -> Length<FontUnit> {
    Length::new(
        font()
            .face
            .capital_height()
            .unwrap_or_else(|| unreachable!("default font is tested"))
            .into(),
    )
}

pub fn x_height() -> Length<FontUnit> {
    Length::new(
        font()
            .face
            .x_height()
            .unwrap_or_else(|| unreachable!("default font is tested"))
            .into(),
    )
}

pub fn line_height() -> Length<FontUnit> {
    Length::new(font().face.ascender().into()) - Length::new(font().face.descender().into())
        + Length::new(font().face.line_gap().into())
}

#[cfg(test)]
mod tests {
    use geom::Length;
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn default_font() {
        let a = font();
        let b = font();

        assert!(std::ptr::eq(a, b));
    }

    #[test]
    fn default_notdef() {
        let notdef = notdef();

        assert_eq!(notdef.path.len(), 26);
        assert_is_close!(notdef.advance, Length::new(550.0));
    }

    #[test]
    fn default_cap_height() {
        assert_is_close!(cap_height(), Length::new(714.0));
    }

    #[test]
    fn default_x_height() {
        assert_is_close!(x_height(), Length::new(523.0));
    }

    #[test]
    fn default_line_height() {
        assert_is_close!(line_height(), Length::new(1165.0));
    }
}
