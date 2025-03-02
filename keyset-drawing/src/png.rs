use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform as SkiaTransform};

use geom::{
    Dot, Inch, PathSegment, Point, Scale, ToTransform, Transform, DOT_PER_INCH, DOT_PER_UNIT,
};

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
                PathSegment::QuadraticBezier(d1, d) => {
                    let (p1, p) = (point + d1, point + d);
                    builder.quad_to(p1.x, p1.y, p.x, p.y);
                    point = p;
                }
                PathSegment::Close => {
                    builder.close();
                    point = origin;
                }
            }
        }

        builder
    };

    let Some(skia_path) = path_builder.finish() else {
        return;
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
    use isclose::assert_is_close_abs_tol;
    use itertools::izip;
    use tiny_skia::{Color, Pixmap, PremultipliedColorU8};

    use key::Key;

    use crate::Template;

    fn premul_u8_to_f32(color: PremultipliedColorU8) -> Color {
        let [r, g, b, a] =
            [color.red(), color.green(), color.blue(), color.alpha()].map(|c| f32::from(c) / 255.0);
        if color.alpha() == 0 {
            Color::TRANSPARENT
        } else {
            Color::from_rgba(r, g, b, a).unwrap()
        }
    }

    #[test]
    fn test_to_png() {
        let template = Template::default();
        let keys = [Key::example()];
        let drawing = template.draw(&keys);

        let png = drawing.to_png(96.0).unwrap();

        let result = Pixmap::decode_png(&png).unwrap();
        let expected = Pixmap::load_png(env!("REFERENCE_PNG")).unwrap();

        assert_eq!(result.width(), expected.width());
        assert_eq!(result.height(), expected.height());

        let result = result.pixels().iter().map(|&c| premul_u8_to_f32(c));
        let expected = expected.pixels().iter().map(|&c| premul_u8_to_f32(c));

        for (res, exp) in izip!(result, expected) {
            let (res_r, res_g, res_b, res_a) = (res.red(), res.green(), res.blue(), res.alpha());
            let (exp_r, exp_g, exp_b, exp_a) = (exp.red(), exp.green(), exp.blue(), exp.alpha());

            // TODO: what's a good tolerance here?
            assert_is_close_abs_tol!(res_r, exp_r, 0.025);
            assert_is_close_abs_tol!(res_g, exp_g, 0.025);
            assert_is_close_abs_tol!(res_b, exp_b, 0.025);
            assert_is_close_abs_tol!(res_a, exp_a, 0.025);
        }
    }
}
