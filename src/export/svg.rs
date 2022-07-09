use svg::Document;

use crate::layout::Layout;
use crate::utils::Size;

#[allow(clippy::module_name_repetitions)]
pub trait ToSvg {
    fn to_svg(&self) -> String;
}

impl ToSvg for Layout {
    fn to_svg(&self) -> String {
        let Size { w, h } = self.size;

        Document::new()
            .set(
                "width",
                format!("{:.5}", w * 72.)
                    .trim_end_matches('0')
                    .trim_end_matches('.'),
            )
            .set(
                "height",
                format!("{:.5}", h * 72.)
                    .trim_end_matches('0')
                    .trim_end_matches('.'),
            )
            .set("viewBox", format!("0 0 {:.0} {:.0}", w * 1e3, h * 1e3))
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_svg() {
        let layout = Layout {
            size: Size::new(1., 1.),
            keys: vec![],
        };

        assert_eq!(
            layout.to_svg(),
            r#"<svg height="72" viewBox="0 0 1000 1000" width="72" xmlns="http://www.w3.org/2000/svg"/>"#
        );
    }
}
