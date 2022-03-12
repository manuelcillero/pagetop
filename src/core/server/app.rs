use crate::{Lazy, base, db, locale, trace};
use crate::config::SETTINGS;
use crate::core::{Server, global, server};
use crate::core::theme::register_theme;
use crate::core::module::register_module;

use std::io::Error;
use std::sync::RwLock;
use actix_web::middleware::normalize::{NormalizePath, TrailingSlash};

#[cfg(feature = "mysql")]
use sqlx::mysql::MySqlPoolOptions as DbPoolOptions;

#[cfg(feature = "postgres")]
use sqlx::postgres::PgPoolOptions as DbPoolOptions;

static DBCONN: Lazy<RwLock<Option<db::Conn>>> = Lazy::new(|| {
    RwLock::new(None)
});

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn build(bootstrap: Option<fn()>) -> Result<Self, Error> {
        // Imprime rótulo (opcional) de bienvenida.
        if SETTINGS.app.startup_banner.to_lowercase() != "off" {
            let figfont = figlet_rs::FIGfont::from_content(
                match SETTINGS.app.startup_banner.to_lowercase().as_str() {
                    "slant"    => include_str!("figfonts/slant.flf"),
                    "small"    => include_str!("figfonts/small.flf"),
                    "speed"    => include_str!("figfonts/speed.flf"),
                    "starwars" => include_str!("figfonts/starwars.flf"),
                    _ => {
                        println!(
                            "FIGfont \"{}\" not found for banner. {}. {}.",
                            SETTINGS.app.startup_banner,
                            "Using \"Small\"",
                            "Check the settings file",
                        );
                        include_str!("figfonts/small.flf")
                    }
                }
            ).unwrap();
            println!("\n{} {}\n\n Powered by PageTop {}\n",
                figfont.convert(&SETTINGS.app.name).unwrap(),
                &SETTINGS.app.description,
                env!("CARGO_PKG_VERSION")
            );
        }

        // Inicia registro de trazas y eventos.
        Lazy::force(&server::tracing::TRACING);

        // Valida el identificador de idioma.
        Lazy::force(&locale::LANGID);

        // Inicializa la conexión con la base de datos.
        trace::info!(
            "Connecting to database \"{}\" with a pool of {} connections.",
            &SETTINGS.database.db_name,
            &SETTINGS.database.max_pool_size
        );

        #[cfg(feature = "mysql")]
        let db_type = "mysql";

        #[cfg(feature = "postgres")]
        let db_type = "postgres";

        // https://github.com/launchbadge/sqlx/issues/1624
        let mut db_uri = db::Uri::parse(format!(
            "{}://{}/{}",
            db_type,
            &SETTINGS.database.db_host,
            &SETTINGS.database.db_name
        ).as_str()).unwrap();
        db_uri.set_username(&SETTINGS.database.db_user.as_str()).unwrap();
        db_uri.set_password(Some(&SETTINGS.database.db_pass.as_str())).unwrap();
        if SETTINGS.database.db_port != 0 {
            db_uri.set_port(Some(SETTINGS.database.db_port)).unwrap();
        }

        let db_pool = DbPoolOptions::new()
            .max_connections(SETTINGS.database.max_pool_size)
            .connect(db_uri.as_str())
            .await
            .expect("Failed to connect to database");

        let mut dbconn = DBCONN.write().unwrap();
        *dbconn = Some(db_pool);

        // Registra los temas predefinidos.
        register_theme(&base::theme::aliner::AlinerTheme);
        register_theme(&base::theme::minimal::MinimalTheme);
        register_theme(&base::theme::bootsier::BootsierTheme);

        // Registra los módulos predeterminados.
        register_module(&base::module::admin::AdminModule);
        register_module(&base::module::user::UserModule);

        // Ejecuta la función de inicio de la aplicación.
        if bootstrap != None {
            trace::info!("Calling application bootstrap.");
            let _ = &(bootstrap.unwrap())();
        }

        // Registra el módulo para la página de inicio de PageTop.
        // Al ser el último, puede sobrecargarse con la función de inicio.
        register_module(&base::module::homepage::HomepageModule);

        // Run migrations.
        trace::info!("Running migrations.");
        global::migrations(db_uri);

        // Prepara el servidor web.
        let server = server::HttpServer::new(|| {
            server::App::new()
                .wrap(tracing_actix_web::TracingLogger)
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .configure(&global::themes)
                .configure(&global::modules)
            })
            .bind(format!("{}:{}",
                &SETTINGS.webserver.bind_address,
                &SETTINGS.webserver.bind_port
            ))?
            .run();

        Ok(Self { server })
    }

    pub fn run(self) -> Result<Server, Error> {
        Ok(self.server)
    }
}
