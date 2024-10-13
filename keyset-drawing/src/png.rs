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
    for key in &drawing.keys {
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
        let stroke = Stroke {
            width: outline.width.get(),
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
        let options = Options::default();
        let keys = [Key::example()];
        let drawing = Drawing::new(&keys, &options);

        let png = drawing.to_png(96.0).unwrap();

        let result = Pixmap::decode_png(&png).unwrap();
        let expected = Pixmap::load_png(env!("REFERENCE_PNG")).unwrap();

        assert_eq!(result.width(), expected.width());
        assert_eq!(result.height(), expected.height());

        for p in 0..result.pixels().len() {
            let res = result.pixels()[p].demultiply();
            let exp = expected.pixels()[p].demultiply();

            let (res_r, res_g, res_b, res_a) = (res.red(), res.green(), res.blue(), res.alpha());
            let (exp_r, exp_g, exp_b, exp_a) = (exp.red(), exp.green(), exp.blue(), exp.alpha());

            // TODO: what's a good tolerance here?
            assert!(
                res_r.abs_diff(exp_r) < 10,
                "res_r = {res_r}, exp_r = {exp_r}"
            );
            assert!(
                res_g.abs_diff(exp_g) < 10,
                "res_g = {res_g}, exp_g = {exp_g}"
            );
            assert!(
                res_b.abs_diff(exp_b) < 10,
                "res_b = {res_b}, exp_b = {exp_b}"
            );
            assert!(
                res_a.abs_diff(exp_a) < 10,
                "res_a = {res_a}, exp_a = {exp_a}"
            );
        }
    }
}
