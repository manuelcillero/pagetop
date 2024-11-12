<div align="center">

<img src="https://raw.githubusercontent.com/manuelcillero/pagetop/main/static/banner.png" />

<h1>PageTop</h1>

<p>An opinionated web framework to build modular <em>Server-Side Rendering</em> web solutions.</p>

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](#-license)
[![API Docs](https://img.shields.io/docsrs/pagetop?label=API%20Docs&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop)
[![Crates.io](https://img.shields.io/crates/v/pagetop.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop)
[![Downloads](https://img.shields.io/crates/d/pagetop.svg?style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop)

</div>

## Overview

The PageTop core API provides a comprehensive toolkit for extending its functionalities to specific
requirements and application scenarios through actions, components, packages, and themes:

  * **Actions** serve as a mechanism to customize PageTop's internal behavior by intercepting its
    execution flow.
  * **Components** encapsulate HTML, CSS, and JavaScript into functional, configurable, and
    well-defined units.
  * **Packages** extend or customize existing functionality by interacting with PageTop APIs or
    third-party package APIs.
  * **Themes** enable developers to alter the appearance of pages and components without affecting
    their functionality.


# ⚡️ Quick start

```rust
use pagetop::prelude::*;

struct HelloWorld;

impl PackageTrait for HelloWorld {
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(hello_world));
    }
}

async fn hello_world(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_component(Html::with(html! { h1 { "Hello World!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).run()?.await
}
```

This program features a `HelloWorld` package, providing a service that serves a greeting web page
accessible via `http://localhost:8088` under default settings.


# 📂 Helpers

* [pagetop-macros](https://github.com/manuelcillero/pagetop/tree/latest/helpers/pagetop-macros):
  A collection of macros that enhance the development experience within PageTop.

* [pagetop-build](https://github.com/manuelcillero/pagetop/tree/latest/helpers/pagetop-build):
  Simplifies the process of embedding resources directly into binary files for PageTop applications.


# 🚧 Warning

**PageTop** framework is currently in active development. The API is unstable and subject to
frequent changes. Production use is not recommended until version **0.1.0**.


# 📜 License

PageTop is free, open source and permissively licensed! Except where noted (below and/or in
individual files), all code in this project is dual-licensed under either:

  * MIT License
    ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

  * Apache License, Version 2.0,
    ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is
the de-facto standard in the Rust ecosystem.


# ✨ Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
