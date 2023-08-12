use kurbo::{PathEl, Point};
use pdf_writer::{Content, Finish, PdfWriter, Rect, Ref, TextStr};

use crate::drawing::Drawing;

use super::{KeyDrawing, Path};

macro_rules! transform {
    (($($x:expr, $y:expr),+), $origin:expr, $scale:expr) => {
        // Negate Y since PDF has rising Y axis
        ($((($origin.x + $x / 1e3) * $scale) as f32, (($origin.y - $y / 1e3) * $scale) as f32),+)
    };
}

struct RefGen(i32);

impl RefGen {
    fn new() -> Self {
        Self(0)
    }

    fn next(&mut self) -> Ref {
        self.0 += 1;
        Ref::new(self.0)
    }
}

pub(crate) fn draw(drawing: &Drawing) -> Vec<u8> {
    let scale = drawing.scale * (72.0 / 96.0);
    let size = drawing.bounds.size() * scale;

    let mut ref_gen = RefGen::new();

    let mut writer = PdfWriter::new();
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

    for key in &drawing.keys {
        draw_key(&mut content, key, drawing.bounds.height(), scale);
    }

    writer.stream(content_id, &content.finish());

    writer
        .document_info(doc_info_id)
        .creator(TextStr("keyset-rs"))
        .producer(TextStr("keyset-rs"))
        .title(TextStr("Keyset Layout"))
        .finish();

    writer.finish()
}

fn draw_key(content: &mut Content, key: &KeyDrawing, height: f64, scale: f64) {
    for path in &key.paths {
        // Flip origin since PDF has rising Y axis
        let origin = Point::new(key.origin.x, height - key.origin.y);
        draw_path(content, path, origin, scale);
    }
}

fn draw_path(content: &mut Content, path: &Path, origin: Point, scale: f64) {
    // previous point, needed for quad => cubic Bézier conversion
    let mut p0 = Point::ORIGIN;

    for el in &path.path {
        match el {
            PathEl::MoveTo(p) => {
                p0 = p;
                let (x, y) = transform!((p.x, p.y), origin, scale);
                content.move_to(x, y);
            }
            PathEl::LineTo(p) => {
                p0 = p;
                let (x, y) = transform!((p.x, p.y), origin, scale);
                content.line_to(x, y);
            }
            PathEl::CurveTo(p1, p2, p) => {
                p0 = p;
                let (x1, y1, x2, y2, x, y) =
                    transform!((p1.x, p1.y, p2.x, p2.y, p.x, p.y), origin, scale);
                content.cubic_to(x1, y1, x2, y2, x, y);
            }
            PathEl::QuadTo(p1, p) => {
                // Convert quad to cubic since PostScript doesn't have quadratic Béziers
                let (p1, p2) = (p0 + (2.0 / 3.0) * (p1 - p0), p + (2.0 / 3.0) * (p1 - p));
                p0 = p;
                let (x1, y1, x2, y2, x, y) =
                    transform!((p1.x, p1.y, p2.x, p2.y, p.x, p.y), origin, scale);
                content.cubic_to(x1, y1, x2, y2, x, y);
            }
            PathEl::ClosePath => {
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
        content.set_line_width((outline.width * scale / 1e3) as f32);
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
        (None, None) => {}
    };
}
