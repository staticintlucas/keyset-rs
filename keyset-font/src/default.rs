use owned_ttf_parser::Face;

pub const FONT: &[u8] = include_bytes!(env!("DEFAULT_TTF"));

pub fn cap_height() -> f64 {
    f64::from(Face::parse(FONT, 0).unwrap().capital_height().unwrap())
}

pub fn x_height() -> f64 {
    f64::from(Face::parse(FONT, 0).unwrap().x_height().unwrap())
}

pub fn line_height() -> f64 {
    let face = Face::parse(FONT, 0).unwrap();
    f64::from(face.ascender()) + f64::from(face.descender()) + f64::from(face.line_gap())
}
