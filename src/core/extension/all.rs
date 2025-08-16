use crate::core::action::add_action;
use crate::core::extension::ExtensionRef;
use crate::core::theme::all::THEMES;
use crate::{global, service, static_files_service, trace};

use parking_lot::RwLock;

use std::sync::LazyLock;

// EXTENSIONES *************************************************************************************

static ENABLED_EXTENSIONS: LazyLock<RwLock<Vec<ExtensionRef>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

static DROPPED_EXTENSIONS: LazyLock<RwLock<Vec<ExtensionRef>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

// REGISTRO DE LAS EXTENSIONES *********************************************************************

pub fn register_extensions(root_extension: Option<ExtensionRef>) {
    // Prepara la lista de extensiones habilitadas.
    let mut enabled_list: Vec<ExtensionRef> = Vec::new();

    // Primero añade el tema básico a la lista de extensiones habilitadas.
    add_to_enabled(&mut enabled_list, &crate::base::theme::Basic);

    // Si se proporciona una extensión raíz inicial, se añade a la lista de extensiones habilitadas.
    if let Some(extension) = root_extension {
        add_to_enabled(&mut enabled_list, extension);
    }

    // Añade la página de bienvenida por defecto a la lista de extensiones habilitadas.
    add_to_enabled(&mut enabled_list, &crate::base::extension::Welcome);

    // Guarda la lista final de extensiones habilitadas.
    ENABLED_EXTENSIONS.write().append(&mut enabled_list);

    // Prepara una lista de extensiones deshabilitadas.
    let mut dropped_list: Vec<ExtensionRef> = Vec::new();

    // Si se proporciona una extensión raíz, analiza su lista de dependencias.
    if let Some(extension) = root_extension {
        add_to_dropped(&mut dropped_list, extension);
    }

    // Guarda la lista final de extensiones deshabilitadas.
    DROPPED_EXTENSIONS.write().append(&mut dropped_list);
}

fn add_to_enabled(list: &mut Vec<ExtensionRef>, extension: ExtensionRef) {
    // Verifica que la extensión no esté en la lista para evitar duplicados.
    if !list.iter().any(|e| e.type_id() == extension.type_id()) {
        // Añade primero (en orden inverso) las dependencias de la extensión.
        for d in extension.dependencies().iter().rev() {
            add_to_enabled(list, *d);
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

fn add_to_dropped(list: &mut Vec<ExtensionRef>, extension: ExtensionRef) {
    // Recorre las extensiones que la actual recomienda deshabilitar.
    for d in &extension.drop_extensions() {
        // Verifica que la extensión no esté ya en la lista.
        if !list.iter().any(|e| e.type_id() == d.type_id()) {
            // Comprueba si la extensión está habilitada. Si es así, registra una advertencia.
            if ENABLED_EXTENSIONS
                .read()
                .iter()
                .any(|e| e.type_id() == extension.type_id())
            {
                trace::warn!(
                    "Trying to drop \"{}\" extension which is enabled",
                    extension.short_name()
                );
            } else {
                // Si la extensión no está habilitada, se añade a la lista y registra la acción.
                list.push(*d);
                trace::debug!("Extension \"{}\" dropped", d.short_name());
                // Añade recursivamente las dependencias de la extensión eliminada.
                // De este modo, todas las dependencias se tienen en cuenta para ser deshabilitadas.
                for dependency in &extension.dependencies() {
                    add_to_dropped(list, *dependency);
                }
            }
        }
    }
}

// REGISTRO DE LAS ACCIONES ************************************************************************

pub fn register_actions() {
    for extension in ENABLED_EXTENSIONS.read().iter() {
        for a in extension.actions().into_iter() {
            add_action(a);
        }
    }
}

// INICIALIZA LAS EXTENSIONES **********************************************************************

pub fn initialize_extensions() {
    trace::info!("Calling application bootstrap");
    for extension in ENABLED_EXTENSIONS.read().iter() {
        extension.initialize();
    }
}

// CONFIGURA LOS SERVICIOS *************************************************************************

pub fn configure_services(scfg: &mut service::web::ServiceConfig) {
    // Sólo compila durante el desarrollo, para evitar errores 400 en la traza de eventos.
    #[cfg(debug_assertions)]
    scfg.route(
        // Ruta automática lanzada por Chrome DevTools.
        "/.well-known/appspecific/com.chrome.devtools.json",
        service::web::get().to(|| async { service::HttpResponse::NotFound().finish() }),
    );

    for extension in ENABLED_EXTENSIONS.read().iter() {
        extension.configure_service(scfg);
    }

    static_files_service!(scfg, [&global::SETTINGS.dev.pagetop_static_dir, assets] => "/");
}
