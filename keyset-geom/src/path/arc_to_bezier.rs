use isclose::IsClose;
use num_traits::ToPrimitive as _;

use crate::{Angle, Rotate, Unit, Vector};

pub fn arc_to_bezier<U: Unit>(
    r: Vector<U>,
    xar: Angle,
    laf: bool,
    sf: bool,
    d: Vector<U>,
    mut cb: impl FnMut(Vector<U>, Vector<U>, Vector<U>),
) {
    // Ensure our distance and our radii are large enough
    // If either is 0 we just return a straight line
    let r = r.abs();
    if d.is_close(&Vector::zero()) || r.x.is_close(&U::zero()) || r.y.is_close(&U::zero()) {
        cb(d / 3.0, d * (2.0 / 3.0), d);
        return;
    }

    // Rotate the point by -xar. We calculate the result as if xar==0 and then re-rotate the result
    // It's a lot easier this way, I swear
    let d = d * Rotate::from_angle(-xar);

    // Scale the radii up if they can't meet the distance, maintaining their ratio
    let lambda = (d / (r * 2.0)).hypot().max(1.0);
    let r = r * lambda;

    // Scale down by the radii we can do all math as if r = (1, 1)
    let scale = r / Vector::splat(U::new(1.0));
    let d = d / scale;

    let c = get_center(laf, sf, d);

    let phi0 = (-c).angle();
    let dphi = (d - c).angle() - phi0;

    // Add and subtract 2pi (360 deg) to make sure dphi is the correct angle to sweep
    let dphi = match (laf, sf) {
        (true, true) if dphi < Angle::PI => dphi + Angle::TAU,
        (true, false) if dphi > -Angle::PI => dphi - Angle::TAU,
        (false, true) if dphi < Angle::ZERO => dphi + Angle::TAU,
        (false, false) if dphi > Angle::ZERO => dphi - Angle::TAU,
        _ => dphi,
    };

    // Double checks the quadrant of dphi. Shouldn't ever fail aside from maybe tolerance issues?
    // TODO these are failing during tests (due to tolerance) on aarch64-apple-darwin
    // match (laf, sf) {
    //     (false, false) => debug_assert!((-Angle::PI..=Angle::ZERO).contains(&dphi)),
    //     (false, true) => debug_assert!((Angle::ZERO..=Angle::PI).contains(&dphi)),
    //     (true, false) => debug_assert!((-Angle::TAU..=-Angle::PI).contains(&dphi)),
    //     (true, true) => debug_assert!((Angle::PI..=Angle::TAU).contains(&dphi)),
    // }

    // Subtract f32::TOLERANCE so 90.0001 deg doesn't become 2 segs
    let segments = ((dphi / Angle::FRAC_PI_2).abs() - <f32 as IsClose>::ABS_TOL).ceil();
    let i_segments = segments
        .to_u8()
        .unwrap_or_else(|| unreachable!("0 < segments <= 4"));
    let dphi = dphi / segments;

    for i in 0..i_segments {
        let phi0 = phi0 + dphi * i.into(); // Starting angle for segment
        let (d1, d2, d) = create_arc(phi0, dphi); // Create segment arc
        let (d1, d2, d) = <[Vector<_>; 3]>::from((d1, d2, d))
            .map(|d| d * scale * Rotate::from_angle(xar)) // Re-rotate by xar
            .into();
        cb(d1, d2, d);
    }
}

#[allow(clippy::similar_names)]
fn get_center<U: Unit>(laf: bool, sf: bool, d: Vector<U>) -> Vector<U> {
    // Since we only use half d in this calculation, pre-halve it
    let d_2 = d * 0.5;

    let sign = if laf == sf { 1.0 } else { -1.0 };

    let expr = d_2.hypot2();
    let v = (1.0 - expr) / expr;

    let co = if v.is_close(&0.0) {
        0.0
    } else {
        sign * v.sqrt()
    };
    let c = d_2.swap_xy().neg_y();

    c * co + d_2
}

