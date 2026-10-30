//! Correos guardados en memoria por el transporte `"memory"`, para comprobarlos en las pruebas.
//!
//! Con `transport = "memory"` en la sección `[mail]` de la configuración de pruebas, los correos no
//! se envían: se guardan aquí, ya completos (con el remitente por defecto y el documento HTML con
//! el diseño común), y las pruebas los recogen con [`take()`] o [`take_to()`].
//!
//! Los correos se acumulan mientras nadie los recoja, así que este transporte no debe usarse fuera
//! de las pruebas.
//!
//! # Ejemplo
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_mail::prelude::*;
//! use pagetop_mail::outbox;
//!
//! #[pagetop::test]
//! async fn sends_welcome_email() {
//!     // ... ejecuta lo que debe enviar el correo a ana@example.com ...
//!
//!     let sent = outbox::take_to("ana@example.com");
//!     assert_eq!(sent.len(), 1);
//!     assert_eq!(sent[0].email().subject(), "Bienvenida");
//! }
//! ```
//!
//! Las pruebas de un mismo binario se ejecutan en paralelo y comparten estos correos: con
//! [`take_to()`] cada prueba recoge sólo los de sus destinatarios, sin llevarse los de otras.
//!
//! Con la *feature* `queue`, los correos de `Email::queue()` sólo llegan aquí cuando se ejecuta el
//! trabajo que vacía la cola. En las pruebas no se ejecuta solo: hay que lanzarlo con
//! `pagetop_cron::run_now("mail.queue")`. Los de [`Email::send_later()`] no esperan al trabajo,
//! pero llegan en segundo plano, como sin la cola.

use pagetop::prelude::*;

use crate::Email;

use std::sync::{Mutex, MutexGuard, PoisonError};

static OUTBOX: Mutex<Vec<SentEmail>> = Mutex::new(Vec::new());

/// Correo guardado por el transporte `"memory"`.
#[derive(Clone, Debug, Getters)]
pub struct SentEmail {
    /// El correo tal como se envió, con el remitente y la dirección de respuesta por defecto ya
    /// aplicados, y el documento HTML completo.
    email: Email,
    /// Direcciones a las que se entregó (todas, incluidas las de `Cc` y `Bcc`, o sólo la de
    /// `mail.redirect_to` si está configurada).
    envelope_to: Vec<String>,
    /// Mensaje completo en formato RFC 5322, con cabeceras y partes codificadas.
    raw: String,
}

impl SentEmail {
    /// Indica si el correo se entregó a la dirección dada (sin distinguir mayúsculas).
    pub fn is_delivered_to(&self, address: &str) -> bool {
        self.envelope_to
            .iter()
            .any(|to| to.eq_ignore_ascii_case(address.trim()))
    }
}

/// Recoge todos los correos guardados, en el orden en que se enviaron.
pub fn take() -> Vec<SentEmail> {
    std::mem::take(&mut *lock())
}

/// Recoge los correos entregados a la dirección dada, en el orden en que se enviaron, y deja los
/// demás.
pub fn take_to(address: &str) -> Vec<SentEmail> {
    let mut outbox = lock();
    let (taken, kept) = outbox
        .drain(..)
        .partition(|sent: &SentEmail| sent.is_delivered_to(address));
    *outbox = kept;
    taken
}

pub(crate) fn push(email: Email, envelope_to: Vec<String>, raw: Vec<u8>) {
    lock().push(SentEmail {
        email,
        envelope_to,
        raw: String::from_utf8_lossy(&raw).into_owned(),
    });
}

// Una prueba que falla con el bloqueo tomado no debe impedir que las demás sigan usándolo.
fn lock() -> MutexGuard<'static, Vec<SentEmail>> {
    OUTBOX.lock().unwrap_or_else(PoisonError::into_inner)
}

// Correos completos de la cola de envío pendientes de entregar con este transporte, por su
// identificador en la cola. La cola sólo guarda el mensaje compuesto, y las pruebas necesitan el
// correo completo para consultarlo con `SentEmail::email()`.
#[cfg(feature = "queue")]
static QUEUED: Mutex<Vec<(i32, Email)>> = Mutex::new(Vec::new());

#[cfg(feature = "queue")]
pub(crate) fn hold(id: i32, email: Email) {
    QUEUED
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .push((id, email));
}

#[cfg(feature = "queue")]
pub(crate) fn release(id: i32) -> Option<Email> {
    let mut queued = QUEUED.lock().unwrap_or_else(PoisonError::into_inner);
    let index = queued.iter().position(|(queued_id, _)| *queued_id == id)?;
    Some(queued.swap_remove(index).1)
}
