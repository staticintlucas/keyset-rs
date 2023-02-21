use svg::node::element::path::{Command, Data, Position};

use crate::utils::{RoundRect, Size};

fn move_to(x: f32, y: f32) -> Command {
    Command::Move(Position::Absolute, (x, y).into())
}

fn line(dx: f32, dy: f32) -> Command {
    Command::Line(Position::Relative, (dx, dy).into())
}

fn h_line(dx: f32) -> Command {
    Command::HorizontalLine(Position::Relative, dx.into())
}

fn v_line(dy: f32) -> Command {
    Command::VerticalLine(Position::Relative, dy.into())
}

fn arc(rx: f32, ry: f32, dx: f32, dy: f32) -> Command {
    Command::EllipticalArc(Position::Relative, (rx, ry, 0., 0., 1., dx, dy).into())
}

fn arc_inset(rx: f32, ry: f32, dx: f32, dy: f32) -> Command {
    Command::EllipticalArc(Position::Relative, (rx, ry, 0., 0., 0., dx, dy).into())
}

fn corner(rx: f32, ry: f32) -> Command {
    arc(f32::abs(rx), f32::abs(ry), rx, ry)
}

fn corner_inset(rx: f32, ry: f32) -> Command {
    arc_inset(f32::abs(rx), f32::abs(ry), rx, ry)
}

