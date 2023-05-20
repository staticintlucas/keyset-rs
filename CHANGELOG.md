# Change Log

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
