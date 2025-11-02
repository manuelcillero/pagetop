use pagetop_build::StaticFilesBundle;

use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_scss("./static/scss/bootsier.scss", "bootstrap.min.css")
        .with_name("bootsier_bs")
        .build()?;
    StaticFilesBundle::from_dir("./static/js", Some(bootstrap_js_files))
        .with_name("bootsier_js")
        .build()
}

fn bootstrap_js_files(path: &Path) -> bool {
    let bootstrap_js = "bootstrap.bundle.min.js";
    // No filtra durante el desarrollo, solo en la compilación "release".
    env::var("PROFILE").unwrap_or_else(|_| "release".to_string()) != "release"
        || path.file_name().is_some_and(|f| f == bootstrap_js)
}