// Calculate radius of arg
pub fn radius(curve: f32, distance: f32) -> f32 {
    (curve.powf(2.) + (distance.powf(2.) / 4.)) / (2. * curve)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeType {
    Line,
    CurveStretch,
    CurveLineCurve,
    InsetCurve,
}

#[derive(Debug, Clone)]
pub struct PathData(Data);

impl From<PathData> for Data {
    fn from(value: PathData) -> Self {
        value.0.add(Command::Close)
    }
}

impl PathData {
    fn add(self, command: Command) -> Self {
        Self(self.0.add(command))
    }

    fn append(&mut self, command: Command) {
        self.0.append(command);
    }

    pub fn new(rect: RoundRect) -> Self {
        let (x, y) = (rect.position() + Size::new(0., rect.ry)).into();
        let slf = Self(Data::new());
        slf.add(move_to(x, y))
    }

    // Simple commands
    pub fn line(self, dx: f32, dy: f32) -> Self {
        self.add(line(dx, dy))
    }

    pub fn h_line(self, dx: f32) -> Self {
        self.add(h_line(dx))
    }

    pub fn v_line(self, dy: f32) -> Self {
        self.add(v_line(dy))
    }

    pub fn arc(self, rx: f32, ry: f32, dx: f32, dy: f32) -> Self {
        self.add(arc(rx, ry, dx, dy))
    }

    pub fn arc_inset(self, rx: f32, ry: f32, dx: f32, dy: f32) -> Self {
        self.add(arc_inset(rx, ry, dx, dy))
    }

    pub fn corner(self, rx: f32, ry: f32) -> Self {
        self.add(corner(rx, ry))
    }

    pub fn corner_inset(self, rx: f32, ry: f32) -> Self {
        self.add(corner_inset(rx, ry))
    }

    // Corners
    pub fn corner_top_left(self, rect: RoundRect) -> Self {
        self.add(corner(rect.rx, -rect.ry))
    }

    pub fn corner_top_right(self, rect: RoundRect) -> Self {
        self.add(corner(rect.rx, rect.ry))
    }

    pub fn corner_bottom_right(self, rect: RoundRect) -> Self {
        self.add(corner(-rect.rx, rect.ry))
    }

    pub fn corner_bottom_left(self, rect: RoundRect) -> Self {
        self.add(corner(-rect.rx, -rect.ry))
    }

    // Edges
    pub fn edge_top(mut self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self {
        let rect_dx = rect.w - 2. * rect.rx;
        let size_dx = size.w - 1e3;
        let dx = rect_dx + size_dx;
        match typ {
            EdgeType::Line => {
                self.append(h_line(dx));
            }
            EdgeType::CurveLineCurve if size_dx > 0.01 => {
                let r = radius(curve, rect_dx);
                self.append(arc(r, r, rect_dx / 2., -curve));
                self.append(h_line(size_dx));
                self.append(arc(r, r, rect_dx / 2., curve));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let r = radius(curve, dx);
                self.append(arc(r, r, dx, 0.));
            }
            EdgeType::InsetCurve => {
                let r = radius(curve, dx);
                self.append(arc_inset(r, r, dx, 0.));
            }
        }
        self
    }

    pub fn edge_right(mut self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self {
        let rect_dy = rect.h - 2. * rect.ry;
        let size_dy = size.h - 1e3;
        let dy = rect_dy + size_dy;
        match typ {
            EdgeType::Line => {
                self.append(v_line(dy));
            }
            EdgeType::CurveLineCurve if size_dy > 0.01 => {
                let r = radius(curve, rect_dy);
                self.append(arc(r, r, curve, rect_dy / 2.));
                self.append(v_line(size_dy));
                self.append(arc(r, r, -curve, rect_dy / 2.));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let r = radius(curve, dy);
                self.append(arc(r, r, 0., dy));
            }
            EdgeType::InsetCurve => {
                let r = radius(curve, dy);
                self.append(arc_inset(r, r, 0., dy));
            }
        }
        self
    }

    pub fn edge_bottom(mut self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self {
        let rect_dx = rect.w - 2. * rect.rx;
        let size_dx = size.w - 1e3;
        let dx = rect_dx + size_dx;
        match typ {
            EdgeType::Line => {
                self.append(h_line(-dx));
            }
            EdgeType::CurveLineCurve if size_dx > 0.01 => {
                let r = radius(curve, rect_dx);
                self.append(arc(r, r, -rect_dx / 2., curve));
                self.append(h_line(-size_dx));
                self.append(arc(r, r, -rect_dx / 2., -curve));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let r = radius(curve, dx);
                self.append(arc(r, r, -dx, 0.));
            }
            EdgeType::InsetCurve => {
                let r = radius(curve, dx);
                self.append(arc_inset(r, r, -dx, 0.));
            }
        }
        self
    }

    pub fn edge_left(mut self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self {
        let rect_dy = rect.h - 2. * rect.ry;
        let size_dy = size.h - 1e3;
        let dy = rect_dy + size_dy;
        match typ {
            EdgeType::Line => {
                self.append(v_line(-dy));
            }
            EdgeType::CurveLineCurve if size_dy > 0.01 => {
                let r = radius(curve, rect_dy);
                self.append(arc(r, r, -curve, -rect_dy / 2.));
                self.append(v_line(-size_dy));
                self.append(arc(r, r, curve, -rect_dy / 2.));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let r = radius(curve, dy);
                self.append(arc(r, r, 0., -dy));
            }
            EdgeType::InsetCurve => {
                let r = radius(curve, dy);
                self.append(arc_inset(r, r, 0., -dy));
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, SQRT_2};

    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;
    use maplit::hashmap;

    use Command::*;
    use Position::*;

    use super::*;

    #[test]
    fn test_move_to() {
        assert_matches!(move_to(1., 1.), Move(Absolute, ..));
    }

    #[test]
    fn test_line() {
        assert_matches!(line(1., 1.), Line(Relative, ..));
    }

    #[test]
    fn test_h_line() {
        assert_matches!(h_line(1.), HorizontalLine(Relative, ..));
    }

    #[test]
    fn test_v_line() {
        assert_matches!(v_line(1.), VerticalLine(Relative, ..));
    }

    #[test]
    fn test_arc() {
        assert_matches!(arc(1., 1., 1., 1.), EllipticalArc(Relative, ..));
    }

    #[test]
    fn test_inset_arc() {
        assert_matches!(arc_inset(1., 1., 1., 1.), EllipticalArc(Relative, ..));
    }

    #[test]
    fn test_corner() {
        assert_matches!(corner(1., 1.), EllipticalArc(Relative, ..));
    }

    #[test]
    fn test_corner_inset() {
        assert_matches!(corner_inset(1., 1.), EllipticalArc(Relative, ..));
    }

    #[test]
    fn test_radius() {
        assert_approx_eq!(radius(1. - FRAC_1_SQRT_2, SQRT_2), 1.);
    }

    #[test]
    fn test_simples() {
        let rect = RoundRect::new(200., 100., 600., 600., 50., 50.);
        let path_data = PathData::new(rect)
            .line(1., 1.)
            .h_line(1.)
            .v_line(1.)
            .arc(1., 1., 1., 1.)
            .arc_inset(1., 1., 1., 1.)
            .corner(1., 1.);

        assert_eq!(path_data.0.len(), 7);
        assert_matches!(path_data.0[0], Move(Absolute, ..));
        assert_matches!(path_data.0[1], Line(Relative, ..));
        assert_matches!(path_data.0[2], HorizontalLine(Relative, ..));
        assert_matches!(path_data.0[3], VerticalLine(Relative, ..));
        assert_matches!(path_data.0[4], EllipticalArc(Relative, ..));
        assert_matches!(path_data.0[5], EllipticalArc(Relative, ..));
        assert_matches!(path_data.0[6], EllipticalArc(Relative, ..));
    }

    #[test]
    fn test_corners() {
        let rect = RoundRect::new(200., 100., 600., 600., 50., 50.);
        let path_data = PathData::new(rect);

        let corner_funcs: Vec<fn(PathData, RoundRect) -> PathData> = vec![
            PathData::corner_top_left,
            PathData::corner_top_right,
            PathData::corner_bottom_right,
            PathData::corner_bottom_left,
        ];

        for func in corner_funcs {
            let path_data = func(path_data.clone(), rect);
            assert_eq!(path_data.0.len(), 2);
            assert_matches!(path_data.0[1], EllipticalArc(Relative, ..));
        }
    }

    #[test]
    fn test_edges() {
        let rect = RoundRect::new(200., 100., 600., 600., 50., 50.);
        let size = Size::new(2e3, 2e3);
        let curve = 20.;
        let path_data = PathData::new(rect);

        let edge_funcs: Vec<fn(PathData, RoundRect, Size, EdgeType, f32) -> PathData> = vec![
            PathData::edge_top,
            PathData::edge_right,
            PathData::edge_bottom,
            PathData::edge_left,
        ];
        let edge_type_len = hashmap! {
            EdgeType::Line => 1,
            EdgeType::CurveStretch => 1,
            EdgeType::CurveLineCurve => 3,
            EdgeType::InsetCurve => 1,
        };

        for func in edge_funcs {
            for (&edge_type, &len) in &edge_type_len {
                let path_data = func(path_data.clone(), rect, size, edge_type, curve);

                assert_eq!(path_data.0.len(), len + 1);
            }
        }
    }
}
