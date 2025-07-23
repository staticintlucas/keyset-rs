use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use pdf_writer::{Content, Filter, Finish as _, Pdf, Rect, Ref, TextStr};

use geom::{
    Dot, PathSegment, Point, Scale, ToTransform as _, Transform, Vector, DOT_PER_INCH, DOT_PER_UNIT,
};

use crate::{Drawing, KeyDrawing, KeyPath};

#[derive(Debug, Clone, Copy)]
struct PdfUnit;

const PDF_SCALE: Scale<Dot, PdfUnit> = Scale::new(72.0 / DOT_PER_INCH.0); // PDF uses 72 dpi
const COMPRESSION_LEVEL: u8 = CompressionLevel::DefaultLevel as u8;

struct RefGen(i32);

impl RefGen {
    const fn new() -> Self {
        Self(0)
    }

    fn next(&mut self) -> Ref {
        self.0 += 1;
        Ref::new(self.0)
    }
}

pub fn draw(drawing: &Drawing) -> Vec<u8> {
    let scale = PDF_SCALE * Scale::<PdfUnit, PdfUnit>::new(drawing.scale);
    let size = drawing.bounds.size() * DOT_PER_UNIT * scale;

    let mut ref_gen = RefGen::new();

    let mut writer = Pdf::new();
    writer.set_version(1, 3);

    let catalog_id = ref_gen.next();
    let tree_id = ref_gen.next();
    let page_id = ref_gen.next();
    let content_id = ref_gen.next();
    let doc_info_id = ref_gen.next();

    _ = writer.catalog(catalog_id).pages(tree_id);
    _ = writer.pages(tree_id).kids([page_id]).count(1);

    writer
        .page(page_id)
        .media_box(Rect::new(0.0, 0.0, size.width, size.height))
        .parent(tree_id)
        .contents(content_id)
        .finish();

    let mut content = Content::new();

    // Flip origin since PDF has rising Y axis
    let transform = scale
        .to_transform()
        .then_scale(1.0, -1.0)
        .then_translate(Vector::new(0.0, size.height));
    for key in &drawing.keys {
        draw_key(&mut content, key, transform);
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

fn draw_key(content: &mut Content, key: &KeyDrawing, transform: Transform<Dot, PdfUnit>) {
    let transform = (key.origin.to_vector() * DOT_PER_UNIT)
        .to_transform()
        .then(&transform);
    for path in &key.paths {
        draw_path(content, path, transform);
    }
}

fn draw_path(content: &mut Content, path: &KeyPath, transform: Transform<Dot, PdfUnit>) {
    // origin needed for close; previous point needed for distance => point and quad => cubic
    // Bézier conversion
    let mut origin = Point::origin();
    let mut point = Point::origin();

    for &el in &path.data {
        let el = el * transform;
        match el {
            PathSegment::Move(p) => {
                _ = content.move_to(p.x, p.y);
                origin = p;
                point = p;
            }
            PathSegment::Line(d) => {
                let p = point + d;
                _ = content.line_to(p.x, p.y);
                point = p;
            }
            PathSegment::CubicBezier(d1, d2, d) => {
                let (p1, p2, p) = (point + d1, point + d2, point + d);
                _ = content.cubic_to(p1.x, p1.y, p2.x, p2.y, p.x, p.y);
                point = p;
            }
            PathSegment::QuadraticBezier(d1, d) => {
                // Convert quad to cubic since PostScript doesn't have quadratic Béziers
                let (d1, d2) = (d1 * (2.0 / 3.0), d + (d1 - d) * (2.0 / 3.0));
                let (p1, p2, p) = (point + d1, point + d2, point + d);
                _ = content.cubic_to(p1.x, p1.y, p2.x, p2.y, p.x, p.y);
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
        let scale = (f32::hypot(transform.m11, transform.m21)
            + f32::hypot(transform.m12, transform.m22))
            / 2.0;
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

    use crate::Template;

    #[test]
    fn test_to_pdf() {
        let template = Template {
            show_margin: true, // to give us an unfilled path
            ..Default::default()
        };
        let keys = [Key::example()];
        let drawing = template.draw(&keys);

        let pdf = drawing.to_pdf();
        let ai = drawing.to_ai();

        assert_eq!(pdf, ai);
    }
}
