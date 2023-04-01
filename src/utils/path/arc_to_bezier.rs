use std::f32::consts::{FRAC_PI_2, PI};

use crate::utils::Vec2;

const TOL: f32 = 1e-6;

pub fn arc_to_bezier(r: Vec2, xar: f32, laf: bool, sf: bool, d: Vec2) -> Vec<(Vec2, Vec2, Vec2)> {
    if d.abs() < TOL {
        return vec![];
    }

    // Ensure our radii are large enough
    // If either radius is 0 we just return a straight line
    let r = Vec2::new(r.x.abs(), r.y.abs());
    if r.x < TOL || r.y < TOL {
        return vec![(d / 3., d * (2. / 3.), d)];
    }

    // Rotate the point by -xar. We calculate the result as if xar==0 and then re-rotate the result
    // It's a lot easier this way, I swear
    let d = d.rotate(-xar);

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
            let [ctrl1, ctrl2, point] = [ctrl1, ctrl2, point].map(|p| p.rotate(xar));
            (ctrl1, ctrl2, point)
        })
        .collect()
}

fn get_center(r: Vec2, laf: bool, sf: bool, d: Vec2) -> Vec2 {
    // Since we only use half d in this calculation, pre-halve it
    let d_2 = d / 2.;

    let sign = if laf == sf { 1. } else { -1. };

    let expr = (r.x * d_2.y).powi(2) + (r.y * d_2.x).powi(2);
    let v = ((r.x * r.y).powi(2) - expr) / expr;

    let co = if v.abs() < TOL { 0. } else { sign * v.sqrt() };
    let c = Vec2::new(r.x * d_2.y / r.y, -r.y * d_2.x / r.x);

    c * co + d_2
}

