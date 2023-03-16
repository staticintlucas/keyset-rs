use crate::error::Result;

use ttf_parser::Face;

#[derive(Debug)]
pub struct Font {
    name: String,
    em_size: f32,
    cap_height: f32,
    x_height: f32,
    ascent: f32,
    descent: f32,
    line_height: f32,
    slope: f32,
}

impl Font {
    pub fn from_ttf(data: &[u8]) -> Result<Self> {
        let face = Face::parse(data, 0)?;

        let name = face
            .names()
            .into_iter()
            .filter(|n| n.name_id == 1) // index 1 = font family name
            .find_map(|n| n.to_string())
            .unwrap_or("unknown".to_string());

        let em_size = f32::from(face.units_per_em());
        let cap_height = f32::from(face.capital_height().unwrap_or(0)); // TODO calculate default
        let x_height = f32::from(face.x_height().unwrap_or(0)); // TODO calculate default
        let ascent = f32::from(face.ascender());
        let descent = f32::from(-face.descender());
        let line_height = ascent + descent + f32::from(face.line_gap());
        let slope = face.italic_angle().unwrap_or(0.);

        Ok(Self {
            name,
            em_size,
            cap_height,
            x_height,
            ascent,
            descent,
            line_height,
            slope,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_from_ttf() {
        let data = std::fs::read("tests/fonts/demo.ttf").unwrap();
        let font = Font::from_ttf(&data).unwrap();

        assert_eq!(font.name, "unknown"); // TODO demo font has no name table
        assert_approx_eq!(font.em_size, 1e3);
        assert_approx_eq!(font.cap_height, 0.); // TODO demo font has no caps height
        assert_approx_eq!(font.x_height, 0.); // TODO demo font has no x-height
        assert_approx_eq!(font.line_height, 1424.);
        assert_approx_eq!(font.slope, 0.);
    }
}
