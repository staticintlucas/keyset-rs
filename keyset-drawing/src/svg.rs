use geom::{PathSegment, Scale, Unit, DOT_PER_UNIT, MM_PER_UNIT};
use svg::node::element::{Group, Path as SvgPath};
use svg::Document;

use super::{Drawing, KeyDrawing, KeyPath};

macro_rules! float {
    ($arg:expr $(,)?) => {
        format!("{}", float!(@round $arg))
    };
    ($arg0:expr, $($args:expr),+ $(,)?) => {
        format!("{}{}", float!(@round $arg0), float!(@inner $($args),+))
    };
    (@inner $arg:expr $(,)?) => {
        format_args!("{}", float!(@format $arg))
    };
    (@inner $arg0:expr, $($args:expr),+ $(,)?) => {
        format_args!("{}{}", float!(@format $arg0), float!(@inner $($args),+))
    };
    (@format $arg:expr) => {
        format_args!("{}{}", if $arg.is_sign_positive() { " " } else { "" }, float!(@round $arg))
    };
    (@round $arg:expr) => {
        ($arg * 1e3).round() / 1e3
    };
}

pub fn draw(drawing: &Drawing) -> String {
    let size = drawing.bounds.size() * Scale::<Unit, Unit>::new(drawing.scale) * MM_PER_UNIT;
    let view_box = drawing.bounds * DOT_PER_UNIT; // Use 1000 user units per key

    let document = Document::new()
        .set("width", format!("{}mm", float!(size.width)))
        .set("height", format!("{}mm", float!(size.height)))
        .set(
            "viewBox",
            float!(
                view_box.min.x,
                view_box.min.y,
                view_box.size().width,
                view_box.size().height
            ),
        );

    let document = drawing
        .keys
        .iter()
        .map(draw_key)
        .fold(document, Document::add);

    document.to_string()
}

fn draw_key(key: &KeyDrawing) -> Group {
    let origin = key.origin * DOT_PER_UNIT;
    let group = Group::new().set(
        "transform",
        format!("translate({},{})", float!(origin.x), float!(origin.y)),
    );
    key.paths.iter().map(draw_path).fold(group, Group::add)
}

fn draw_path(path: &KeyPath) -> SvgPath {
    let data: String = path
        .data
        .iter()
        .map(|el| match *el {
            PathSegment::Move(p) => format!("M{}", float!(p.x, p.y)),
            PathSegment::Line(d) => format!("l{}", float!(d.x, d.y)),
            PathSegment::CubicBezier(c1, c2, d) => {
                format!("c{}", float!(c1.x, c1.y, c2.x, c2.y, d.x, d.y))
            }
            // GRCOV_EXCL_START - no quads in example
            PathSegment::QuadraticBezier(c1, d) => format!("q{}", float!(c1.x, c1.y, d.x, d.y)),
            // GRCOV_EXCL_STOP
            PathSegment::Close => "z".into(),
        })
        .collect();

    let fill = path
        .fill
        .map_or_else(|| "none".to_owned(), |color| format!("{color:x}"));
    let svg_path = SvgPath::new().set("d", data).set("fill", fill);

    if let Some(outline) = path.outline {
        svg_path
            .set("stroke", format!("{:x}", outline.color))
            .set("stroke-width", float!(outline.width.get()))
    } else {
        svg_path.set("stroke", "none")
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use key::Key;

    use crate::{Drawing, Options};

    #[test]
    fn test_to_svg() {
        let options = Options {
            show_margin: true,
            ..Default::default()
        };
        let keys = [Key::example()];
        let drawing = Drawing::new(&keys, &options);

        let svg = drawing.to_svg();

        assert_eq!(
            svg,
            indoc!(
                r##"
                <svg height="19.05mm" viewBox="0 0 1000 1000" width="19.05mm" xmlns="http://www.w3.org/2000/svg">
                <g transform="translate(0,0)">
                <path d="M25 90c0-35.899 29.101-65 65-65l820 0c35.899 0 65 29.101 65 65l0 820c0 35.899-29.101 65-65 65l-820 0c-35.899 0-65-29.101-65-65z" fill="#cccccc" stroke="#aeaeae" stroke-width="10"/>
                <path d="M170 120c0-35.899 29.101-65 65-65l530 0c35.899 0 65 29.101 65 65l0 605c0 35.899-29.101 65-65 65l-530 0c-35.899 0-65-29.101-65-65z" fill="#cccccc" stroke="#aeaeae" stroke-width="10"/>
                <path d="M220 105l560 0l0 635l-560 0z" fill="none" stroke="#ff0000" stroke-width="5"/>
                <path d="M220 299.444l0-194.444l126.362 0l0 194.444l-126.362-0zM235.523 270.305l37.037-68.083l-37.037-68.083l0 136.166zM244.237 120.523l38.943 69.989l38.943-69.989l-77.887-0zM330.839 134.139l-37.037 68.083l37.037 68.083l0-136.166zM322.124 283.922l-38.943-69.989l-38.943 69.989l77.887 0z" fill="#000000" stroke="none"/>
                <path d="M653.638 299.444l0-194.444l126.362 0l0 194.444l-126.362-0zM669.161 270.305l37.037-68.083l-37.037-68.083l0 136.166zM677.876 120.523l38.943 69.989l38.943-69.989l-77.887-0zM764.477 134.139l-37.037 68.083l37.037 68.083l0-136.166zM755.763 283.922l-38.943-69.989l-38.943 69.989l77.887 0z" fill="#000000" stroke="none"/>
                <path d="M220 740l0-194.444l126.362 0l0 194.444l-126.362-0zM235.523 710.861l37.037-68.083l-37.037-68.083l0 136.166zM244.237 561.078l38.943 69.989l38.943-69.989l-77.887-0zM330.839 574.695l-37.037 68.083l37.037 68.083l0-136.166zM322.124 724.477l-38.943-69.989l-38.943 69.989l77.887 0z" fill="#000000" stroke="none"/>
                <path d="M653.638 740l0-194.444l126.362 0l0 194.444l-126.362-0zM669.161 710.861l37.037-68.083l-37.037-68.083l0 136.166zM677.876 561.078l38.943 69.989l38.943-69.989l-77.887-0zM764.477 574.695l-37.037 68.083l37.037 68.083l0-136.166zM755.763 724.477l-38.943-69.989l-38.943 69.989l77.887 0z" fill="#000000" stroke="none"/>
                </g>
                </svg>"##
            )
        );
    }
}
