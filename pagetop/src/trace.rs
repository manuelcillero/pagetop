//! Registro de trazas y eventos de la aplicación.
//!
//! PageTop recopila la información de diagnóstico de la aplicación de manera estructurada y basada
//! en eventos.
//!
//! En sistemas asíncronos, interpretar los mensajes de registro tradicionales (*log*) a menudo
//! resulta complicado. Las tareas individuales se multiplexan para el mismo subproceso y los
//! eventos y mensajes de registro asociados se entremezclan, dificultando el seguimiento de la
//! secuencia lógica.
//!
//! PageTop usa [`tracing`](https://docs.rs/tracing) para permitir a las **aplicaciones** y los
//! **módulos** registrar eventos estructurados con información añadida sobre *temporalidad* y
//! *causalidad*. A diferencia de un mensaje de registro, un intervalo (*span*) tiene una hora de
//! inicio y de finalización, puede entrar y salir del flujo de la ejecución y puede existir dentro
//! de un árbol anidado de intervalos similares. Además, estos intervalos están *estructurados*, con
//! capacidad para grabar tipos de datos y mensajes de texto.

pub use tracing::{debug, error, info, trace, warn};
pub use tracing::{event, span, Level};
