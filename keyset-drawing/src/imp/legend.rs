use font::{Font, Glyph};
use geom::{Affine, Rect, Shape, Vec2};
use log::warn;
use profile::Profile;

use super::Path;

pub(crate) fn draw(
    legend: &::key::Legend,
    font: &Font,
    profile: &Profile,
    top_rect: Rect,
    align: Vec2,
) -> Path {
    let mut chars = legend.text.chars();
    let Some(first) = chars.next() else {
        unreachable!() // We should never have an empty legend here
    };
    let first = font.glyph_or_default(first);
    let mut text_path = first.path;
    let mut pos = first.advance;

    for (lhs, rhs) in legend.text.chars().zip(chars) {
        let Glyph { path, advance, .. } = font.glyph_or_default(rhs);
        text_path.extend(Affine::translate((pos, 0.0)) * path);
        pos += advance + font.kerning(lhs, rhs);
    }

    let height = profile.text_height.get(legend.size_idx);
    text_path.apply_affine(Affine::scale(height / font.cap_height())); // Scale to correct height

    // Calculate legend bounds. For x this is based on actual size while for y we use the base line
    // and text height so each character (especially symbols) are still aligned across keys
    let bounds = text_path.bounding_box();
    let bounds = bounds
        .with_origin((bounds.origin().x, -height))
        .with_size((bounds.width(), height));

    // Check to ensure our legend fits
    let margin = top_rect + profile.text_margin.get(legend.size_idx);
    if bounds.width() > margin.width() {
        let text = &legend.text;
        let percent = 100. * (bounds.width() / margin.width() - 1.);
        warn!(r#"legend "{text}" is {percent}% too wide; squishing legend to fit"#);
        text_path.apply_affine(Affine::scale_non_uniform(
            margin.width() / bounds.width(),
            1.,
        ));
    }

    // Align the legend within the margins
    let size = margin.size() - bounds.size();
    let point = margin.origin() + (align.x * size.width, align.y * size.height);
    text_path.apply_affine(Affine::translate(point - bounds.origin()));

    Path {
        path: text_path,
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
        let font = Font::from_ttf(std::fs::read(env!("DEMO_TTF")).unwrap()).unwrap();
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
            path.path.elements().len(),
            font.notdef().path.elements().len()
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
