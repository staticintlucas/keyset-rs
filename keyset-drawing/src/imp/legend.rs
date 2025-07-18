use log::warn;
use saturate::SaturatingFrom as _;

use font::Font;
use geom::{Dot, Path, Point, Rect, ToTransform as _, Vector};
use profile::Profile;

use super::KeyPath;

pub fn draw(
    legend: &::key::Legend,
    font: &Font,
    profile: &Profile,
    top_rect: Rect<Dot>,
    align: Vector<()>,
) -> KeyPath {
    // Get transform to correct height & flip y-axis
    let text_height = profile.text_height.get(legend.size_idx);
    let text_scale = text_height / font.cap_height();
    let text_xform = text_scale.to_transform().then_scale(1.0, -1.0);

    // Dimensions used to position text
    let line_height = font.line_height() * text_scale;
    let n_lines = f32::saturating_from(legend.text.lines().count());
    let margin = top_rect.inner_box(profile.text_margin.get(legend.size_idx));

    let text_path: Path<_> = legend
        .text
        .lines()
        .enumerate()
        .map(|(i, text)| {
            let line_offset = n_lines - f32::saturating_from(i) - 1.0;

            let path = font.render_string(text) * text_xform;
            let width = path.bounds.width();

            // Check to ensure our legend fits
            let h_scale = if width > margin.width() {
                let percent = 100.0 * (width / margin.width() - 1.0);
                warn!(r#"legend "{text}" is {percent}% too wide; squishing legend to fit"#);
                margin.width() / width
            } else {
                1.0
            };

            path.translate(Vector::new(
                -width * align.x,
                -line_offset * line_height.get(),
            ))
            .scale(h_scale, 1.0)
        })
        .collect();

    // Calculate legend bounds. For x this is based on actual size while for y we use the base line
    // and text height so each character (especially symbols) are still aligned across keys
    let height = text_height + line_height * (n_lines - 1.0);
    let bounds = Rect::new(
        Point::new(text_path.bounds.min.x, -height.get()),
        Point::new(text_path.bounds.max.x, 0.0),
    );

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
    use isclose::assert_is_close;

    use color::Color;
    use geom::{PathSegment, Size};
    use key::Text;

    use super::*;

    #[test]
    fn test_legend_draw() {
        let legend = ::key::Legend {
            text: Text::parse_from("AV"),
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
            text: Text::parse_from("😎"),
            size_idx: 5,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let path = draw(&legend, &font, &profile, top_rect, Vector::new(1.0, 1.0));

        assert_eq!(path.data.len(), 12); // == .notdef length

        let legend = ::key::Legend {
            text: Text::parse_from("Some really long legend that will totally need to be squished"),
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

        let legend = ::key::Legend {
            text: Text::parse_from("Two<br>lines!"),
            size_idx: 5,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let path = draw(&legend, &font, &profile, top_rect, Vector::new(1.0, 1.0));

        assert!(path.data.bounds.height() > profile.text_height.get(legend.size_idx).get() * 2.0);
    }
}
