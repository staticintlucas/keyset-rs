# keyset.rs &emsp; [![Test Status]][actions]&thinsp;[![Test Coverage]][codecov]&thinsp;[![Crate Version]][crates]&thinsp;[![Rust Version]][crates]

[test status]: https://img.shields.io/github/actions/workflow/status/staticintlucas/keyset-rs/test.yml?branch=main&label=tests&style=flat-square
[test coverage]: https://img.shields.io/codecov/c/gh/staticintlucas/keyset-rs?style=flat-square
[crate version]: https://img.shields.io/crates/v/keyset?style=flat-square
[rust version]: https://img.shields.io/crates/msrv/keyset?style=flat-square

[actions]: https://github.com/staticintlucas/keyset-rs/actions?query=branch%3Amain
[codecov]: https://app.codecov.io/github/staticintlucas/keyset-rs
[crates]: https://crates.io/crates/keyset

<!-- cargo-rdme start -->

A library for creating pretty keyset layout diagrams using correct fonts and icons

This project is primarily intended to serve as the backend for [pykeyset], but can also be used
directly in Rust.

[pykeyset]: https://github.com/staticintlucas/pykeyset

## Example

```rust
use keyset::{Drawing, Font, kle, Profile};

// JSON output from http://www.keyboard-layout-editor.com/
let kle = r#"[
    [{"f": 4}, "¬\n`", "!\n1", "\"\n2", "£\n3", "$\n4", "%\n5", "^\n6", "&\n7", "*\n8", "(\n9", ")\n0", "_\n-", "+\n=", {"w": 2, "f": 3, "a": 6}, "Backspace"],
    [{"w": 1.5}, "Tab", {"f": 5, "a": 4}, "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", {"f": 4}, "{\n[", "}\n]", {"x": 0.25, "w": 1.25, "h": 2, "w2": 1.5, "h2": 1, "x2": -0.25, "f": 3, "a": 6}, "Enter"],
    [{"w": 1.25, "w2": 1.75, "l": true}, "Caps<br>Lock", {"f": 5, "a": 4}, "A", "S", "D", {"n": true}, "F", "G", "H", {"n": true}, "J", "K", "L", {"f": 4}, ":\n;", "@\n'", "~\n#"],
    [{"w": 1.25, "f": 3, "a": 6}, "Shift", {"f": 4, "a": 4}, "|\n\\", {"f": 5}, "Z",  "X", "C", "V", "B", "N", "M", {"f": 4}, "<\n,", ">\n.", "?\n/", {"w": 2.75, "f": 3, "a": 6}, "Shift"],
    [{"w": 1.5}, "Ctrl", "Win", {"w": 1.5}, "Alt", {"p": "space", "w": 7}, "", {"p": "", "w": 1.5}, "AltGr", "Win", {"w": 1.5}, "Ctrl"]
]"#;

// Approximation of Cherry profile
let profile = r#"{
    "type": "cylindrical",
    "depth": 0.5,
    "bottom": { "width": 18.29, "height": 18.29, "radius": 0.38 },
    "top": { "width": 11.81, "height": 13.91, "radius": 1.52, "y-offset": -1.62 },
    "legend": {
        "5": { "size": 4.84, "width": 9.45, "height": 11.54, "y-offset": 0 },
        "4": { "size": 3.18, "width": 9.53, "height": 9.56, "y-offset": 0.40 },
        "3": { "size": 2.28, "width": 9.45, "height": 11.30, "y-offset": -0.12 }
    },
    "homing": {
        "default": "scoop",
        "scoop": { "depth": 1.5 },
        "bar": { "width": 3.85, "height": 0.4, "y-offset": 5.05 },
        "bump": { "diameter": 0.4, "y-offset": -0.2 }
    }
}"#;

// Use `keyset` to load layout, profile and font
let keys = kle::from_json(kle)?;
let profile = Profile::from_json(profile)?;
let font = Font::default();
// Or load an actual font with Font::from_ttf(std::fs::read("font.ttf")?)?

// Create template (tells keyset-rs how to draw the keys)
let template = drawing::Template {
    profile,
    font,
    ..Default::default()
};

// Create drawing
let drawing = template.draw(&keys);

// Save output
let path = std::env::current_dir()?;
std::fs::write(path.join("output.svg"), drawing.to_svg())?;
std::fs::write(path.join("output.png"), drawing.to_png(96.0)?)?;
std::fs::write(path.join("output.pdf"), drawing.to_pdf())?;
```

<!-- cargo-rdme end -->

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Licence

Licensed under either of

* Apache License, Version 2.0 ([LICENCE-APACHE](LICENCE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0][apache-licence])
* MIT license ([LICENCE-MIT](LICENCE-MIT) or [http://opensource.org/licenses/MIT][mit-licence])

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

[apache-licence]: http://www.apache.org/licenses/LICENSE-2.0
[mit-licence]: http://opensource.org/licenses/MIT
