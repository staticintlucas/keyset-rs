use font::{Font, Glyph};
use geom::{Dot, Path, Point, Rect, ToTransform, Vector};
use log::warn;
use profile::Profile;

use super::KeyPath;

pub fn draw(
    legend: &::key::Legend,
    font: &Font,
    profile: &Profile,
    top_rect: Rect<Dot>,
    align: Vector<Dot>,
) -> KeyPath {
    let mut chars = legend.text.chars();
    let Some(first) = chars.next() else {
        unreachable!() // We should never have an empty legend here
    };
    let first = font.glyph_or_default(first);
    let mut char_paths = Vec::with_capacity(legend.text.chars().count());
    char_paths.push(first.path);
    let mut pos = first.advance;

    for (lhs, rhs) in legend.text.chars().zip(chars) {
        pos += font.kerning(lhs, rhs);
        let Glyph { path, advance, .. } = font.glyph_or_default(rhs);
        char_paths.push(path.translate(Vector::new(pos.get(), 0.0)));
        pos += advance;
    }

    let height = profile.text_height.get(legend.size_idx);
    // Scale to correct height & flip y-axis
    let transform = (height / font.cap_height())
        .to_transform()
        .then_scale(1.0, -1.0);
    let text_path = Path::from_slice(&char_paths) * transform;

    // Calculate legend bounds. For x this is based on actual size while for y we use the base line
    // and text height so each character (especially symbols) are still aligned across keys
    let bounds = Rect::new(
        Point::new(text_path.bounds.min.x, -height.get()),
        Point::new(text_path.bounds.max.x, 0.0),
    );

    // Check to ensure our legend fits
    let margin = top_rect.inner_box(profile.text_margin.get(legend.size_idx));
    let text_path = if bounds.width() > margin.width() {
        let text = &legend.text;
        let percent = 100.0 * (bounds.width() / margin.width() - 1.0);
        warn!(r#"legend "{text}" is {percent}% too wide; squishing legend to fit"#);
        text_path.scale(margin.width() / bounds.width(), 1.0)
    } else {
        text_path
    };

    // Align the legend within the margins
    let size = margin.size() - bounds.size();
    let point = margin.min + Vector::new(align.x * size.width, align.y * size.height);
    let text_path = text_path.translate(point - bounds.min);

    KeyPath {
        data: text_path,
        outline: None,
        fill: Some(legend.color),
    }
}

#[cfg(test)]
mod tests {
    use color::Color;
    use geom::{PathSegment, Size};
    use isclose::assert_is_close;

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
        let top_rect = profile.top_with_size(Size::new(1.0, 1.0)).rect();
        let path = draw(&legend, &font, &profile, top_rect, Vector::zero());

        assert_eq!(
            path.data
                .into_iter()
                .filter(|el| matches!(*el, PathSegment::Move(..)))
                .count(),
            3
        );

        let legend = ::key::Legend {
            text: "😎".into(),
            size_idx: 5,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let path = draw(&legend, &font, &profile, top_rect, Vector::new(1.0, 1.0));

        assert_eq!(path.data.len(), font.notdef().path.len());

        let legend = ::key::Legend {
            text: "Some really long legend that will totally need to be squished".into(),
            size_idx: 5,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let path = draw(&legend, &font, &profile, top_rect, Vector::new(1.0, 1.0));

        assert_is_close!(
            path.data.bounds.width(),
            (profile
                .top_with_size(Size::new(1.0, 1.0))
                .rect()
                .inner_box(profile.text_margin.get(5)))
            .width()
        );
    }
}
