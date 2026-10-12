use crate::core::action::publish_actions;
use crate::core::extension::ExtensionRef;
use crate::core::theme::ThemeRef;
use crate::core::theme::all::register_theme;
use crate::web::Router;
use crate::{BoxError, config, global, serve_static_files, trace, web};

use std::sync::OnceLock;

static EXTENSIONS: OnceLock<Vec<ExtensionRef>> = OnceLock::new();

// **< REGISTRO DE LAS EXTENSIONES >****************************************************************

pub fn register_extensions(root_extension: Option<ExtensionRef>) {
    // Garantiza que ocurre sólo una vez cuando los tests se ejecutan en paralelo.
    EXTENSIONS.get_or_init(|| {
        let mut list: Vec<ExtensionRef> = Vec::new();

        // Primero añade el tema básico a la lista de extensiones habilitadas.
        add_to_enabled(&mut list, &crate::base::theme::Basic);

        // Si se proporciona la extensión raíz inicial, se añade a las extensiones habilitadas.
        if let Some(extension) = root_extension {
            add_to_enabled(&mut list, extension);
        }

        // Añade la página de bienvenida si no hay extensión raíz.
        if root_extension.is_none() {
            add_to_enabled(&mut list, &crate::base::extension::Welcome);
        }

        list
    });
}

fn add_to_enabled(list: &mut Vec<ExtensionRef>, extension: ExtensionRef) {
    // Verifica que la extensión no esté en la lista para evitar duplicados.
    if !list.iter().any(|e| e.type_id() == extension.type_id()) {
        // Añade primero (en orden inverso) las dependencias de la extensión.
        for d in extension.dependencies().into_iter().rev() {
            add_to_enabled(list, d);
        }

        // Añade la propia extensión a la lista.
        list.push(extension);

        // Comprueba si la extensión tiene un tema asociado que deba registrarse.
        if let Some(theme) = extension.theme() {
            check_theme_parent_chain(theme);
            if register_theme(theme) {
                trace::debug!("Enabling \"{}\" theme", theme.short_name());
            }
        } else {
            trace::debug!("Enabling \"{}\" extension", extension.short_name());
        }
    }
}

// Recorre la cadena de `Theme::parent()` para detectar referencias circulares. `parent()` se
// resuelve en tiempo de ejecución, así que un ciclo no puede descartarse al compilar. Se rechaza el
// arranque si detecta uno, antes de provocar un bucle infinito en el `ComponentRender::render()` o
// un desbordamiento de pila en los métodos predefinidos de `Theme` que delegan recursivamente en el
// tema padre.
fn check_theme_parent_chain(theme: ThemeRef) {
    let mut chain: Vec<ThemeRef> = vec![theme];
    let mut current = theme;
    while let Some(parent) = current.parent() {
        if let Some(pos) = chain.iter().position(|t| t.type_id() == parent.type_id()) {
            let cycle: Vec<&str> = chain[pos..]
                .iter()
                .map(|t| t.short_name())
                .chain(std::iter::once(parent.short_name()))
                .collect();
            panic!(
                "Circular theme parent chain detected: {}",
                cycle.join(" -> ")
            );
        }
        chain.push(parent);
        current = parent;
    }
}

// **< REGISTRO DE LAS ACCIONES >*******************************************************************

pub fn register_actions() {
    publish_actions(|| {
        EXTENSIONS
            .get()
            .into_iter()
            .flatten()
            .flat_map(|extension| extension.actions())
            .collect()
    });
}

// **< CARGA LOS AJUSTES >**************************************************************************

/// Carga los ajustes de configuración de todas las extensiones, en el orden en que se registraron.
///
/// Avisa además de las secciones de los archivos de configuración que no corresponden a PageTop ni
/// a ninguna extensión, que suelen ser erratas.
///
/// # Panics
///
/// Si dos extensiones, o una extensión y PageTop, reclaman la misma sección: cada una la leería con
/// su propia estructura de ajustes.
pub fn load_settings() {
    trace::info!("Loading settings for all extensions");
    let mut known: Vec<&str> = global::SECTIONS.to_vec();
    for settings in EXTENSIONS
        .get()
        .into_iter()
        .flatten()
        .filter_map(|e| e.settings())
    {
        let section = settings.section();
        assert!(
            !known.contains(&section),
            "Settings section [{section}] is claimed by more than one extension"
        );
        settings.load();
        known.push(section);
    }
    for section in config::sections() {
        if !known.contains(&section.as_str()) {
            trace::warn!("Unknown settings section [{section}], ignored. Check settings");
        }
    }
}

// **< INICIALIZA LAS EXTENSIONES >*****************************************************************

/// Inicializa las extensiones en el orden en que se registraron.
///
/// Se detiene en la primera que falle y devuelve su nombre corto junto con el error.
pub async fn initialize_extensions() -> Result<(), (&'static str, BoxError)> {
    trace::info!("Calling application bootstrap");
    for e in EXTENSIONS.get().into_iter().flatten() {
        e.initialize().await.map_err(|err| (e.short_name(), err))?;
    }
    Ok(())
}

// **< CONFIGURA LAS RUTAS >************************************************************************

pub fn configure_routes(router: Router) -> Router {
    // Sólo compila durante el desarrollo, para evitar errores 400 en la traza de eventos.
    #[cfg(debug_assertions)]
    let router = router.route(
        "/.well-known/appspecific/com.chrome.devtools.json",
        web::get(|| async { web::http::StatusCode::NOT_FOUND }),
    );

    EXTENSIONS
        .get()
        .into_iter()
        .flatten()
        .fold(router, |r, e| e.configure_router(r))
}

// **< CONFIGURA EL MIDDLEWARE GLOBAL >*************************************************************

/// Aplica las capas de middleware globales de todas las extensiones sobre el router ya enrutado.
///
/// Se llama después de [`configure_routes`] para que las capas envuelvan todas sus rutas,
/// independientemente del orden de las extensiones, y antes de [`configure_static_files`], cuyas
/// rutas quedan fuera.
pub fn configure_middleware(router: Router) -> Router {
    EXTENSIONS
        .get()
        .into_iter()
        .flatten()
        .fold(router, |r, e| e.configure_middleware(r))
}

// **< CONFIGURA LOS ARCHIVOS ESTÁTICOS >***********************************************************

/// Añade los archivos estáticos de PageTop y de todas las extensiones.
///
/// Se llama después de [`configure_middleware`], para que estas rutas no pasen por las capas de
/// middleware de las extensiones.
pub fn configure_static_files(router: Router) -> Router {
    let router = EXTENSIONS
        .get()
        .into_iter()
        .flatten()
        .fold(router, |r, e| e.configure_static_files(r));

    serve_static_files!(router, [&global::SETTINGS.dev.pagetop_static_dir, assets] => "/pagetop");

    router
}
