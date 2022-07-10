#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
// TODO add docs and don't allow these
#![allow(
    missing_docs,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    dead_code
)]

mod error;
mod export;
mod kle;
mod layout;
mod profile;
mod utils;

pub use export::ToSvg;
pub use kle::FromKle;
pub use layout::Layout;
pub use profile::Profile;
