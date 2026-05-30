//! Prepara y ejecuta una aplicación creada con PageTop.

mod figfont;

use crate::core::{extension, extension::ExtensionRef};
use crate::html::Markup;
use crate::locale::Locale;
use crate::response::page::{ErrorPage, ResultPage};
use crate::web::{HttpRequest, Router};
use crate::{PAGETOP_VERSION, global, trace};

use std::future::Future;
use std::io::Error;
use std::sync::LazyLock;

/// Punto de entrada de una aplicación PageTop.
///
/// No almacena datos, **encapsula** el inicio completo de la configuración y puesta en marcha de la
/// aplicación. Para instanciarla se puede usar [`new()`](Application::new) o
/// [`prepare()`](Application::prepare). Después sólo hay que llamar a [`run()`](Application::run)
/// para ejecutar la aplicación (o a [`test()`](Application::test) si se está preparando un entorno
/// de pruebas).
pub struct Application;

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    /// Crea una instancia mínima de la aplicación, sin extensión raíz.
    ///
    /// Útil para verificar que el servidor arranca correctamente. Para una aplicación real, usa
    /// [`prepare()`](Application::prepare) con una extensión raíz.
    pub fn new() -> Self {
        Self::internal_prepare(None)
    }

    /// Prepara una instancia de la aplicación a partir de una extensión raíz.
    ///
    /// Las dependencias se habilitan en orden: primero las que no dependen de ninguna otra, luego
    /// las que dependen de extensiones ya habilitadas, y así sucesivamente hasta dejar habilitada
    /// la extensión raíz.
    pub fn prepare(root_extension: ExtensionRef) -> Self {
        Self::internal_prepare(Some(root_extension))
    }

    // Secuencia de arranque común a new() y prepare().
    fn internal_prepare(root_extension: Option<ExtensionRef>) -> Self {
        // Al arrancar muestra una cabecera para la aplicación.
        Self::show_banner();

        // Inicia gestión de trazas y registro de eventos (logging).
        LazyLock::force(&trace::TRACING);

        // Inicializa el idioma predeterminado.
        Locale::init();

        // Registra las extensiones de la aplicación.
        extension::all::register_extensions(root_extension);

        // Registra las acciones de las extensiones.
        extension::all::register_actions();

        // Inicializa las extensiones.
        extension::all::initialize_extensions();

        Self
    }

    // Muestra la cabecera de arranque si está habilitada en la configuración.
    fn show_banner() {
        use colored::Colorize;
        use terminal_size::{Width, terminal_size};

        if global::SETTINGS.app.startup_banner != global::StartupBanner::Off {
            // Nombre de la aplicación, ajustado al ancho del terminal si es necesario.
            let mut app_ff = String::new();
            let app_name = &global::SETTINGS.app.name;
            if let Some((Width(term_width), _)) = terminal_size() {
                if term_width >= 80 {
                    let maxlen: usize = ((term_width / 10) - 2).into();
                    let mut app: String = app_name.chars().take(maxlen).collect();
                    if app_name.chars().count() > maxlen {
                        app = format!("{app}...");
                    }
                    if let Some(ff) = figfont::FIGFONT.convert(&app) {
                        app_ff = ff.to_string();
                    }
                }
            }
            if app_ff.is_empty() {
                println!("\n{app_name}");
            } else {
                print!("\n{app_ff}");
            }

            // Descripción de la aplicación.
            if !global::SETTINGS.app.description.is_empty() {
                println!("{}", global::SETTINGS.app.description.cyan());
            }

            // Versión de PageTop.
            println!(
                "{} {}\n",
                "Powered by PageTop".yellow(),
                PAGETOP_VERSION.yellow()
            );
        }
    }

    // Construye el router con las rutas de todas las extensiones habilitadas.
    fn build_router() -> Router {
        let router = extension::all::configure_routes(Router::new());
        router.fallback(route_not_found)
    }

    /// Arranca el servidor web de la aplicación.
    ///
    /// Enlaza el puerto del servidor web de forma síncrona (puede fallar con [`std::io::Error`] si
    /// el puerto ya está en uso o el proceso carece de permisos) y devuelve un [`Future`] que
    /// ejecuta el bucle de atención de peticiones. El patrón habitual es:
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    ///
    /// struct MyApp;
    ///
    /// impl Extension for MyApp {}
    ///
    /// #[pagetop::main]
    /// async fn main() -> std::io::Result<()> {
    ///     Application::prepare(&MyApp).run()?.await
    /// }
    /// ```
    pub fn run(self) -> Result<impl Future<Output = Result<(), Error>>, Error> {
        let addr = format!(
            "{}:{}",
            global::SETTINGS.server.bind_address,
            global::SETTINGS.server.bind_port
        );

        // Enlaza el puerto de forma síncrona para detectar errores antes del *await*.
        let std_listener = std::net::TcpListener::bind(&addr)?;
        std_listener.set_nonblocking(true)?;

        let router = Self::build_router();

        Ok(async move {
            let listener = tokio::net::TcpListener::from_std(std_listener)?;
            axum::serve(listener, router).await
        })
    }

    /// Devuelve el servidor web configurado para usarlo en pruebas de integración.
    pub fn test(self) -> Router {
        Self::build_router()
    }
}

async fn route_not_found(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Err(ErrorPage::NotFound(request))
}
