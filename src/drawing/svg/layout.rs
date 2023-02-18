use super::ToSvg;
use crate::layout::{Key, Layout};
use crate::utils::Size;

use svg::Document;

impl ToSvg<Document> for Layout {
    fn to_svg(&self) -> Document {
        let Size { w, h } = self.size;

        let document = Document::new()
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
            .set("viewBox", format!("0 0 {:.0} {:.0}", w * 1e3, h * 1e3));

        self.keys
            .iter()
            .map(Key::to_svg)
            .fold(document, Document::add)
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
        let elem = layout.to_svg();
        let attr = elem.get_attributes();

        assert_eq!(&*attr["width"], "72");
        assert_eq!(&*attr["height"], "72");
        assert_eq!(&*attr["viewBox"], "0 0 1000 1000");
        assert_eq!(&*attr["xmlns"], "http://www.w3.org/2000/svg");
    }
}
