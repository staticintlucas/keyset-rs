mod font;
mod profile;

use kurbo::{Affine, Point, Rect, Size};
use svg::node::element::Group;
use svg::Document;

use crate::drawing::Drawing;
use crate::key::Key;

use font::Draw as _;
use profile::Draw as _;

pub trait ToSvg {
    fn to_svg(&self) -> String;
}

impl ToSvg for Drawing {
    #[must_use]
    fn to_svg(&self) -> String {
        let bounds = self
            .keys
            .iter()
            .map(|k| k.position + k.shape.size().to_vec2())
            .fold(
                Rect::from_origin_size(Point::ORIGIN, Size::new(1., 1.)),
                |r, p| r.union_pt(p),
            );

        // w and h are in dpi units, the 0.75 is keyboard units per inch
        let Size { width, height } = bounds.size() * self.options.dpi * 0.75;

        // scale from keyboard units to drawing units (milliunits)
        let bounds = bounds.scale_from_origin(1e3);

        let document = Document::new()
            .set("width", format!("{}", (1e5 * width).floor() / 1e5))
            .set("height", format!("{}", (1e5 * height).floor() / 1e5))
            .set(
                "viewBox",
                format!(
                    "{:.0} {:.0} {:.0} {:.0}",
                    bounds.origin().x,
                    bounds.origin().y,
                    bounds.size().width,
                    bounds.size().height
                ),
            );

        let document = self
            .keys
            .iter()
            .map(|key| self.draw_key(key))
            .fold(document, Document::add);

        document.to_string()
    }
}

impl Drawing {
    fn draw_key(&self, key: &Key) -> Group {
        // scale from keyboard units to drawing units (milliunits)
        let pos = Affine::scale(1e3) * key.position;

        let result = Group::new().set("transform", format!("translate({}, {})", pos.x, pos.y));

        let mut elements = vec![];
        if self.options.show_keys {
            elements.extend(self.profile.draw_key(key));
        }
        if self.options.show_margin {
            elements.extend(self.profile.draw_margin(key));
        }
        elements.extend(self.font.draw_legends(&self.profile, key));

        elements.into_iter().fold(result, Group::add)
    }
}

#[cfg(test)]
mod tests {
    use crate::drawing::DrawingOptions;
    use crate::font::Font;
    use crate::profile::Profile;

    use super::*;

    #[test]
    fn test_to_svg() {
        let layout = vec![];
        let profile = Profile::default();
        let font = Font::from_ttf(&std::fs::read("tests/fonts/demo.ttf").unwrap()).unwrap();
        let options = DrawingOptions::default();
        let drawing = Drawing::new(layout, profile, font, options);

        assert_eq!(
            drawing.to_svg(),
            r#"<svg height="72" viewBox="0 0 1000 1000" width="72" xmlns="http://www.w3.org/2000/svg"/>"#
        );
    }

    #[test]
    fn test_draw_key() {
        let key = Key::example();
        let layout = vec![];
        let profile = Profile::default();
        let font = Font::from_ttf(&std::fs::read("tests/fonts/demo.ttf").unwrap()).unwrap();

        let test_config = vec![
            (DrawingOptions::default(), 6),
            (
                DrawingOptions {
                    show_keys: false,
                    ..DrawingOptions::default()
                },
                4,
            ),
            (
                DrawingOptions {
                    show_margin: true,
                    ..DrawingOptions::default()
                },
                7,
            ),
        ];

        for (options, len) in test_config {
            let drawing = Drawing::new(layout.clone(), profile.clone(), font.clone(), options);
            let group = drawing.draw_key(&key);

            assert_eq!(group.get_children().len(), len);
        }
    }
}
