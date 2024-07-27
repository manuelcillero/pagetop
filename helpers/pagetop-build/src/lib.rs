//! This function uses the [static_files](https://docs.rs/static-files/latest/static_files/) library
//! to embed at compile time a bundle of static files in your binary.
//!
//! Just create folder with static resources in your project (for example `static`):
//!
//! ```bash
//! cd project_dir
//! mkdir static
//! echo "Hello, world!" > static/hello
//! ```
//!
//! Add to `Cargo.toml` the required dependencies:
//!
//! ```toml
//! [build-dependencies]
//! pagetop-build = { ... }
//! ```
//!
//! Add `build.rs` with call to bundle resources (*guides* will be the magic word in this example):
//!
//! ```rust#ignore
//! use pagetop_build::StaticFilesBundle;
//!
//! fn main() -> std::io::Result<()> {
//!     StaticFilesBundle::from_dir("./static")
//!         .with_name("guides")
//!         .build()
//! }
//! ```
//!
//! Optionally, you can pass a function to filter those files into the `./static` folder which
//! should be excluded in the resources bundle:
//!
//! ```rust#ignore
//! use pagetop_build::StaticFilesBundle;
//!
//! fn main() -> std::io::Result<()> {
//!     StaticFilesBundle::from_dir("./static")
//!         .with_name("guides")
//!         .with_filter(except_css_dir)
//!         .build()
//! }
//!
//! fn except_css_dir(p: &Path) -> bool {
//!     if let Some(parent) = p.parent() {
//!         !matches!(parent.to_str(), Some("/css"))
//!     }
//!     true
//! }
//! ```
//!
//! This will create a file called `guides.rs` in the standard directory
//! [OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html) where all
//! intermediate and output artifacts are placed during compilation.
//!
//! You don't need to access this file, just include it in your project using the builder name as an
//! identifier:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! static_files!(guides);
//! ```
//!
//! Also you can get the bundle as a static reference to the generated `HashMap` resources
//! collection:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! static_files!(guides => BUNDLE_GUIDES);
//! ```
//!
//! You can build more than one resources file to compile with your project.

use std::path::Path;

pub struct StaticFilesBundle(static_files::ResourceDir);

impl StaticFilesBundle {
    pub fn from_dir(dir: &'static str) -> Self {
        StaticFilesBundle(static_files::resource_dir(dir))
    }

    /// Configures the name for the bundle of static files.
    ///
    /// # Panics
    ///
    /// This function will panic if the standard `OUT_DIR` environment variable is not set.
    pub fn with_name(mut self, name: &'static str) -> Self {
        self.0.with_generated_filename(
            Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join(format!("{name}.rs")),
        );
        self.0.with_module_name(format!("bundle_{name}"));
        self.0.with_generated_fn(name);
        self
    }

    pub fn with_filter(mut self, filter: fn(p: &Path) -> bool) -> Self {
        self.0.with_filter(filter);
        self
    }

    /// Builds the bundle.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an issue with I/O operations, such as failing
    /// to read or write to a file.
    pub fn build(self) -> std::io::Result<()> {
        self.0.build()
    }
}
