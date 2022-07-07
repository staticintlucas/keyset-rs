#![allow(unknown_lints)]
#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::must_use_candidate)]
// TODO add docs and don't allow these
#![allow(
    missing_docs,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    dead_code
)]

mod drawing;
mod error;
mod kle;
mod layout;
mod profile;
mod utils;
