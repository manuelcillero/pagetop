use actix_web_static_files::resource_dir;

use std::env;
use std::path::Path;

fn main() {
    resource_dir("./resources/assets")
        .with_generated_filename(
            Path::new(env::var("OUT_DIR").unwrap().as_str())
                .join("assets.rs")
        )
        .with_generated_fn("assets")
        .build()
        .unwrap();

    resource_dir("./src/base/theme/aliner/assets")
        .with_generated_filename(
            Path::new(env::var("OUT_DIR").unwrap().as_str())
                .join("aliner.rs")
        )
        .build()
        .unwrap();

    resource_dir("./src/base/theme/bootsier/assets")
        .with_generated_filename(
            Path::new(env::var("OUT_DIR").unwrap().as_str())
                .join("bootsier.rs")
        )
        .build()
        .unwrap();
}
