use actix_web_static_files::resource_dir;

use std::env;
use std::path::Path;

fn main() {
    resource_dir("./static/theme")
        .with_generated_filename(
            Path::new(env::var("OUT_DIR").unwrap().as_str())
                .join("theme.rs")
        )
        .build()
        .unwrap();

    resource_dir("./static/aliner")
        .with_generated_filename(
            Path::new(env::var("OUT_DIR").unwrap().as_str())
                .join("aliner.rs")
        )
        .build()
        .unwrap();

    resource_dir("./static/bootsier")
        .with_generated_filename(
            Path::new(env::var("OUT_DIR").unwrap().as_str())
                .join("bootsier.rs")
        )
        .build()
        .unwrap();

    resource_dir("./static/bulmix")
        .with_generated_filename(
            Path::new(env::var("OUT_DIR").unwrap().as_str())
                .join("bulmix.rs")
        )
        .build()
        .unwrap();
}
