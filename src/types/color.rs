use std::convert::TryInto;
use std::fmt;
use std::result::Result as StdResult;

use itertools::{Chunk, Itertools};
use serde::{Deserialize, Deserializer};

use crate::error::Result;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Component(f32);

impl Component {
    #[inline]
    fn new(val: f32) -> Self {
        Component(val.max(0.).min(1.))
    }
}

impl std::ops::Add<Self> for Component {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Component) -> Self {
        // We call new here for bounds checks
        Component::new(self.0 + rhs.0)
    }
}

impl std::ops::Sub<Self> for Component {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        // We call new here for bounds checks
        Component::new(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f32> for Component {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        // We call new here for bounds checks
        Component::new(self.0 * rhs)
    }
}

impl std::ops::Div<f32> for Component {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        // We call new here since we still need bounds checks here, if other < 1 we could overflow.
        Component::new(self.0 / rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: Component,
    g: Component,
    b: Component,
}

impl Color {
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: Component::new(r),
            g: Component::new(g),
            b: Component::new(b),
        }
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        // Return quickly for any non-ASCII characters since they are invalid anyway and it makes
        // the rest of the implementation a lot easier
        if !hex.is_ascii() {
            return Err(InvalidColor {
                color: hex.to_string(),
            }
            .into());
        }

        let chars = if let Some('#') = hex.chars().next() {
            &hex[1..]
        } else {
            hex
        };

        let char_iter = if let Some('#') = hex.chars().next() {
            hex[1..].chars() // Note: it's safe to slice by index here since we know '#' is 1b
        } else {
            hex.chars()
        };

        let digits = match chars.len() {
            3 => char_iter
                .map(|x| {
                    u8::from_str_radix(&x.to_string(), 16)
                        .map(|v| v * 17)
                        .map_err(|_| {
                            InvalidColor {
                                color: hex.to_string(),
                            }
                            .into()
                        })
                })
                .collect::<Result<Vec<u8>>>(),
            6 => char_iter
                .chunks(2)
                .into_iter()
                .map(Chunk::collect::<String>)
                .map(|x| {
                    u8::from_str_radix(&x, 16).map_err(|_| {
                        InvalidColor {
                            color: hex.to_string(),
                        }
                        .into()
                    })
                })
                .collect::<Result<Vec<u8>>>(),
            _ => Err(InvalidColor {
                color: hex.to_string(),
            }
            .into()),
        }?;

        // Note this unwrap will never panic since the iterator explicitly has the right number of
        // elements
        let [r, g, b]: [u8; 3] = digits[0..3].try_into().unwrap();

