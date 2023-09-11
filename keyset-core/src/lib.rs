#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
#![allow(
    clippy::cast_precision_loss, // We don't care about precision *that* much
    clippy::module_name_repetitions, // This lint is dumb
    clippy::redundant_pub_crate, // https://github.com/rust-lang/rust-clippy/issues/7862
    clippy::suboptimal_flops // Optimiser is pretty good, and mul_add is pretty ugly
)]
// TODO add docs and don't allow these
#![allow(missing_docs, clippy::missing_errors_doc)]

mod drawing;
mod error;
mod font;
mod utils;

pub use drawing::{Drawing, DrawingOptions};
pub use error::{Error, Result};
pub use font::{Font, Glyph, Kerning};
pub use profile::Profile;
