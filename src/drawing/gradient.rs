use std::collections::HashMap;
use std::fmt::{self, Debug, Display};

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

impl Display for GradientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

enum SvgGradient {
    Linear(LinearGradient),
    Radial(RadialGradient),
}

struct SvgGradients {
    gradients: HashMap<String, SvgGradient>,
}

impl SvgGradients {
    fn new() -> Self {
        Self {
            gradients: hashmap! {},
        }
    }

    fn add(self, id: String, color: Color, depth: f32, gradient_type: GradientType) -> Self {
        if self.gradients.contains_key(&id) {
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
                .gradients
                .into_iter()
                .chain(std::iter::once((id, gradient)))
                .collect();

            Self { gradients } // { gradients, ..self }
        }
    }

    fn into_defs(self) -> Defs {
        self.gradients
            .into_iter()
            .fold(Defs::new(), |defs, (_id, gradient)| match gradient {
                SvgGradient::Linear(grad) => defs.add(grad),
                SvgGradient::Radial(grad) => defs.add(grad),
            })
    }
}
