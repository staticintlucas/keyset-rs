#![warn(
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    missing_debug_implementations,
    // missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    semicolon_in_expressions_from_macros,
    trivial_casts,
    trivial_numeric_casts,
    unaligned_references,
    unreachable_pub,
    unsafe_code,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
)]

// Remove unused dependency warning
#[cfg(test)]
use cargo_husky as _;

mod error;
mod interfaces;
mod layout;
mod types;

pub use error::{Error, Result};
pub use interfaces::kle;
pub use layout::{HomingType, Key, KeyType};
pub use types::{Color, Length, Point, Rect, Size};
