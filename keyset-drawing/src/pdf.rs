use geom::{Affine, PathEl, Point};
use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use pdf_writer::{Content, Filter, Finish, Pdf, Rect, Ref, TextStr};

use crate::{Drawing, KeyDrawing, Path};

const PDF_DPI: f64 = 72.0; // PDF uses 72 dpi
const COMPRESSION_LEVEL: u8 = CompressionLevel::DefaultLevel as u8;

macro_rules! transform {
    ($p:expr, $affine:expr) => {{
        let p = $affine * $p;
        (p.x as f32, p.y as f32)
    }};
    ($p1:expr, $p:expr, $affine:expr) => {{
        let (p1, p) = ($affine * $p1, $affine * $p);
        (p1.x as f32, p1.y as f32, p.x as f32, p.y as f32)
    }};
    ($p1:expr, $p2:expr, $p:expr, $affine:expr) => {{
        let (p1, p2, p) = ($affine * $p1, $affine * $p2, $affine * $p);
        (
            p1.x as f32,
            p1.y as f32,
            p2.x as f32,
            p2.y as f32,
            p.x as f32,
            p.y as f32,
        )
    }};
}

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
    let scale = drawing.scale * PDF_DPI * 0.75; // 0.75 in/key
    let size = drawing.bounds.size() * scale;

    let mut ref_gen = RefGen::new();

    let mut writer = Pdf::new();
    writer.set_version(1, 3);

    let catalog_id = ref_gen.next();
    let tree_id = ref_gen.next();
    let page_id = ref_gen.next();
    let content_id = ref_gen.next();
    let doc_info_id = ref_gen.next();

    writer.catalog(catalog_id).pages(tree_id);
    writer.pages(tree_id).kids([page_id]).count(1);

    #[allow(clippy::cast_possible_truncation)]
    writer
        .page(page_id)
        .media_box(Rect::new(0.0, 0.0, size.width as f32, size.height as f32))
        .parent(tree_id)
        .contents(content_id)
        .finish();

    let mut content = Content::new();

    // Flip origin since PDF has rising Y axis
    let affine = Affine::scale_non_uniform(scale, -scale).then_translate((0., size.height).into());
    for key in &drawing.keys {
        draw_key(&mut content, key, &affine);
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

fn draw_key(content: &mut Content, key: &KeyDrawing, affine: &Affine) {
    let affine = *affine * Affine::scale(1e-3).then_translate(key.origin.to_vec2());
    for path in &key.paths {
        draw_path(content, path, &affine);
    }
}

fn draw_path(content: &mut Content, path: &Path, affine: &Affine) {
    // previous point, needed for quad => cubic Bézier conversion
    let mut origin = Point::ORIGIN;
    let mut p0 = Point::ORIGIN;

    for el in &path.path {
        match el {
            PathEl::MoveTo(p) => {
                origin = p;
                p0 = p;
                let (x, y) = transform!(p, *affine);
                content.move_to(x, y);
            }
            PathEl::LineTo(p) => {
                p0 = p;
                let (x, y) = transform!(p, *affine);
                content.line_to(x, y);
            }
            PathEl::CurveTo(p1, p2, p) => {
                p0 = p;
                let (x1, y1, x2, y2, x, y) = transform!(p1, p2, p, *affine);
                content.cubic_to(x1, y1, x2, y2, x, y);
            }
            // GRCOV_EXCL_START - no quads in example
            PathEl::QuadTo(p1, p) => {
                // Convert quad to cubic since PostScript doesn't have quadratic Béziers
                let (p1, p2) = (p0 + (2.0 / 3.0) * (p1 - p0), p + (2.0 / 3.0) * (p1 - p));
                p0 = p;
                let (x1, y1, x2, y2, x, y) = transform!(p1, p2, p, *affine);
                content.cubic_to(x1, y1, x2, y2, x, y);
            }
            // GRCOV_EXCL_STOP
            PathEl::ClosePath => {
                p0 = origin;
                content.close_path();
            }
        }
    }

    if let Some(color) = path.fill {
        let (r, g, b) = color.into();
        content.set_fill_rgb(r, g, b);
    }

    if let Some(outline) = path.outline {
        let (r, g, b) = outline.color.into();
        content.set_stroke_rgb(r, g, b);
        #[allow(clippy::cast_possible_truncation)]
        content.set_line_width((outline.width * affine.as_coeffs()[0]) as f32);
    }

    match (path.fill, path.outline) {
        (Some(_), Some(_)) => {
            content.fill_even_odd_and_stroke();
        }
        (Some(_), None) => {
            content.fill_even_odd();
        }
        (None, Some(_)) => {
            content.stroke();
        }
        (None, None) => {} // GRCOV_EXCL_LINE - unreachable?
    };
}

#[cfg(test)]
mod tests {
    use key::Key;

    use crate::Options;

    #[test]
    fn test_to_pdf() {
        let options = Options {
            show_margin: true, // to give us an unfilled path
            ..Default::default()
        };
        let keys = [Key::example()];
        let drawing = options.draw(&keys);

        let pdf = drawing.to_pdf();
        let ai = drawing.to_ai();

        assert_eq!(pdf, ai);
    }
}
