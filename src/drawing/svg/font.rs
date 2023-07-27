use itertools::Itertools;
use kurbo::{Affine, BezPath, Shape, Vec2};
use svg::node::element::Path as SvgPath;

use crate::font::Font;
use crate::key::Key;
use crate::profile::Profile;

pub trait Draw {
    fn draw_legends(&self, profile: &Profile, key: &Key) -> Vec<SvgPath>;
}

impl Draw for Font {
    fn draw_legends(&self, profile: &Profile, key: &Key) -> Vec<SvgPath> {
        let mut legends = vec![];

        for (i, row) in key.legends.iter().enumerate() {
            for (j, legend) in row.iter().enumerate() {
                let legend = if let Some(l) = legend { l } else { continue };

                let mut path = self.text_path(&legend.text);

                let scale = profile.text_height.get(legend.size) / self.cap_height;
                path.apply_affine(Affine::scale(scale));

                let align = Vec2::new(
                    (j as f64) / ((key.legends.len() - 1) as f64),
                    (i as f64) / ((key.legends[0].len() - 1) as f64),
                );
                let margin = profile.text_margin.get(legend.size);
                let bounds = path.bounding_box();
                let size = margin.size() - bounds.size();
                let point = margin.origin() + (align.x * size.width, align.y * size.height);
                path.apply_affine(Affine::translate(point - bounds.origin()));

                let svg_path = SvgPath::new()
                    .set("d", path.to_svg())
                    .set("fill", legend.color.to_string())
                    .set("stroke", "none");

                legends.push(svg_path);
            }
        }

        legends
    }
}

impl Font {
    fn text_path(&self, text: &str) -> BezPath {
        let mut path = BezPath::new();

        let first = if let Some(first) = text.chars().next() {
            self.glyphs.get(&first).unwrap_or(&self.notdef)
        } else {
            return path;
        };

        path.extend(first.path.clone());
        text.chars()
            .map(|char| self.glyphs.get(&char).unwrap_or(&self.notdef))
            .tuple_windows()
            .scan(0., |pos, (lhs, rhs)| {
                *pos += lhs.advance
                    + lhs
                        .codepoint
                        .zip(rhs.codepoint)
                        .map_or(0., |(l, r)| self.kerning.get(l, r));
                Some((*pos, rhs))
            })
            .for_each(|(pos, glyph)| {
                let mut glyph = glyph.path.clone();
                glyph.apply_affine(Affine::translate((pos, 0.)));
                path.extend(glyph);
            });
        path
    }
}

#[cfg(test)]
mod tests {
    use kurbo::PathEl;

    use super::*;

    #[test]
    fn test_draw_legends() {
        let font = Font::from_ttf(&std::fs::read("tests/fonts/demo.ttf").unwrap()).unwrap();
        let profile = Profile::default();
        let key = Key::example();
        let path = font.draw_legends(&profile, &key);

        assert_eq!(path.len(), 4);
        path.into_iter()
            .for_each(|p| assert!(p.get_attributes().contains_key("d")));
    }

    #[test]
    fn test_text_path() {
        let font = Font::from_ttf(&std::fs::read("tests/fonts/demo.ttf").unwrap()).unwrap();
        let path = font.text_path("AV");
        assert_eq!(
            path.into_iter()
                .filter(|seg| matches!(seg, PathEl::MoveTo(..)))
                .count(),
            3
        );

        let path = font.text_path("");
        assert_eq!(path.into_iter().count(), 0);
    }
}
