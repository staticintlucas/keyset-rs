use std::fmt;

use kurbo::{PathEl, Point};
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
    (($($x:expr, $y:expr),+), $origin:expr, $scale:expr) => {
        ($((($origin.x + $x / 1e3) * $scale) as f32, (($origin.y + $y / 1e3) * $scale) as f32),+)
    };
}

impl std::error::Error for PngEncodingError {}

pub(crate) fn draw(drawing: &Drawing) -> Result<Vec<u8>> {
    let size = drawing.bounds.size() * drawing.scale;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (width, height) = (size.width.ceil() as u32, size.height.ceil() as u32);

    let mut pixmap = Pixmap::new(width, height).ok_or(PngEncodingError {
        message: format!("cannot create pixmap with size (w: {width}, h: {height})"),
    })?;

    pixmap.fill(tiny_skia::Color::TRANSPARENT);

    for key in &drawing.keys {
        draw_key(&mut pixmap, key, drawing.scale);
    }

    Ok(pixmap.encode_png().map_err(|e| PngEncodingError {
        message: e.to_string(),
    })?)
}

fn draw_key(pixmap: &mut Pixmap, key: &KeyDrawing, scale: f64) {
    for path in &key.paths {
        draw_path(pixmap, path, key.origin, scale);
    }
}

fn draw_path(pixmap: &mut Pixmap, path: &Path, origin: Point, scale: f64) {
    let mut path_builder = PathBuilder::new();
    for el in &path.path {
        match el {
            PathEl::MoveTo(p) => {
                let (x, y) = transform!((p.x, p.y), origin, scale);
                path_builder.move_to(x, y);
            }
            PathEl::LineTo(p) => {
                let (x, y) = transform!((p.x, p.y), origin, scale);
                path_builder.line_to(x, y);
            }
            PathEl::CurveTo(p1, p2, p) => {
                let (x1, y1, x2, y2, x, y) =
                    transform!((p1.x, p1.y, p2.x, p2.y, p.x, p.y), origin, scale);
                path_builder.cubic_to(x1, y1, x2, y2, x, y);
            }
            PathEl::QuadTo(p1, p) => {
                let (x1, y1, x, y) = transform!((p1.x, p1.y, p.x, p.y), origin, scale);
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
            width: (outline.width * scale / 1e3) as f32,
            ..Default::default()
        };
        pixmap.stroke_path(&skia_path, &paint, &stroke, Transform::default(), None);
    }
}
