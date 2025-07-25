//! Extractor y generador de respuestas JSON (reexporta [`actix_web::web::Json`]).
//!
//! # Uso como extractor JSON
//!
//! Convierte automáticamente el cuerpo de una petición con `Content-Type: application/json` en un
//! tipo Rust fuertemente tipado, validando el formato y deserializando con *serde*.
//!
//! ```rust
//! use pagetop::prelude::*;
//!
//! #[derive(serde::Deserialize)]
//! struct NuevoUsuario { nombre: String, email: String }
//!
//! // Manejador configurado para la ruta POST "/usuarios".
//! async fn crear_usuario(payload: Json<NuevoUsuario>) -> HttpResponse {
//!     // `payload` ya es `NuevoUsuario`; si la deserialización falla,
//!     // devolverá automáticamente 400 Bad Request con un cuerpo JSON que describe el error.
//!     HttpResponse::Ok().finish()
//! }
//! ```
//!
//! # Uso como generador de respuestas JSON
//!
//! Serializa valores Rust a JSON y genera una respuesta HTTP con el encabezado apropiado
//! `application/json; charset=utf-8`, todo con una llamada compacta.
//!
//! ```rust
//! use pagetop::prelude::*;
//!
//! #[derive(serde::Serialize)]
//! struct Usuario { id: u32, nombre: String }
//!
//! async fn obtener_usuario() -> Json<Usuario> {
//!     Json(Usuario { id: 1, nombre: "Ada".into() })
//! }
//! ```
//!
//! `Json<T>` funciona con cualquier tipo que implemente `serde::Serialize` (para respuestas) y/o
//! `serde::Deserialize` (para peticiones).

pub use actix_web::web::Json;
