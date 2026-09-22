//! Script de compilacion de activos estaticos.
//!
//! Genera el directorio `static/` a partir de `assets/` y embebe su contenido en el binario:
//!
//! - `static/css/` - CSS compilado a partir de los archivos SCSS de `assets/`.
//! - `static/js/` - JS copiado o minificado desde `assets/`.
//! - `static/fonts/` - Fuentes copiadas desde `assets/`.
//!
//! Los archivos `.map` se copian a `static/js/` para uso en desarrollo pero no se incluyen en el
//! binario embebido.

use pagetop_build::{StaticFilesBundle, compile_scss, copy_file, minify_js};

use std::path::Path;

fn main() -> std::io::Result<()> {
    // Regenera `static/` desde cero sólo si hay cambios en `assets/`.
    println!("cargo:rerun-if-changed=assets");
    let _ = std::fs::remove_dir_all("static");

    // CSS: Bootstrap 5.3.8 + Bootstrap Icons 1.13.1.
    compile_scss("assets/bootsier.scss", "static/css/bootsier.min.css")?;

    // JS: Bootstrap bundle.
    copy_file(
        "assets/bootstrap-5.3.8/js/bootstrap.bundle.min.js",
        "static/js/bootsier.bundle.min.js",
    )?;
    copy_file(
        "assets/bootstrap-5.3.8/js/bootstrap.bundle.min.js.map",
        "static/js/bootsier.bundle.min.js.map",
    )?;
    // JS: selector de modo de color (claro / oscuro / automático).
    minify_js(
        "assets/bootsier.theme.js",
        "static/js/bootsier.theme.min.js",
    )?;
    // JS: fix de apilamiento para Dialog.
    minify_js(
        "assets/bootsier.dialog.js",
        "static/js/bootsier.dialog.min.js",
    )?;
    // JS: sustituye el `confirm()` nativo de htmx por un modal de Bootstrap.
    minify_js(
        "assets/bootsier.confirm.js",
        "static/js/bootsier.confirm.min.js",
    )?;

    // Fuentes: Bootstrap Icons.
    copy_file(
        "assets/bootstrap-icons-1.13.1/fonts/bootstrap-icons.woff2",
        "static/fonts/bootsier.icons.woff2",
    )?;
    copy_file(
        "assets/bootstrap-icons-1.13.1/fonts/bootstrap-icons.woff",
        "static/fonts/bootsier.icons.woff",
    )?;
    // Fuentes: Source Sans 3 (SIL OFL 1.1).
    copy_file(
        "assets/fonts/SourceSans3VF-Upright.otf.woff2",
        "static/fonts/bootsier.font.woff2",
    )?;
    copy_file(
        "assets/fonts/SourceSans3VF-Italic.otf.woff2",
        "static/fonts/bootsier.font.italic.woff2",
    )?;

    // Preparación de los paquetes para embeber en el binario.
    StaticFilesBundle::from_dir("./static/css", None)
        .with_name("bootsier_css")
        .build()?;

    StaticFilesBundle::from_dir("./static/js", Some(only_js_files))
        .with_name("bootsier_js")
        .build()?;

    StaticFilesBundle::from_dir("./static/fonts", None)
        .with_name("bootsier_fonts")
        .build()
}

// Los archivos .map no se embeben en el binario; solo se sirven desde disco en desarrollo.
fn only_js_files(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "js")
}
