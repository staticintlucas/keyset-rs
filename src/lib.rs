#![allow(unknown_lints)]
#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
// TODO add docs and don't allow these
#![allow(missing_docs, clippy::missing_panics_doc, clippy::missing_errors_doc)]

mod error;
mod interfaces;
mod layout;
mod types;

pub use error::{Error, Result};
pub use interfaces::kle;
pub use layout::{HomingType, Key, KeyType};
pub use types::{Color, Length, Point, Rect, Size};
