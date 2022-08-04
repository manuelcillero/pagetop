use std::path::Path;

fn main() -> std::io::Result<()> {
    bundle_resources("./static/theme", "theme")?;
    bundle_resources("./static/aliner", "aliner")?;
    bundle_resources("./static/bootsier", "bootsier")?;
    bundle_resources("./static/bulmix", "bulmix")
}

/// This function is a simplified version of pagetop::util::bundle_resources().
pub fn bundle_resources(from_dir: &str, with_name: &str) -> std::io::Result<()> {
    let mut r = static_files::resource_dir(from_dir);
    r.with_generated_filename(
        Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join(format!("{}.rs", with_name)),
    );
    r.with_module_name(format!("resources_{}", with_name));
    r.with_generated_fn(format!("bundle_{}", with_name));
    r.build()
}
