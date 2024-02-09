<div align="center">

  <img src="https://raw.githubusercontent.com/manuelcillero/pagetop/main/static/banner.png" />

  <h1>PageTop</h1>

  [![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](https://github.com/manuelcillero/pagetop#-license)
  [![API Docs](https://img.shields.io/docsrs/pagetop?label=API%20Docs&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop)
  [![Crates.io](https://img.shields.io/crates/v/pagetop.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop)
  [![Downloads](https://img.shields.io/crates/d/pagetop.svg?style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop)

</div>

**PageTop** is an opinionated [Rust](https://www.rust-lang.org) web development framework to build
secure and modular Server-Side Rendering (SSR) web solutions.

PageTop stands on the shoulders of giants. It leverages some of the most stable and popular Rust
crates to provide extensible and easily configurable features.

# 🚧 Warning

**PageTop** framework is currently in active development. The API is unstable and subject to
frequent changes. Production use is not recommended until version **0.1.0**.


# 📂 Repository Structure

This repository is organized into a workspace that includes several subprojects, each serving a
distinct role within the PageTop ecosystem:

## Application

* **[drust](https://github.com/manuelcillero/pagetop/tree/main/drust)**, a simple Content Management
  System (CMS) built on PageTop, which enables the creation, editing, and maintenance of dynamic,
  fast, and secure websites. It uses the following essential packages to provide standard CMS
  functionalities.

## Packages

* **[pagetop-homedemo](https://github.com/manuelcillero/pagetop/tree/main/pagetop-homedemo)**:
  Showcases a demo homepage, offering a glimpse into PageTop's capabilities.

* **[pagetop-admin](https://github.com/manuelcillero/pagetop/tree/main/pagetop-admin)**: Offers a
  unified interface for administrators to configure and manage package settings.

* **[pagetop-user](https://github.com/manuelcillero/pagetop/tree/main/pagetop-user)**: Facilitates
  user management, including roles, permissions, and session handling, for PageTop-based
  applications.

* **[pagetop-node](https://github.com/manuelcillero/pagetop/tree/main/pagetop-node)**: Enables the
  creation and customization of content types, enhancing website content management.

## Themes

* **[pagetop-bootsier](https://github.com/manuelcillero/pagetop/tree/main/pagetop-bootsier)**:
  Utilizes the [Bootstrap](https://getbootstrap.com/) framework to offer versatile page layouts and
  component stylings.

* **[pagetop-bulmix](https://github.com/manuelcillero/pagetop/tree/main/pagetop-bulmix)**: Employs
  the [Bulma](https://bulma.io/) framework for sleek, responsive design elements.

## Helpers

* **[pagetop-macros](https://github.com/manuelcillero/pagetop/tree/main/pagetop-macros)**: A
  collection of procedural macros that enhance the development experience within PageTop.

* **[pagetop-build](https://github.com/manuelcillero/pagetop/tree/main/pagetop-build)**: Simplifies
  the process of embedding resources directly into binary files for PageTop applications.


# 📜 License

PageTop is free, open source and permissively licensed! Except where noted (below and/or in
individual files), all code in this repository is dual-licensed under either:

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
