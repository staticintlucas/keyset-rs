use svg::node::element::{Group, Path as SvgPath};
use svg::Document;

use geom::{ConvertFrom as _, ConvertInto as _, Dot, Mm, Point, Rect, Unit as _, Vector};

use self::utils::{ToSvg as _, ToViewBox as _};
use super::{Drawing, KeyDrawing, KeyPath};

mod utils;

pub fn draw(drawing: &Drawing) -> String {
    let size = Vector::<Mm>::convert_from(drawing.bounds.size()) * drawing.scale;
    let view_box: Rect<Dot> = drawing.bounds.convert_into(); // Use 1000 user units per key

    let document = Document::new()
        .set("width", format!("{}mm", size.x.get().to_svg()))
        .set("height", format!("{}mm", size.y.get().to_svg()))
        .set("viewBox", view_box.to_view_box().to_string());

    let document = drawing
        .keys
        .iter()
        .map(draw_key)
        .fold(document, Document::add);

    document.to_string()
}

fn draw_key(key: &KeyDrawing) -> Group {
    let origin: Point<Dot> = key.origin.convert_into();
    let group = Group::new().set(
        "transform",
        format!("translate({},{})", origin.x.to_svg(), origin.y.to_svg()),
    );
    key.paths.iter().map(draw_path).fold(group, Group::add)
}

fn draw_path(path: &KeyPath) -> SvgPath {
    let data = path.data.to_svg().to_string();
    let fill = path
        .fill
        .map_or_else(|| "none".to_owned(), |color| format!("{color:x}"));
    let svg_path = SvgPath::new().set("d", data).set("fill", fill);

    if let Some(outline) = path.outline {
        svg_path
            .set("stroke", format!("{:x}", outline.color))
            .set("stroke-width", outline.width.to_svg().to_string())
    } else {
        svg_path.set("stroke", "none")
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use indoc::indoc;

    use key::Key;

    use crate::Stencil;

    #[test]
    fn test_to_svg() {
        let stencil = Stencil {
            show_margin: true,
            ..Default::default()
        };
        let keys = [Key::example()];
        let drawing = stencil.draw(&keys).unwrap();

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
                <path d="M653.638 299.444l0-194.444l126.362 0l0 194.444l-126.362-0zM669.161 270.305l37.037-68.083l-37.037-68.083l0 136.166zM677.876 120.523l38.943 69.989l38.943-69.989l-77.887-0zM764.477 134.139l-37.037 68.083l37.037 68.083l0-136.166zM755.762 283.922l-38.943-69.989l-38.943 69.989l77.887 0z" fill="#000000" stroke="none"/>
                <path d="M220 740l0-194.444l126.362 0l0 194.444l-126.362-0zM235.523 710.861l37.037-68.083l-37.037-68.083l0 136.166zM244.237 561.078l38.943 69.989l38.943-69.989l-77.887-0zM330.839 574.695l-37.037 68.083l37.037 68.083l0-136.166zM322.124 724.477l-38.943-69.989l-38.943 69.989l77.887 0z" fill="#000000" stroke="none"/>
                <path d="M653.638 740l0-194.444l126.362 0l0 194.444l-126.362-0zM669.161 710.861l37.037-68.083l-37.037-68.083l0 136.166zM677.876 561.078l38.943 69.989l38.943-69.989l-77.887-0zM764.477 574.695l-37.037 68.083l37.037 68.083l0-136.166zM755.762 724.477l-38.943-69.989l-38.943 69.989l77.887 0z" fill="#000000" stroke="none"/>
                </g>
                </svg>"##
            )
        );
    }
}
