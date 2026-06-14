use pagetop_build::{StaticFilesBundle, copy_dir};

fn main() -> std::io::Result<()> {
    // Regenera `static/` desde cero sólo si hay cambios en `assets/`.
    println!("cargo:rerun-if-changed=assets");
    let _ = std::fs::remove_dir_all("static");

    copy_dir("assets", "static")?;

    StaticFilesBundle::from_dir("./static", None)
        .with_name("htmx")
        .build()
}
