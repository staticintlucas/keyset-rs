use geom::{Affine, Rect, Shape, Vec2};
use itertools::Itertools;
use log::warn;

use crate::font::Font;
use profile::Profile;

use super::Path;

pub(crate) fn draw(
    legend: &::key::Legend,
    font: &Font,
    profile: &Profile,
    top_rect: Rect,
    align: Vec2,
) -> Path {
    let text = &legend.text;
    let Some(first) = text.chars().next() else {
        unreachable!()
    };
    let first = font.glyphs.get(&first).unwrap_or(&font.notdef);

    let mut path = text
        .chars()
        .tuple_windows()
        .map(|(lhs, rhs)| {
            let glyph = font.glyphs.get(&rhs).unwrap_or(&font.notdef);
            let kern = font.kerning.get(lhs, rhs);
            (glyph, kern)
        })
        .scan(first.advance, |pos, (glyph, kern)| {
            let result = Some((*pos + kern, glyph));
            *pos += kern + glyph.advance;
            result
        })
        .fold(first.path.clone(), |mut path, (pos, glyph)| {
            let mut p = glyph.path.clone();
            p.apply_affine(Affine::translate((pos, 0.)));
            path.extend(p);
            path
        });

    let height = profile.text_height.get(legend.size_idx);
    path.apply_affine(Affine::scale(height / font.cap_height)); // Scale to correct height

    // Calculate legend bounds. For x this is based on actual size while for y we use the base line
    // and text height so each character (especially symbols) are still aligned across keys
    let bounds = path.bounding_box();
    let bounds = bounds
        .with_origin((bounds.origin().x, -height))
        .with_size((bounds.width(), height));

    // Check to ensure our legend fits
    let margin = top_rect + profile.text_margin.get(legend.size_idx);
    if bounds.width() > margin.width() {
        let percent = 100. * (bounds.width() / margin.width() - 1.);
        warn!(r#"legend "{text}" is {percent}% too wide; squishing legend to fit"#);
        path.apply_affine(Affine::scale_non_uniform(
            margin.width() / bounds.width(),
            1.,
        ));
    }

    // Align the legend within the margins
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
    use assert_approx_eq::assert_approx_eq;
    use geom::PathEl;

    use color::Color;

    use super::*;

    #[test]
    fn test_legend_draw() {
        let legend = ::key::Legend {
            text: "AV".into(),
            size_idx: 5,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let font = Font::from_ttf(
            &std::fs::read(concat!(env!("CARGO_WORKSPACE_DIR"), "tests/fonts/demo.ttf")).unwrap(),
        )
        .unwrap();
        let profile = Profile::default();
        let top_rect = profile.top_with_size((1., 1.)).rect();
        let path = draw(&legend, &font, &profile, top_rect, Vec2::new(0., 0.));

        assert_eq!(
            path.path
                .into_iter()
                .filter(|el| matches!(el, PathEl::MoveTo(..)))
                .count(),
            3
        );

        let legend = ::key::Legend {
            text: "😎".into(),
            size_idx: 5,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let path = draw(&legend, &font, &profile, top_rect, Vec2::new(1., 1.));

        assert_eq!(
            path.path.into_iter().count(),
            font.notdef.path.iter().count()
        );

        let legend = ::key::Legend {
            text: "Some really long legend that will totally need to be squished".into(),
            size_idx: 5,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let path = draw(&legend, &font, &profile, top_rect, Vec2::new(1., 1.));

        assert_approx_eq!(
            path.path.bounding_box().width(),
            (profile.top_with_size((1., 1.)).rect() + profile.text_margin.get(5)).width()
        );
    }
}
