use std::fmt;
use std::result::Result as StdResult;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use palette::{Clamp, Hsl, LinSrgb, Srgb};
use palette::{FromColor, Shade};

use crate::error::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(LinSrgb);

impl Color {
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Srgb::from_components((r, g, b)).clamp().into_linear())
    }

    #[inline]
    pub fn from_hex(hex: &str) -> Result<Self> {
        Srgb::from_str(hex)
            .map(|c| Self(c.into_format().into_linear()))
            .map_err(|_| {
                InvalidColor {
                    color: hex.to_string(),
                }
                .into()
            })
    }

    #[inline]
    pub fn default_key() -> Self {
        Self(Srgb::from_components((0.8, 0.8, 0.8)).into_linear())
    }

    #[inline]
    pub fn default_legend() -> Self {
        Self(Srgb::from_components((0., 0., 0.)).into_linear())
    }

    #[inline]
    pub fn to_hex(self) -> String {
        format!("#{:02x}", Srgb::from_linear(self.0).into_format::<u8>())
    }

    #[inline]
    pub(crate) fn lighter(&self, val: f32) -> Self {
        Self(self.0.lighten(val))
    }

    #[inline]
    pub(crate) fn darker(&self, val: f32) -> Self {
        Self(self.0.darken(val))
    }

    #[inline]
    pub(crate) fn highlight(&self, val: f32) -> Self {
        if Hsl::from_color(self.0).lightness > 0.25 {
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

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_color_new() {
        let color0 = Srgb::from_linear(Color::new(0.2, 0.5, 0.8).0);
        assert_approx_eq!(color0.red, 0.2);
        assert_approx_eq!(color0.green, 0.5);
        assert_approx_eq!(color0.blue, 0.8);

        let color1 = Srgb::from_linear(Color::new(-0.8, 42.0, std::f32::NEG_INFINITY).0);
        assert_approx_eq!(color1.red, 0.);
        assert_approx_eq!(color1.green, 1.);
        assert_approx_eq!(color1.blue, 0.);
    }

    #[test]
    fn test_color_from_hex() {
        let color0 = Srgb::from_linear(Color::from_hex("#ccff33").unwrap().0);
        assert_approx_eq!(color0.red, 0.8);
        assert_approx_eq!(color0.green, 1.0);
        assert_approx_eq!(color0.blue, 0.2);

        let color1 = Srgb::from_linear(Color::from_hex("069").unwrap().0);
        assert_approx_eq!(color1.red, 0.0);
        assert_approx_eq!(color1.green, 0.4);
        assert_approx_eq!(color1.blue, 0.6);
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
        let expected = Color::from_hex("#cccccc").unwrap();
        assert_approx_eq!(Color::default_key().0.red, expected.0.red);
        assert_approx_eq!(Color::default_key().0.green, expected.0.green);
        assert_approx_eq!(Color::default_key().0.blue, expected.0.blue);
    }

    #[test]
    fn test_color_default_legend() {
        let expected = Color::from_hex("#000000").unwrap();
        assert_approx_eq!(Color::default_legend().0.red, expected.0.red);
        assert_approx_eq!(Color::default_legend().0.green, expected.0.green);
        assert_approx_eq!(Color::default_legend().0.blue, expected.0.blue);
    }

    #[test]
    fn test_color_to_hex() {
        let hex = Color::new(0.2, 0.4, 0.6).to_hex();
        assert_eq!(hex, "#336699");

        let hex = Color::new(0.8, 1.0, 0.0).to_hex();
        assert_eq!(hex, "#ccff00");
    }

    #[test]
    fn test_color_lighter() {
        let color = Color::new(0.4, 0.2, 0.6).lighter(0.5);
        assert_approx_eq!(color.0.red, 0.566434);
        assert_approx_eq!(color.0.green, 0.516552);
        assert_approx_eq!(color.0.blue, 0.659273);
    }

    #[test]
    fn test_color_darker() {
        let color = Srgb::from_linear(Color::new(0.4, 0.2, 0.6).darker(0.5).0);
        assert_approx_eq!(color.red, 0.285865);
        assert_approx_eq!(color.green, 0.136034);
        assert_approx_eq!(color.blue, 0.435695);
    }

    #[test]
    fn test_color_highlight() {
        let color1 = Srgb::from_linear(Color::new(0.4, 0.2, 0.6).highlight(0.5).0);
        assert_approx_eq!(color1.red, 0.777526);
        assert_approx_eq!(color1.green, 0.746155);
        assert_approx_eq!(color1.blue, 0.831876);

        let color2 = Srgb::from_linear(Color::new(0.8, 0.4, 0.6).highlight(0.5).0);
        assert_approx_eq!(color2.red, 0.585526);
        assert_approx_eq!(color2.green, 0.285865);
        assert_approx_eq!(color2.blue, 0.435696);
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
