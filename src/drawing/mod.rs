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
