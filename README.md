# keyset.rs

[![Test status](https://img.shields.io/github/actions/workflow/status/staticintlucas/keyset-rs/test.yml?branch=main&label=tests&style=flat-square)][tests]
[![Code coverage](https://img.shields.io/codecov/c/gh/staticintlucas/keyset-rs?style=flat-square)][coverage]
[![Rust version](https://img.shields.io/badge/rust-1.60%2B-informational?style=flat-square)][rust version]

A (WIP) reimplementation of [pykeyset] in Rust for improved performance.
Eventually this aims to become the backend for pykeyset using a Python wrapper around this project.

Current minimum supported Rust version is 1.60.0, although this is subject to change as development continues.

[tests]: https://github.com/staticintlucas/keyset-rs/actions
[coverage]: https://codecov.io/gh/staticintlucas/keyset-rs
[rust version]: #readme
[pykeyset]: https://github.com/staticintlucas/pykeyset

## Installation

```sh
cargo install keyset
```

## Usage

Coming soon

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

You can install the [pre-commit] hook (which checks formatting, etc) by running:

```sh
pip install -U pre-commit
pre-commit install
```

[pre-commit]: https://pre-commit.com/

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
