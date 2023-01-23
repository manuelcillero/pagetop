//! This function uses the [static_files](https://docs.rs/static-files/latest/static_files/) library
//! to embed at compile time a bundle of static files in your binary.
//!
//! Just create folder with static resources in your project (for example `static`):
//!
//! ```bash
//! cd project_dir
//! mkdir static
//! echo "Hello, world" > static/hello
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
//! fn main() -> std::io::Result<()> {
//!     pagetop_build::bundle_resources("./static", "guides", None)
//! }
//! ```
//!
//! Optionally, you can pass a function to filter those files into the `./static` folder which
//! should be excluded in the resources file:
//!
//! ```rust#ignore
//! fn main() -> std::io::Result<()> {
//!     pagetop_build::bundle_resources("./static", "guides", Some(except_css_dir))
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
//! You don't need to access this file, just include it in your project source code and a module
//! called `resources_guides` will be added. Then simply reference the `bundle_guides` function to
//! embed the generated HashMap resources collection:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! include!(concat!(env!("OUT_DIR"), "/guides.rs"));
//! static RESOURCES: LazyStatic<HashMapResources> = LazyStatic::new(bundle_guides);
//! ```
//!
//! You can build more than one resources file to compile with your project.

use std::path::Path;

pub fn bundle_resources(
    from_dir: &str,
    with_name: &str,
    filtering: Option<fn(p: &Path) -> bool>,
) -> std::io::Result<()> {
    let mut bundle = static_files::resource_dir(from_dir);
    bundle.with_generated_filename(
        Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join(format!("{}.rs", with_name)),
    );
    bundle.with_module_name(format!("resources_{}", with_name));
    bundle.with_generated_fn(format!("bundle_{}", with_name));
    if let Some(filter_files) = filtering {
        bundle.with_filter(filter_files);
    }
    bundle.build()
}
