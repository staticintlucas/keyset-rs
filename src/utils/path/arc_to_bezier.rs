use std::f32::consts::{FRAC_PI_2, PI};

use crate::utils::{Scale, Size};

const TOL: f32 = 1e-6;

pub fn arc_to_bezier(r: Size, xar: f32, laf: bool, sf: bool, d: Size) -> Vec<(Size, Size, Size)> {
    if d.abs() < TOL {
        return vec![];
    }

    // Ensure our radii are large enough
    // If either radius is 0 we just return a straight line
    let r = Size::new(r.w.abs(), r.h.abs());
    if r.w < TOL || r.h < TOL {
        return vec![(d / 3., d * (2. / 3.), d)];
    }

    // Rotate the point by -xar. We calculate the result as if xar==0 and then re-rotate the result
    // It's a lot easier this way, I swear
    let (sin, cos) = (-xar).sin_cos();
    let d = Size {
        w: d.w * cos - d.h * sin,
        h: d.w * sin + d.h * cos,
    };

    // Scale the radii up if they can't meet the distance, maintaining their ratio
    let lambda = (d / (r * 2.)).abs().max(1.);
    let r = r * lambda;

    let c = get_center(r, laf, sf, d);

    let phi0 = (-c / r).arg();

    let dphi = ((d - c) / r).arg() - phi0;

    // Add and subtract 2pi (360 deg) to make sure dphi is the correct angle to sweep
    let dphi = match (laf, sf) {
        (true, true) if dphi < PI => dphi + 2. * PI,
        (true, false) if dphi > -PI => dphi - 2. * PI,
        (false, true) if dphi < 0. => dphi + 2. * PI,
        (false, false) if dphi > 0. => dphi - 2. * PI,
        _ => dphi,
    };

    // Double checks the quadrant of dphi
    // TODO remove these? They shouldn't ever fail I think aside from the odd tolerance issue
    // match (laf, sf) {
    //     (false, false) => assert!((-PI..=0.).contains(&dphi)),
    //     (false, true) => assert!((0. ..=PI).contains(&dphi)),
    //     (true, false) => assert!((-(2. * PI)..=-PI).contains(&dphi)),
    //     (true, true) => assert!((PI..=(2. * PI)).contains(&dphi)),
    // }

    // Subtract TOL so 90.0001 deg doesn't become 2 segs
    let segments = ((dphi / FRAC_PI_2).abs() - TOL).ceil();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let i_segments = segments as u8; // u8 is fine since segments <= 4
    let dphi = dphi / segments;

    (0..i_segments)
        .map(|i| phi0 + f32::from(i) * dphi) // Starting angle for segment
        .map(|phi0| create_arc(r, phi0, dphi)) // Create seggment arc
        .map(|(ctrl1, ctrl2, point)| {
            // Re-rotate by xar
            let (sin, cos) = xar.sin_cos();
            let [ctrl1, ctrl2, point] = [ctrl1, ctrl2, point].map(|p| Size {
                w: p.w * cos - p.h * sin,
                h: p.w * sin + p.h * cos,
            });
            (ctrl1, ctrl2, point)
        })
        .collect()
}

fn get_center(r: Size, laf: bool, sf: bool, d: Size) -> Size {
    // Since we only use half d in this calculation, pre-halve it
    let d_2 = d / 2.;

    let sign = if laf == sf { 1. } else { -1. };

    let expr = (r.w * d_2.h).powi(2) + (r.h * d_2.w).powi(2);
    let v = ((r.w * r.h).powi(2) - expr) / expr;

    let co = if v.abs() < TOL { 0. } else { sign * v.sqrt() };
    let c = Size {
        w: r.w * d_2.h / r.h,
        h: -r.h * d_2.w / r.w,
    };

    c * co + d_2
}

