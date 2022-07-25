use std::fmt;
use std::result::Result as StdResult;

use itertools::Itertools;
use rgb::RGB16;
use serde::{Deserialize, Deserializer};

use crate::error::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(RGB16);

impl Color {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(
            [r, g, b]
                .into_iter()
                .map(u16::from)
                .map(|c| c << 8 | c)
                .collect(),
        )
    }

    #[inline]
    pub fn new_rgb16(r: u16, g: u16, b: u16) -> Self {
        Self(RGB16::new(r, g, b))
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        #[inline]
        fn parse_hex(hex: &[u8]) -> Option<[u32; 3]> {
            match hex.len() {
                3 => Some([
                    (hex[0] as char).to_digit(16).map(|c| c << 4 | c)?,
                    (hex[1] as char).to_digit(16).map(|c| c << 4 | c)?,
                    (hex[2] as char).to_digit(16).map(|c| c << 4 | c)?,
                ]),
                6 => Some([
                    (hex[0] as char).to_digit(16)? << 4 | (hex[1] as char).to_digit(16)?,
                    (hex[2] as char).to_digit(16)? << 4 | (hex[3] as char).to_digit(16)?,
                    (hex[4] as char).to_digit(16)? << 4 | (hex[5] as char).to_digit(16)?,
                ]),
                _ => None,
            }
        }

        let digits = hex.strip_prefix('#').unwrap_or(hex).as_bytes();
        let rgb = parse_hex(digits).ok_or(InvalidColor { color: hex.into() })?;

        #[allow(clippy::cast_possible_truncation)]
        // Can't truncate here since the values are in range 0..=255
        Ok(Color(
            rgb.into_iter()
                .map(|c| (c as u16) << 8 | (c as u16))
                .collect(),
        ))
    }

    #[inline]
    pub fn to_rgb(self) -> (u8, u8, u8) {
        self.0
            .iter()
            .map(|c| (c >> 8) as u8)
            .collect_tuple()
            .unwrap()
    }

    #[inline]
    pub fn to_rgb16(self) -> (u16, u16, u16) {
        self.0.into()
    }

    #[inline]
    pub fn default_key() -> Self {
        Self([0xCCCC; 3].into())
    }

    #[inline]
    pub fn default_legend() -> Self {
        Self([0x0000; 3].into())
    }

    #[inline]
    pub fn to_hex(self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    #[inline]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    // TODO implement this with val: u16
    pub(crate) fn lighter(self, val: f32) -> Self {
        let (r, g, b) = self.0.into();
        Self(RGB16::new(
            r + (f32::from(u16::MAX - r) * val) as u16,
            g + (f32::from(u16::MAX - g) * val) as u16,
            b + (f32::from(u16::MAX - b) * val) as u16,
        ))
    }

    #[inline]
    // TODO implement this with val: u16
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub(crate) fn darker(self, val: f32) -> Self {
        let (r, g, b) = self.0.into();
        Self(RGB16::new(
            (f32::from(r) * (1.0 - val)) as u16,
            (f32::from(g) * (1.0 - val)) as u16,
            (f32::from(b) * (1.0 - val)) as u16,
        ))
    }

    #[inline]
    // TODO implement this with val: u16
    pub(crate) fn highlight(self, val: f32) -> Self {
        let c_max = self.0.iter().max().unwrap();
        let c_min = self.0.iter().min().unwrap();
        let lum_x2 = u32::from(c_max) + u32::from(c_min);

        // If (lum * 2) > (u16::MAX / 2 * 2)
        if lum_x2 > u32::from(u16::MAX) {
            self.darker(val)
        } else {
            self.lighter(val)
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> StdResult<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected};

        let hex = String::deserialize(deserializer)?;
        Color::from_hex(&hex)
            .map_err(|_| D::Error::invalid_value(Unexpected::Str(&hex), &"a hex color code"))
    }
}

