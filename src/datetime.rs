//! Soporte a fechas y horas según estándar [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
//! (basado en [chrono](https://docs.rs/chrono)).

pub use chrono::prelude::*;

// `Duration` no forma parte de `chrono::prelude`, pero es de uso tan habitual junto al resto de
// tipos de este módulo (sumar/restar intervalos a un `NaiveDateTime`, calcular expiraciones...) que
// se reexporta igualmente.
pub use chrono::Duration;
