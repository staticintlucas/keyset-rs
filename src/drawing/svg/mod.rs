mod path;
mod profile;

use svg::node::element::Group;
use svg::Document;

use crate::drawing::Drawing;
use crate::layout::Key;
use crate::utils::{Scale, Size, Trim};

use profile::Draw;

pub trait ToSvg {
    fn to_svg(&self) -> String;
}

impl ToSvg for Drawing {
    #[must_use]
    fn to_svg(&self) -> String {
        // scale from keyboard units to drawing units (milliunits)
        let scale = Scale::new(1e3, 1e3);
        let size = self.layout.size * scale;

        // w and h are in dpi units, the 0.75 is keyboard units per inch
        let Size { w, h } = self.layout.size * self.options.dpi * 0.75;

        let document = Document::new()
            .set("width", format!("{}", Trim(w)))
            .set("height", format!("{}", Trim(h)))
            .set("viewBox", format!("0 0 {:.0} {:.0}", size.w, size.h));

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
        let scale = Scale::new(1e3, 1e3);
        let pos = key.position * scale;

        let result = Group::new().set("transform", format!("translate({}, {})", pos.x, pos.y));

        let result = if self.options.show_keys {
            self.profile
                .draw_key(key)
                .into_iter()
                .fold(result, Group::add)
        } else {
            result
        };

        // let result =
        if self.options.show_margin {
            self.profile
                .draw_margin(key)
                .into_iter()
                .fold(result, Group::add)
        } else {
            result
        }

        // if show_legend_border {
        //     result.add(self.profile.draw_legend_rect(key));
        // }
        //
        // let result = result.add(self.font.draw_legends(key))
    }
}

#[cfg(test)]
mod tests {
    use crate::drawing::DrawingOptions;
    use crate::layout::tests::test_key;
    use crate::layout::Layout;
    use crate::profile::Profile;
    use crate::utils::Size;

    use super::*;

    #[test]
    fn test_to_svg() {
        let layout = Layout {
            size: Size::new(1., 1.),
            keys: vec![],
        };
        let profile = Profile::default();
        let options = DrawingOptions::default();
        let drawing = Drawing::new(layout, profile, options);

        assert_eq!(
            drawing.to_svg(),
            r#"<svg height="72" viewBox="0 0 1000 1000" width="72" xmlns="http://www.w3.org/2000/svg"/>"#
        );
    }

    #[test]
    fn test_draw_key() {
        let key = test_key();
        let layout = Layout {
            size: Size::new(1., 1.),
            keys: vec![],
        };

        let test_config = vec![
            (DrawingOptions::default(), 2),
            (
                DrawingOptions {
                    show_keys: false,
                    ..DrawingOptions::default()
                },
                0,
            ),
            (
                DrawingOptions {
                    show_margin: true,
                    ..DrawingOptions::default()
                },
                3,
            ),
        ];

        let profile = Profile::default();

        for (options, len) in test_config {
            let drawing = Drawing::new(layout.clone(), profile.clone(), options);
            let group = drawing.draw_key(&key);

            assert_eq!(group.get_children().len(), len);
        }
    }
}
