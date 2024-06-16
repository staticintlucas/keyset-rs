use geom::{
    Dot, Inch, PathSegment, Point, Scale, ToTransform, Transform, DOT_PER_INCH, DOT_PER_UNIT,
};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform as SkiaTransform};

use crate::{Drawing, Error, KeyDrawing, KeyPath};

#[derive(Debug, Clone, Copy)]
pub struct Pixel;

pub fn draw(drawing: &Drawing, ppi: Scale<Inch, Pixel>) -> Result<Vec<u8>, Error> {
    let scale = (DOT_PER_INCH.inverse() * ppi) * Scale::<Pixel, Pixel>::new(drawing.scale);
    let size = drawing.bounds.size() * DOT_PER_UNIT * scale;

    let mut pixmap = size
        .try_cast()
        .and_then(|size| Pixmap::new(size.width, size.height))
        .ok_or(Error::PngDimensionsError(size))?;

    pixmap.fill(tiny_skia::Color::TRANSPARENT);

    let transform = scale.to_transform();
    for key in drawing.keys.iter() {
        draw_key(&mut pixmap, key, transform);
    }

    Ok(pixmap
        .encode_png()
        .unwrap_or_else(|_| unreachable!("writing to Vec<_> should not fail")))
}

fn draw_key(pixmap: &mut Pixmap, key: &KeyDrawing, transform: Transform<Dot, Pixel>) {
    let transform = (key.origin.to_vector() * DOT_PER_UNIT)
        .to_transform()
        .then(&transform);
    for path in key.paths.iter() {
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

    let skia_transform = SkiaTransform {
        sx: transform.m11,
        kx: transform.m12,
        ky: transform.m21,
        sy: transform.m22,
        tx: transform.m31,
        ty: transform.m32,
    };

    if let Some(color) = path.fill {
        let paint = Paint {
            shader: Shader::SolidColor(color.into()),
            anti_alias: true,
            ..Default::default()
        };
        pixmap.fill_path(&skia_path, &paint, FillRule::EvenOdd, skia_transform, None);
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
        pixmap.stroke_path(&skia_path, &paint, &stroke, skia_transform, None);
    }
}

#[cfg(test)]
mod tests {
    use key::Key;
    use tiny_skia::Pixmap;

    use crate::{Drawing, Options};

    #[test]
    fn test_to_png() {
        // It's pretty difficult to test this stuff without visually inspecting
        // the image, so we just test a few pixels

        let options = Options::default();
        let keys = [Key::example()];
        let drawing = Drawing::new(&keys, &options);

        let png = drawing.to_png(96.0).unwrap();

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
