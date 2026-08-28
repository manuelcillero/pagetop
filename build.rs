use pagetop_build::{StaticFilesBundle, compile_scss, copy_dir, minify_js};

fn main() -> std::io::Result<()> {
    // Regenera `static/` desde cero sólo si hay cambios en `assets/`.
    println!("cargo:rerun-if-changed=assets");
    let _ = std::fs::remove_dir_all("static");

    copy_dir("assets", "static")?;

    // CSS: genera la variante minificada de `basic.css` del tema Basic.
    compile_scss("assets/css/basic.css", "static/css/basic.min.css")?;

    // JS: minifica el manejo de `Dialog` del tema Basic.
    minify_js(
        "assets/js/basic.dialog.init.js",
        "static/js/basic.dialog.min.js",
    )?;

    // JS: minifica la integración de `Dropdown` con `accessible-menu` del tema Basic.
    minify_js(
        "assets/js/basic.dropdown.init.js",
        "static/js/basic.dropdown.min.js",
    )?;

    StaticFilesBundle::from_dir("./static", None)
        .with_name("assets")
        .build()
}
