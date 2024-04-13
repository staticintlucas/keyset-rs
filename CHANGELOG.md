# Change Log

## [v0.3.2](https://github.com/staticintlucas/keyset-rs/releases/tag/v0.3.2)

### Fixes

* Remove erroneous `core::*` reexport

## [v0.3.1](https://github.com/staticintlucas/keyset-rs/releases/tag/v0.3.1)

### Fixes

* Fix documentation build failures in Docs.rs's environment

## [v0.3.0](https://github.com/staticintlucas/keyset-rs/releases/tag/v0.3.0)

### Changes

* Expose `Glyph` and `Kerning` types from `keyset-font`
* Refactor of `Font` and `Glyph` types, allow for lazy font parsing for improved performance
* Store references in `drawing::Options` instead of cloning data internally
* `Color` now uses `f32` internally rather than `u16`
* Added new `Legends` type to wrap around previous `[[Option<Legend>; 3]; 3]` arrays and allow for
  easier conversion to `[Option<Legend>; 9]`, etc
* Added JSON profile file support
* Improve documentation for large parts of the public API (still incomplete though)
* Major internal refactoring, by moving to a workspace-based structure and splitting this crate into
  multiple smaller subcrates

## [v0.2.1](https://github.com/staticintlucas/keyset-rs/releases/tag/v0.2.1)

### Changes

* Expose `Drawing` type in public API

## [v0.2.0](https://github.com/staticintlucas/keyset-rs/releases/tag/v0.2.0)

### New features

* New default font size which better matches KLE
* Automatic squishing of legends that don't otherwise fit (and associated warning)
* Add PNG, PDF, and AI output formats

### Changes

* Rework of `key` module
* Removed `layout` module and `Layout` struct in favour of directly using `Vec<Key>`
* Move all KLE import functionality to separate crate: [kle-serial]
* Remove all internal geometry primitives and instead rely on [kurbo]
* Major rewrite of `drawing` module to be generic over different output formats

[kle-serial]: https://crates.io/crates/kle-serial
[kurbo]: https://crates.io/crates/kurbo

### Fixes

* Fix bug in ISO enter positioning (or more generally keys with negative `x2` or `y2` in KLE)

## [v0.1.1](https://github.com/staticintlucas/keyset-rs/releases/tag/v0.1.1)

### Changes

* Simplify some KLE parsing logic.
* Reduce the number of 2D vector/rectangle types.

### Fixes

* Don't emit a ton of `NaN`s when a font doesn't set caps- or x-height.

## [v0.1.0](https://github.com/staticintlucas/keyset-rs/releases/tag/v0.1.0)

### New

* Initial release.
* Support parsing TTF and OTF font files.
* Support updated version of pykeyset's TOML profile format, with the following changes:
  * Uses direct mapping of KLE font sizes to diagram font sizes/margins.
  * Uses `homing.bump.diameter` instead of `homing.bump.radius`.
* Remove key top gradients as they weren't very pretty.
