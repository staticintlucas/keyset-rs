use itertools::Itertools;
use kurbo::{Affine, PathEl, Point};
use svg::node::element::{Group, Path as SvgPath};
use svg::Document;

use crate::drawing::Drawing;

use super::{KeyDrawing, Path};

macro_rules! fmt_num {
    ($fmt:literal, $($args:expr),*) => {
        format!($fmt, $(($args * 1e5).round() / 1e5),*)
    };
}

pub(crate) fn draw(drawing: &Drawing) -> String {
    let size = drawing.bounds.size() * drawing.scale;
    let view_box = drawing.bounds.scale_from_origin(1e3); // Use 1000 user units per key

    let document = Document::new()
        .set("width", fmt_num!("{}mm", size.width * 19.05))
        .set("height", fmt_num!("{}mm", size.height * 19.05))
        .set(
            "viewBox",
            fmt_num!(
                "{} {} {} {}",
                view_box.origin().x,
                view_box.origin().y,
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
    // scale from keyboard units to drawing units (milliunits)
    let pos = Affine::scale(1e3) * key.origin;

    let group = Group::new().set("transform", fmt_num!("translate({}, {})", pos.x, pos.y));

    key.paths.iter().map(draw_path).fold(group, Group::add)
}

fn draw_path(path: &Path) -> SvgPath {
    let data = path
        .path
        .iter()
        .scan((Point::ORIGIN, Point::ORIGIN), |(origin, point), el| {
            let str = match el {
                PathEl::MoveTo(p) => {
                    *origin = p;
                    *point = p;
                    fmt_num!("M{} {}", p.x, p.y)
                }
                PathEl::LineTo(p) => {
                    let d = p - *point;
                    *point = p;
                    fmt_num!("l{} {}", d.x, d.y)
                }
                PathEl::CurveTo(p1, p2, p) => {
                    let d1 = p1 - *point;
                    let d2 = p2 - *point;
                    let d = p - *point;
                    *point = p;
                    fmt_num!("c{} {} {} {} {} {}", d1.x, d1.y, d2.x, d2.y, d.x, d.y)
                }
                // GRCOV_EXCL_START - no quads in example
                PathEl::QuadTo(p1, p) => {
                    let d1 = p1 - *point;
                    let d = p - *point;
                    *point = p;
                    fmt_num!("q{} {} {} {}", d1.x, d1.y, d.x, d.y)
                }
                // GRCOV_EXCL_STOP
                PathEl::ClosePath => {
                    *point = *origin;
                    "z".into()
                }
            };

            Some(str)
        })
        .join("");

    let fill = path
        .fill
        .map_or_else(|| "none".to_owned(), |color| color.to_string());
    let svg_path = SvgPath::new().set("d", data).set("fill", fill);

    if let Some(outline) = path.outline {
        svg_path
            .set("stroke", outline.color.to_string())
            .set("stroke-width", fmt_num!("{}", outline.width))
    } else {
        svg_path.set("stroke", "none")
    }
}

#[cfg(test)]
mod tests {
    use unindent::unindent;

    use crate::{DrawingOptions, Key};

    #[test]
    fn test_to_svg() {
        let options = DrawingOptions {
            show_margin: true,
            ..Default::default()
        };
        let keys = [Key::example()];
        let drawing = options.draw(&keys);

        let svg = drawing.to_svg();

        assert_eq!(
            svg,
            unindent(
                r##"<svg height="19.05mm" viewBox="0 0 1000 1000" width="19.05mm" xmlns="http://www.w3.org/2000/svg">
                <g transform="translate(0, 0)">
                <path d="M25 90c0 -35.89851 29.10149 -65 65 -65l820 0c35.89851 -0 65 29.10149 65 65l0 820c0 35.89851 -29.10149 65 -65 65l-820 0c-35.89851 0 -65 -29.10149 -65 -65z" fill="#cccccc" stroke="#aeaeae" stroke-width="10"/>
                <path d="M170 120c0 -35.89851 29.10149 -65 65 -65l530 0c35.89851 -0 65 29.10149 65 65l0 605c0 35.89851 -29.10149 65 -65 65l-530 0c-35.89851 0 -65 -29.10149 -65 -65z" fill="#cccccc" stroke="#aeaeae" stroke-width="10"/>
                <path d="M220 105l560 0l0 635l-560 0z" fill="none" stroke="#ff0000" stroke-width="5"/>
                <path d="M220 299.44444l0 194.44444l126.38889 0l0 -194.44444l-126.38889 0zM235.55556 328.61111l36.94444 68.05556l-36.94444 68.05556l0 -136.11111zM244.30556 478.33333l38.88889 -70l38.88889 70l-77.77778 0zM330.83333 464.72222l-36.94444 -68.05556l36.94444 -68.05556l0 136.11111zM322.08333 315l-38.88889 70l-38.88889 -70l77.77778 0z" fill="#000000" stroke="none"/>
                <path d="M653.61111 299.44444l0 194.44444l126.38889 0l0 -194.44444l-126.38889 0zM669.16667 328.61111l36.94444 68.05556l-36.94444 68.05556l0 -136.11111zM677.91667 478.33333l38.88889 -70l38.88889 70l-77.77778 0zM764.44444 464.72222l-36.94444 -68.05556l36.94444 -68.05556l0 136.11111zM755.69444 315l-38.88889 70l-38.88889 -70l77.77778 0z" fill="#000000" stroke="none"/>
                <path d="M220 740l0 194.44444l126.38889 0l0 -194.44444l-126.38889 0zM235.55556 769.16667l36.94444 68.05556l-36.94444 68.05556l0 -136.11111zM244.30556 918.88889l38.88889 -70l38.88889 70l-77.77778 0zM330.83333 905.27778l-36.94444 -68.05556l36.94444 -68.05556l0 136.11111zM322.08333 755.55556l-38.88889 70l-38.88889 -70l77.77778 0z" fill="#000000" stroke="none"/>
                <path d="M653.61111 740l0 194.44444l126.38889 0l0 -194.44444l-126.38889 0zM669.16667 769.16667l36.94444 68.05556l-36.94444 68.05556l0 -136.11111zM677.91667 918.88889l38.88889 -70l38.88889 70l-77.77778 0zM764.44444 905.27778l-36.94444 -68.05556l36.94444 -68.05556l0 136.11111zM755.69444 755.55556l-38.88889 70l-38.88889 -70l77.77778 0z" fill="#000000" stroke="none"/>
                </g>
                </svg>"##
            )
        );
    }
}
