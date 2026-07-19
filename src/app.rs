//! Prepara y ejecuta una aplicación creada con PageTop.

mod figfont;

use crate::core::{extension, extension::ExtensionRef};
use crate::locale::Locale;
use crate::response::{render_error_pages, response_for_panic, route_not_found};
use crate::web::Router;
use crate::{PAGETOP_VERSION, global, trace};

use tower_http::catch_panic::CatchPanicLayer;

use std::io::Error;
use std::sync::LazyLock;

/// Punto de entrada de una aplicación PageTop.
///
/// Orquesta el arranque de la aplicación. Primero se instancia con [`Application::new()`] o
/// [`Application::prepare()`], y después se ejecuta usando [`run()`](Application::run). Si se está
/// preparando un entorno de pruebas, se usa [`test()`](Application::test).
///
/// Los **errores controlados** (403, 404, o un fallo que un handler devuelva explícitamente como
/// [`ErrorPage`](crate::response::ErrorPage)) se renderizan usando el tema activo (ver
/// [`Theme::error_403()`](crate::core::theme::Theme::error_403),
/// [`Theme::error_404()`](crate::core::theme::Theme::error_404) y
/// [`Theme::error_fatal()`](crate::core::theme::Theme::error_fatal)).
///
/// La última capa del router captura cualquier **fallo catastrófico** (`panic!`) de la aplicación
/// en lugar de abortar la conexión. Devuelve una respuesta mínima HTTP 500 que es independiente del
/// tema y del ciclo de renderizado de componentes.
pub struct Application;

impl Application {
    /// Crea una instancia mínima de la aplicación, sin extensión raíz.
    ///
    /// Útil para verificar que el servidor arranca correctamente. Para una aplicación real, usa
    /// [`prepare()`](Application::prepare) con una extensión raíz.
    pub async fn new() -> Self {
        Self::internal_prepare(None).await
    }

    /// Prepara una instancia de la aplicación a partir de una extensión raíz.
    ///
    /// Inicializa la aplicación habilitando las extensiones en orden de dependencia: primero las
    /// que no dependen de ninguna otra, luego las que dependen de extensiones ya habilitadas, y así
    /// hasta habilitar la extensión raíz.
    ///
    /// Es `async` porque cada extensión puede realizar operaciones asíncronas en su
    /// [`initialize()`](crate::core::extension::Extension::initialize) (conexión a base de datos,
    /// migraciones, semillas de datos...).
    pub async fn prepare(root_extension: ExtensionRef) -> Self {
        Self::internal_prepare(Some(root_extension)).await
    }

    // Secuencia de arranque común a new() y prepare().
    async fn internal_prepare(root_extension: Option<ExtensionRef>) -> Self {
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
        extension::all::initialize_extensions().await;

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

            // Versión de PageTop.
            println!(
                "{} {}\n",
                "Powered by PageTop".yellow(),
                PAGETOP_VERSION.yellow()
            );
        }
    }

    // Construye el router con las rutas y el middleware de todas las extensiones habilitadas.
    //
    // Con `CatchPanicLayer` en la última capa se capturan incluso los `panic!` que se produzcan
    // dentro del propio renderizado de una página de error.
    fn build_router() -> Router {
        let router = extension::all::configure_routes(Router::new());
        let router = extension::all::configure_middleware(router);
        router
            .fallback(route_not_found)
            .layer(axum::middleware::from_fn(render_error_pages))
            .layer(CatchPanicLayer::custom(response_for_panic))
    }

    /// Arranca el servidor web de la aplicación.
    ///
    /// Enlaza el puerto del servidor web (puede fallar con [`std::io::Error`] si el puerto ya está
    /// en uso o el proceso carece de permisos) y ejecuta el bucle de atención de peticiones. El
    /// patrón habitual es:
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    ///
    /// struct MyApp;
    ///
    /// #[async_trait]
    /// impl Extension for MyApp {}
    ///
    /// #[pagetop::main]
    /// async fn main() -> std::io::Result<()> {
    ///     Application::prepare(&MyApp).await.run().await
    /// }
    /// ```
    pub async fn run(self) -> Result<(), Error> {
        let addr = format!(
            "{}:{}",
            global::SETTINGS.server.bind_address,
            global::SETTINGS.server.bind_port
        );

        // Enlaza el puerto de forma síncrona para detectar errores.
        let std_listener = std::net::TcpListener::bind(&addr)?;
        std_listener.set_nonblocking(true)?;

        let router = Self::build_router();

        let listener = tokio::net::TcpListener::from_std(std_listener)?;
        axum::serve(listener, router).await
    }

    /// Devuelve el servidor web configurado para usarlo en pruebas de integración.
    pub fn test(self) -> Router {
        Self::build_router()
    }
}
