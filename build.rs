use pagetop_build::{StaticFilesBundle, compile_scss, copy_dir, copy_file, minify_js};

fn main() -> std::io::Result<()> {
    // Regenera `static/` desde cero sólo si hay cambios en `assets/`.
    println!("cargo:rerun-if-changed=assets");
    let _ = std::fs::remove_dir_all("static");

    // Copia sin transformar. `assets/scss/` queda fuera porque sólo contiene fuentes SCSS.
    copy_file("assets/banner.png", "static/banner.png")?;
    copy_file("assets/favicon.ico", "static/favicon.ico")?;
    copy_dir("assets/img", "static/img")?;
    copy_dir("assets/js", "static/js")?;

    // CSS: compila los estilos del tema Basic.
    compile_scss("assets/scss/basic.scss", "static/css/basic.min.css")?;

    // CSS: compila los estilos del componente `Intro`, independientes del tema.
    compile_scss("assets/scss/intro.scss", "static/css/intro.min.css")?;

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
