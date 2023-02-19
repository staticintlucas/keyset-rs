use svg::node::element::path::{Command, Data, Position};

use crate::utils::{RoundRect, Size};

fn move_to(x: f32, y: f32) -> Command {
    Command::Move(Position::Absolute, (x, y).into())
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

fn inset_arc(rx: f32, ry: f32, dx: f32, dy: f32) -> Command {
    Command::EllipticalArc(Position::Relative, (rx, ry, 0., 0., 0., dx, dy).into())
}

fn corner(rx: f32, ry: f32) -> Command {
    arc(f32::abs(rx), f32::abs(ry), rx, ry)
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

    pub fn edge_top(mut self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self {
        let rect_dx = rect.w - 2. * rect.rx;
        let size_dx = size.w - 1e3;
        let dx = rect_dx + size_dx;
        match typ {
            EdgeType::Line => {
                self.append(h_line(dx));
            }
            EdgeType::CurveLineCurve if size_dx > 0.01 => {
                let r = (curve.powf(2.) + (rect_dx.powf(2.) / 4.)) / (2. * curve);
                self.append(arc(r, r, rect_dx / 2., curve));
                self.append(h_line(size_dx));
                self.append(arc(r, r, rect_dx / 2., -curve));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let r = (curve.powf(2.) + (dx.powf(2.) / 4.)) / (2. * curve);
                self.append(arc(r, r, dx, 0.));
            }
            EdgeType::InsetCurve => {
                let r = (curve.powf(2.) + (dx.powf(2.) / 4.)) / (2. * curve);
                self.append(inset_arc(r, r, dx, 0.));
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
                let r = (curve.powf(2.) + (rect_dy.powf(2.) / 4.)) / (2. * curve);
                self.append(arc(r, r, rect_dy / 2., curve));
                self.append(v_line(size_dy));
                self.append(arc(r, r, rect_dy / 2., -curve));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let r = (curve.powf(2.) + (dy.powf(2.) / 4.)) / (2. * curve);
                self.append(arc(r, r, 0., dy));
            }
            EdgeType::InsetCurve => {
                let r = (curve.powf(2.) + (dy.powf(2.) / 4.)) / (2. * curve);
                self.append(inset_arc(r, r, 0., dy));
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
                let r = (curve.powf(2.) + (rect_dx.powf(2.) / 4.)) / (2. * curve);
                self.append(arc(r, r, -rect_dx / 2., -curve));
                self.append(h_line(-size_dx));
                self.append(arc(r, r, -rect_dx / 2., curve));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let r = (curve.powf(2.) + (dx.powf(2.) / 4.)) / (2. * curve);
                self.append(arc(r, r, -dx, 0.));
            }
            EdgeType::InsetCurve => {
                let r = (curve.powf(2.) + (dx.powf(2.) / 4.)) / (2. * curve);
                self.append(inset_arc(r, r, -dx, 0.));
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
                let r = (curve.powf(2.) + (rect_dy.powf(2.) / 4.)) / (2. * curve);
                self.append(arc(r, r, -rect_dy / 2., -curve));
                self.append(v_line(size_dy));
                self.append(arc(r, r, -rect_dy / 2., curve));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let r = (curve.powf(2.) + (dy.powf(2.) / 4.)) / (2. * curve);
                self.append(arc(r, r, 0., -dy));
            }
            EdgeType::InsetCurve => {
                let r = (curve.powf(2.) + (dy.powf(2.) / 4.)) / (2. * curve);
                self.append(inset_arc(r, r, 0., -dy));
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use maplit::hashmap;

    use super::*;

    #[test]
    fn test_move_to() {
        assert_matches!(move_to(1., 1.), Command::Move(Position::Absolute, ..));
    }

    #[test]
    fn test_h_line() {
        assert_matches!(h_line(1.), Command::HorizontalLine(Position::Relative, ..));
    }

    #[test]
    fn test_v_line() {
        assert_matches!(v_line(1.), Command::VerticalLine(Position::Relative, ..));
    }

    #[test]
    fn test_arc() {
        assert_matches!(
            arc(1., 1., 1., 1.),
            Command::EllipticalArc(Position::Relative, ..)
        );
    }

    #[test]
    fn test_inset_arc() {
        assert_matches!(
            inset_arc(1., 1., 1., 1.),
            Command::EllipticalArc(Position::Relative, ..)
        );
    }

    #[test]
    fn test_corner() {
        assert_matches!(
            corner(1., 1.),
            Command::EllipticalArc(Position::Relative, ..)
        );
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
