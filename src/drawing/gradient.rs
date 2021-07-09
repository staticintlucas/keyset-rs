use std::collections::HashMap;
use std::fmt::Debug;

use maplit::hashmap;
use svg::node::element::{Definitions as Defs, LinearGradient, RadialGradient, Stop};

use crate::utils::Color;

// This constant controls the lightening and darkening of the gradient w.r.t. the depth of a key.
// A larger value decreases the contrast, and vice versa. This is a pretty good value for all
// reasonable depths (<1.5mm), but we might want to use a different method to determine these
// gradients in future
const DEPTH_CONSTANT: f32 = 525.;

#[derive(Debug, Clone, Copy)]
enum GradientType {
    HorizontalLinear,
    VerticalLinear,
    Radial,
}

#[derive(Debug, Clone)]
enum SvgGradient {
    Linear(LinearGradient),
    Radial(RadialGradient),
}

#[derive(Debug, Clone)]
struct SvgGradients(HashMap<String, SvgGradient>);

impl SvgGradients {
    fn new() -> Self {
        Self(hashmap! {})
    }

    fn add(self, id: String, color: Color, depth: f32, gradient_type: GradientType) -> Self {
        if self.0.contains_key(&id) {
            self
        } else {
            // Generate the gradient type
            let gradient = match gradient_type {
                GradientType::HorizontalLinear => SvgGradient::Linear(
                    LinearGradient::new()
                        .set("id", &id[..])
                        .set("x1", "100%")
                        .set("y1", "0%")
                        .set("x2", "0%")
                        .set("y2", "0%"),
                ),
                GradientType::VerticalLinear => SvgGradient::Linear(
                    LinearGradient::new()
                        .set("id", &id[..])
                        .set("x1", "0%")
                        .set("y1", "0%")
                        .set("x2", "0%")
                        .set("y2", "100%"),
                ),
                GradientType::Radial => SvgGradient::Radial(
                    RadialGradient::new()
                        .set("id", &id[..])
                        .set("cx", "100%")
                        .set("cy", "100%")
                        .set("fx", "100%")
                        .set("fy", "100%")
                        .set("fr", "0%")
                        // Radius is √2 so that it reaches the diagonally opposite corner
                        .set("r", "141%"),
                ),
            };

            // The three color values of the gradient and the offsets where they occur
            let colors = [
                color.lighter(depth / 525.),
                color,
                color.darker(depth / 525.),
            ];
            let offsets = ["0%", "50%", "100%"];

            // Generate the three gradient stops
            let stops = colors.iter().zip(offsets.iter()).map(|(&color, &offset)| {
                Stop::new()
                    .set("offset", offset)
                    .set("color", color.to_hex())
            });

            // Add the stops to the gradient
            let gradient = stops.fold(gradient, |gradient, stop| match gradient {
                SvgGradient::Linear(grad) => SvgGradient::Linear(grad.add(stop)),
                SvgGradient::Radial(grad) => SvgGradient::Radial(grad.add(stop)),
            });

            // And add the gradient to the map and return a new Self
            let gradients = self
                .0
                .into_iter()
                .chain(std::iter::once((id, gradient)))
                .collect();

            Self(gradients)
        }
    }

    fn into_defs(self) -> Defs {
        self.0
            .into_iter()
            .fold(Defs::new(), |defs, (_id, gradient)| match gradient {
                SvgGradient::Linear(grad) => defs.add(grad),
                SvgGradient::Radial(grad) => defs.add(grad),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_gradient_new() {
        let gradients = SvgGradients::new();
        assert_eq!(gradients.0.len(), 0);
    }

    #[test]
    fn test_svg_gradient_add() {
        let gradients = SvgGradients::new()
            .add(
                "test1".into(),
                Color::new(0.8, 0., 0.),
                52.5,
                GradientType::HorizontalLinear,
            )
            .add(
                "test2".into(),
                Color::new(0.8, 0., 0.),
                52.5,
                GradientType::VerticalLinear,
            )
            .add(
                "test3".into(),
                Color::new(0.8, 0., 0.),
                52.5,
                GradientType::Radial,
            );

        assert_eq!(gradients.0.len(), 3);
        assert!(matches!(
            gradients.0["test1".into()],
            SvgGradient::Linear(_)
        ));
        assert!(matches!(
            gradients.0["test2".into()],
            SvgGradient::Linear(_)
        ));
        assert!(matches!(
            gradients.0["test3".into()],
            SvgGradient::Radial(_)
        ));

        let gradients = gradients.add(
            "test1".into(),
            Color::new(0.8, 0., 0.),
            0.5,
            GradientType::Radial,
        );
        assert_eq!(gradients.0.len(), 3);
    }

    #[test]
    fn test_svg_gradient_into_defs() {
        let linear = SvgGradients::new().add(
            "test".into(),
            Color::new(0.8, 0., 0.),
            52.5,
            GradientType::HorizontalLinear,
        );
        let radial = SvgGradients::new().add(
            "test".into(),
            Color::new(0.8, 0., 0.),
            52.5,
            GradientType::Radial,
        );

        assert_eq!(
            svg::Document::new().add(linear.into_defs()).to_string(),
            r##"<svg xmlns="http://www.w3.org/2000/svg">
<defs>
<linearGradient id="test" x1="100%" x2="0%" y1="0%" y2="0%">
<stop color="#d11a1a" offset="0%"/>
<stop color="#cc0000" offset="50%"/>
<stop color="#b80000" offset="100%"/>
</linearGradient>
</defs>
</svg>"##
        );

        assert_eq!(
            svg::Document::new().add(radial.into_defs()).to_string(),
            r##"<svg xmlns="http://www.w3.org/2000/svg">
<defs>
<radialGradient cx="100%" cy="100%" fr="0%" fx="100%" fy="100%" id="test" r="141%">
<stop color="#d11a1a" offset="0%"/>
<stop color="#cc0000" offset="50%"/>
<stop color="#b80000" offset="100%"/>
</radialGradient>
</defs>
</svg>"##
        );
    }
}
