use std::sync::OnceLock;

use ttf_parser::GlyphId;

use crate::{Font, Glyph};

const FONT_TTF: &[u8] = include_bytes!(env!("DEFAULT_TTF"));
static FONT: OnceLock<Font> = OnceLock::new();

pub fn font() -> &'static Font {
    FONT.get_or_init(|| Font::from_ttf(FONT_TTF.to_owned()).expect("default font is tested"))
}

pub fn notdef() -> Glyph {
    Glyph::parse_from(&font().face, GlyphId(0)).expect("default font is tested")
}

pub fn cap_height() -> f64 {
    f64::from(
        font()
            .face
            .capital_height()
            .expect("default font is tested"),
    )
}

pub fn x_height() -> f64 {
    f64::from(font().face.x_height().expect("default font is tested"))
}

pub fn line_height() -> f64 {
    f64::from(font().face.ascender()) - f64::from(font().face.descender())
        + f64::from(font().face.line_gap())
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

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

        eprintln!("{:?}", notdef.path.elements());
        assert_eq!(notdef.path.elements().len(), 26);
        assert_approx_eq!(notdef.advance, 550.0);
    }

    #[test]
    fn default_cap_height() {
        assert_approx_eq!(cap_height(), 714.0);
    }

    #[test]
    fn default_x_height() {
        assert_approx_eq!(x_height(), 523.0);
    }

    #[test]
    fn default_line_height() {
        assert_approx_eq!(line_height(), 1165.0);
    }
}
