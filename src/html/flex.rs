//! Definiciones para el posicionamiento de componentes con [Flexbox].
//!
//! [`Flex`] configura un contenedor y sus hijos como un grupo sobre el que se aplican propiedades
//! de presentación (dirección, ajuste de línea, alineación, espaciado). Lo usan componentes que
//! ofrecen su propio `with_flex()`, como [`Container`] o [`Navbar`].
//!
//! [`FlexItem`] configura, en cambio, un único elemento en relación con el contenedor flex de su
//! padre (crecimiento, reducción, alineación individual, orden, ancho y desplazamiento). No tiene
//! un builder propio ya que puede acabar aplicándose sobre cualquier componente (no sólo los que
//! ofrecen `with_flex()`). Por eso se aplica con [`PropsOp::flex_item()`] sobre el `with_prop()`
//! que normalmente ya expone cualquier componente.
//!
//! # Un entorno nativo autosuficiente
//!
//! Toda la configuración de `Flex`/`FlexItem` se resuelve generando clases CSS dinámicamente y de
//! manera independiente a cualquier tema o framework CSS. El nombre interno de cada clase se deriva
//! de la propiedad y el valor que representa, así que dos elementos con la misma configuración
//! comparten la misma regla en vez de duplicarla. Las declaraciones correspondientes se registran
//! vía [`AssetsOp::add_responsive_style()`] y se renderizan como reglas en el `<head>` del
//! documento. Funciona igual conviva con quien conviva en la misma página, sin necesidad de
//! coordinar nombres de clase ni orden alguno en la carga de hojas de estilo.
//!
//! [Flexbox]: https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Flexible_box_layout
//! [`AssetsOp::add_responsive_style()`]: crate::core::component::AssetsOp::add_responsive_style
//! [`PropsOp::flex_item()`]: crate::html::props::PropsOp::flex_item
//! [`Container`]: crate::base::component::Container
//! [`Navbar`]: crate::base::component::Navbar

use crate::CowStr;
use crate::core::component::{AssetsOp, Context, Contextual};
use crate::core::theme::BreakpointEntry;

mod props_container;
pub use props_container::{Align, AlignContent, Behavior, ContentJustify, Direction, Gap};

mod props_item;
pub use props_item::{ItemAlign, ItemGrow, ItemOffset, ItemOrder, ItemShrink, ItemSize};

mod container;
pub use container::Flex;

mod item;
pub use item::FlexItem;

// **< Flex / FlexItem PRIVATE >********************************************************************

// Sustituye, en un valor CSS ya resuelto, los únicos caracteres (`.`, `%`) que no podrían usarse
// como fragmento de un nombre de clase. Así, `"1.5rem"` sería `"1_5rem"` y `"33.3333%"` quedaría
// como `"33_3333pct"`.
fn value_to_token(value: &str) -> String {
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
fn styles(
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
use responsive_class;

// Recorre las entradas para una propiedad `Responsive<T>` cuyo valor CSS es un único `T::value()`,
// generando y registrando (vía `styles()`) una clase por punto de corte con valor.
//
// La forma con el marcador final `val` es para propiedades cuyo valor puede contener `.`/`%` (como
// `ItemSize` o `ItemOffset` en `FlexItem`) y necesitan pasar por `value_to_token()`.
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
use apply;
