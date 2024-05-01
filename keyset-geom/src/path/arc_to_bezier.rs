use isclose::IsClose;

use crate::{Angle, ExtVec, Vector};

pub fn arc_to_bezier<U>(
    r: Vector<U>,
    xar: Angle,
    laf: bool,
    sf: bool,
    d: Vector<U>,
) -> Vec<(Vector<U>, Vector<U>, Vector<U>)> {
    // Ensure our radii are large enough
    // If either radius is 0 we just return a straight line
    let r = r.abs();
    if d.length().is_close(0.0) || r.x.is_close(0.0) || r.y.is_close(0.0) {
        return vec![(d / 3.0, d * (2.0 / 3.0), d)];
    }

    // Rotate the point by -xar. We calculate the result as if xar==0 and then re-rotate the result
    // It's a lot easier this way, I swear
    let d = d.rotate(-xar);

    // Scale the radii up if they can't meet the distance, maintaining their ratio
    let lambda = d.component_div(r * 2.0).length().max(1.0);
    let r = r * lambda;

    let c = get_center(r, laf, sf, d);

    let phi0 = {
        // Vector::angle_from_x_axis is super inaccurate, so we use atan2 directly
        let c_r = (-c).component_div(r);
        Angle::radians(f32::atan2(c_r.y, c_r.x))
    };
    let dphi = {
        let dc_r = (d - c).component_div(r);
        Angle::radians(f32::atan2(dc_r.y, dc_r.x)) - phi0
    };

    // Add and subtract 2pi (360 deg) to make sure dphi is the correct angle to sweep
    let dphi = match (laf, sf) {
        (true, true) if dphi < Angle::pi() => dphi + Angle::two_pi(),
        (true, false) if dphi > -Angle::pi() => dphi - Angle::two_pi(),
        (false, true) if dphi < Angle::zero() => dphi + Angle::two_pi(),
        (false, false) if dphi > Angle::zero() => dphi - Angle::two_pi(),
        _ => dphi,
    };

    // Double checks the quadrant of dphi
    // TODO remove these? They shouldn't ever fail I think aside from the odd tolerance issue
    match (laf, sf) {
        (false, false) => debug_assert!((-Angle::pi()..=Angle::zero()).contains(&dphi)),
        (false, true) => debug_assert!((Angle::zero()..=Angle::pi()).contains(&dphi)),
        (true, false) => debug_assert!((-Angle::two_pi()..=-Angle::pi()).contains(&dphi)),
        (true, true) => debug_assert!((Angle::pi()..=Angle::two_pi()).contains(&dphi)),
    }

    // Subtract f32::TOLERANCE so 90.0001 deg doesn't become 2 segs
    let segments = ((dphi / Angle::frac_pi_2()).abs() - f32::ABS_TOL).ceil();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let i_segments = segments as u8; // u8 is fine since segments <= 4
    let dphi = dphi / segments;

    (0..i_segments)
        .map(|i| phi0 + dphi * i.into()) // Starting angle for segment
        .map(|phi0| create_arc(r, phi0, dphi)) // Create segment arc
        .map(|(ctrl1, ctrl2, point)| {
            // Re-rotate by xar
            let [ctrl1, ctrl2, point] = [ctrl1, ctrl2, point].map(|p| p.rotate(xar));
            (ctrl1, ctrl2, point)
        })
        .collect()
}

fn get_center<U>(r: Vector<U>, laf: bool, sf: bool, d: Vector<U>) -> Vector<U> {
    // Since we only use half d in this calculation, pre-halve it
    let d_2 = d / 2.0;

    let sign = if laf == sf { 1.0 } else { -1.0 };

    let expr = (r.x * d_2.y).powi(2) + (r.y * d_2.x).powi(2);
    let v = ((r.x * r.y).powi(2) - expr) / expr;

    let co = if v.is_close(0.0) {
        0.0
    } else {
        sign * v.sqrt()
    };
    let c = Vector::new(r.x * d_2.y / r.y, -r.y * d_2.x / r.x);

    c * co + d_2
}

