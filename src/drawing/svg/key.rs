use svg::node::element::path::Data;
use svg::node::element::{Group, Path, Rectangle};

use super::ToSvg;
use crate::layout::Key;
use crate::utils::{Point, Size};

impl ToSvg<Group> for Key {
    fn to_svg(&self) -> Group {
        let Point { x, y } = self.position;
        let Size { w, h } = self.size.size();

        let bottom = Rectangle::new()
            .set("x", "20")
            .set("y", "20")
            .set("width", format!("{}", (w - 0.04) * 1e3))
            .set("height", format!("{}", (h - 0.04) * 1e3))
            .set("fill", self.key_color.to_hex())
            .set("stroke", self.key_color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
            .set("rx", "15")
            .set("ry", "15");

        // TODO vary based on profile, homing, space, etc.

        // Radius of the curved edge at the bottom of the key top rectangle
        let width = 620f32 + 1e3 * (w - 1.);
        let height = 730f32 + 1e3 * (h - 1.);
        let radius = 80f32;
        let offset = -85f32;
        let curve = (0.5f32 / 19.05 * 1e3) * 0.381;
        let hr = (curve.powf(2.) + ((width - 2. * radius).powf(2.) / 4.)) / (2. * curve);
        let top = Path::new()
            .set("fill", self.key_color.to_hex())
            .set("stroke", self.key_color.highlight(0.15).to_hex())
            .set("stroke-width", "10")
            .set(
                "d",
                Data::new()
                    .move_to((
                        0.5 * (1e3 * w - width),
                        0.5 * (1e3 * h - height) + offset + radius,
                    ))
                    .elliptical_arc_by((radius, radius, 0., 0., 1., radius, -radius))
                    .horizontal_line_by(width - 2. * radius)
                    .elliptical_arc_by((radius, radius, 0., 0., 1., radius, radius))
                    .vertical_line_by(height - 2. * radius)
                    .elliptical_arc_by((radius, radius, 0., 0., 1., -radius, radius))
                    .elliptical_arc_by((hr, hr, 0., 0., 1., -(width - 2. * radius), 0.))
                    .elliptical_arc_by((radius, radius, 0., 0., 1., -radius, -radius))
                    .close(),
            );

        Group::new()
            .set("transform", format!("translate({}, {})", x * 1e3, y * 1e3))
            .add(bottom)
            .add(top)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::layout::{KeySize, KeyType};
    use crate::utils::Color;

    #[test]
    fn test_to_svg() {
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
        let attr = elem.get_attributes();
        assert_eq!(&*attr["transform"], "translate(2000, 1000)");

        let children = elem.get_children();
        assert_eq!(children.len(), 2);
    }
}
