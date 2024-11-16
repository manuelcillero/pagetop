use pagetop_build::StaticFilesBundle;

use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_scss("./static/bootstrap-5.3.3/bootstrap.scss", "bootstrap.css")
        .with_name("bootsier")
        .build()?;
    StaticFilesBundle::from_dir("./static/js", Some(bootstrap_js_files))
        .with_name("bootsier-js")
        .build()
}

fn bootstrap_js_files(path: &Path) -> bool {
    // No filtering during development, only on "release" compilation.
    env::var("PROFILE").unwrap_or_else(|_| "release".to_string()) != "release"
        || path.file_name().map_or(false, |n| n == "bootstrap.min.js")
}
