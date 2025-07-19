use isclose::IsClose as _;
use saturate::SaturatingFrom as _;

use crate::{Angle, ExtVec as _, Vector};

pub fn arc_to_bezier<U>(
    r: Vector<U>,
    xar: Angle,
    laf: bool,
    sf: bool,
    d: Vector<U>,
    mut cb: impl FnMut(Vector<U>, Vector<U>, Vector<U>),
) {
    // Ensure our radii are large enough
    // If either radius is 0 we just return a straight line
    let r = r.abs();
    if d.length().is_close(0.0) || r.x.is_close(0.0) || r.y.is_close(0.0) {
        cb(d / 3.0, d * (2.0 / 3.0), d);
        return;
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

    // Double checks the quadrant of dphi. Shouldn't ever fail aside from maybe tolerance issues?
    // TODO these are failing during tests (due to tolerance) on aarch64-apple-darwin
    // match (laf, sf) {
    //     (false, false) => debug_assert!((-Angle::pi()..=Angle::zero()).contains(&dphi)),
    //     (false, true) => debug_assert!((Angle::zero()..=Angle::pi()).contains(&dphi)),
    //     (true, false) => debug_assert!((-Angle::two_pi()..=-Angle::pi()).contains(&dphi)),
    //     (true, true) => debug_assert!((Angle::pi()..=Angle::two_pi()).contains(&dphi)),
    // }

    // Subtract f32::TOLERANCE so 90.0001 deg doesn't become 2 segs
    let segments = ((dphi / Angle::frac_pi_2()).abs() - f32::ABS_TOL).ceil();
    let i_segments = u8::saturating_from(segments); // 0 < segments <= 4
    let dphi = dphi / segments;

    for i in 0..i_segments {
        let phi0 = phi0 + dphi * i.into(); // Starting angle for segment
        let (d1, d2, d) = create_arc(r, phi0, dphi); // Create segment arc
        let (d1, d2, d) = <[Vector<_>; 3]>::from((d1, d2, d))
            .map(|d| d.rotate(xar)) // Re-rotate by xar
            .into();
        cb(d1, d2, d);
    }
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
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use std::f32::consts::SQRT_2;

    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn test_arc_to_bezier() {
        struct Params {
            r: Vector<()>,
            xar: Angle,
            laf: bool,
            sf: bool,
            d: Vector<()>,
        }
        fn params(r: (f32, f32), xar: f32, laf: bool, sf: bool, d: (f32, f32)) -> Params {
            Params {
                r: r.into(),
                xar: Angle::degrees(xar),
                laf,
                sf,
                d: d.into(),
            }
        }
        let params = [
            params((1.0, 1.0), 0.0, false, false, (1.0, 1.0)),
            params((1.0, 1.0), 0.0, true, false, (1.0, 1.0)),
            params((1.0, 1.0), 0.0, true, true, (1.0, 1.0)),
            params((1.0, 1.0), 0.0, true, true, (1.0, -1.0)),
            params((1.0, 2.0), 0.0, false, false, (1.0, 2.0)),
            params((1.0, 2.0), 90.0, false, false, (2.0, -1.0)),
            params((1.0, 1.0), 0.0, false, false, (0.0, 0.0)),
            params((SQRT_2, SQRT_2), 0.0, false, true, (0.0, -2.0)),
            params((SQRT_2, SQRT_2), 0.0, false, false, (0.0, 2.0)),
            params((1.0, 1.0), 0.0, false, false, (2.0, 0.0)),
            params((1.0, 1.0), 0.0, false, false, (4.0, 0.0)),
            params((0.0, 0.0), 0.0, false, false, (1.0, 0.0)),
        ];
        let expected = [
            vec![Vector::new(1.0, 1.0)],
            vec![
                Vector::new(-1.0, 1.0),
                Vector::new(1.0, 1.0),
                Vector::new(1.0, -1.0),
            ],
            vec![
                Vector::new(1.0, -1.0),
                Vector::new(1.0, 1.0),
                Vector::new(-1.0, 1.0),
            ],
            vec![
                Vector::new(-1.0, -1.0),
                Vector::new(1.0, -1.0),
                Vector::new(1.0, 1.0),
            ],
            vec![Vector::new(1.0, 2.0)],
            vec![Vector::new(2.0, -1.0)],
            vec![Vector::new(0.0, 0.0)],
            vec![Vector::new(0.0, -2.0)],
            vec![Vector::new(0.0, 2.0)],
            vec![Vector::new(1.0, 1.0), Vector::new(1.0, -1.0)],
            vec![Vector::new(2.0, 2.0), Vector::new(2.0, -2.0)],
            vec![Vector::new(1.0, 0.0)],
        ];

        for (p, exp) in params.into_iter().zip(expected) {
            let mut points = vec![];
            arc_to_bezier(p.r, p.xar, p.laf, p.sf, p.d, |_p1, _p2, p| points.push(p));

            assert_eq!(points.len(), exp.len());
            for (pnt, exp) in points.into_iter().zip(exp) {
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
        }
        fn params(r: (f32, f32), laf: bool, sf: bool, d: (f32, f32)) -> Params {
            Params {
                r: r.into(),
                laf,
                sf,
                d: d.into(),
            }
        }
        let params = [
            params((1.0, 1.0), false, false, (1.0, 1.0)),
            params((1.0, 1.0), true, false, (1.0, 1.0)),
            params((1.0, 1.0), false, true, (1.0, 1.0)),
            params((1.0, 1.0), true, true, (1.0, 1.0)),
            params((1.0, 1.0), false, false, (2.0, 0.0)),
        ];
        let expected = [
            Vector::new(1.0, 0.0),
            Vector::new(0.0, 1.0),
            Vector::new(0.0, 1.0),
            Vector::new(1.0, 0.0),
            Vector::new(1.0, 0.0),
        ];

        for (p, exp) in params.into_iter().zip(expected) {
            let point = get_center(p.r, p.laf, p.sf, p.d);
            assert_is_close!(point, exp);
        }
    }

    #[test]
    fn test_create_arc() {
        struct Params {
            r: Vector<()>,
            phi0: Angle,
            dphi: Angle,
        }
        fn params(r: (f32, f32), phi0: f32, dphi: f32) -> Params {
            Params {
                r: r.into(),
                phi0: Angle::degrees(phi0),
                dphi: Angle::degrees(dphi),
            }
        }
        let a = (4.0 / 3.0) * Angle::degrees(90.0 / 4.0).radians.tan();
        let params = [
            params((1.0, 1.0), 0.0, 90.0),
            params((1.0, 1.0), 90.0, 90.0),
            params((1.0, 1.0), 180.0, 90.0),
            params((1.0, 1.0), -90.0, 90.0),
            params((1.0, 1.0), 0.0, -90.0),
            params((1.0, 1.0), 90.0, -90.0),
            params((1.0, 1.0), 180.0, -90.0),
            params((1.0, 1.0), -90.0, -90.0),
            params((2.0, 1.0), 0.0, 90.0),
        ];
        let expected = [
            [(0.0, a), (a - 1.0, 1.0), (-1.0, 1.0)],
            [(-a, 0.0), (-1.0, a - 1.0), (-1.0, -1.0)],
            [(0.0, -a), (1.0 - a, -1.0), (1.0, -1.0)],
            [(a, 0.0), (1.0, 1.0 - a), (1.0, 1.0)],
            [(0.0, -a), (a - 1.0, -1.0), (-1.0, -1.0)],
            [(a, 0.0), (1.0, a - 1.0), (1.0, -1.0)],
            [(0.0, a), (1.0 - a, 1.0), (1.0, 1.0)],
            [(-a, 0.0), (-1.0, 1.0 - a), (-1.0, 1.0)],
            [(0.0, a), (2.0 * (a - 1.0), 1.0), (-2.0, 1.0)],
        ]
        .map(|pts| pts.map(Vector::from));

        for (p, exp) in params.into_iter().zip(expected) {
            let points = create_arc(p.r, p.phi0, p.dphi);

            assert_is_close!(points.0, exp[0]);
            assert_is_close!(points.1, exp[1]);
            assert_is_close!(points.2, exp[2]);
        }
    }
}
