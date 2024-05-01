use geom::{
    Dot, Inch, PathSegment, Point, Scale, ToTransform, Transform, DOT_PER_INCH, DOT_PER_UNIT,
};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform as SkiaTransform};

use crate::{Drawing, KeyDrawing, KeyPath};

#[derive(Debug, Clone, Copy)]
pub struct Pixel;

pub fn draw(drawing: &Drawing, ppi: Scale<Inch, Pixel>) -> Vec<u8> {
    let scale = (DOT_PER_INCH.inverse() * ppi) * Scale::<Pixel, Pixel>::new(drawing.scale);
    let size = drawing.bounds.size() * DOT_PER_UNIT * scale;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // We want truncation
    let (width, height) = (size.width.ceil() as u32, size.height.ceil() as u32);

    // Will panic if width/height = 0 (which we prevent with .max(1))
    // TODO Will also panic if width/height are too large
    let mut pixmap = Pixmap::new(width.max(1), height.max(1)).unwrap();

    pixmap.fill(tiny_skia::Color::TRANSPARENT);

    let transform = scale.to_transform();
    for key in &drawing.keys {
        draw_key(&mut pixmap, key, transform);
    }

    // Will panic for an IO error, but writing to a Vec<_> is infallible
    pixmap.encode_png().unwrap()
}

fn draw_key(pixmap: &mut Pixmap, key: &KeyDrawing, transform: Transform<Dot, Pixel>) {
    let transform = (key.origin.to_vector() * DOT_PER_UNIT)
        .to_transform()
        .then(&transform);
    for path in &key.paths {
        draw_path(pixmap, path, transform);
    }
}

fn draw_path(pixmap: &mut Pixmap, path: &KeyPath, transform: Transform<Dot, Pixel>) {
    let path_builder = {
        let mut builder = PathBuilder::new();

        // origin needed for close; previous point needed for distance => point conversion
        let mut point = Point::zero();
        let mut origin = Point::zero();

        for &el in &path.data {
            let el = el * transform;
            match el {
                PathSegment::Move(p) => {
                    builder.move_to(p.x, p.y);
                    origin = p;
                    point = p;
                }
                PathSegment::Line(d) => {
                    let p = point + d;
                    builder.line_to(p.x, p.y);
                    point = p;
                }
                PathSegment::CubicBezier(d1, d2, d) => {
                    let (p1, p2, p) = (point + d1, point + d2, point + d);
                    builder.cubic_to(p1.x, p1.y, p2.x, p2.y, p.x, p.y);
                    point = p;
                }
                // GRCOV_EXCL_START - no quads in example
                PathSegment::QuadraticBezier(d1, d) => {
                    let (p1, p) = (point + d1, point + d);
                    builder.quad_to(p1.x, p1.y, p.x, p.y);
                    point = p;
                }
                // GRCOV_EXCL_STOP
                PathSegment::Close => {
                    builder.close();
                    point = origin;
                }
            }
        }

        builder
    };

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
            SkiaTransform::default(),
            None,
        );
    }

    if let Some(outline) = path.outline {
        let paint = Paint {
            shader: Shader::SolidColor(outline.color.into()),
            anti_alias: true,
            ..Default::default()
        };
        let scale = Scale::<Dot, Pixel>::new(
            (f32::hypot(transform.m11, transform.m21) + f32::hypot(transform.m12, transform.m22))
                / 2.0,
        );
        let stroke = Stroke {
            width: (outline.width * scale).get(),
            ..Default::default()
        };
        pixmap.stroke_path(&skia_path, &paint, &stroke, SkiaTransform::default(), None);
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
