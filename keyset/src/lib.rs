//!
//!
//! ## Example
//!
//! ```
//! use keyset::{Font, kle, Profile};
//!
//! // JSON output from http://www.keyboard-layout-editor.com/
//! let kle = r#"[
//!     [{"f": 4}, "¬\n`", "!\n1", "\"\n2", "£\n3", "$\n4", "%\n5", "^\n6", "&\n7", "*\n8", "(\n9", ")\n0", "_\n-", "+\n=", {"w": 2, "f": 3, "a": 6}, "Backspace"],
//!     [{"w": 1.5}, "Tab", {"f": 5, "a": 4}, "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", {"f": 4}, "{\n[", "}\n]", {"x": 0.25, "w": 1.25, "h": 2, "w2": 1.5, "h2": 1, "x2": -0.25, "f": 3, "a": 6}, "Enter"],
//!     [{"w": 1.25, "w2": 1.75, "l": true}, "Caps Lock", {"f": 5, "a": 4}, "A", "S", "D", {"n": true}, "F", "G", "H", {"n": true}, "J", "K", "L", {"f": 4}, ":\n;", "@\n'", "~\n#"],
//!     [{"w": 1.25, "f": 3, "a": 6}, "Shift", {"f": 4, "a": 4}, "|\n\\", {"f": 5}, "Z",  "X", "C", "V", "B", "N", "M", {"f": 4}, "<\n,", ">\n.", "?\n/", {"w": 2.75, "f": 3, "a": 6}, "Shift"],
//!     [{"w": 1.5}, "Ctrl", "Win", {"w": 1.5}, "Alt", {"p": "space", "w": 7}, "", {"p": "", "w": 1.5}, "AltGr", "Win", {"w": 1.5}, "Ctrl"]
//! ]"#;
//!
//! // Approximation of Cherry profile
//! let profile = r#"{
//!     "type": "cylindrical",
//!     "depth": 0.5,
//!     "bottom": { "width": 18.29, "height": 18.29, "radius": 0.38 },
//!     "top": { "width": 11.81, "height": 13.91, "radius": 1.52, "y-offset": -1.62 },
//!     "legend": {
//!         "5": { "size": 4.84, "width": 9.45, "height": 11.54, "y-offset": 0 },
//!         "4": { "size": 3.18, "width": 9.53, "height": 9.56, "y-offset": 0.40 },
//!         "3": { "size": 2.28, "width": 9.45, "height": 11.30, "y-offset": -0.12 }
//!     },
//!     "homing": {
//!         "default": "scoop",
//!         "scoop": { "depth": 1.5 },
//!         "bar": { "width": 3.85, "height": 0.4, "y-offset": 5.05 },
//!         "bump": { "diameter": 0.4, "y-offset": -0.2 }
//!     }
//! }"#;
//!
//! // Use `fontdb` to load system fonts
//! let font = {
//!     use fontdb::*;
//!     let mut db = Database::new();
//!     db.load_system_fonts();
//!     let query = Query {
//!         families: &[Family::Name("Helvetica"), Family::Name("Arial"), Family::Name("Liberation Sans"), Family::SansSerif],
//!         ..Default::default()
//!     };
//!     let id = db.query(&query).unwrap();
//!     match db.face_source(id).unwrap().0 {
//!         Source::File(path) => std::fs::read(path).unwrap(),
//!         Source::Binary(bytes) | Source::SharedFile(_, bytes) => (*bytes).as_ref().to_vec(),
//!     }
//! };
//!
//! // Use `keyset` to load layout, profile and font
//! let keys = kle::from_json(kle).unwrap();
//! let profile = Profile::from_json(profile).unwrap();
//! let font = Font::from_ttf(font).unwrap();
//!
//! // Set drawing options
//! let options = drawing::Options::new().profile(&profile).font(&font);
//!
//! // Create drawing
//! let drawing = options.draw(&keys);
//!
//! // Save output
//! let path = std::env::current_dir().unwrap();
//! std::fs::write(path.join("output.svg"), drawing.to_svg()).unwrap();
//! std::fs::write(path.join("output.png"), drawing.to_png(96.0)).unwrap();
//! std::fs::write(path.join("output.pdf"), drawing.to_pdf()).unwrap();
//!
//! ```

pub use color::Color;
pub use drawing::{self, Drawing};
pub use font::{self, Font, Glyph};
pub use key::{self, kle, Key};
pub use profile::{self, Profile};
