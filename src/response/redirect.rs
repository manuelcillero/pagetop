//! Realiza redirecciones HTTP.
//!
//! **La redirección de URL** (o *URL forwarding*) es una técnica que permite asignar más de una
//! dirección a un mismo recurso web. HTTP define respuestas ***HTTP redirect*** para ello (ver
//! *[Redirections in HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP/Redirections)*).
//!
//! Existen varios tipos de redirección, agrupados en tres grandes categorías:
//!
//! - **Redirecciones permanentes**. Se usan cuando el cambio de ubicación es definitivo. Indican
//!   que la URL original ya no debe emplearse y que ha sido sustituida por la nueva. Los robots de
//!   los buscadores, lectores RSS y otros *crawlers* suelen actualizar sus índices con la nueva
//!   dirección.
//!
//! - **Redirecciones temporales**. Se aplican cuando el recurso no puede servirse desde su
//!   ubicación canónica pero sí desde otra provisional. En este caso los buscadores **no** deben
//!   memorizar la URL alternativa. También son útiles para mostrar páginas de progreso al crear,
//!   actualizar o eliminar recursos.
//!
//! - **Respuestas especiales**.

use crate::web::{IntoResponse, Response, http};

/// Funciones predefinidas para generar respuestas HTTP de redirección.
///
/// Ofrece atajos para construir respuestas con el código de estado apropiado y la cabecera
/// `Location`, evitando repetir la misma secuencia en cada controlador.
pub struct Redirect;

impl Redirect {
    /// Redirección **permanente**. Código de estado **301**. El método GET se conserva tal cual.
    /// Otros métodos pueden degradarse a GET. Es una redirección típica para la reorganización de
    /// un sitio o aplicación web.
    ///
    /// Emplear cuando un recurso se ha movido de forma definitiva y la URL antigua debe dejar de
    /// usarse.
    #[must_use]
    pub fn moved(redirect_to_url: &str) -> Response {
        (
            http::StatusCode::MOVED_PERMANENTLY,
            [(http::header::LOCATION, redirect_to_url.to_owned())],
        )
            .into_response()
    }

    /// Redirección **permanente**. Código de estado **308**. Mantiene método y cuerpo sin cambios.
    ///
    /// Indicada para reorganizaciones de un sitio o aplicación web en las que también existen
    /// métodos distintos de GET (POST, PUT, ...) que no deben degradarse a GET.
    #[must_use]
    pub fn permanent(redirect_to_url: &str) -> Response {
        (
            http::StatusCode::PERMANENT_REDIRECT,
            [(http::header::LOCATION, redirect_to_url.to_owned())],
        )
            .into_response()
    }

    /// Redirección **temporal**. Código de estado **302**. El método GET (y normalmente HEAD) se
    /// mantiene tal cual. Otros métodos pueden degradarse a GET.
    ///
    /// Útil cuando un recurso está fuera de servicio de forma imprevista (mantenimiento breve,
    /// sobrecarga, ...).
    #[must_use]
    pub fn found(redirect_to_url: &str) -> Response {
        (
            http::StatusCode::FOUND,
            [(http::header::LOCATION, redirect_to_url.to_owned())],
        )
            .into_response()
    }

    /// Redirección **temporal**. Código de estado **303**. Método GET se mantiene tal cual. Los
    /// demás métodos se cambian a GET (se pierde el cuerpo).
    ///
    /// Se usa típicamente tras un POST o PUT para aplicar el patrón *Post/Redirect/Get*, permite
    /// recargar la página de resultados sin volver a ejecutar la operación.
    #[must_use]
    pub fn see_other(redirect_to_url: &str) -> Response {
        (
            http::StatusCode::SEE_OTHER,
            [(http::header::LOCATION, redirect_to_url.to_owned())],
        )
            .into_response()
    }

    /// Redirección **temporal**. Código de estado **307**. Conserva método y cuerpo íntegros.
    ///
    /// Preferible a [`found`](Self::found) cuando el sitio expone operaciones diferentes de GET que
    /// deben respetarse durante la redirección.
    #[must_use]
    pub fn temporary(redirect_to_url: &str) -> Response {
        (
            http::StatusCode::TEMPORARY_REDIRECT,
            [(http::header::LOCATION, redirect_to_url.to_owned())],
        )
            .into_response()
    }

    /// Respuesta **especial**. Código de estado **304**. Se envía tras una petición condicional,
    /// para indicar que la copia en caché sigue siendo válida y puede utilizarse, evitando
    /// transferir de nuevo el recurso.
    ///
    /// No es una redirección, el cliente debe reutilizar su copia local.
    #[must_use]
    pub fn not_modified() -> Response {
        http::StatusCode::NOT_MODIFIED.into_response()
    }
}
