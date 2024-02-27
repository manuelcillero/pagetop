//! <div align="center">
//!
//!   <img src="https://raw.githubusercontent.com/manuelcillero/pagetop/main/static/banner.png" />
//!
//!   <h1>PageTop</h1>
//!
//!   [![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](https://github.com/manuelcillero/pagetop#-license)
//!   [![API Docs](https://img.shields.io/docsrs/pagetop?label=API%20Docs&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop)
//!   [![Crates.io](https://img.shields.io/crates/v/pagetop.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop)
//!   [![Downloads](https://img.shields.io/crates/d/pagetop.svg?style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop)
//!
//! </div>
//!
//! **PageTop** is an opinionated Rust web development framework to build modular Server-Side
//! Rendering (SSR) web solutions.
//!
//! The PageTop core API provides a comprehensive toolkit for extending its functionalities to
//! specific requirements and application scenarios through actions, components, packages, and
//! themes:
//!
//! * **Actions** serve as a mechanism to customize PageTop's internal behavior by intercepting its
//!   execution flow.
//! * **Components** encapsulate HTML, CSS, and JavaScript into functional, configurable, and
//!   well-defined units.
//! * **Packages** extend or customize existing functionality by interacting with PageTop APIs or
//!   third-party package APIs.
//! * **Themes** enable developers to alter the appearance of pages and components without affecting
//!   their functionality.
//!
//! # ⚡️ Quick start
//!
//! ```rust
//! use pagetop::prelude::*;
//!
//! struct HelloWorld;
//!
//! impl PackageTrait for HelloWorld {
//!     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
//!         scfg.route("/", service::web::get().to(hello_world));
//!     }
//! }
//!
//! async fn hello_world(request: service::HttpRequest) -> ResultPage<Markup, ErrorPage> {
//!     Page::new(request)
//!         .with_component(Html::with(html! { h1 { "Hello World!" } }))
//!         .render()
//! }
//!
//! #[pagetop::main]
//! async fn main() -> std::io::Result<()> {
//!     Application::prepare(&HelloWorld).run()?.await
//! }
//! ```
//! This program implements a package named `HelloWorld` with one service that returns a web page
//! that greets the world whenever it is accessed from the browser at `http://localhost:8088` (using
//! the [default configuration settings](`config::Server`)). You can find this code in the PageTop
//! [basic examples repository](https://github.com/manuelcillero/pagetop/tree/main/examples/basics).
//!
//! # 🧩 Dependency Management
//!
//! Projects leveraging PageTop will use `cargo` to resolve dependencies, similar to any other Rust
//! project.
//!
//! Nevertheless, it’s crucial that each package explicitly declares its
//! [dependencies](core::package::PackageTrait#method.dependencies), if any, to assist PageTop in
//! structuring and initializing the application in a modular fashion.
//!
//! # 🚧 Warning
//!
//! **PageTop** framework is currently in active development. The API is unstable and subject to
//! frequent changes. Production use is not recommended until version **0.1.0**.

#![cfg_attr(docsrs, feature(doc_cfg))]

// *************************************************************************************************
// RE-EXPORTED MACROS AND DERIVES.
// *************************************************************************************************

pub use concat_string::concat_string;

/// Enables flexible identifier concatenation in macros, allowing new items with pasted identifiers.
pub use paste::paste;

/// Custom derive for automatically implementing the [Default] trait with customized default values.
pub use smart_default::SmartDefault;

pub use pagetop_macros::{fn_with, main, test, ComponentClasses};

// *************************************************************************************************
// GLOBAL.
// *************************************************************************************************

pub use once_cell::sync::Lazy as LazyStatic;
pub use static_files::Resource as StaticResource;

pub use std::any::TypeId;

pub type Weight = i8;

pub type HashMapResources = std::collections::HashMap<&'static str, StaticResource>;

static_locales!(LOCALES_PAGETOP);

// *************************************************************************************************
// PUBLIC API.
// *************************************************************************************************

// Functions and macro helpers.
pub mod util;

// Retrieve and apply settings values from configuration files.
pub mod config;
// Application tracing and event logging.
pub mod trace;
// HTML in code.
pub mod html;
// Localization.
pub mod locale;
// Date and time handling.
pub mod datetime;

// Database access.
#[cfg_attr(docsrs, doc(cfg(feature = "database")))]
#[cfg(feature = "database")]
pub mod db;

// Essential web framework.
pub mod service;

// Key types and functions for creating actions, components, packages, and themes.
pub mod core;

// Web request response variants.
pub mod response;

// Base actions, components, packages, and themes.
pub mod base;

// Prepare and run the application.
pub mod app;

// *************************************************************************************************
// The PageTop Prelude.
// *************************************************************************************************

pub mod prelude;
