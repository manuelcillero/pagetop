use pagetop_build::{StaticFilesBundle, copy_dir, minify_js};

fn main() -> std::io::Result<()> {
    // Regenera `static/` desde cero sólo si hay cambios en `assets/`.
    println!("cargo:rerun-if-changed=assets");
    let _ = std::fs::remove_dir_all("static");

    copy_dir("assets", "static")?;

    // JS: confirmación en el diálogo compartido de `hx_confirm`.
    minify_js("assets/js/hx-confirm.js", "static/js/hx-confirm.min.js")?;

    StaticFilesBundle::from_dir("./static", None)
        .with_name("htmx")
        .build()
}
