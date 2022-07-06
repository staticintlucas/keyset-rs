mod gradient;

use svg::node::element::Rectangle as Rect;
use svg::{Document as Svg, Node};

use self::gradient::SvgGradients;
use crate::utils::Color;

#[derive(Debug, Clone)]
struct SvgDrawing {
    width: f32,
    height: f32,
    background: Option<Color>,
    gradients: SvgGradients,
}

impl SvgDrawing {
    fn new(width: f32, height: f32, background: Option<Color>) -> Self {
        Self {
            width,
            height,
            background,
            gradients: SvgGradients::new(),
        }
    }

    fn draw(self) -> Svg {
        let mut root = Svg::new()
            .set("width", format!("{:.5}", self.width * (96. / 1000.)))
            .set("height", format!("{:.5}", self.height * (96. / 1000.)))
            .set(
                "viewBox",
                format!("0 0 {:.5} {:.5}", self.width, self.height),
            );

        root.append(self.gradients.into_defs());

        if let Some(bg_color) = self.background {
            root.append(
                Rect::new()
                    .set("x", "0")
                    .set("y", "0")
                    .set("width", format!("{:.5}", self.width))
                    .set("height", format!("{:.5}", self.height))
                    .set("fill", bg_color.to_hex()),
            );
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_svg_drawing_new() {
        let drawing1 = SvgDrawing::new(10., 10., None);
        assert_approx_eq!(drawing1.width, 10.);
        assert_approx_eq!(drawing1.height, 10.);
        assert_eq!(drawing1.background, None);

        let drawing2 = SvgDrawing::new(10., 10., Some(Color::new(0., 0., 0.)));
        assert_eq!(drawing2.background, Some(Color::new(0., 0., 0.)));
    }

    #[test]
    fn test_svg_drawing_draw() {
        let drawing = SvgDrawing::new(10., 10., Some(Color::new(0., 0., 0.)));

        assert_eq!(
            drawing.draw().to_string(),
            r##"<svg height="0.96000" viewBox="0 0 10.00000 10.00000" width="0.96000" xmlns="http://www.w3.org/2000/svg">
<defs/>
<rect fill="#000000" height="10.00000" width="10.00000" x="0" y="0"/>
</svg>"##
        )
    }
}
