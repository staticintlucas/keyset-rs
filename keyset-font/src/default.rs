use std::sync::OnceLock;

use geom::Length;
use ttf_parser::GlyphId;

use crate::{Font, FontUnit, Glyph};

const FONT_TTF: &[u8] = include_bytes!(env!("DEFAULT_TTF"));
static FONT: OnceLock<Font> = OnceLock::new();

pub fn font() -> &'static Font {
    FONT.get_or_init(|| Font::from_ttf(FONT_TTF.to_owned()).expect("default font is tested"))
}

pub fn notdef() -> Glyph {
    Glyph::parse_from(&font().face, GlyphId(0)).expect("default font is tested")
}

pub fn cap_height() -> Length<FontUnit> {
    Length::new(
        font()
            .face
            .capital_height()
            .expect("default font is tested")
            .into(),
    )
}

pub fn x_height() -> Length<FontUnit> {
    Length::new(
        font()
            .face
            .x_height()
            .expect("default font is tested")
            .into(),
    )
}

pub fn line_height() -> Length<FontUnit> {
    Length::new(font().face.ascender().into()) - Length::new(font().face.descender().into())
        + Length::new(font().face.line_gap().into())
}

#[cfg(test)]
mod tests {
    use geom::{ApproxEq, Length};

    use super::*;

    #[test]
    fn default_font() {
        let a = font();
        let b = font();

        assert_eq!(a as *const _, b as *const _);
    }

    #[test]
    fn default_notdef() {
        let notdef = notdef();

        assert_eq!(notdef.path.data.len(), 26);
        assert!(notdef.advance.approx_eq(&Length::new(550.0)));
    }

    #[test]
    fn default_cap_height() {
        assert!(cap_height().approx_eq(&Length::new(714.0)));
    }

    #[test]
    fn default_x_height() {
        assert!(x_height().approx_eq(&Length::new(523.0)));
    }

    #[test]
    fn default_line_height() {
        assert!(line_height().approx_eq(&Length::new(1165.0)));
    }
}
