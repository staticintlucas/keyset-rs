use std::fmt;
use std::convert::TryInto;

use itertools::Itertools;

use crate::error::Result;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
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

        let chars = hex
            .strip_prefix('#')
            .unwrap_or(hex);

        let char_iter = chars.chars();

        let digits = match chars.len() {
            3 => {
                char_iter
                    .map(|x| {
                        u8::from_str_radix(&x.to_string(), 16)
                        .map(|v| v * 17)
                        .map_err(|_| InvalidColor { color: hex.to_string() }.into() )
                    })
                    .collect::<Result<Vec<u8>>>()},
            6 => {
                char_iter
                .chunks(2)
                .into_iter()
                .map(|i| i.collect::<String>())
                .map(|x| {
                    u8::from_str_radix(&x.to_string(), 16)
                    .map_err(|_| InvalidColor { color: hex.to_string() }.into())
                })
                .collect::<Result<Vec<u8>>>()
            },
            _ => Err(InvalidColor {
                color: hex.to_string(),
            }.into()),
        }?;

        // Note this unwrap will never panic since the iterator explicitly has the right number of
        // elements
        let [r, g, b]: [u8; 3] = digits[1..3].try_into().unwrap();

        Ok(Self {
            r: r,
            g: g,
            b: b,
        })
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn lighter(&self, val: u8) -> Self {
        Self {
            r: val + (255 - val) * self.r,
            g: val + (255 - val) * self.g,
            b: val + (255 - val) * self.b,
        }
    }

    pub fn darker(&self, val: u8) -> Self {
        Self {
            r: (255 - val) * self.r,
            g: (255 - val) * self.g,
            b: (255 - val) * self.b,
        }
    }

    pub fn highlight(&self, val: u8) -> Self {
        let c_max = [self.r, self.g, self.b]
            .iter()
            .copied()
            .fold(0, u8::max);
        let c_min = [self.r, self.g, self.b]
            .iter()
            .copied()
            .fold(0, u8::max);
        let lum = (c_max + c_min) / 2;

        if lum > 127 {
            self.lighter(val)
        } else {
            self.darker(val)
        }
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
