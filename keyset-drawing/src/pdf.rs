use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use pdf_writer::{Content, Filter, Finish as _, Pdf, Rect as PdfRect, Ref, TextStr};

use geom::{
    declare_units, Conversion, ConvertFrom as _, ConvertInto as _, Dot, PathSegment, Point,
    Unit as _, Vector,
};

use crate::{Drawing, KeyDrawing, KeyPath};

declare_units! {
    pub PdfUnit = 1.0;
}

const PDF_DPI: f32 = 72.0; // PDF uses 72 dpi
const PDF_SCALE: f32 = PDF_DPI / Dot::PER_INCH;
const COMPRESSION_LEVEL: u8 = CompressionLevel::DefaultLevel as u8;

pub fn draw(drawing: &Drawing) -> Vec<u8> {
    let scale = PDF_SCALE * drawing.scale;
    let size = Vector::<Dot>::convert_from(drawing.bounds.size());

    // Flip origin since PDF has rising Y axis
    let conv = Conversion::from_translate(0.0, size.y.get()).then_scale(scale, -scale);
    let size: Vector<PdfUnit> = size * conv;

    let mut ref_alloc = Ref::new(1);

    let mut writer = Pdf::new();
    writer.set_version(1, 3);

    let catalog_id = ref_alloc.bump();
    let tree_id = ref_alloc.bump();
    let page_id = ref_alloc.bump();
    let content_id = ref_alloc.bump();
    let doc_info_id = ref_alloc.bump();

    _ = writer.catalog(catalog_id).pages(tree_id);
    _ = writer.pages(tree_id).kids([page_id]).count(1);

    writer
        .page(page_id)
        .media_box(PdfRect::new(0.0, 0.0, size.x.get(), size.x.get()))
        .parent(tree_id)
        .contents(content_id)
        .finish();

    let mut content = Content::new();

    for key in &drawing.keys {
        draw_key(&mut content, key, conv);
    }

    let data = compress_to_vec_zlib(&content.finish(), COMPRESSION_LEVEL);
    writer
        .stream(content_id, &data)
        .filter(Filter::FlateDecode)
        .finish();

    writer
        .document_info(doc_info_id)
        .creator(TextStr("keyset-rs"))
        .producer(TextStr("keyset-rs"))
        .title(TextStr("Keyset Layout"))
        .finish();

    writer.finish()
}

fn draw_key(content: &mut Content, key: &KeyDrawing, conv: Conversion<PdfUnit, Dot>) {
    // Convert global conversion to local (per-key) conversion
    let conv = conv.pre_translate(Vector::new(
        key.origin.x.convert_into(),
        key.origin.y.convert_into(),
    ));
    for path in &key.paths {
        draw_path(content, path, conv);
    }
}

fn draw_path(content: &mut Content, path: &KeyPath, conv: Conversion<PdfUnit, Dot>) {
    // origin needed for close; previous point needed for distance => point and quad => cubic
    // Bézier conversion
    let mut origin = Point::origin();
    let mut point = Point::origin();

    for &el in &path.data {
        let el = el * conv;
        match el {
            PathSegment::Move(p) => {
                _ = content.move_to(p.x.get(), p.y.get());
                origin = p;
                point = p;
            }
            PathSegment::Line(d) => {
                let p = point + d;
                _ = content.line_to(p.x.get(), p.y.get());
                point = p;
            }
            PathSegment::CubicBezier(d1, d2, d) => {
                let (p1, p2, p) = (point + d1, point + d2, point + d);
                _ = content.cubic_to(
                    p1.x.get(),
                    p1.y.get(),
                    p2.x.get(),
                    p2.y.get(),
                    p.x.get(),
                    p.y.get(),
                );
                point = p;
            }
            PathSegment::QuadraticBezier(d1, d) => {
                // Convert quad to cubic since PostScript doesn't have quadratic Béziers
                let (d1, d2) = (d1 * (2.0 / 3.0), d + (d1 - d) * (2.0 / 3.0));
                let (p1, p2, p) = (point + d1, point + d2, point + d);
                _ = content.cubic_to(
                    p1.x.get(),
                    p1.y.get(),
                    p2.x.get(),
                    p2.y.get(),
                    p.x.get(),
                    p.y.get(),
                );
                point = p;
            }
            PathSegment::Close => {
                point = origin;
                _ = content.close_path();
            }
        }
    }

    if let Some(color) = path.fill {
        let (r, g, b) = color.into();
        _ = content.set_fill_rgb(r, g, b);
    }

    if let Some(outline) = path.outline {
        let (r, g, b) = outline.color.into();
        _ = content.set_stroke_rgb(r, g, b);
        // Use mean of x and y scales
        let scale = (f32::hypot(conv.a_xx, conv.a_yx) + f32::hypot(conv.a_xy, conv.a_yy)) / 2.0;
        _ = content.set_line_width((outline.width * scale).get());
    }

    match (path.fill, path.outline) {
        (Some(_), Some(_)) => {
            _ = content.fill_even_odd_and_stroke();
        }
        (Some(_), None) => {
            _ = content.fill_even_odd();
        }
        (None, Some(_)) => {
            _ = content.stroke();
        }
        (None, None) => {} // unreachable!() ? // it makes sense to just do nothing here regardless
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use key::Key;

    use crate::Stencil;

    #[test]
    fn test_to_pdf() {
        let stencil = Stencil {
            show_margin: true, // to give us an unfilled path
            ..Default::default()
        };
        let keys = [Key::example()];
        let drawing = stencil.draw(&keys).unwrap();

        let pdf = drawing.to_pdf();
        let ai = drawing.to_ai();

        assert_eq!(pdf, ai);
    }
}
