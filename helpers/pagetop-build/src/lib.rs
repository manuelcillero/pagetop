//! Provide an easy way to embed static files or compiled SCSS files into your binary at compile
//! time.
//!
//! ## Adding to your project
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! pagetop-build = { ... }
//! ```
//!
//! Next, create a `build.rs` file to configure how your static resources or SCSS files will be
//! bundled in your PageTop application, package, or theme.
//!
//! ## Usage examples
//!
//! ### 1. Embedding static files from a directory
//!
//! Include all files from a directory:
//!
//! ```rust#ignore
//! use pagetop_build::StaticFilesBundle;
//!
//! fn main() -> std::io::Result<()> {
//!     StaticFilesBundle::from_dir("./static", None)
//!         .with_name("guides")
//!         .build()
//! }
//! ```
//!
//! Apply a filter to include only specific files:
//!
//! ```rust#ignore
//! use pagetop_build::StaticFilesBundle;
//! use std::path::Path;
//!
//! fn main() -> std::io::Result<()> {
//!     fn only_css_files(path: &Path) -> bool {
//!         // Include only files with `.css` extension.
//!         path.extension().map_or(false, |ext| ext == "css")
//!     }
//!
//!     StaticFilesBundle::from_dir("./static", Some(only_css_files))
//!         .with_name("guides")
//!         .build()
//! }
//! ```
//!
//! ### 2. Compiling SCSS files to CSS
//!
//! Compile a SCSS file into CSS and embed it:
//!
//! ```rust#ignore
//! use pagetop_build::StaticFilesBundle;
//!
//! fn main() -> std::io::Result<()> {
//!     StaticFilesBundle::from_scss("./styles/main.scss", "main.css")
//!         .with_name("main_styles")
//!         .build()
//! }
//! ```
//!
//! This compiles the `main.scss` file, including all imported SCSS files, into `main.css`. All
//! imports are resolved automatically, and the result is accessible within the binary file.
//!
//! ## Generated module
//!
//! [`StaticFilesBundle`] generates a file in the standard directory
//! [OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html) where all
//! intermediate and output artifacts are placed during compilation. For example, if you use
//! `with_name("guides")`, it generates a file named `guides.rs`:
//!
//! You don't need to access this file, just include it in your project using the builder name as an
//! identifier:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! include_files!(guides);
//! ```
//!
//! Or, access the entire bundle as a global static `HashMap`:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! include_files!(guides => BUNDLE_GUIDES);
//! ```
//!
//! You can build more than one resources file to compile with your project.

use grass::{from_path, Options, OutputStyle};
use static_files::{resource_dir, ResourceDir};

use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::Path;

/// Generates the resources to embed at compile time using
/// [static_files](https://docs.rs/static-files/latest/static_files/).
pub struct StaticFilesBundle {
    resource_dir: ResourceDir,
}

impl StaticFilesBundle {
    /// Creates a bundle from a directory of static files, with an optional filter.
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory containing the static files.
    /// * `filter` - An optional function to filter files or directories to include.
    pub fn from_dir(dir: &'static str, filter: Option<fn(p: &Path) -> bool>) -> Self {
        let mut resource_dir = resource_dir(dir);

        // Apply the filter if provided.
        if let Some(f) = filter {
            resource_dir.with_filter(f);
        }

        StaticFilesBundle { resource_dir }
    }

    /// Creates a bundle starting from a SCSS file.
    ///
    /// # Arguments
    ///
    /// * `path` - The SCSS file to compile.
    /// * `target_name` - The name for the CSS file in the bundle.
    ///
    /// This function will panic:
    ///
    /// * If the environment variable `OUT_DIR` is not set.
    /// * If it is unable to create a temporary directory in the `OUT_DIR`.
    /// * If the SCSS file cannot be compiled due to syntax errors in the SCSS file or missing
    ///   dependencies or import paths required for compilation.
    /// * If it is unable to create the output CSS file in the temporary directory due to an invalid
    ///   `target_name` or insufficient permissions to create files in the temporary directory.
    /// * If the function fails to write the compiled CSS content to the file.
    pub fn from_scss<P>(path: P, target_name: &str) -> Self
    where
        P: AsRef<Path>,
    {
        // Create a temporary directory for the CSS file.
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let temp_dir = Path::new(&out_dir).join("from_scss_files");
        // Clean up the temporary directory from previous runs, if it exists.
        if temp_dir.exists() {
            remove_dir_all(&temp_dir).unwrap_or_else(|e| {
                panic!(
                    "Failed to clean temporary directory `{}`: {e}",
                    temp_dir.display()
                );
            });
        }
        create_dir_all(&temp_dir).unwrap_or_else(|e| {
            panic!(
                "Failed to create temporary directory `{}`: {e}",
                temp_dir.display()
            );
        });

        // Compile SCSS to CSS.
        let css_content = from_path(
            path.as_ref(),
            &Options::default().style(OutputStyle::Compressed),
        )
        .unwrap_or_else(|e| {
            panic!(
                "Failed to compile SCSS file `{}`: {e}",
                path.as_ref().display(),
            )
        });

        // Write the compiled CSS to the temporary directory.
        let css_path = temp_dir.join(target_name);
        File::create(&css_path)
            .expect(&format!(
                "Failed to create CSS file `{}`",
                css_path.display()
            ))
            .write_all(css_content.as_bytes())
            .expect(&format!(
                "Failed to write CSS content to `{}`",
                css_path.display()
            ));

        // Initialize ResourceDir with the temporary directory.
        StaticFilesBundle {
            resource_dir: resource_dir(temp_dir.to_str().unwrap()),
        }
    }

    /// Configures the name for the bundle of static files.
    ///
    /// # Panics
    ///
    /// This function will panic if the standard `OUT_DIR` environment variable is not set.
    pub fn with_name(mut self, name: &'static str) -> Self {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let filename = Path::new(&out_dir).join(format!("{name}.rs"));
        self.resource_dir.with_generated_filename(filename);
        self.resource_dir.with_module_name(format!("bundle_{name}"));
        self.resource_dir.with_generated_fn(name);
        self
    }

    /// Builds the bundle.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an issue with I/O operations, such as failing
    /// to read or write to a file.
    pub fn build(self) -> std::io::Result<()> {
        self.resource_dir.build()
    }
}
