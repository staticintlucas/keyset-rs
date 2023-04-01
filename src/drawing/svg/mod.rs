mod font;
mod path;
mod profile;

use svg::node::element::Group;
use svg::Document;

use crate::drawing::Drawing;
use crate::layout::Key;
use crate::utils::{Trim, Vec2};

use font::Draw as _;
use profile::Draw as _;

pub trait ToSvg {
    fn to_svg(&self) -> String;
}

impl ToSvg for Drawing {
    #[must_use]
    fn to_svg(&self) -> String {
        // scale from keyboard units to drawing units (milliunits)
        let scale = Vec2::from(1e3);
        let size = self.layout.size * scale;

        // w and h are in dpi units, the 0.75 is keyboard units per inch
        let Vec2 { x, y } = self.layout.size * self.options.dpi * 0.75;

        let document = Document::new()
            .set("width", format!("{}", Trim(x)))
            .set("height", format!("{}", Trim(y)))
            .set("viewBox", format!("0 0 {:.0} {:.0}", size.x, size.y));

        let document = self
            .layout
            .keys
            .iter()
            .map(|key| self.draw_key(key))
            .fold(document, Document::add);

        document.to_string()
    }
}

impl Drawing {
    fn draw_key(&self, key: &Key) -> Group {
        let scale = Vec2::from(1e3);
        let pos = key.position * scale;

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
    use crate::layout::tests::test_key;
    use crate::layout::Layout;
    use crate::profile::Profile;
    use crate::utils::Vec2;

    use super::*;

    #[test]
    fn test_to_svg() {
        let layout = Layout {
            size: Vec2::from(1.),
            keys: vec![],
        };
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
        let key = test_key();
        let layout = Layout {
            size: Vec2::from(1.),
            keys: vec![],
        };
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