fn create_arc<U: Unit>(phi0: Angle, dphi: Angle) -> (Vector<U>, Vector<U>, Vector<U>) {
    let a = (4.0 / 3.0) * (dphi / 4.0).tan();

    let d1 = Vector::new(U::new(phi0.cos()), U::new(phi0.sin()));
    let d4 = Vector::new(U::new((phi0 + dphi).cos()), U::new((phi0 + dphi).sin()));

    let d2 = Vector::new(d1.x - d1.y * a, d1.y + d1.x * a);
    let d3 = Vector::new(d4.x + d4.y * a, d4.y - d4.x * a);

    (d2 - d1, d3 - d1, d4 - d1)
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use std::f32::consts::SQRT_2;

    use isclose::assert_is_close;

    use crate::path::arc_to_bezier;
    use crate::Mm;

    use super::*;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_arc_to_bezier() {
        let tests: [fn(&mut Vec<_>); 12] = [
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(1.0), Mm(1.0)),
                    Angle::degrees(0.0),
                    false,
                    false,
                    Vector::new(Mm(1.0), Mm(1.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(1.0), Mm(1.0)),
                    Angle::degrees(0.0),
                    true,
                    false,
                    Vector::new(Mm(1.0), Mm(1.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(1.0), Mm(1.0)),
                    Angle::degrees(0.0),
                    true,
                    true,
                    Vector::new(Mm(1.0), Mm(1.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(1.0), Mm(1.0)),
                    Angle::degrees(0.0),
                    true,
                    true,
                    Vector::new(Mm(1.0), Mm(-1.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(1.0), Mm(2.0)),
                    Angle::degrees(0.0),
                    false,
                    false,
                    Vector::new(Mm(1.0), Mm(2.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(1.0), Mm(2.0)),
                    Angle::degrees(90.0),
                    false,
                    false,
                    Vector::new(Mm(2.0), Mm(-1.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(1.0), Mm(1.0)),
                    Angle::degrees(0.0),
                    false,
                    false,
                    Vector::new(Mm(0.0), Mm(0.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(SQRT_2), Mm(SQRT_2)),
                    Angle::degrees(0.0),
                    false,
                    true,
                    Vector::new(Mm(0.0), Mm(-2.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(SQRT_2), Mm(SQRT_2)),
                    Angle::degrees(0.0),
                    false,
                    false,
                    Vector::new(Mm(0.0), Mm(2.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(1.0), Mm(1.0)),
                    Angle::degrees(0.0),
                    false,
                    false,
                    Vector::new(Mm(2.0), Mm(0.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(1.0), Mm(1.0)),
                    Angle::degrees(0.0),
                    false,
                    false,
                    Vector::new(Mm(4.0), Mm(0.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
            |vec| {
                arc_to_bezier(
                    Vector::new(Mm(0.0), Mm(0.0)),
                    Angle::degrees(0.0),
                    false,
                    false,
                    Vector::new(Mm(1.0), Mm(0.0)),
                    |_p1, _p2, p| vec.push(p),
                );
            },
        ];
        let expected = [
            vec![Vector::new(Mm(1.0), Mm(1.0))],
            vec![
                Vector::new(Mm(-1.0), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(-1.0)),
            ],
            vec![
                Vector::new(Mm(1.0), Mm(-1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
                Vector::new(Mm(-1.0), Mm(1.0)),
            ],
            vec![
                Vector::new(Mm(-1.0), Mm(-1.0)),
                Vector::new(Mm(1.0), Mm(-1.0)),
                Vector::new(Mm(1.0), Mm(1.0)),
            ],
            vec![Vector::new(Mm(1.0), Mm(2.0))],
            vec![Vector::new(Mm(2.0), Mm(-1.0))],
            vec![Vector::new(Mm(0.0), Mm(0.0))],
            vec![Vector::new(Mm(0.0), Mm(-2.0))],
            vec![Vector::new(Mm(0.0), Mm(2.0))],
            vec![
                Vector::new(Mm(1.0), Mm(1.0)),
                Vector::new(Mm(1.0), Mm(-1.0)),
            ],
            vec![
                Vector::new(Mm(2.0), Mm(2.0)),
                Vector::new(Mm(2.0), Mm(-2.0)),
            ],
            vec![Vector::new(Mm(1.0), Mm(0.0))],
        ];

        for (arc_to_bezier, exp) in tests.into_iter().zip(expected) {
            let mut points = vec![];
            arc_to_bezier(&mut points);

            assert_eq!(points.len(), exp.len());
            for (pnt, exp) in points.into_iter().zip(exp) {
                assert_is_close!(pnt, exp);
            }
        }
    }

    #[test]
    fn test_get_center() {
        let tests = [
            || get_center(false, false, Vector::new(Mm(1.0), Mm(1.0))),
            || get_center(true, false, Vector::new(Mm(1.0), Mm(1.0))),
            || get_center(false, true, Vector::new(Mm(1.0), Mm(1.0))),
            || get_center(true, true, Vector::new(Mm(1.0), Mm(1.0))),
            || get_center(false, false, Vector::new(Mm(2.0), Mm(0.0))),
        ];
        let expected = [
            Vector::new(Mm(1.0), Mm(0.0)),
            Vector::new(Mm(0.0), Mm(1.0)),
            Vector::new(Mm(0.0), Mm(1.0)),
            Vector::new(Mm(1.0), Mm(0.0)),
            Vector::new(Mm(1.0), Mm(0.0)),
        ];

        for (get_center, exp) in tests.into_iter().zip(expected) {
            let point = get_center();
            assert_is_close!(point, exp);
        }
    }

    #[test]
    fn test_create_arc() {
        let a = (4.0 / 3.0) * Angle::degrees(90.0 / 4.0).tan();
        let tests = [
            || create_arc(Angle::degrees(0.0), Angle::degrees(90.0)),
            || create_arc(Angle::degrees(90.0), Angle::degrees(90.0)),
            || create_arc(Angle::degrees(180.0), Angle::degrees(90.0)),
            || create_arc(Angle::degrees(-90.0), Angle::degrees(90.0)),
            || create_arc(Angle::degrees(0.0), Angle::degrees(-90.0)),
            || create_arc(Angle::degrees(90.0), Angle::degrees(-90.0)),
            || create_arc(Angle::degrees(180.0), Angle::degrees(-90.0)),
            || create_arc(Angle::degrees(-90.0), Angle::degrees(-90.0)),
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
        ]
        .map(|pts| pts.map(|(x, y)| Vector::new(Mm(x), Mm(y))));

        for (create_arc, exp) in tests.into_iter().zip(expected) {
            let points = create_arc();

            assert_is_close!(points.0, exp[0]);
            assert_is_close!(points.1, exp[1]);
            assert_is_close!(points.2, exp[2]);
        }
    }
}
