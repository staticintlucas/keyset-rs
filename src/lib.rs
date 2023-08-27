#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::cast_precision_loss, clippy::module_name_repetitions)]
// TODO add docs and don't allow these
#![allow(missing_docs, clippy::missing_errors_doc)]

mod drawing;
mod error;
mod font;
mod key;
pub mod kle;
mod profile;
mod utils;

pub use drawing::{Drawing, DrawingOptions};
pub use font::Font;
pub use key::Key;
pub use profile::Profile;
