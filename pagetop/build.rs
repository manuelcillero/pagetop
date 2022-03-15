use static_files::resource_dir;

use std::env;
use std::path::Path;

fn main() {
    build_resource_dir("./static/theme", "theme.rs", "assets");
    build_resource_dir("./static/aliner", "aliner.rs", "assets");
    build_resource_dir("./static/bootsier", "bootsier.rs", "assets");
}

fn build_resource_dir(dir: &str, with_filename: &str, with_fn: &str) {
    let mut resource = resource_dir(dir);
    resource.with_generated_filename(
        Path::new(env::var("OUT_DIR").unwrap().as_str()).join(with_filename)
    );
    resource.with_generated_fn(with_fn);
    resource.build().unwrap();
}
