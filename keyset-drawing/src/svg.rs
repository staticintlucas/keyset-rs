use geom::{PathSegment, Scale, Unit, DOT_PER_UNIT, MM_PER_UNIT};
use svg::node::element::{Group, Path as SvgPath};
use svg::Document;

use super::{Drawing, KeyDrawing, KeyPath};

macro_rules! fmt_num {
    ($fmt:literal, $($args:expr),*) => {
        format!($fmt, $(($args * 1e5).round() / 1e5),*)
    };
}

pub fn draw(drawing: &Drawing) -> String {
    let size = drawing.bounds.size() * Scale::<Unit, Unit>::new(drawing.scale) * MM_PER_UNIT;
    let view_box = drawing.bounds * DOT_PER_UNIT; // Use 1000 user units per key

    let document = Document::new()
        .set("width", fmt_num!("{}mm", size.width))
        .set("height", fmt_num!("{}mm", size.height))
        .set(
            "viewBox",
            fmt_num!(
                "{} {} {} {}",
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
        fmt_num!("translate({},{})", origin.x, origin.y),
    );
    key.paths.iter().map(draw_path).fold(group, Group::add)
}

fn draw_path(path: &KeyPath) -> SvgPath {
    let data: String = path
        .data
        .iter()
        .map(|el| match *el {
            PathSegment::Move(p) => fmt_num!("M{} {}", p.x, p.y),
            PathSegment::Line(d) => fmt_num!("l{} {}", d.x, d.y),
            PathSegment::CubicBezier(c1, c2, d) => {
                fmt_num!("c{} {} {} {} {} {}", c1.x, c1.y, c2.x, c2.y, d.x, d.y)
            }
            PathSegment::QuadraticBezier(c1, d) => fmt_num!("q{} {} {} {}", c1.x, c1.y, d.x, d.y),
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
            .set("stroke-width", fmt_num!("{}", outline.width.get()))
    } else {
        svg_path.set("stroke", "none")
    }
}

#[cfg(test)]
mod tests {
    use key::Key;
    use unindent::unindent;

    use crate::Options;

    #[test]
    fn test_to_svg() {
        let options = Options {
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
                <g transform="translate(0,0)">
                <path d="M25 90c0 -35.89851 29.10149 -65.00001 64.99999 -65.00001l820 0c35.89851 0 65 29.10149 65 65l0 820c0 35.89851 -29.1015 65 -65 65l-820 0c-35.89851 0 -64.99999 -29.1015 -64.99999 -65.00001z" fill="#cccccc" stroke="#aeaeae" stroke-width="10"/>
                <path d="M170 120c0 -35.89851 29.10149 -65.00001 64.99999 -65.00001l530 0c35.89851 0 65 29.10149 65 65l0 605c0 35.89851 -29.1015 65 -65 65l-530 0c-35.89851 0 -64.99999 -29.1015 -64.99999 -65.00001z" fill="#cccccc" stroke="#aeaeae" stroke-width="10"/>
                <path d="M220 105l560 0l0 635l-560 0z" fill="none" stroke="#ff0000" stroke-width="5"/>
                <path d="M220 299.44446l0 -194.44446l126.36166 0l0 194.44446l-126.36166 -0zM235.52287 270.30502l37.03704 -68.0828l-37.03704 -68.0828l0 136.16559zM244.23747 120.52287l38.94336 69.98912l38.94336 -69.98912l-77.88672 -0zM330.8388 134.13943l-37.03704 68.0828l37.03704 68.0828l0 -136.16559zM322.1242 283.92157l-38.94336 -69.98912l-38.94336 69.98912l77.88672 0z" fill="#000000" stroke="none"/>
                <path d="M653.63837 299.44446l0 -194.44446l126.36166 0l0 194.44446l-126.36166 -0zM669.1612 270.30502l37.03704 -68.0828l-37.03704 -68.0828l0 136.16559zM677.87573 120.52287l38.94336 69.98912l38.94336 -69.98912l-77.88672 -0zM764.4771 134.13943l-37.03704 68.0828l37.03704 68.0828l0 -136.16559zM755.76245 283.92157l-38.94336 -69.98912l-38.94336 69.98912l77.88672 0z" fill="#000000" stroke="none"/>
                <path d="M220 740l0 -194.44446l126.36166 0l0 194.44446l-126.36166 -0zM235.52287 710.86053l37.03704 -68.0828l-37.03704 -68.0828l0 136.16559zM244.23747 561.0784l38.94336 69.98912l38.94336 -69.98912l-77.88672 -0zM330.8388 574.69495l-37.03704 68.0828l37.03704 68.0828l0 -136.16559zM322.1242 724.4771l-38.94336 -69.98912l-38.94336 69.98912l77.88672 0z" fill="#000000" stroke="none"/>
                <path d="M653.63837 740l0 -194.44446l126.36166 0l0 194.44446l-126.36166 -0zM669.1612 710.86053l37.03704 -68.0828l-37.03704 -68.0828l0 136.16559zM677.87573 561.0784l38.94336 69.98912l38.94336 -69.98912l-77.88672 -0zM764.4771 574.69495l-37.03704 68.0828l37.03704 68.0828l0 -136.16559zM755.76245 724.4771l-38.94336 -69.98912l-38.94336 69.98912l77.88672 0z" fill="#000000" stroke="none"/>
                </g>
                </svg>"##
            )
        );
    }
}