fn create_arc(r: Size, phi0: f32, dphi: f32) -> (Size, Size, Size) {
    let a = (4. / 3.) * (dphi / 4.).tan();

    let swap = |(a, b)| (b, a);
    let d1: Size = swap(phi0.sin_cos()).into();
    let d4: Size = swap((phi0 + dphi).sin_cos()).into();

    let d2 = Size {
        w: d1.w - d1.h * a,
        h: d1.h + d1.w * a,
    };
    let d3 = Size {
        w: d4.w + d4.h * a,
        h: d4.h - d4.w * a,
    };

    let r_scale = Scale { x: r.w, y: r.h };

    (
        (d2 - d1) * r_scale,
        (d3 - d1) * r_scale,
        (d4 - d1) * r_scale,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f32::consts::{FRAC_PI_2, PI, SQRT_2};

    use assert_approx_eq::assert_approx_eq;

    use crate::utils::Size;

    #[test]
    fn test_arc_to_bezier() {
        struct Params {
            r: Size,
            xar: f32,
            laf: bool,
            sf: bool,
            d: Size,
            exp: Vec<Size>,
        }
        let params = vec![
            Params {
                r: Size::new(1., 1.),
                xar: 0.,
                laf: false,
                sf: false,
                d: Size::new(1., 1.),
                exp: vec![Size::new(1., 1.)],
            },
            Params {
                r: Size::new(1., 1.),
                xar: 0.,
                laf: true,
                sf: false,
                d: Size::new(1., 1.),
                exp: vec![Size::new(-1., 1.), Size::new(1., 1.), Size::new(1., -1.)],
            },
            Params {
                r: Size::new(1., 1.),
                xar: 0.,
                laf: true,
                sf: true,
                d: Size::new(1., 1.),
                exp: vec![Size::new(1., -1.), Size::new(1., 1.), Size::new(-1., 1.)],
            },
            Params {
                r: Size::new(1., 1.),
                xar: 0.,
                laf: true,
                sf: true,
                d: Size::new(1., -1.),
                exp: vec![Size::new(-1., -1.), Size::new(1., -1.), Size::new(1., 1.)],
            },
            Params {
                r: Size::new(1., 2.),
                xar: 0.,
                laf: false,
                sf: false,
                d: Size::new(1., 2.),
                exp: vec![Size::new(1., 2.)],
            },
            Params {
                r: Size::new(1., 2.),
                xar: FRAC_PI_2,
                laf: false,
                sf: false,
                d: Size::new(2., -1.),
                exp: vec![Size::new(2., -1.)],
            },
            Params {
                r: Size::new(1., 1.),
                xar: 0.,
                laf: false,
                sf: false,
                d: Size::new(0., 0.),
                exp: vec![],
            },
            Params {
                r: Size::new(SQRT_2, SQRT_2),
                xar: 0.,
                laf: false,
                sf: true,
                d: Size::new(0., -2.),
                exp: vec![Size::new(0., -2.)],
            },
            Params {
                r: Size::new(SQRT_2, SQRT_2),
                xar: 0.,
                laf: false,
                sf: false,
                d: Size::new(0., 2.),
                exp: vec![Size::new(0., 2.)],
            },
            Params {
                r: Size::new(1., 1.),
                xar: 0.,
                laf: false,
                sf: false,
                d: Size::new(4., 0.),
                exp: vec![Size::new(2., 2.), Size::new(2., -2.)],
            },
            Params {
                r: Size::new(0., 0.),
                xar: 0.,
                laf: false,
                sf: false,
                d: Size::new(1., 0.),
                exp: vec![Size::new(1., 0.)],
            },
        ];

        for Params {
            r,
            xar,
            laf,
            sf,
            d,
            exp,
        } in params
        {
            let points = arc_to_bezier(r, xar, laf, sf, d);
            let points = points.into_iter().map(|i| i.2);

            assert_eq!(points.len(), exp.len());
            for (pnt, res) in points.zip(exp) {
                assert_approx_eq!(pnt.w, res.w);
                assert_approx_eq!(pnt.h, res.h);
            }
        }
    }

    #[test]
    fn test_get_center() {
        struct Params {
            r: Size,
            laf: bool,
            sf: bool,
            d: Size,
            exp: Size,
        }
        let params = vec![
            Params {
                r: Size::new(1., 1.),
                laf: false,
                sf: false,
                d: Size::new(1., 1.),
                exp: Size::new(1., 0.),
            },
            Params {
                r: Size::new(1., 1.),
                laf: true,
                sf: false,
                d: Size::new(1., 1.),
                exp: Size::new(0., 1.),
            },
            Params {
                r: Size::new(1., 1.),
                laf: false,
                sf: true,
                d: Size::new(1., 1.),
                exp: Size::new(0., 1.),
            },
            Params {
                r: Size::new(1., 1.),
                laf: true,
                sf: true,
                d: Size::new(1., 1.),
                exp: Size::new(1., 0.),
            },
            Params {
                r: Size::new(1., 1.),
                laf: false,
                sf: false,
                d: Size::new(2., 0.),
                exp: Size::new(1., 0.),
            },
        ];

        for Params { r, laf, sf, d, exp } in params {
            let point = get_center(r, laf, sf, d);
            assert_approx_eq!(point.w, exp.w);
            assert_approx_eq!(point.h, exp.h);
        }
    }

    #[test]
    fn test_create_arc() {
        const A: f32 = (4. / 3.) * (SQRT_2 - 1.); // (4 / 3) * tan(90deg / 4)
        struct Params {
            r: Size,
            phi0: f32,
            dphi: f32,
            p: (Size, Size, Size),
        }
        let params = vec![
            Params {
                r: Size::new(1., 1.),
                phi0: 0.,
                dphi: FRAC_PI_2,
                p: (Size::new(0., A), Size::new(A - 1., 1.), Size::new(-1., 1.)),
            },
            Params {
                r: Size::new(1., 1.),
                phi0: FRAC_PI_2,
                dphi: FRAC_PI_2,
                p: (
                    Size::new(-A, 0.),
                    Size::new(-1., A - 1.),
                    Size::new(-1., -1.),
                ),
            },
            Params {
                r: Size::new(1., 1.),
                phi0: PI,
                dphi: FRAC_PI_2,
                p: (
                    Size::new(0., -A),
                    Size::new(1. - A, -1.),
                    Size::new(1., -1.),
                ),
            },
            Params {
                r: Size::new(1., 1.),
                phi0: -FRAC_PI_2,
                dphi: FRAC_PI_2,
                p: (Size::new(A, 0.), Size::new(1., 1. - A), Size::new(1., 1.)),
            },
            Params {
                r: Size::new(1., 1.),
                phi0: 0.,
                dphi: -FRAC_PI_2,
                p: (
                    Size::new(0., -A),
                    Size::new(A - 1., -1.),
                    Size::new(-1., -1.),
                ),
            },
            Params {
                r: Size::new(1., 1.),
                phi0: FRAC_PI_2,
                dphi: -FRAC_PI_2,
                p: (Size::new(A, 0.), Size::new(1., A - 1.), Size::new(1., -1.)),
            },
            Params {
                r: Size::new(1., 1.),
                phi0: PI,
                dphi: -FRAC_PI_2,
                p: (Size::new(0., A), Size::new(1. - A, 1.), Size::new(1., 1.)),
            },
            Params {
                r: Size::new(1., 1.),
                phi0: -FRAC_PI_2,
                dphi: -FRAC_PI_2,
                p: (
                    Size::new(-A, 0.),
                    Size::new(-1., 1. - A),
                    Size::new(-1., 1.),
                ),
            },
            Params {
                r: Size::new(2., 1.),
                phi0: 0.,
                dphi: FRAC_PI_2,
                p: (
                    Size::new(0., A),
                    Size::new(2. * (A - 1.), 1.),
                    Size::new(-2., 1.),
                ),
            },
        ];

        for Params { r, phi0, dphi, p } in params {
            let points = create_arc(r, phi0, dphi);

            assert_approx_eq!(p.0.w, points.0.w);
            assert_approx_eq!(p.0.h, points.0.h);
            assert_approx_eq!(p.1.w, points.1.w);
            assert_approx_eq!(p.1.h, points.1.h);
            assert_approx_eq!(p.2.w, points.2.w);
            assert_approx_eq!(p.2.h, points.2.h);
        }
    }
}
