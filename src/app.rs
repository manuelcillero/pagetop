//! Prepara y ejecuta una aplicación creada con `Pagetop`.

use crate::service;

use std::io::Error;

pub struct Application;

impl Application {
    /// Crea una instancia de la aplicación.
    pub fn new() -> Self {
        Self
    }

    /// Ejecuta el servidor web de la aplicación.
    pub fn run(self) -> Result<service::Server, Error> {
        // Prepara el servidor web.
        Ok(service::HttpServer::new(move || Self::service_app())
            .bind("localhost:8080")?
            .run())
    }

    /// Prepara el servidor web de la aplicación para pruebas.
    pub fn test(
        self,
    ) -> service::App<
        impl service::Factory<
            service::Request,
            Config = (),
            Response = service::Response<service::BoxBody>,
            Error = service::Error,
            InitError = (),
        >,
    > {
        Self::service_app()
    }

    // Configura el servicio web de la aplicación.
    fn service_app() -> service::App<
        impl service::Factory<
            service::Request,
            Config = (),
            Response = service::Response<service::BoxBody>,
            Error = service::Error,
            InitError = (),
        >,
    > {
        service::App::new()
    }
}