fn create_arc<U>(r: Vector<U>, phi0: Angle, dphi: Angle) -> (Vector<U>, Vector<U>, Vector<U>) {
    let a = (4.0 / 3.0) * (dphi / 4.0).radians.tan();

    let d1 = Vector::from(phi0.sin_cos()).yx();
    let d4 = Vector::from((phi0 + dphi).sin_cos()).yx();

    let d2 = Vector::new(d1.x - d1.y * a, d1.y + d1.x * a);
    let d3 = Vector::new(d4.x + d4.y * a, d4.y - d4.x * a);

    (
        (d2 - d1).component_mul(r),
        (d3 - d1).component_mul(r),
        (d4 - d1).component_mul(r),
    )
}

#[cfg(test)]
mod tests {
    use isclose::assert_is_close;

    use super::*;

    use std::f32::consts::SQRT_2;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_arc_to_bezier() {
        struct Params {
            r: Vector<()>,
            xar: Angle,
            laf: bool,
            sf: bool,
            d: Vector<()>,
            exp: Vec<Vector<()>>,
        }
        let params = vec![
            Params {
                r: Vector::new(1.0, 1.0),
                xar: Angle::zero(),
                laf: false,
                sf: false,
                d: Vector::new(1.0, 1.0),
                exp: vec![Vector::new(1.0, 1.0)],
            },
            Params {
                r: Vector::new(1.0, 1.0),
                xar: Angle::zero(),
                laf: true,
                sf: false,
                d: Vector::new(1.0, 1.0),
                exp: vec![
                    Vector::new(-1.0, 1.0),
                    Vector::new(1.0, 1.0),
                    Vector::new(1.0, -1.0),
                ],
            },
            Params {
                r: Vector::new(1.0, 1.0),
                xar: Angle::zero(),
                laf: true,
                sf: true,
                d: Vector::new(1.0, 1.0),
                exp: vec![
                    Vector::new(1.0, -1.0),
                    Vector::new(1.0, 1.0),
                    Vector::new(-1.0, 1.0),
                ],
            },
            Params {
                r: Vector::new(1.0, 1.0),
                xar: Angle::zero(),
                laf: true,
                sf: true,
                d: Vector::new(1.0, -1.0),
                exp: vec![
                    Vector::new(-1.0, -1.0),
                    Vector::new(1.0, -1.0),
                    Vector::new(1.0, 1.0),
                ],
            },
            Params {
                r: Vector::new(1.0, 2.0),
                xar: Angle::zero(),
                laf: false,
                sf: false,
                d: Vector::new(1.0, 2.0),
                exp: vec![Vector::new(1.0, 2.0)],
            },
            Params {
                r: Vector::new(1.0, 2.0),
                xar: Angle::frac_pi_2(),
                laf: false,
                sf: false,
                d: Vector::new(2.0, -1.0),
                exp: vec![Vector::new(2.0, -1.0)],
            },
            Params {
                r: Vector::new(1.0, 1.0),
                xar: Angle::zero(),
                laf: false,
                sf: false,
                d: Vector::zero(),
                exp: vec![Vector::zero()],
            },
            Params {
                r: Vector::new(SQRT_2, SQRT_2),
                xar: Angle::zero(),
                laf: false,
                sf: true,
                d: Vector::new(0.0, -2.0),
                exp: vec![Vector::new(0.0, -2.0)],
            },
            Params {
                r: Vector::new(SQRT_2, SQRT_2),
                xar: Angle::zero(),
                laf: false,
                sf: false,
                d: Vector::new(0.0, 2.0),
                exp: vec![Vector::new(0.0, 2.0)],
            },
            Params {
                r: Vector::new(1.0, 1.0),
                xar: Angle::zero(),
                laf: false,
                sf: false,
                d: Vector::new(2.0, 0.0),
                exp: vec![Vector::new(1.0, 1.0), Vector::new(1.0, -1.0)],
            },
            Params {
                r: Vector::new(1.0, 1.0),
                xar: Angle::zero(),
                laf: false,
                sf: false,
                d: Vector::new(4.0, 0.0),
                exp: vec![Vector::new(2.0, 2.0), Vector::new(2.0, -2.0)],
            },
            Params {
                r: Vector::zero(),
                xar: Angle::zero(),
                laf: false,
                sf: false,
                d: Vector::new(1.0, 0.0),
                exp: vec![Vector::new(1.0, 0.0)],
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
            for (pnt, exp) in points.zip(exp) {
                assert_is_close!(pnt, exp);
            }
        }
    }

    #[test]
    fn test_get_center() {
        struct Params {
            r: Vector<()>,
            laf: bool,
            sf: bool,
            d: Vector<()>,
            exp: Vector<()>,
        }
        let params = vec![
            Params {
                r: Vector::new(1.0, 1.0),
                laf: false,
                sf: false,
                d: Vector::new(1.0, 1.0),
                exp: Vector::new(1.0, 0.0),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                laf: true,
                sf: false,
                d: Vector::new(1.0, 1.0),
                exp: Vector::new(0.0, 1.0),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                laf: false,
                sf: true,
                d: Vector::new(1.0, 1.0),
                exp: Vector::new(0.0, 1.0),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                laf: true,
                sf: true,
                d: Vector::new(1.0, 1.0),
                exp: Vector::new(1.0, 0.0),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                laf: false,
                sf: false,
                d: Vector::new(2.0, 0.0),
                exp: Vector::new(1.0, 0.0),
            },
        ];

        for Params { r, laf, sf, d, exp } in params {
            let point = get_center(r, laf, sf, d);
            assert_is_close!(point, exp);
        }
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_create_arc() {
        struct Params {
            r: Vector<()>,
            phi0: Angle,
            dphi: Angle,
            p: (Vector<()>, Vector<()>, Vector<()>),
        }
        let a = (4.0 / 3.0) * Angle::degrees(90.0 / 4.0).radians.tan();
        let params = vec![
            Params {
                r: Vector::new(1.0, 1.0),
                phi0: Angle::zero(),
                dphi: Angle::frac_pi_2(),
                p: (
                    Vector::new(0.0, a),
                    Vector::new(a - 1.0, 1.0),
                    Vector::new(-1.0, 1.0),
                ),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                phi0: Angle::frac_pi_2(),
                dphi: Angle::frac_pi_2(),
                p: (
                    Vector::new(-a, 0.0),
                    Vector::new(-1.0, a - 1.0),
                    Vector::new(-1.0, -1.0),
                ),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                phi0: Angle::pi(),
                dphi: Angle::frac_pi_2(),
                p: (
                    Vector::new(0.0, -a),
                    Vector::new(1.0 - a, -1.0),
                    Vector::new(1.0, -1.0),
                ),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                phi0: -Angle::frac_pi_2(),
                dphi: Angle::frac_pi_2(),
                p: (
                    Vector::new(a, 0.0),
                    Vector::new(1.0, 1.0 - a),
                    Vector::new(1.0, 1.0),
                ),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                phi0: Angle::zero(),
                dphi: -Angle::frac_pi_2(),
                p: (
                    Vector::new(0.0, -a),
                    Vector::new(a - 1.0, -1.0),
                    Vector::new(-1.0, -1.0),
                ),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                phi0: Angle::frac_pi_2(),
                dphi: -Angle::frac_pi_2(),
                p: (
                    Vector::new(a, 0.0),
                    Vector::new(1.0, a - 1.0),
                    Vector::new(1.0, -1.0),
                ),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                phi0: Angle::pi(),
                dphi: -Angle::frac_pi_2(),
                p: (
                    Vector::new(0.0, a),
                    Vector::new(1.0 - a, 1.0),
                    Vector::new(1.0, 1.0),
                ),
            },
            Params {
                r: Vector::new(1.0, 1.0),
                phi0: -Angle::frac_pi_2(),
                dphi: -Angle::frac_pi_2(),
                p: (
                    Vector::new(-a, 0.0),
                    Vector::new(-1.0, 1.0 - a),
                    Vector::new(-1.0, 1.0),
                ),
            },
            Params {
                r: Vector::new(2.0, 1.0),
                phi0: Angle::zero(),
                dphi: Angle::frac_pi_2(),
                p: (
                    Vector::new(0.0, a),
                    Vector::new(2.0 * (a - 1.0), 1.0),
                    Vector::new(-2.0, 1.0),
                ),
            },
        ];

        for Params { r, phi0, dphi, p } in params {
            let points = create_arc(r, phi0, dphi);

            assert_is_close!(p.0, points.0);
            assert_is_close!(p.1, points.1);
            assert_is_close!(p.2, points.2);
        }
    }
}
