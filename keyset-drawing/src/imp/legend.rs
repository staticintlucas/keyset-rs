use font::{Font, FontUnit};
use geom::{
    Conversion, ConvertInto as _, Dot, Path, Point, Rect, Scale, Translate, Unit as _, Vector,
};
use profile::Profile;

use crate::Warning;

use super::KeyPath;

pub fn draw(
    legend: &::key::Legend,
    font: &Font,
    profile: &Profile,
    top_rect: Rect<Dot>,
    align: Scale, // Not really a scale, but it's a unitless vector so...
    warnings: &mut Vec<Warning>,
) -> KeyPath {
    // Get transform to correct height & flip y-axis
    let text_height = profile.text_height.get(legend.size_idx);
    let text_scale = text_height.get() / font.cap_height().get();
    let text_conv = Conversion::<Dot, FontUnit>::from_scale(text_scale, -text_scale);

    // Dimensions used to position text
    let line_height = Dot(font.line_height().get() * text_scale);
    #[expect(
        clippy::cast_precision_loss,
        reason = "if there are more lines than f32 allows we have bigger issues"
    )]
    let num_lines = legend.text.lines().count() as f32;
    let margin = top_rect - profile.text_margin.get(legend.size_idx);

    let text_path: Path<_> = legend
        .text
        .lines()
        .enumerate()
        .map(|(i, text)| {
            #[expect(clippy::cast_precision_loss, reason = "i <= 9")]
            let i = i as f32;
            let line_offset = num_lines - i - 1.0;

            let path = font.render_string(text) * text_conv;

            // Check to ensure our legend width fits (checked & scaled individually for each line)
            let legend_width = path.bounds.width();
            let margin_width = margin.width();

            let path = if legend_width > margin_width {
                warnings.push(Warning::LegendTooWide {
                    text: text.to_string(),
                    legend: legend_width.convert_into(),
                    bounds: margin_width.convert_into(),
                });

                path * Scale::new(margin_width / legend_width, 1.0)
            } else {
                path
            };

            path * Translate::new(Dot(0.0), -line_offset * line_height)
        })
        .collect();

    // Check to ensure our legend height fits
    let legend_height = text_path.bounds.height();
    let margin_height = margin.height();

    let text_path = if legend_height > margin_height {
        warnings.push(Warning::LegendTooTall {
            text: legend.text.to_string(),
            legend: legend_height.convert_into(),
            bounds: margin_height.convert_into(),
        });

        text_path * Scale::new(1.0, margin_height / legend_height)
    } else {
        text_path
    };

    // Calculate legend bounds. For x this is based on actual size while for y we use the base line
    // and text height so each character (especially symbols) are still aligned across keys
    let height = text_height + line_height * (num_lines - 1.0);
    let bounds = Rect::new(
        Point::new(text_path.bounds.min.x, -height),
        Point::new(text_path.bounds.max.x, Dot(0.0)),
    );

    // Align the legend within the margins
    let size = margin.size() - bounds.size();
    let point = margin.min + Vector::new(size.x * align.x, size.y * align.y);
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
        let top_rect = profile.top_rect().to_rect();
        let align = Scale::new(0.0, 0.0);

        let mut warnings = Vec::new();
        let path = draw(&legend, &font, &profile, top_rect, align, &mut warnings);

        assert_eq!(
            path.data
                .into_iter()
                .filter(|el| matches!(*el, PathSegment::Move(..)))
                .count(),
            3
        );
        assert!(warnings.is_empty());

        let legend = ::key::Legend {
            text: Text::parse_from("😎"),
            size_idx: 5,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let align = Scale::new(1.0, 1.0);
        let mut warnings = Vec::new();
        let path = draw(&legend, &font, &profile, top_rect, align, &mut warnings);

        assert_eq!(path.data.len(), 12); // == .notdef length
        assert!(warnings.is_empty());

        let legend = ::key::Legend {
            text: Text::parse_from("Some really long legend that will totally need to be squished"),
            size_idx: 3,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let align = Scale::new(1.0, 1.0);
        let mut warnings = Vec::new();
        let path = draw(&legend, &font, &profile, top_rect, align, &mut warnings);

        assert_is_close!(
            path.data.bounds.width(),
            (profile.top_rect().to_rect() - profile.text_margin.get(legend.size_idx)).width()
        );
        assert!(warnings
            .iter()
            .any(|w| matches!(*w, Warning::LegendTooWide { .. })));

        let legend = ::key::Legend {
            text: Text::parse_from("One<br>Two"),
            size_idx: 3,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let align = Scale::new(1.0, 1.0);
        let mut warnings = Vec::new();
        let path = draw(&legend, &font, &profile, top_rect, align, &mut warnings);

        assert!(path.data.bounds.height() > profile.text_height.get(legend.size_idx) * 2.0);
        assert!(warnings.is_empty());

        let legend = ::key::Legend {
            text: Text::parse_from("Too<br>many<br>lines<br>to<br>realistically<br>fit"),
            size_idx: 3,
            color: Color::new(0.0, 0.0, 0.0),
        };
        let align = Scale::new(1.0, 1.0);
        let mut warnings = Vec::new();
        let path = draw(&legend, &font, &profile, top_rect, align, &mut warnings);

        assert_is_close!(
            path.data.bounds.height(),
            (profile.top_rect().to_rect() - profile.text_margin.get(legend.size_idx)).height()
        );
        assert!(warnings
            .iter()
            .any(|w| matches!(*w, Warning::LegendTooTall { .. })));
    }
}
