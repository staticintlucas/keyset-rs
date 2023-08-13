use std::fmt;

use kurbo::{Affine, PathEl};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::drawing::Drawing;
use crate::error::Result;

use super::{KeyDrawing, Path};

#[derive(Debug)]
pub(crate) struct PngEncodingError {
    message: String,
}

impl fmt::Display for PngEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

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

impl std::error::Error for PngEncodingError {}

pub(crate) fn draw(drawing: &Drawing, dpi: f64) -> Result<Vec<u8>> {
    let scale = drawing.scale * dpi * 0.75; // 0.75 in/key
    let size = drawing.bounds.size() * scale;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (width, height) = (size.width.ceil() as u32, size.height.ceil() as u32);

    let mut pixmap = Pixmap::new(width, height).ok_or(PngEncodingError {
        message: format!("cannot create pixmap with size (w: {width}, h: {height})"),
    })?;

    pixmap.fill(tiny_skia::Color::TRANSPARENT);

    let affine = Affine::scale(scale);
    for key in &drawing.keys {
        draw_key(&mut pixmap, key, &affine);
    }

    Ok(pixmap.encode_png().map_err(|e| PngEncodingError {
        message: e.to_string(),
    })?)
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
            PathEl::QuadTo(p1, p) => {
                let (x1, y1, x, y) = transform!(p1, p, *affine);
                path_builder.quad_to(x1, y1, x, y);
            }
            PathEl::ClosePath => path_builder.close(),
        }
    }
    let Some(skia_path) = path_builder.finish() else { return };

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
