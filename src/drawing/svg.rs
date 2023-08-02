use kurbo::Affine;
use svg::node::element::{Group, Path};
use svg::Document;

use crate::drawing::Drawing;

use super::{KeyDrawing, KeyPath};

fn fmt(num: f64) -> f64 {
    (1e5 * num).round() / 1e5
}

pub(crate) fn draw(drawing: &Drawing) -> String {
    let size = drawing.bounds.size() * drawing.scale;
    let view_box = drawing.bounds.scale_from_origin(1e3);

    let document = Document::new()
        .set("width", format!("{}", fmt(size.width)))
        .set("height", format!("{}", fmt(size.height)))
        .set(
            "viewBox",
            format!(
                "{} {} {} {}",
                fmt(view_box.origin().x),
                fmt(view_box.origin().y),
                fmt(view_box.size().width),
                fmt(view_box.size().height)
            ),
        );

    let document = drawing
        .keys
        .iter()
        .map(|k| draw_key(k, drawing.outline))
        .fold(document, Document::add);

    document.to_string()
}

fn draw_key(key: &KeyDrawing, outline: f64) -> Group {
    // scale from keyboard units to drawing units (milliunits)
    let pos = Affine::scale(1e3) * key.origin;

    let group = Group::new().set(
        "transform",
        format!("translate({}, {})", fmt(pos.x), fmt(pos.y)),
    );

    key.paths
        .iter()
        .map(|p| draw_path(p, outline))
        .fold(group, Group::add)
}

fn draw_path(key: &KeyPath, outline: f64) -> Path {
    let data = key.path.to_svg(); // TODO to_svg is far from optimal

    let path = Path::new().set("d", data).set("fill", key.fill.to_string());

    if outline > 1e-3 {
        path.set("stroke", key.outline.to_string())
            .set("stroke-width", format!("{}", fmt(outline)))
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
