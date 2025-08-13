use num_traits::ToPrimitive as _;
use tiny_skia::{
    FillRule, Paint, PathBuilder as SkPathBuilder, Pixmap, Shader, Stroke, Transform as SkTransform,
};

use geom::{
    declare_units, Conversion, ConvertFrom as _, Dot, PathSegment, Point, Unit as _, Vector,
};

use crate::{Drawing, Error, KeyDrawing, KeyPath};

declare_units! {
    pub Pixel = 1.0;
}

pub fn draw(drawing: &Drawing, ppi: f32) -> Result<Vec<u8>, Error> {
    let scale = (ppi / Dot::PER_INCH) * drawing.scale;
    let conv = Conversion::<Pixel, Dot>::from_scale(scale, scale);
    let size = Vector::<Dot>::convert_from(drawing.bounds.size()) * conv;

    let mut pixmap =
        (|| Pixmap::new(size.x.get().to_u32()?.max(1), size.y.get().to_u32()?.max(1)))()
            .ok_or(Error::PngDimensionsError(size))?;

    pixmap.fill(tiny_skia::Color::TRANSPARENT);

    for key in &drawing.keys {
        draw_key(&mut pixmap, key, conv);
    }

    Ok(pixmap
        .encode_png()
        .unwrap_or_else(|_| unreachable!("writing to Vec<_> should not fail")))
}

fn draw_key(pixmap: &mut Pixmap, key: &KeyDrawing, conv: Conversion<Pixel, Dot>) {
    // Convert global conversion to local (per-key) conversion
    let conv = conv.pre_translate(Point::<Dot>::convert_from(key.origin) - Point::origin());

    for path in &key.paths {
        draw_path(pixmap, path, conv);
    }
}

fn draw_path(pixmap: &mut Pixmap, path: &KeyPath, conv: Conversion<Pixel, Dot>) {
    let path_builder = {
        let mut builder = SkPathBuilder::new();

        // origin needed for close; previous point needed for distance => point conversion
        let mut point = Point::origin();
        let mut origin = Point::origin();

        for &el in &path.data {
            match el {
                PathSegment::Move(p) => {
                    builder.move_to(p.x.get(), p.y.get());
                    origin = p;
                    point = p;
                }
                PathSegment::Line(d) => {
                    let p = point + d;
                    builder.line_to(p.x.get(), p.y.get());
                    point = p;
                }
                PathSegment::CubicBezier(d1, d2, d) => {
                    let (p1, p2, p) = (point + d1, point + d2, point + d);
                    builder.cubic_to(
                        p1.x.get(),
                        p1.y.get(),
                        p2.x.get(),
                        p2.y.get(),
                        p.x.get(),
                        p.y.get(),
                    );
                    point = p;
                }
                PathSegment::QuadraticBezier(d1, d) => {
                    let (p1, p) = (point + d1, point + d);
                    builder.quad_to(p1.x.get(), p1.y.get(), p.x.get(), p.y.get());
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

    let skia_transform = SkTransform {
        sx: conv.a_xx,
        kx: conv.a_xy,
        ky: conv.a_yx,
        sy: conv.a_yy,
        tx: conv.t_x,
        ty: conv.t_y,
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
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;
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
        let (drawing, _warnings) = template.draw(&keys).unwrap();

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
            assert_is_close!(res_r, exp_r, abs_tol = 0.025);
            assert_is_close!(res_g, exp_g, abs_tol = 0.025);
            assert_is_close!(res_b, exp_b, abs_tol = 0.025);
            assert_is_close!(res_a, exp_a, abs_tol = 0.025);
        }
    }
}
