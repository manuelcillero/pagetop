use static_files::resource_dir;

use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    build_resource_dir("./static", "mdbook")
}

fn build_resource_dir(dir: &str, name: &str) -> std::io::Result<()> {
    let mut resource = resource_dir(dir);
    resource.with_generated_filename(
        Path::new(env::var("OUT_DIR").unwrap().as_str()).join(format!("{}.rs", name)),
    );
    resource.with_module_name(format!("resources_{}", name));
    resource.build()
}
