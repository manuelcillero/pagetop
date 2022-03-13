use actix_web_static_files::resource_dir;

use std::env;
use std::path::Path;

fn main() {
    resource_dir("./static/theme")
        .with_generated_filename(
            Path::new(env::var("OUT_DIR").unwrap().as_str())
                .join("theme.rs")
        )
        .with_generated_fn("assets")
        .build()
        .unwrap();

    resource_dir("./static/aliner")
        .with_generated_filename(
            Path::new(env::var("OUT_DIR").unwrap().as_str())
                .join("aliner.rs")
        )
        .with_generated_fn("assets")
        .build()
        .unwrap();

    resource_dir("./static/bootsier")
        .with_generated_filename(
            Path::new(env::var("OUT_DIR").unwrap().as_str())
                .join("bootsier.rs")
        )
        .with_generated_fn("assets")
        .build()
        .unwrap();
}
