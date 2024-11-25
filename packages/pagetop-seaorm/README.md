<div align="center">

<h1>PageTop SeaORM</h1>

<p>Integrate SeaORM as the database framework for PageTop applications.</p>

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](#-license)
[![API Docs](https://img.shields.io/docsrs/pagetop-seaorm?label=API%20Docs&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-seaorm)
[![Crates.io](https://img.shields.io/crates/v/pagetop-seaorm.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-seaorm)
[![Downloads](https://img.shields.io/crates/d/pagetop-seaorm.svg?style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-seaorm)

</div>

PageTop SeaORM employs [SQLx](https://crates.io/crates/sqlx) and
[SeaQuery](https://crates.io/crates/sea-query), complemented by a custom version of
[SeaORM Migration](https://github.com/SeaQL/sea-orm/tree/1.1.1/sea-orm-migration/src) (v1.1.1). The
modified SeaORM Migration ensures migrations are scoped per package, providing greater control and
reducing coupling between database components.

# 📦 About PageTop

[PageTop](https://docs.rs/pagetop) is an opinionated web framework to build modular *Server-Side
Rendering* web solutions.


# 🚧 Warning

**PageTop** framework is currently in active development. The API is unstable and subject to
frequent changes. Production use is not recommended until version **0.1.0**.


# 📜 License

All code in this crate is dual-licensed under either:

  * MIT License
    ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

  * Apache License, Version 2.0,
    ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is
the de-facto standard in the Rust ecosystem.
