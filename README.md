# Neotron API

This crate defines the API between the [Neotron OS] and any Neotron Applications running on it.

If you are writing a Neotron Application, you might prefer to use the [Neotron SDK], which wraps up this API into something slightly more useable.

Note that this API must be FFI-safe, because the [Neotron OS] and the Application may be compiled with different versions of Rust.

[Neotron OS]: https://github.com/neotron-compute/neotron-os
[Neotron SDK]: https://github.com/neotron-compute/neotron-sdk

## Changelog

### Unreleased Changes

* First Version

## Licence

Copyright (c) The Neotron Developers, 2023

Licensed under either [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
