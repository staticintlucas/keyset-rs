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
    let view_box = drawing.bounds.scale_from_origin(1e3);

    let document = Document::new()
        .set("width", fmt_num!("{}", size.width))
        .set("height", fmt_num!("{}", size.height))
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

fn draw_path(key: &Path) -> SvgPath {
    let data = key
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
                PathEl::QuadTo(p1, p) => {
                    let d1 = p1 - *point;
                    let d = p - *point;
                    *point = p;
                    fmt_num!("q{} {} {} {}", d1.x, d1.y, d.x, d.y)
                }
                PathEl::ClosePath => {
                    *point = *origin;
                    "z".into()
                }
            };

            Some(str)
        })
        .join("");

    let fill = if let Some(color) = key.fill {
        color.to_string()
    } else {
        "none".into()
    };
    let path = SvgPath::new().set("d", data).set("fill", fill);

    if let Some(outline) = key.outline {
        path.set("stroke", outline.color.to_string())
            .set("stroke-width", fmt_num!("{}", outline.width))
    } else {
        path.set("stroke", "none")
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::drawing::DrawingOptions;
//     use crate::font::Font;
//     use crate::profile::Profile;

//     use super::*;

//     #[test]
//     fn test_to_svg() {
//         let layout = vec![];
//         let profile = Profile::default();
//         let font = Font::from_ttf(&std::fs::read("tests/fonts/demo.ttf").unwrap()).unwrap();
//         let options = DrawingOptions::default();
//         let drawing = DrawingOptions::new(layout, profile, font, options);

//         assert_eq!(
//             drawing.to_svg(),
//             r#"<svg height="72" viewBox="0 0 1000 1000" width="72" xmlns="http://www.w3.org/2000/svg"/>"#
//         );
//     }

//     #[test]
//     fn test_draw_key() {
//         let key = Key::example();
//         let layout = vec![];
//         let profile = Profile::default();
//         let font = Font::from_ttf(&std::fs::read("tests/fonts/demo.ttf").unwrap()).unwrap();

//         let test_config = vec![
//             (DrawingOptions::default(), 6),
//             (
//                 DrawingOptions {
//                     show_keys: false,
//                     ..DrawingOptions::default()
//                 },
//                 4,
//             ),
//             (
//                 DrawingOptions {
//                     show_margin: true,
//                     ..DrawingOptions::default()
//                 },
//                 7,
//             ),
//         ];

//         for (options, len) in test_config {
//             let drawing = DrawingOptions::new(layout.clone(), profile.clone(), font.clone(), options);
//             let group = drawing.draw_key(&key);

//             assert_eq!(group.get_children().len(), len);
//         }
//     }
// }
