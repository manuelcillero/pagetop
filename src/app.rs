//! Prepara y ejecuta una aplicación creada con PageTop.

mod figfont;

use crate::core::{extension, extension::ExtensionRef, theme};
use crate::datetime::Timezone;
use crate::locale::Locale;
use crate::response::{render_error_pages, response_for_panic, route_not_found};
use crate::web::Router;
use crate::{PAGETOP_VERSION, global, trace};

use tower_http::catch_panic::CatchPanicLayer;
use tracing_appender::non_blocking::WorkerGuard;

use std::io::Error;

/// Punto de entrada de una aplicación PageTop.
///
/// Orquesta el arranque de la aplicación. Primero se instancia con [`Application::new()`] o
/// [`Application::prepare()`], y después se ejecuta usando [`run()`](Application::run). Si se está
/// preparando un entorno de pruebas, se usa [`test()`](Application::test).
///
/// Hay **una aplicación por proceso**: la configuración, las extensiones habilitadas, el tema
/// predeterminado y las trazas se inician una sola vez, con la primera que se prepara. Preparar más
/// en el mismo proceso, como hacen las pruebas de integración para obtener su router, reutilizaría
/// todos estos recursos.
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
pub struct Application {
    // Vuelca las trazas pendientes al destruirse, cuando termina `run()` (ver `trace::init()`).
    tracing: Option<WorkerGuard>,
}

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
        let tracing = trace::init();

        // Inicializa el idioma predeterminado.
        Locale::init();

        // Inicializa la zona horaria predeterminada y las regiones para elegir.
        Timezone::init();

        // Registra las extensiones de la aplicación.
        extension::all::register_extensions(root_extension);

        // Carga los ajustes de las extensiones. Un error de configuración detiene aquí el arranque.
        extension::all::load_settings();

        // Inicializa el tema predeterminado, que ya puede ser el de una extensión.
        theme::all::init_default_theme();

        // Registra las acciones de las extensiones.
        extension::all::register_actions();

        // Inicializa las extensiones.
        extension::all::initialize_extensions().await;

        Self { tracing }
    }

    // Muestra la cabecera de arranque si está habilitada en la configuración.
    fn show_banner() {
        use colored::Colorize;
        use terminal_size::{Width, terminal_size};

        if global::SETTINGS.app.startup_banner != global::StartupBanner::Off {
            // Nombre de la aplicación, ajustado al ancho del terminal si es necesario.
            let mut app_ff = String::new();
            let app_name = &global::SETTINGS.app.name;
            if let Some((Width(term_width), _)) = terminal_size()
                && term_width >= 80
            {
                let maxlen: usize = ((term_width / 10) - 2).into();
                let mut app: String = app_name.chars().take(maxlen).collect();
                if app_name.chars().count() > maxlen {
                    app = format!("{app}...");
                }
                if let Some(ff) = figfont::FIGFONT.convert(&app) {
                    app_ff = ff.to_string();
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

    // Construye el router con las rutas, el middleware y los archivos estáticos de todas las
    // extensiones habilitadas.
    //
    // Las capas del servidor web sólo envuelven las rutas (y el `fallback`) registradas antes de
    // añadirlas, así que los archivos estáticos se registran después del middleware de las
    // extensiones para no pasar por él. Una página con muchos recursos repetiría, si no, el mismo
    // trabajo (resolver la sesión, etc.) por cada uno. El `fallback` de rutas inexistentes, en
    // cambio, se registra antes: la página 404 debe ver lo que el middleware añade a la petición
    // (p. ej. el usuario actual).
    //
    // Las capas de páginas de error y `CatchPanicLayer` cubren todo, porque se añaden al final. Con
    // `CatchPanicLayer` en la última capa se capturan incluso los `panic!` que se produzcan dentro
    // del propio renderizado de una página de error.
    fn build_router() -> Router {
        let router = extension::all::configure_routes(Router::new()).fallback(route_not_found);
        let router = extension::all::configure_middleware(router);
        let router = extension::all::configure_static_files(router);
        router
            .layer(axum::middleware::from_fn(render_error_pages))
            .layer(CatchPanicLayer::custom(response_for_panic))
    }

    /// Arranca el servidor web de la aplicación.
    ///
    /// Enlaza el puerto del servidor web (puede fallar con [`std::io::Error`] si el puerto ya está
    /// en uso o el proceso carece de permisos) y ejecuta el bucle de atención de peticiones.
    ///
    /// Se detiene de forma ordenada con Ctrl+C o, en Unix, con la señal `SIGTERM` (la que envían
    /// `systemd` o Docker al detener un servicio). En ese momento deja de aceptar conexiones,
    /// termina las peticiones en curso y vuelca las trazas pendientes antes de volver. Una segunda
    /// Ctrl+C lo detiene en el acto, sin esperar a las peticiones en curso.
    ///
    /// El patrón habitual es:
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
        let result = axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal())
            .await;

        // Detenido el servidor, vuelca las trazas pendientes.
        drop(self.tracing);
        result
    }

    /// Devuelve el servidor web configurado para usarlo en pruebas de integración.
    pub fn test(self) -> Router {
        Self::build_router()
    }
}

// Espera la señal para detener el servidor: Ctrl+C o, en Unix, `SIGTERM`. Si no se puede escuchar
// alguna, espera sólo a la otra en lugar de detener el servidor nada más arrancar.
async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            trace::error!("Cannot listen for Ctrl+C: {err}");
            std::future::pending::<()>().await;
        }
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        match signal(SignalKind::terminate()) {
            Ok(mut sigterm) => {
                sigterm.recv().await;
            }
            Err(err) => {
                trace::error!("Cannot listen for SIGTERM: {err}");
                std::future::pending::<()>().await;
            }
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {}
        () = terminate => {}
    }
    trace::info!("Shutting down, finishing the requests in progress");

    // Una vez capturada, Ctrl+C ya no detiene el proceso por sí sola: una segunda lo detiene en el
    // acto si alguna petición en curso no termina. El aviso va directamente a la salida de errores
    // porque `process::exit()` no deja volcar las trazas.
    tokio::spawn(async {
        if tokio::signal::ctrl_c().await.is_ok() {
            eprintln!("\nSecond Ctrl+C, stopping without waiting for the requests in progress");
            std::process::exit(130);
        }
    });
}
