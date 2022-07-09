use svg::node::element::{Group, Rectangle};
use svg::Document;

use crate::layout::{Key, Layout};
use crate::utils::{Point, Size};

#[allow(clippy::module_name_repetitions)]
pub trait ToSvg {
    fn to_svg(&self) -> String;
}

impl ToSvg for Layout {
    fn to_svg(&self) -> String {
        self.to_svg_elem().to_string()
    }
}

trait ToSvgElem<T> {
    fn to_svg_elem(&self) -> T;
}

impl ToSvgElem<Document> for Layout {
    fn to_svg_elem(&self) -> Document {
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
            .map(Key::to_svg_elem)
            .fold(document, Document::add)
    }
}

impl ToSvgElem<Group> for Key {
    fn to_svg_elem(&self) -> Group {
        let Point { x, y } = self.position;
        let Size { w, h } = self.size.size();

        Group::new()
            .set("transform", format!("translate({}, {})", x * 1e3, y * 1e3))
            .add(
                Rectangle::new()
                    .set("x", "20")
                    .set("y", "20")
                    .set("width", format!("{}", 960. + (w - 1.) * 1e3))
                    .set("height", format!("{}", 960. + (h - 1.) * 1e3))
                    .set("fill", self.key_color.to_hex())
                    .set("stroke", self.key_color.highlight(0.15).to_hex())
                    .set("stroke-width", "10")
                    .set("rx", "15")
                    .set("ry", "15"),
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::layout::{KeySize, KeyType};
    use crate::utils::Color;

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

    #[test]
    fn test_layout_to_svg_elem() {
        let layout = Layout {
            size: Size::new(1., 1.),
            keys: vec![],
        };
        let elem = layout.to_svg_elem();
        let attr = elem.get_inner().get_attributes();

        assert_eq!(&*attr["width"], "72");
        assert_eq!(&*attr["height"], "72");
        assert_eq!(&*attr["viewBox"], "0 0 1000 1000");
        assert_eq!(&*attr["xmlns"], "http://www.w3.org/2000/svg");
    }

    #[test]
    fn test_key_to_svg_elem() {
        let key = Key::new(
            Point::new(2., 1.),
            KeySize::new(1.5, 1., 0., 0., 1.5, 1.).unwrap(),
            KeyType::Normal,
            Color::default_key(),
            vec!["".to_string(); 9],
            vec![4; 9],
            vec![Color::default_legend(); 9],
        );
        let elem = key.to_svg_elem();
        let attr = elem.get_inner().get_attributes();
        assert_eq!(&*attr["transform"], "translate(2000, 1000)");

        let children = elem.get_inner().get_children();
        assert_eq!(children.len(), 1);
    }
}
