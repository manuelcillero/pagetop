<div align="center">

<h1>PageTop Macros</h1>

<p>A collection of macros that boost PageTop development.</p>

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](#-license)
[![API Docs](https://img.shields.io/docsrs/pagetop-macros?label=API%20Docs&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-macros)
[![Crates.io](https://img.shields.io/crates/v/pagetop-macros.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-macros)
[![Downloads](https://img.shields.io/crates/d/pagetop-macros.svg?style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-macros)

</div>

# 📦 About PageTop

[PageTop](https://docs.rs/pagetop) is an opinionated web framework to build modular *Server-Side
Rendering* web solutions.


# 🚧 Warning

**PageTop** framework is currently in active development. The API is unstable and subject to
frequent changes. Production use is not recommended until version **0.1.0**.


# 🔖 Credits

This crate includes an adapted version of [maud-macros](https://crates.io/crates/maud_macros),
version [0.25.0](https://github.com/lambda-fairy/maud/tree/v0.25.0/maud_macros), by
[Chris Wong](https://crates.io/users/lambda-fairy).

It also includes an adapted version of [SmartDefault](https://crates.io/crates/smart_default)
(version 0.7.1) by [Jane Doe](https://crates.io/users/jane-doe), renamed as `AutoDefault`, to
streamline the implementation of `Default` in **PageTop** projects.

Both adaptations eliminate the need to explicitly add `maud` or `smart_default` as dependencies in
`Cargo.toml` files.


# 📜 License

All code in this crate is dual-licensed under either:

  * MIT License
    ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

  * Apache License, Version 2.0,
    ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is
the de-facto standard in the Rust ecosystem.
