use pagetop::prelude::*;

use include_dir::{include_dir, Dir};
use tera::Tera;

use std::sync::LazyLock;

include_locales!(LOCALES_ALINER);

include_files!(aliner);

// ALINER THEME ************************************************************************************

pub const TEMPLATE_GLOB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html");
pub const TEMPLATE_BASE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// Static instance of Tera used for rendering HTML templates for components.
///
/// - In `debug` mode, templates are dynamically loaded from the file system, allowing for rapid
///   iteration.
/// - In `release` mode (`cargo build --release`), templates are embedded directly into the binary
///   for optimal performance and portability.
pub static ALINER_THEME: LazyLock<Tera> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        // In debug mode, load templates directly from the file system.
        Tera::new(TEMPLATE_GLOB).expect("Failed to initialize Tera from disk in debug mode")
    } else {
        // In release mode (cargo build --release), embed templates into the binary.
        let mut tera = Tera::default();
        for file in TEMPLATE_BASE_DIR.files() {
            if let Some(path) = file.path().to_str() {
                let content = file
                    .contents_utf8()
                    .expect("Non UTF-8 content in template file");
                tera.add_raw_template(path, content)
                    .expect("Failed to add template to Tera");
            }
        }
        tera
    }
});

// ALINER DEFINITION *******************************************************************************

pub struct Aliner;

impl PackageTrait for Aliner {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Aliner)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        include_files_service!(scfg, aliner => "/aliner");
    }
}

impl ThemeTrait for Aliner {}
