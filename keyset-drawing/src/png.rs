use geom::{Affine, PathEl};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::{Drawing, KeyDrawing, Path};

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

pub fn draw(drawing: &Drawing, dpi: f64) -> Vec<u8> {
    let scale = drawing.scale * dpi * 0.75; // 0.75 in/key
    let size = drawing.bounds.size() * scale;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (width, height) = (size.width.ceil() as u32, size.height.ceil() as u32);

    // Will panic if width/height = 0 which we prevent with max, or if width/height are too large
    // in which case we'll likely end up with OOM aborts here anyway
    let mut pixmap = Pixmap::new(width.max(1), height.max(1)).unwrap();

    pixmap.fill(tiny_skia::Color::TRANSPARENT);

    let affine = Affine::scale(scale);
    for key in &drawing.keys {
        draw_key(&mut pixmap, key, &affine);
    }

    // Will panic for an IO error, but writing to a Vec<_> is infallible
    pixmap.encode_png().unwrap()
}

fn draw_key(pixmap: &mut Pixmap, key: &KeyDrawing, affine: &Affine) {
    let affine = *affine * Affine::scale(1e-3).then_translate(key.origin.to_vec2());
    for path in &key.paths {
        draw_path(pixmap, path, &affine);
    }
}

fn draw_path(pixmap: &mut Pixmap, path: &Path, affine: &Affine) {
    let mut path_builder = PathBuilder::new();
    for el in &path.path {
        match el {
            PathEl::MoveTo(p) => {
                let (x, y) = transform!(p, *affine);
                path_builder.move_to(x, y);
            }
            PathEl::LineTo(p) => {
                let (x, y) = transform!(p, *affine);
                path_builder.line_to(x, y);
            }
            PathEl::CurveTo(p1, p2, p) => {
                let (x1, y1, x2, y2, x, y) = transform!(p1, p2, p, *affine);
                path_builder.cubic_to(x1, y1, x2, y2, x, y);
            }
            // GRCOV_EXCL_START - no quads in example
            PathEl::QuadTo(p1, p) => {
                let (x1, y1, x, y) = transform!(p1, p, *affine);
                path_builder.quad_to(x1, y1, x, y);
            }
            // GRCOV_EXCL_STOP
            PathEl::ClosePath => path_builder.close(),
        }
    }
    let Some(skia_path) = path_builder.finish() else {
        return; // GRCOV_EXCL_LINE
    };

    if let Some(color) = path.fill {
        let paint = Paint {
            shader: Shader::SolidColor(color.into()),
            anti_alias: true,
            ..Default::default()
        };
        pixmap.fill_path(
            &skia_path,
            &paint,
            FillRule::EvenOdd,
            Transform::default(),
            None,
        );
    }

    if let Some(outline) = path.outline {
        let paint = Paint {
            shader: Shader::SolidColor(outline.color.into()),
            anti_alias: true,
            ..Default::default()
        };
        #[allow(clippy::cast_possible_truncation)]
        let stroke = Stroke {
            width: (outline.width * affine.as_coeffs()[0]) as f32,
            ..Default::default()
        };
        pixmap.stroke_path(&skia_path, &paint, &stroke, Transform::default(), None);
    }
}

#[cfg(test)]
mod tests {
    use key::Key;
    use tiny_skia::Pixmap;

    use crate::Options;

    #[test]
    fn test_to_png() {
        // It's pretty difficult to test this stuff without visually inspecting
        // the image, so we just test a few pixels

        let options = Options::default();
        let keys = [Key::example()];
        let drawing = options.draw(&keys);

        let png = drawing.to_png(96.0);

        let pixmap = Pixmap::decode_png(&png).unwrap();
        assert_eq!(pixmap.width(), 72);
        assert_eq!(pixmap.height(), 72);

        let pixel = pixmap
            .pixel(pixmap.width() / 2, pixmap.height() / 2)
            .unwrap();
        assert_eq!(pixel.demultiply().red(), 0xCC);
        assert_eq!(pixel.demultiply().green(), 0xCC);
        assert_eq!(pixel.demultiply().blue(), 0xCC);
    }
}
