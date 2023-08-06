use itertools::Itertools;
use kurbo::{Affine, Rect, Shape, Vec2};

use crate::font::Font;
use crate::key::Legend;
use crate::profile::Profile;

use super::Path;

pub(crate) fn draw(
    legend: &Legend,
    font: &Font,
    profile: &Profile,
    top_rect: Rect,
    align: Vec2,
) -> Path {
    let text = &legend.text;
    let Some(first) = text.chars().next() else { unreachable!() };
    let first = font.glyphs.get(&first).unwrap_or(&font.notdef);

    let mut path = text
        .chars()
        .map(|ch| font.glyphs.get(&ch).unwrap_or(&font.notdef))
        .tuple_windows()
        .scan(0., |pos, (lhs, rhs)| {
            let kern = Option::zip(lhs.codepoint, rhs.codepoint)
                .map_or(0., |(l, r)| font.kerning.get(l, r));
            *pos += lhs.advance + kern;
            Some((*pos, rhs))
        })
        .fold(first.path.clone(), |mut path, (pos, glyph)| {
            let mut p = glyph.path.clone();
            p.apply_affine(Affine::translate((pos, 0.)));
            path.extend(p);
            path
        });

    let scale = profile.text_height.get(legend.size) / font.cap_height;
    path.apply_affine(Affine::scale(scale));

    let margin = top_rect + profile.text_margin.get(legend.size);
    let bounds = path.bounding_box();
    if bounds.width() > margin.width() {
        // TODO warn about shrinking legends?
        path.apply_affine(Affine::scale_non_uniform(
            margin.width() / bounds.width(),
            1.,
        ));
    }
    let size = margin.size() - bounds.size();
    let point = margin.origin() + (align.x * size.width, align.y * size.height);
    path.apply_affine(Affine::translate(point - bounds.origin()));

    Path {
        path,
        outline: None,
        fill: Some(legend.color),
    }
}

#[cfg(test)]
mod tests {
    use kurbo::PathEl;

    use crate::utils::Color;

    use super::*;

    #[test]
    fn test_legend_draw() {
        let legend = Legend {
            text: "AV".into(),
            size: 5,
            color: Color::new(0, 0, 0),
        };
        let font = Font::from_ttf(&std::fs::read("tests/fonts/demo.ttf").unwrap()).unwrap();
        let profile = Profile::default();
        let top_rect = profile.top_rect.rect();
        let path = draw(&legend, &font, &profile, top_rect, Vec2::new(0., 0.));

        assert_eq!(
            path.path
                .into_iter()
                .filter(|el| matches!(el, PathEl::MoveTo(..)))
                .count(),
            3
        );

        let legend = Legend {
            text: "😎".into(),
            size: 5,
            color: Color::new(0, 0, 0),
        };
        let path = draw(&legend, &font, &profile, top_rect, Vec2::new(1., 1.));

        assert_eq!(
            path.path.into_iter().count(),
            font.notdef.path.into_iter().count()
        );
    }
}