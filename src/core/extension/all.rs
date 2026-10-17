use crate::core::action::publish_actions;
use crate::core::extension::ExtensionRef;
use crate::core::theme::ThemeRef;
use crate::core::theme::all::register_theme;
use crate::web::Router;
use crate::{BoxError, config, global, serve_static_files, trace, web};

use std::sync::OnceLock;

static EXTENSIONS: OnceLock<Vec<ExtensionRef>> = OnceLock::new();

/// Errores de las extensiones que detienen el arranque de la aplicación.
#[derive(Debug, thiserror::Error)]
pub enum StartupError {
    /// Los archivos de configuración no se pueden leer (p. ej. un TOML mal formado).
    #[error("Failed to read the configuration files")]
    ReadConfig(#[source] BoxError),
    /// Dos extensiones, o una extensión y PageTop, reclaman la misma sección de la configuración:
    /// cada una la leería con su propia estructura de ajustes.
    #[error("Settings section [{0}] is already claimed by PageTop or another extension")]
    DuplicateSection(&'static str),
    /// Los ajustes de una sección no se pueden cargar (p. ej. por un valor no válido o una clave
    /// desconocida).
    #[error("Invalid settings in section [{section}]")]
    InvalidSettings {
        section: &'static str,
        #[source]
        source: BoxError,
    },
    /// Una extensión ha devuelto un error en su `initialize()`.
    #[error("Extension \"{name}\" failed to initialize")]
    Initialize {
        name: &'static str,
        #[source]
        source: BoxError,
    },
}

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
/// Se detiene en el primer error: una sección reclamada por más de una extensión o unos ajustes que
/// no se pueden cargar. Si todo va bien, avisa de las secciones de los archivos de configuración
/// que no corresponden a PageTop ni a ninguna extensión, que suelen ser erratas.
pub fn load_settings() -> Result<(), StartupError> {
    load_settings_of(EXTENSIONS.get().map(Vec::as_slice).unwrap_or_default())
}

// Separada de `load_settings()` para poder probarla sin el registro global de extensiones.
fn load_settings_of(extensions: &[ExtensionRef]) -> Result<(), StartupError> {
    trace::info!("Loading settings for all extensions");
    let mut known: Vec<&str> = global::SECTIONS.to_vec();
    for settings in extensions.iter().filter_map(|e| e.settings()) {
        let section = settings.section();
        if known.contains(&section) {
            return Err(StartupError::DuplicateSection(section));
        }
        settings
            .load()
            .map_err(|source| StartupError::InvalidSettings { section, source })?;
        known.push(section);
    }
    for section in config::sections() {
        if !known.contains(&section.as_str()) {
            trace::warn!("Unknown settings section [{section}], ignored. Check settings");
        }
    }
    Ok(())
}

// **< INICIALIZA LAS EXTENSIONES >*****************************************************************

/// Inicializa las extensiones en el orden en que se registraron.
///
/// Se detiene en la primera que falle.
pub async fn initialize_extensions() -> Result<(), StartupError> {
    trace::info!("Calling application bootstrap");
    for e in EXTENSIONS.get().into_iter().flatten() {
        e.initialize()
            .await
            .map_err(|source| StartupError::Initialize {
                name: e.short_name(),
                source,
            })?;
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

#[cfg(test)]
mod tests {
    use super::{StartupError, load_settings_of};
    use crate::config::LoadSettings;
    use crate::core::extension::Extension;
    use crate::{BoxError, async_trait};

    // Settings for a given section that load fine or fail, without touching the configuration.
    struct FakeSettings {
        section: &'static str,
        fails: bool,
    }

    impl LoadSettings for FakeSettings {
        fn load(&self) -> Result<(), BoxError> {
            if self.fails {
                Err("bad value".into())
            } else {
                Ok(())
            }
        }

        fn section(&self) -> &'static str {
            self.section
        }
    }

    struct Claims(&'static FakeSettings);

    #[async_trait]
    impl Extension for Claims {
        fn settings(&self) -> Option<&'static dyn LoadSettings> {
            Some(self.0)
        }
    }

    static APP: Claims = Claims(&FakeSettings {
        section: "app",
        fails: false,
    });
    static FIRST: Claims = Claims(&FakeSettings {
        section: "fake",
        fails: false,
    });
    static SECOND: Claims = Claims(&FakeSettings {
        section: "fake",
        fails: false,
    });
    static BROKEN: Claims = Claims(&FakeSettings {
        section: "broken",
        fails: true,
    });

    #[test]
    fn valid_settings_load() {
        assert!(load_settings_of(&[&FIRST]).is_ok());
    }

    #[test]
    fn a_section_of_pagetop_cannot_be_claimed() {
        let err = load_settings_of(&[&APP]).unwrap_err();
        assert!(matches!(err, StartupError::DuplicateSection("app")));
    }

    #[test]
    fn a_section_cannot_be_claimed_twice() {
        let err = load_settings_of(&[&FIRST, &SECOND]).unwrap_err();
        assert!(matches!(err, StartupError::DuplicateSection("fake")));
    }

    #[test]
    fn invalid_settings_stop_with_their_section_and_cause() {
        let err = load_settings_of(&[&FIRST, &BROKEN]).unwrap_err();
        assert!(matches!(
            err,
            StartupError::InvalidSettings {
                section: "broken",
                ..
            }
        ));
        assert_eq!(err.to_string(), "Invalid settings in section [broken]");
        assert_eq!(
            std::error::Error::source(&err).unwrap().to_string(),
            "bad value"
        );
    }
}
