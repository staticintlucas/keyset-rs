//! A library for creating pretty keyset layout diagrams using correct fonts and icons
//!
//! This project is primarily intended to serve as the backend for [pykeyset], but can also be used
//! directly in Rust.
//!
//! [pykeyset]: https://github.com/staticintlucas/pykeyset
//!
//! # Example
//!
//! ```
//! use keyset::{Drawing, Font, kle, Profile};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #
//! // JSON output from http://www.keyboard-layout-editor.com/
//! let kle = r#"[
//!     [{"f": 4}, "¬\n`", "!\n1", "\"\n2", "£\n3", "$\n4", "%\n5", "^\n6", "&\n7", "*\n8", "(\n9", ")\n0", "_\n-", "+\n=", {"w": 2, "f": 3, "a": 6}, "Backspace"],
//!     [{"w": 1.5}, "Tab", {"f": 5, "a": 4}, "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", {"f": 4}, "{\n[", "}\n]", {"x": 0.25, "w": 1.25, "h": 2, "w2": 1.5, "h2": 1, "x2": -0.25, "f": 3, "a": 6}, "Enter"],
//!     [{"w": 1.25, "w2": 1.75, "l": true}, "Caps<br>Lock", {"x": 0.5, "f": 5, "a": 4}, "A", "S", "D", {"n": true}, "F", "G", "H", {"n": true}, "J", "K", "L", {"f": 4}, ":\n;", "@\n'", "~\n#"],
//!     [{"w": 1.25, "f": 3, "a": 6}, "Shift", {"f": 4, "a": 4}, "|\n\\", {"f": 5}, "Z",  "X", "C", "V", "B", "N", "M", {"f": 4}, "<\n,", ">\n.", "?\n/", {"w": 2.75, "f": 3, "a": 6}, "Shift"],
//!     [{"w": 1.5}, "Ctrl", "Win", {"w": 1.5}, "Alt", {"p": "space", "w": 7}, "", {"p": "", "w": 1.5}, "AltGr", "Win", {"w": 1.5}, "Ctrl"]
//! ]"#;
//!
//! // Approximation of Cherry profile
//! let profile = r#"
//!     type = "cylindrical"
//!     depth = 0.5
//!
//!     [bottom]
//!     width = 18.29
//!     height = 18.29
//!     radius = 0.38
//!
//!     [top]
//!     width = 11.81
//!     height = 13.91
//!     radius = 1.52
//!     y-offset = -1.62
//!
//!     [legend.alpha]
//!     size = 4.84
//!     margin = { top-bottom = 1.185, left-right = 1.18 }
//!
//!     [legend.symbol]
//!     size = 3.18
//!     margin = { top = 2.575, bottom = 1.775, left-right = 1.14 }
//!
//!     [legend.modifier]
//!     size = 2.28
//!     margin = { top = 1.185, bottom = 1.425, left-right = 1.18 }
//!
//!     [homing]
//!     default = "scoop"
//!     scoop = { depth = 1.5 }
//!     bar = { width = 3.85, height = 0.4, y-offset = 5.05 }
//!     bump = { diameter = 0.4, y-offset = -0.2 }
//! "#;
//!
//! // Use `keyset` to load layout, profile and font
//! let keys = kle::from_json(kle)?;
//! let profile = Profile::from_toml(profile)?;
//! let font = Font::default();
//! // Or load an actual font with Font::from_ttf(std::fs::read("font.ttf")?)?
//!
//! // Create template (tells keyset-rs how to draw the keys)
//! let template = drawing::Template {
//!     profile,
//!     font,
//!     ..Default::default()
//! };
//!
//! // Create drawing
//! let (drawing, warnings) = template.draw(&keys)?;
//!
//! // Optionally handle warnings (if any)
//! assert!(warnings.is_empty());
//!
//! // Save output
//! let path = std::env::current_dir()?;
//! std::fs::write(path.join("output.svg"), drawing.to_svg())?;
//! std::fs::write(path.join("output.png"), drawing.to_png(96.0)?)?;
//! std::fs::write(path.join("output.pdf"), drawing.to_pdf())?;
//! #
//! # Ok(())
//! # }
//! ```

pub use color::Color;
pub use drawing::{self, Drawing};
pub use font::{self, Font};
pub use key::{self, kle, Key};
pub use profile::{self, Profile};