#[derive(Debug)]
pub(crate) struct InvalidColor {
    color: String,
}

impl fmt::Display for InvalidColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid hex code {}", self.color)
    }
}

impl std::error::Error for InvalidColor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(51, 127, 204).to_rgb();
        assert_eq!(color.0, 0x33);
        assert_eq!(color.1, 0x7f);
        assert_eq!(color.2, 0xcc);
    }

    #[test]
    fn test_color_new_rgb16() {
        let color = Color::new_rgb16(13107, 32767, 52428).to_rgb16();
        assert_eq!(color.0, 0x3333);
        assert_eq!(color.1, 0x7fff);
        assert_eq!(color.2, 0xcccc);
    }

    #[test]
    fn test_color_from_hex() {
        let color0 = Color::from_hex("#ccff33").unwrap().to_rgb();
        assert_eq!(color0.0, 204);
        assert_eq!(color0.1, 255);
        assert_eq!(color0.2, 51);

        let color1 = Color::from_hex("069").unwrap().to_rgb();
        assert_eq!(color1.0, 0);
        assert_eq!(color1.1, 102);
        assert_eq!(color1.2, 153);
    }

    #[test]
    fn test_invalid_color_from_hex() {
        let invalids = ["", "hex", "lööps", "#f000", "#eeffgg"];
        for invalid in &invalids {
            let result = Color::from_hex(invalid);
            assert!(result.is_err());
            assert_eq!(
                format!("{}", result.unwrap_err()),
                format!("error parsing color: invalid hex code {}", invalid),
            )
        }
    }

    #[test]
    fn test_color_default_key() {
        let expected = Color::from_hex("#cccccc").unwrap().to_rgb();
        let default_key = Color::default_key().to_rgb();
        assert_eq!(default_key.0, expected.0);
        assert_eq!(default_key.1, expected.1);
        assert_eq!(default_key.2, expected.2);
    }

    #[test]
    fn test_color_default_legend() {
        let expected = Color::from_hex("#000000").unwrap().to_rgb();
        let default_legend = Color::default_legend().to_rgb();
        assert_eq!(default_legend.0, expected.0);
        assert_eq!(default_legend.1, expected.1);
        assert_eq!(default_legend.2, expected.2);
    }

    #[test]
    fn test_color_to_hex() {
        let hex = Color::new(51, 102, 153).to_hex();
        assert_eq!(hex, "#336699");

        let hex = Color::new(204, 255, 0).to_hex();
        assert_eq!(hex, "#ccff00");
    }

    #[test]
    fn test_color_lighter() {
        let color = Color::new(102, 51, 153).lighter(0.5).to_rgb();
        assert_eq!(color.0, 179);
        assert_eq!(color.1, 153);
        assert_eq!(color.2, 204);
    }

    #[test]
    fn test_color_darker() {
        let color = Color::new(102, 51, 153).darker(0.5).to_rgb();
        assert_eq!(color.0, 51);
        assert_eq!(color.1, 25);
        assert_eq!(color.2, 76);
    }

    #[test]
    fn test_color_highlight() {
        let color1 = Color::new(102, 51, 153).highlight(0.5).to_rgb();
        assert_eq!(color1.0, 179);
        assert_eq!(color1.1, 153);
        assert_eq!(color1.2, 204);

        let color2 = Color::new(204, 102, 153).highlight(0.5).to_rgb();
        assert_eq!(color2.0, 102);
        assert_eq!(color2.1, 51);
        assert_eq!(color2.2, 76);
    }

    #[test]
    fn test_deserialize_color() {
        use serde_json::Error;

        let color = Color::deserialize(&mut serde_json::Deserializer::from_str(r##""#ff0000""##));
        assert!(matches!(color, Ok(Color { .. })));

        let color = Color::deserialize(&mut serde_json::Deserializer::from_str(r#""invalid""#));
        assert!(matches!(color, Err(Error { .. })));
    }
}
