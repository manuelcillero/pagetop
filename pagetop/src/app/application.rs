use super::{fatal_error::FatalError, AppTrait};
use crate::config::SETTINGS;
use crate::core::{module, theme};
use crate::html::Markup;
use crate::response::page::{Page, ResultPage};
use crate::{base, trace, Lazy};

use actix_web::dev::Server;
use std::io::Error;

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn prepare(app: impl AppTrait) -> Result<Self, Error> {
        // Rótulo de presentación.
        super::banner::print_on_startup();

        // Inicia registro de trazas y eventos.
        Lazy::force(&super::tracing::TRACING);

        // Valida el identificador de idioma.
        Lazy::force(&super::locale::LANGID);

        // Conecta con la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        Lazy::force(&super::db::DBCONN);

        // Habilita los módulos predeterminados.
        module::all::enable_modules(vec![&base::module::homepage::DefaultHomePage]);
        // Habilita los módulos de la aplicación.
        module::all::enable_modules(app.enable_modules());

        // Registra los temas predeterminados.
        theme::all::register_themes(vec![
            &base::theme::aliner::Aliner,
            &base::theme::minimal::Minimal,
            &base::theme::bootsier::Bootsier,
            &base::theme::bulmix::Bulmix,
        ]);
        // Registra los temas de la aplicación.
        theme::all::register_themes(app.themes());

        // Registra las acciones de todos los módulos.
        module::all::register_actions();

        // Ejecuta actualizaciones pendientes de la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        module::all::run_migrations();

        // Ejecuta la función de inicio de la aplicación.
        trace::info!("Calling application bootstrap");
        app.bootstrap();

        // Prepara el servidor web.
        let server = super::HttpServer::new(move || {
            super::App::new()
                .wrap(tracing_actix_web::TracingLogger::default())
                .configure(&module::all::modules)
                .configure(&theme::all::themes)
                .default_service(super::web::route().to(service_not_found))
        })
        .bind(format!(
            "{}:{}",
            &SETTINGS.webserver.bind_address, &SETTINGS.webserver.bind_port
        ))?
        .run();

        Ok(Self { server })
    }

    pub fn run(self) -> Result<Server, Error> {
        Ok(self.server)
    }
}

async fn service_not_found() -> ResultPage<Markup, FatalError> {
    let mut page = Page::new();
    let content_error = page.context().theme().error_404_not_found();
    page
        .with_title("Error RESOURCE NOT FOUND")
        .using_template("error")
        .add_to("content", content_error)
        .render()
}

async fn _access_denied() -> ResultPage<Markup, FatalError> {
    let mut page = Page::new();
    let content_error = page.context().theme().error_403_access_denied();
    page
        .with_title("Error FORBIDDEN ACCESS")
        .using_template("error")
        .add_to("content", content_error)
        .render()
}
