mod gradient;

use std::collections::HashMap;

use maplit::hashmap;
use svg::node::element::Rectangle as Rect;
use svg::Document as SvgRoot;
use svg::Node as SvgNode;

use crate::utils::Color;

struct SvgDrawing {
    root: SvgRoot,
    gradients: HashMap<String, Box<dyn SvgNode>>,
}

impl SvgDrawing {
    fn new(width: f32, height: f32, background: Option<Color>) -> Self {
        let root = SvgRoot::new()
            .set("width", format!("{:.5}", width * (96. / 1000.)))
            .set("height", format!("{:.5}", height * (96. / 1000.)))
            .set("viewBox", format!("0 0 {:.5} {:.5}", width, height));

        let root = if let Some(bg_color) = background {
            root.add(
                Rect::new()
                    .set("x", "0")
                    .set("y", "0")
                    .set("width", format!("{:.5}", width))
                    .set("height", format!("{:.5}", height))
                    .set("fill", bg_color.to_hex()),
            )
        } else {
            root
        };

        Self {
            root,
            gradients: hashmap! {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_drawing_new() {
        let drawing1 = SvgDrawing::new(10., 10., None);
        assert_eq!(
            drawing1.root.to_string(),
            r#"<svg height="0.96000" viewBox="0 0 10.00000 10.00000" width="0.96000" xmlns="http://www.w3.org/2000/svg"/>"#
        );

        let drawing2 = SvgDrawing::new(10., 10., Some(Color::new(0., 0., 0.)));
        assert_eq!(
            drawing2.root.to_string(),
            r##"<svg height="0.96000" viewBox="0 0 10.00000 10.00000" width="0.96000" xmlns="http://www.w3.org/2000/svg">
<rect fill="#000000" height="10.00000" width="10.00000" x="0" y="0"/>
</svg>"##
        )
    }
}
