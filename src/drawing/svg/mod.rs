mod layout;

use svg::node::element::{Group, Rectangle};

use crate::drawing::Drawing;
use crate::layout::Key;
use crate::utils::{Point, Size};

pub trait ToSvg<T> {
    fn to_svg(&self) -> T;
}

impl Drawing {
    #[must_use]
    pub fn to_svg(&self) -> String {
        self.layout.to_svg().to_string()
    }
}

impl ToSvg<Group> for Key {
    fn to_svg(&self) -> Group {
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
    use crate::layout::{KeySize, KeyType, Layout};
    use crate::utils::Color;

    use super::*;

    #[test]
    fn test_to_svg() {
        let layout = Layout {
            size: Size::new(1., 1.),
            keys: vec![],
        };
        let drawing = Drawing {
            layout,
            // profile,
            dpi: 96.,
        };

        assert_eq!(
            drawing.to_svg(),
            r#"<svg height="72" viewBox="0 0 1000 1000" width="72" xmlns="http://www.w3.org/2000/svg"/>"#
        );
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
        let elem = key.to_svg();
        let attr = elem.get_inner().get_attributes();
        assert_eq!(&*attr["transform"], "translate(2000, 1000)");

        let children = elem.get_inner().get_children();
        assert_eq!(children.len(), 1);
    }
}
