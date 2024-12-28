use pagetop_build::StaticFilesBundle;

use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_scss("./static/bs-5.3.3/scss/bootstrap.scss", "bootstrap.min.css")
        .with_name("bootsier_bs")
        .build()?;
    StaticFilesBundle::from_dir("./static/bs-5.3.3/js", Some(bootstrap_js_files))
        .with_name("bootsier_js")
        .build()
}

fn bootstrap_js_files(path: &Path) -> bool {
    // No filtra durante el desarrollo, solo en la compilación "release".
    env::var("PROFILE").unwrap_or_else(|_| "release".to_string()) != "release"
        || path.file_name().map_or(false, |n| n == "bootstrap.min.js")
}
