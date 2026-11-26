use pagetop_build::{StaticFilesBundle, compile_scss, copy_dir, copy_file, minify_js};

fn main() -> std::io::Result<()> {
    // Fecha de publicación de la versión que se compila, tomada de su entrada en `CHANGELOG.md`
    // (`## 0.5.0 (2026-05-03)`), para que `Intro` la muestre sin consultar ningún servicio externo.
    // Queda vacía si la versión aún no figura en el `CHANGELOG`.
    println!("cargo:rerun-if-changed=CHANGELOG.md");
    let heading = format!("## {} (", env!("CARGO_PKG_VERSION"));
    let date = std::fs::read_to_string("CHANGELOG.md")
        .ok()
        .and_then(|changelog| {
            changelog.lines().find_map(|line| {
                line.trim_end()
                    .strip_prefix(&heading)
                    .and_then(|rest| rest.strip_suffix(')'))
                    .map(str::to_owned)
            })
        })
        .unwrap_or_default();
    println!("cargo:rustc-env=PAGETOP_RELEASE_DATE={date}");

    // **< Preparación del directorio de recursos estáticos >***************************************

    // Regenera `static/` desde cero sólo si hay cambios en `assets/`.
    println!("cargo:rerun-if-changed=assets");
    let _ = std::fs::remove_dir_all("static");

    // Copia sin transformar. `assets/scss/` queda fuera porque sólo contiene fuentes SCSS.
    copy_file("assets/banner.png", "static/banner.png")?;
    copy_file("assets/favicon.ico", "static/favicon.ico")?;
    copy_dir("assets/img", "static/img")?;
    copy_dir("assets/js", "static/js")?;

    // **< Recursos independientes del tema, por tipo y orden alfabético >**************************

    // CSS: estilos del componente `Intro`.
    compile_scss("assets/scss/intro.scss", "static/css/intro.min.css")?;

    // CSS: disposición del componente `Mosaic`.
    compile_scss("assets/scss/mosaic.scss", "static/css/mosaic.min.css")?;

    // JS: marcas de cambios sin aplicar de `ListFilter`.
    minify_js("assets/js/list-filter.js", "static/js/list-filter.min.js")?;

    // JS: aviso del resumen de `ListView` a los lectores de pantalla.
    minify_js("assets/js/list-view.js", "static/js/list-view.min.js")?;

    // JS: reparto en columnas de `Mosaic`.
    minify_js("assets/js/mosaic.js", "static/js/mosaic.min.js")?;

    // **< Recursos del tema Basic >****************************************************************

    // CSS: estilos del tema.
    compile_scss("assets/scss/basic.scss", "static/css/basic.min.css")?;

    // JS: manejo de `Dialog`.
    minify_js(
        "assets/js/basic.dialog.init.js",
        "static/js/basic.dialog.min.js",
    )?;

    // JS: integración de `Dropdown` con `accessible-menu`.
    minify_js(
        "assets/js/basic.dropdown.init.js",
        "static/js/basic.dropdown.min.js",
    )?;

    StaticFilesBundle::from_dir("./static", None)
        .with_name("assets")
        .build()
}
