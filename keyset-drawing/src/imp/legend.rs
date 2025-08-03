use log::warn;
use saturate::SaturatingFrom as _;

use font::{Font, FontUnit};
use geom::{Conversion, Dot, Path, Point, Rect, Scale, Translate, Unit as _, Vector};
use profile::Profile;

use super::KeyPath;

pub fn draw(
    legend: &::key::Legend,
    font: &Font,
    profile: &Profile,
    top_rect: Rect<Dot>,
    align: Scale, // Not really a scale, but it's a unitless vector so...
) -> KeyPath {
    // Get transform to correct height & flip y-axis
    let text_height = profile.text_height.get(legend.size_idx);
    let text_scale = text_height.length.get() / font.cap_height().length.get();
    let text_conv = Conversion::<Dot, FontUnit>::from_scale(text_scale, -text_scale);

    // Dimensions used to position text
    let line_height = Dot(font.line_height().length.get() * text_scale);
    let n_lines = legend.text.lines().count();
    let margin = top_rect - profile.text_margin.get(legend.size_idx);

    let text_path: Path<_> = legend
        .text
        .lines()
        .enumerate()
        .map(|(i, text)| {
            let line_offset = n_lines - i - 1;

            let path = font.render_string(text) * text_conv;

            // Check to ensure our legend fits
            let width_factor = path.bounds.width() / margin.width();
            if width_factor > 1.0 {
                let percent = 100.0 * (width_factor - 1.0);
                warn!(r#"legend "{text}" is {percent}% too wide; squishing legend to fit"#);
            }
            let width_factor = width_factor.max(1.0);

            path * Translate::from_units(Dot(0.0), -f32::saturating_from(line_offset) * line_height)
                * Scale::new(1.0 / width_factor, 1.0)
        })
        .collect();

    // Calculate legend bounds. For x this is based on actual size while for y we use the base line
    // and text height so each character (especially symbols) are still aligned across keys
    let height = text_height.length + line_height * f32::saturating_from(n_lines - 1);
    let bounds = Rect::new(
        Point::from_units(text_path.bounds.min.x, -height),
        Point::from_units(text_path.bounds.max.x, Dot(0.0)),
    );

    // Align the legend within the margins
    let size = margin.size() - bounds.size();
    let point = margin.min + Vector::from_units(size.x * align.x, size.y * align.y);
    let text_path = text_path * Translate::from(point - bounds.min);

    KeyPath {
        data: text_path,
        outline: None,
        fill: Some(legend.color),
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use color::Color;
    use geom::PathSegment;
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
        let top_rect = profile.top_with_size(Vector::new(1.0, 1.0)).to_rect();
        let path = draw(&legend, &font, &profile, top_rect, Scale::new(0.0, 0.0));

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
        let path = draw(&legend, &font, &profile, top_rect, Scale::new(1.0, 1.0));

        assert_eq!(path.data.len(), 12); // == .notdef length

        let legend = ::key::Legend {
            text: Text::parse_from("Some really long legend that will totally need to be squished"),
            size_idx: 5,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let path = draw(&legend, &font, &profile, top_rect, Scale::new(1.0, 1.0));

        assert_is_close!(
            path.data.bounds.width(),
            (profile.top_with_size(Vector::new(1.0, 1.0)).to_rect() - profile.text_margin.get(5))
                .width()
        );

        let legend = ::key::Legend {
            text: Text::parse_from("Two<br>lines!"),
            size_idx: 5,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let path = draw(&legend, &font, &profile, top_rect, Scale::new(1.0, 1.0));

        assert!(path.data.bounds.height() > profile.text_height.get(legend.size_idx).length * 2.0);
    }
}