fn create_arc(r: Vec2, phi0: f32, dphi: f32) -> (Vec2, Vec2, Vec2) {
    let a = (4. / 3.) * (dphi / 4.).tan();

    let swap = |(a, b)| (b, a);
    let d1: Vec2 = swap(phi0.sin_cos()).into();
    let d4: Vec2 = swap((phi0 + dphi).sin_cos()).into();

    let d2 = Vec2::new(d1.x - d1.y * a, d1.y + d1.x * a);
    let d3 = Vec2::new(d4.x + d4.y * a, d4.y - d4.x * a);

    ((d2 - d1) * r, (d3 - d1) * r, (d4 - d1) * r)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f32::consts::{FRAC_PI_2, PI, SQRT_2};

    use assert_approx_eq::assert_approx_eq;

    use crate::utils::Vec2;

    #[test]
    fn test_arc_to_bezier() {
        struct Params {
            r: Vec2,
            xar: f32,
            laf: bool,
            sf: bool,
            d: Vec2,
            exp: Vec<Vec2>,
        }
        let params = vec![
            Params {
                r: Vec2::new(1., 1.),
                xar: 0.,
                laf: false,
                sf: false,
                d: Vec2::new(1., 1.),
                exp: vec![Vec2::new(1., 1.)],
            },
            Params {
                r: Vec2::new(1., 1.),
                xar: 0.,
                laf: true,
                sf: false,
                d: Vec2::new(1., 1.),
                exp: vec![Vec2::new(-1., 1.), Vec2::new(1., 1.), Vec2::new(1., -1.)],
            },
            Params {
                r: Vec2::new(1., 1.),
                xar: 0.,
                laf: true,
                sf: true,
                d: Vec2::new(1., 1.),
                exp: vec![Vec2::new(1., -1.), Vec2::new(1., 1.), Vec2::new(-1., 1.)],
            },
            Params {
                r: Vec2::new(1., 1.),
                xar: 0.,
                laf: true,
                sf: true,
                d: Vec2::new(1., -1.),
                exp: vec![Vec2::new(-1., -1.), Vec2::new(1., -1.), Vec2::new(1., 1.)],
            },
            Params {
                r: Vec2::new(1., 2.),
                xar: 0.,
                laf: false,
                sf: false,
                d: Vec2::new(1., 2.),
                exp: vec![Vec2::new(1., 2.)],
            },
            Params {
                r: Vec2::new(1., 2.),
                xar: FRAC_PI_2,
                laf: false,
                sf: false,
                d: Vec2::new(2., -1.),
                exp: vec![Vec2::new(2., -1.)],
            },
            Params {
                r: Vec2::new(1., 1.),
                xar: 0.,
                laf: false,
                sf: false,
                d: Vec2::ZERO,
                exp: vec![],
            },
            Params {
                r: Vec2::new(SQRT_2, SQRT_2),
                xar: 0.,
                laf: false,
                sf: true,
                d: Vec2::new(0., -2.),
                exp: vec![Vec2::new(0., -2.)],
            },
            Params {
                r: Vec2::new(SQRT_2, SQRT_2),
                xar: 0.,
                laf: false,
                sf: false,
                d: Vec2::new(0., 2.),
                exp: vec![Vec2::new(0., 2.)],
            },
            Params {
                r: Vec2::new(1., 1.),
                xar: 0.,
                laf: false,
                sf: false,
                d: Vec2::new(4., 0.),
                exp: vec![Vec2::new(2., 2.), Vec2::new(2., -2.)],
            },
            Params {
                r: Vec2::ZERO,
                xar: 0.,
                laf: false,
                sf: false,
                d: Vec2::new(1., 0.),
                exp: vec![Vec2::new(1., 0.)],
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
                assert_approx_eq!(pnt.x, res.x);
                assert_approx_eq!(pnt.y, res.y);
            }
        }
    }

    #[test]
    fn test_get_center() {
        struct Params {
            r: Vec2,
            laf: bool,
            sf: bool,
            d: Vec2,
            exp: Vec2,
        }
        let params = vec![
            Params {
                r: Vec2::new(1., 1.),
                laf: false,
                sf: false,
                d: Vec2::new(1., 1.),
                exp: Vec2::new(1., 0.),
            },
            Params {
                r: Vec2::new(1., 1.),
                laf: true,
                sf: false,
                d: Vec2::new(1., 1.),
                exp: Vec2::new(0., 1.),
            },
            Params {
                r: Vec2::new(1., 1.),
                laf: false,
                sf: true,
                d: Vec2::new(1., 1.),
                exp: Vec2::new(0., 1.),
            },
            Params {
                r: Vec2::new(1., 1.),
                laf: true,
                sf: true,
                d: Vec2::new(1., 1.),
                exp: Vec2::new(1., 0.),
            },
            Params {
                r: Vec2::new(1., 1.),
                laf: false,
                sf: false,
                d: Vec2::new(2., 0.),
                exp: Vec2::new(1., 0.),
            },
        ];

        for Params { r, laf, sf, d, exp } in params {
            let point = get_center(r, laf, sf, d);
            assert_approx_eq!(point.x, exp.x);
            assert_approx_eq!(point.y, exp.y);
        }
    }

    #[test]
    fn test_create_arc() {
        const A: f32 = (4. / 3.) * (SQRT_2 - 1.); // (4 / 3) * tan(90deg / 4)
        struct Params {
            r: Vec2,
            phi0: f32,
            dphi: f32,
            p: (Vec2, Vec2, Vec2),
        }
        let params = vec![
            Params {
                r: Vec2::new(1., 1.),
                phi0: 0.,
                dphi: FRAC_PI_2,
                p: (Vec2::new(0., A), Vec2::new(A - 1., 1.), Vec2::new(-1., 1.)),
            },
            Params {
                r: Vec2::new(1., 1.),
                phi0: FRAC_PI_2,
                dphi: FRAC_PI_2,
                p: (
                    Vec2::new(-A, 0.),
                    Vec2::new(-1., A - 1.),
                    Vec2::new(-1., -1.),
                ),
            },
            Params {
                r: Vec2::new(1., 1.),
                phi0: PI,
                dphi: FRAC_PI_2,
                p: (
                    Vec2::new(0., -A),
                    Vec2::new(1. - A, -1.),
                    Vec2::new(1., -1.),
                ),
            },
            Params {
                r: Vec2::new(1., 1.),
                phi0: -FRAC_PI_2,
                dphi: FRAC_PI_2,
                p: (Vec2::new(A, 0.), Vec2::new(1., 1. - A), Vec2::new(1., 1.)),
            },
            Params {
                r: Vec2::new(1., 1.),
                phi0: 0.,
                dphi: -FRAC_PI_2,
                p: (
                    Vec2::new(0., -A),
                    Vec2::new(A - 1., -1.),
                    Vec2::new(-1., -1.),
                ),
            },
            Params {
                r: Vec2::new(1., 1.),
                phi0: FRAC_PI_2,
                dphi: -FRAC_PI_2,
                p: (Vec2::new(A, 0.), Vec2::new(1., A - 1.), Vec2::new(1., -1.)),
            },
            Params {
                r: Vec2::new(1., 1.),
                phi0: PI,
                dphi: -FRAC_PI_2,
                p: (Vec2::new(0., A), Vec2::new(1. - A, 1.), Vec2::new(1., 1.)),
            },
            Params {
                r: Vec2::new(1., 1.),
                phi0: -FRAC_PI_2,
                dphi: -FRAC_PI_2,
                p: (
                    Vec2::new(-A, 0.),
                    Vec2::new(-1., 1. - A),
                    Vec2::new(-1., 1.),
                ),
            },
            Params {
                r: Vec2::new(2., 1.),
                phi0: 0.,
                dphi: FRAC_PI_2,
                p: (
                    Vec2::new(0., A),
                    Vec2::new(2. * (A - 1.), 1.),
                    Vec2::new(-2., 1.),
                ),
            },
        ];

        for Params { r, phi0, dphi, p } in params {
            let points = create_arc(r, phi0, dphi);

            assert_approx_eq!(p.0.x, points.0.x);
            assert_approx_eq!(p.0.y, points.0.y);
            assert_approx_eq!(p.1.x, points.1.x);
            assert_approx_eq!(p.1.y, points.1.y);
            assert_approx_eq!(p.2.x, points.2.x);
            assert_approx_eq!(p.2.y, points.2.y);
        }
    }
}