        Ok(Self {
            r: Component::new(f32::from(r) / f32::from(std::u8::MAX)),
            g: Component::new(f32::from(g) / f32::from(std::u8::MAX)),
            b: Component::new(f32::from(b) / f32::from(std::u8::MAX)),
        })
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            (self.r.0 * 255.).round() as u8,
            (self.g.0 * 255.).round() as u8,
            (self.b.0 * 255.).round() as u8,
        )
    }

    #[inline]
    pub(crate) fn lighter(&self, val: f32) -> Self {
        let val = val.max(0.).min(1.);
        Self {
            r: (Component::new(1.) * val) + (self.r * (1. - val)),
            g: (Component::new(1.) * val) + (self.g * (1. - val)),
            b: (Component::new(1.) * val) + (self.b * (1. - val)),
        }
    }

    #[inline]
    pub(crate) fn darker(&self, val: f32) -> Self {
        let val = val.max(0.).min(1.);
        Self {
            r: (Component::new(0.) * val) + (self.r * (1. - val)),
            g: (Component::new(0.) * val) + (self.g * (1. - val)),
            b: (Component::new(0.) * val) + (self.b * (1. - val)),
        }
    }

    #[inline]
    pub(crate) fn highlight(&self, val: f32) -> Self {
        let c_max = [self.r.0, self.g.0, self.b.0]
            .iter()
            .copied()
            .fold(0., f32::max);
        let c_min = [self.r.0, self.g.0, self.b.0]
            .iter()
            .copied()
            .fold(std::f32::INFINITY, f32::min);
        let lum = (c_max + c_min) / 2.;

        if lum > 0.5 {
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
    fn test_component_new() {
        let component0 = Component::new(0.4);
        assert_eq!(component0.0, 0.4);
        let component1 = Component::new(-256.0);
        assert_eq!(component1.0, 0.);
        let component2 = Component::new(std::f32::INFINITY);
        assert_eq!(component2.0, 1.);
    }

    #[test]
    fn test_component_add() {
        let component0 = Component::new(0.4);
        let component1 = Component::new(0.2);
        assert_approx_eq!((component0 + component1).0, 0.6);

        let component2 = Component::new(0.8);
        assert_eq!((component0 + component2).0, 1.);
    }

    #[test]
    fn test_component_sub() {
        let component0 = Component::new(0.4);
        let component1 = Component::new(0.2);
        assert_approx_eq!((component0 - component1).0, 0.2);

        let component2 = Component::new(0.8);
        assert_eq!((component0 - component2).0, 0.);
    }

    #[test]
    fn test_component_mul() {
        let component0 = Component::new(0.8);
        assert_approx_eq!((component0 * 0.5).0, 0.4);
        assert_eq!((component0 * 1.5).0, 1.);
    }

    #[test]
    fn test_component_div() {
        let component0 = Component::new(0.8);
        assert_approx_eq!((component0 / 2.).0, 0.4);
        assert_eq!((component0 / 0.5).0, 1.);
    }

    #[test]
    fn test_color_new() {
        let color0 = Color::new(0.2, 0.5, 0.8);
        assert_eq!(color0.r.0, 0.2);
        assert_eq!(color0.g.0, 0.5);
        assert_eq!(color0.b.0, 0.8);

        let color1 = Color::new(-0.8, 42.0, std::f32::NEG_INFINITY);
        assert_eq!(color1.r.0, 0.);
        assert_eq!(color1.g.0, 1.);
        assert_eq!(color1.b.0, 0.);
    }

    #[test]
    fn test_color_from_hex() {
        let color0 = Color::from_hex("#ccff33").unwrap();
        assert_approx_eq!(color0.r.0, 0.8);
        assert_approx_eq!(color0.g.0, 1.0);
        assert_approx_eq!(color0.b.0, 0.2);

        let color1 = Color::from_hex("069").unwrap();
        assert_approx_eq!(color1.r.0, 0.0);
        assert_approx_eq!(color1.g.0, 0.4);
        assert_approx_eq!(color1.b.0, 0.6);
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
    fn test_color_to_hex() {
        let hex = Color::new(0.2, 0.4, 0.6).to_hex();
        assert_eq!(hex, "#336699");

        let hex = Color::new(0.8, 1.0, 0.0).to_hex();
        assert_eq!(hex, "#ccff00");
    }

    #[test]
    fn test_color_lighter() {
        let color = Color::new(0.4, 0.2, 0.6).lighter(0.5);
        assert_approx_eq!(color.r.0, 0.7);
        assert_approx_eq!(color.g.0, 0.6);
        assert_approx_eq!(color.b.0, 0.8);
    }

    #[test]
    fn test_color_darker() {
        let color = Color::new(0.4, 0.2, 0.6).darker(0.5);
        assert_approx_eq!(color.r.0, 0.2);
        assert_approx_eq!(color.g.0, 0.1);
        assert_approx_eq!(color.b.0, 0.3);
    }

    #[test]
    fn test_color_highlight() {
        let color1 = Color::new(0.4, 0.2, 0.6).highlight(0.5);
        assert_approx_eq!(color1.r.0, 0.7);
        assert_approx_eq!(color1.g.0, 0.6);
        assert_approx_eq!(color1.b.0, 0.8);

        let color2 = Color::new(0.8, 0.4, 0.6).highlight(0.5);
        assert_approx_eq!(color2.r.0, 0.4);
        assert_approx_eq!(color2.g.0, 0.2);
        assert_approx_eq!(color2.b.0, 0.3);
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
