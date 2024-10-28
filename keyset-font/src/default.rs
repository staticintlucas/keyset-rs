use std::sync::OnceLock;

use crate::Font;

const FONT_TTF: &[u8] = include_bytes!(env!("DEFAULT_TTF"));
static FONT: OnceLock<Font> = OnceLock::new();

pub fn font() -> &'static Font {
    FONT.get_or_init(|| {
        Font::from_ttf(FONT_TTF.to_owned())
            .unwrap_or_else(|_| unreachable!("default font is tested"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_font() {
        let a = font();
        let b = font();

        assert!(std::ptr::eq(a, b));
    }
}
