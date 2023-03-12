use crate::error::Result;

use ttf_parser::Face;

#[derive(Debug)]
pub struct Font {
    name: String,
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

        Ok(Self { name })
    }
}
