use crate::core::action::add_action;
use crate::core::extension::ExtensionRef;
use crate::core::theme::all::THEMES;
use crate::web::Router;
use crate::{global, serve_static_files, trace, web};

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
            let mut registered_themes = THEMES.write();
            // Asegura que el tema no esté ya registrado para evitar duplicados.
            if !registered_themes
                .iter()
                .any(|t| t.type_id() == theme.type_id())
            {
                registered_themes.push(theme);
                trace::debug!("Enabling \"{}\" theme", theme.short_name());
            }
        } else {
            trace::debug!("Enabling \"{}\" extension", extension.short_name());
        }
    }
}

// **< REGISTRO DE LAS ACCIONES >*******************************************************************

pub fn register_actions() {
    for extension in EXTENSIONS.get().into_iter().flatten() {
        for a in extension.actions() {
            add_action(a);
        }
    }
}

// **< INICIALIZA LAS EXTENSIONES >*****************************************************************

pub fn initialize_extensions() {
    trace::info!("Calling application bootstrap");
    for e in EXTENSIONS.get().into_iter().flatten() {
        e.initialize();
    }
}

// **< CONFIGURA LAS RUTAS >************************************************************************

pub fn configure_routes(router: Router) -> Router {
    // Sólo compila durante el desarrollo, para evitar errores 400 en la traza de eventos.
    #[cfg(debug_assertions)]
    let router = router.route(
        "/.well-known/appspecific/com.chrome.devtools.json",
        web::get(|| async { web::http::StatusCode::NOT_FOUND }),
    );

    let router = EXTENSIONS
        .get()
        .into_iter()
        .flatten()
        .fold(router, |r, e| e.configure_router(r));

    serve_static_files!(router, [&global::SETTINGS.dev.pagetop_static_dir, assets] => "/pagetop");

    router
}
