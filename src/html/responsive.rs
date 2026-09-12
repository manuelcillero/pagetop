//! Mecanismo interno compartido para resolver clases CSS nativas *responsive*.
//!
//! Usado por [`flex`] y [`spacing`]. Ambos módulos resuelven su configuración generando clases CSS
//! dinámicamente, de manera independiente a cualquier tema o framework CSS, registradas vía
//! [`AssetsOp::add_responsive_style()`] y renderizadas como reglas en el `<head>` del documento.
//! El nombre interno de cada clase se deriva de la propiedad y el valor que representa, así que dos
//! elementos con la misma configuración comparten la misma regla en vez de duplicarla.
//!
//! [`flex`]: crate::html::flex
//! [`spacing`]: crate::html::spacing
//! [`AssetsOp::add_responsive_style()`]: crate::core::component::AssetsOp::add_responsive_style

use crate::CowStr;
use crate::core::component::{AssetsOp, Context, Contextual};
use crate::core::theme::BreakpointEntry;

// Sustituye, en un valor CSS ya resuelto, los únicos caracteres (`.`, `%`) que no podrían usarse
// como fragmento de un nombre de clase. Así, `"1.5rem"` sería `"1_5rem"` y `"33.3333%"` quedaría
// como `"33_3333pct"`.
pub(crate) fn value_to_token(value: &str) -> String {
    value.replace('.', "_").replace('%', "pct")
}

// Añade un estilo (`property: value`) al punto de corte indicado, y la clase a `classes`, separada
// con un espacio de las que ya hubiera. Recibe un `BreakpointEntry` ya resuelto (ver
// `Breakpoint::resolved()`) y extrae aquí el `Breakpoint` que `AddResponsiveStyle` necesita, que
// puede ser nulo si aplica siempre.
//
// La clase se copia al acumulador y se mueve al `AssetsOp`, sin clonarla. Recibirla como `CowStr`
// permite además que las clases fijas, las que no dependen de ningún punto de corte, lleguen como
// `&'static str` sin asignar memoria.
pub(crate) fn styles(
    cx: &mut Context,
    classes: &mut String,
    entry: Option<BreakpointEntry>,
    class: CowStr,
    property: &'static str,
    value: CowStr,
) {
    if !classes.is_empty() {
        classes.push(' ');
    }
    classes.push_str(&class);

    cx.alter_assets(AssetsOp::add_responsive_style(
        entry.map(|e| e.breakpoint),
        class,
        property,
        value,
    ));
}

// Nombre de clase según el punto de corte: `prefix` ya incluye el guion bajo final antes del valor
// (p. ej. `"_flex-direction_"`), y `entry` añade su sufijo si aplica (`"_flex-direction_row_md_"`),
// ya resuelto para el tema activo (ver `Breakpoint::resolved()`).
macro_rules! responsive_class {
    ($prefix:expr, $token:expr, $entry:expr) => {
        match $entry {
            None => util::join!($prefix, $token, "_"),
            Some(entry) => util::join!($prefix, $token, "_", entry.name, "_"),
        }
    };
}
pub(crate) use responsive_class;

// Recorre las entradas para una propiedad `Responsive<T>` cuyo valor CSS es un único `T::value()`,
// generando y registrando (vía `styles()`) una clase por punto de corte con valor.
//
// La forma con el marcador final `val` es para propiedades cuyo valor puede contener `.`/`%` (como
// `ItemSize`/`ItemOffset` en `FlexItem`, o `UnitValue` en `Margin`/`Padding`) y necesitan pasar por
// `value_to_token()`.
macro_rules! apply {
    ($cx:expr, $classes:expr, $field:expr, $prefix:literal, $property:literal) => {
        for (bp, value) in $field.by_breakpoint() {
            let value = value.value();
            if !value.is_empty() {
                let entry = bp.resolved($cx);
                let class = responsive_class!($prefix, value, entry);
                styles($cx, $classes, entry, class.into(), $property, value);
            }
        }
    };
    ($cx:expr, $classes:expr, $field:expr, $prefix:literal, $property:literal, val) => {
        for (bp, value) in $field.by_breakpoint() {
            let value = value.value();
            if !value.is_empty() {
                let entry = bp.resolved($cx);
                let class = responsive_class!($prefix, value_to_token(&value), entry);
                styles($cx, $classes, entry, class.into(), $property, value);
            }
        }
    };
}
pub(crate) use apply;
