#![warn(
    missing_docs,
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
    missing_docs, // TODO
    clippy::multiple_crate_versions // TODO syn 1.0 through keyset-font > ourboros > proc-macro-error (which is unmaintained)
)]

pub use color;
pub use drawing;
pub use font;
pub use key;
pub use profile;

pub use core::*;
