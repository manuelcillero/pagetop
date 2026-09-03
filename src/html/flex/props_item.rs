//! Enums semánticos que configuran [`FlexItem`](super::FlexItem), a nivel de ítem.

use crate::html::unit::UnitValue;
use crate::{AutoDefault, CowStr};

// **< ItemAlign >**********************************************************************************

/// Alineación en [`FlexItem`](super::FlexItem) para un ítem, sobrescribiendo la del contenedor.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ItemAlign {
    /// Por defecto, hereda la alineación del contenedor (`align-self: auto` no explícito).
    #[default]
    Default,
    /// Alinea el ítem al inicio del eje transversal (`align-self: flex-start`).
    Start,
    /// Alinea el ítem al final del eje transversal (`align-self: flex-end`).
    End,
    /// Centra el ítem en el eje transversal (`align-self: center`).
    Center,
    /// Alinea el ítem por su línea base de texto (`align-self: baseline`).
    Baseline,
    /// Estira el ítem para ocupar todo el eje transversal (`align-self: stretch`).
    Stretch,
}

impl ItemAlign {
    // Devuelve el valor CSS de `align-self`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Default => "".into(),
            Self::Start => "flex-start".into(),
            Self::End => "flex-end".into(),
            Self::Center => "center".into(),
            Self::Baseline => "baseline".into(),
            Self::Stretch => "stretch".into(),
        }
    }
}

// **< ItemGrow >***********************************************************************************

/// Factor de crecimiento en [`FlexItem`](super::FlexItem) para un ítem dentro de un contenedor.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ItemGrow {
    /// Por defecto, no crece más allá de su tamaño base (`flex-grow: 0` no explícito).
    #[default]
    Default,
    /// Crece para ocupar el espacio sobrante (`flex-grow: 1`).
    Is1,
}

impl ItemGrow {
    // Devuelve el valor CSS de `flex-grow`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Default => "".into(),
            Self::Is1 => "1".into(),
        }
    }
}

// **< ItemOffset >*********************************************************************************

/// Desplazamiento en [`FlexItem`](super::FlexItem) para un ítem respecto al inicio del contenedor.
///
/// Junto con [`ItemSize`], permite maquetar rejillas de columnas fijas sobre Flexbox. Un ítem con
/// [`ItemOffset::Percent33`] deja libre el primer tercio del contenedor antes de empezar. No tiene
/// relación con [`FlexItem::push_end()`](super::FlexItem::push_end). Ambos aplican la misma
/// propiedad CSS (`margin-inline-start`), pero para casos de uso distintos (un desplazamiento fijo
/// en fracción del contenedor, frente a "ocupa todo el espacio sobrante"); combinarlos no tiene
/// sentido, y si se aplican los dos, gana el último que se llame.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ItemOffset {
    /// Por defecto, sin desplazamiento (`margin-inline-start: 0` no explícito).
    #[default]
    None,
    /// Se desplaza el 10% del ancho del contenedor (`margin-inline-start: 10%`).
    Percent10,
    /// Se desplaza el 20% del ancho del contenedor (`margin-inline-start: 20%`).
    Percent20,
    /// Se desplaza el 25% del ancho del contenedor (`margin-inline-start: 25%`).
    Percent25,
    /// Se desplaza un tercio del ancho del contenedor (`margin-inline-start: 33.3333%`).
    Percent33,
    /// Se desplaza el 40% del ancho del contenedor (`margin-inline-start: 40%`).
    Percent40,
    /// Se desplaza la mitad del ancho del contenedor (`margin-inline-start: 50%`).
    Percent50,
    /// Se desplaza el 60% del ancho del contenedor (`margin-inline-start: 60%`).
    Percent60,
    /// Se desplaza dos tercios del ancho del contenedor (`margin-inline-start: 66.6667%`).
    Percent66,
    /// Se desplaza el 75% del ancho del contenedor (`margin-inline-start: 75%`).
    Percent75,
    /// Se desplaza el 80% del ancho del contenedor (`margin-inline-start: 80%`).
    Percent80,
    /// Se desplaza el 90% del ancho del contenedor (`margin-inline-start: 90%`).
    Percent90,
    /// Cualquier otro valor, incluidas unidades absolutas (p. ej. un desplazamiento fijo en
    /// píxeles).
    Custom(UnitValue),
}

impl ItemOffset {
    // Devuelve el valor CSS de `margin-inline-start`, o cadena vacía para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::None => "".into(),
            Self::Percent10 => "10%".into(),
            Self::Percent20 => "20%".into(),
            Self::Percent25 => "25%".into(),
            Self::Percent33 => "33.3333%".into(),
            Self::Percent40 => "40%".into(),
            Self::Percent50 => "50%".into(),
            Self::Percent60 => "60%".into(),
            Self::Percent66 => "66.6667%".into(),
            Self::Percent75 => "75%".into(),
            Self::Percent80 => "80%".into(),
            Self::Percent90 => "90%".into(),
            Self::Custom(value) => value.into(),
        }
    }
}

// **< ItemOrder >**********************************************************************************

/// Posición en [`FlexItem`](super::FlexItem) para un ítem en el orden visual.
///
/// # Accesibilidad
///
/// Con `ItemOrder` se cambia únicamente el **orden visual**, no el orden del documento que siguen
/// la navegación por tabulador y los lectores de pantalla. Al reordenar con `ItemOrder` se puede
/// desalinear lo que se ve en pantalla de lo que se lee o se recorre con teclado, sin ningún aviso
/// del navegador.
///
/// Por eso se recomienda usar únicamente en reordenaciones puramente cosméticas, donde ese
/// desajuste no importe (p. ej. dos bloques intercambiables sin relación de lectura entre sí). Si
/// el orden tiene significado real, cambia el orden en el propio documento en lugar de maquillarlo
/// con `ItemOrder`.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ItemOrder {
    /// Por defecto, el orden visual coincide con el del documento (`order: 0` no explícito).
    #[default]
    Default,
    /// Se muestra antes que cualquier ítem, incluidos los que usan [`Self::Custom`]
    /// (`order: -129`).
    First,
    /// Se muestra después de cualquier ítem, incluidos los que usan [`Self::Custom`]
    /// (`order: 128`).
    Last,
    /// Posición `1` en el orden visual (`order: 1`).
    Is1,
    /// Posición `2` en el orden visual (`order: 2`).
    Is2,
    /// Posición `3` en el orden visual (`order: 3`).
    Is3,
    /// Posición `4` en el orden visual (`order: 4`).
    Is4,
    /// Posición `5` en el orden visual (`order: 5`).
    Is5,
    /// Cualquier otra posición no cubierta por `Default` (posición `0`) ni `Is1`..`Is5`.
    Custom(i8),
}

impl ItemOrder {
    // Devuelve el valor CSS de `order`, o "" para el valor por defecto. `First`/`Last` usan el
    // primer entero fuera del rango de `Custom` (`i8::MIN - 1` / `i8::MAX + 1`), para quedar
    // siempre antes o después de cualquier valor que éste pueda representar.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Default => "".into(),
            Self::First => "-129".into(),
            Self::Last => "128".into(),
            Self::Is1 => "1".into(),
            Self::Is2 => "2".into(),
            Self::Is3 => "3".into(),
            Self::Is4 => "4".into(),
            Self::Is5 => "5".into(),
            Self::Custom(value) => value.to_string().into(),
        }
    }
}

// **< ItemShrink >*********************************************************************************

/// Factor de reducción en [`FlexItem`](super::FlexItem) para un ítem dentro de un contenedor.
///
/// # Cuándo usar `Is0`
///
/// Para un tamaño fijo ([`ItemSize::Custom`]) es la opción natural. Un icono, un avatar o una barra
/// lateral con un ancho fijo definido por diseño no debe deformarse si falta espacio, que sea otro
/// elemento el que ceda (uno con [`ItemGrow::Is1`] y contenido que sí admita reajuste, como texto),
/// no éste.
///
/// Con [`ItemSize`] en porcentaje, `Is0` es seguro si el contenedor no tiene [`Gap`](super::Gap)
/// (sin `gap` no hay nada que compensar). Pero **si el contenedor tiene `Gap`, no combines `Is0`
/// con un tamaño porcentual** porque desactivas la única pieza (el reparto del espacio negativo
/// entre elementos) que compensa el hueco por ti. La explicación completa está en [`ItemSize`].
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ItemShrink {
    /// Por defecto, puede encoger si hace falta (`flex-shrink: 1` no explícito).
    #[default]
    Default,
    /// No encoge nunca, aunque no quepa en el contenedor (`flex-shrink: 0`).
    Is0,
}

impl ItemShrink {
    // Devuelve el valor CSS de `flex-shrink`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Default => "".into(),
            Self::Is0 => "0".into(),
        }
    }
}

// **< ItemSize >***********************************************************************************

/// Ancho en [`FlexItem`](super::FlexItem) para un ítem como fracción del contenedor.
///
/// Permite maquetar rejillas de columnas fijas. Un ítem con [`ItemSize::Percent33`] ocupa un tercio
/// del ancho del contenedor con independencia de su contenido.
///
/// # Cómo combinarlo con `Gap`
///
/// Un porcentaje se resuelve contra el ancho del contenedor sin contar el espacio que va a ocupar
/// el [`Gap`](super::Gap). Es una limitación del propio CSS, porque `flex-basis` en porcentaje usa
/// la misma regla de resolución que cualquier `width: %`. Si los porcentajes de una fila suman el
/// 100% (una rejilla completa, el caso habitual), el hueco que añade `gap` sobra respecto al ancho
/// del contenedor.
///
/// Ese sobrante se compensa solo, sin ningún ajuste manual, siempre que:
///
/// - **No se fuerce [`ItemShrink::Is0`]** en los ítems de esa fila. Déjalos en su valor por
///   defecto, [`ItemShrink::Default`](super::ItemShrink::Default). El reparto por defecto del
///   espacio negativo entre elementos, proporcional al tamaño de partida de cada uno, reproduce
///   exactamente el resultado de restar el `gap` antes de repartir. Forzar `ItemShrink::Is0`
///   desactiva esa compensación y el hueco sobrante pasa a desbordar de verdad.
/// - **El contenedor use [`Behavior::NoWrap`](super::Behavior::NoWrap)** (su valor por defecto).
///   Con [`Behavior::Wrap`](super::Behavior::Wrap) el navegador decide si rompe la línea a partir
///   de los tamaños *antes* de aplicar el `shrink`, así que una fila que encajaría perfectamente en
///   una sola línea puede saltar de línea antes de que la compensación llegue a actuar. Combinar
///   `ItemSize` porcentual, `Gap` y ajuste de línea sigue siendo el caso sin resolver porque no hay
///   compensación automática posible cuando distintas líneas acaban con un número distinto de
///   elementos.
///
/// Con un tamaño fijo ([`ItemSize::Custom`]) ninguna de estas condiciones aplica: un `gap` nunca
/// sorprende a un tamaño que no dependía de un porcentaje del contenedor, así que ahí
/// `ItemShrink::Is0` es siempre seguro (ver [`ItemShrink`]).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ItemSize {
    /// Por defecto, el tamaño se calcula según el contenido (`flex-basis: auto` no explícito).
    #[default]
    Default,
    /// Ocupa el 10% del ancho del contenedor (`flex-basis: 10%`).
    Percent10,
    /// Ocupa el 20% del ancho del contenedor (`flex-basis: 20%`).
    Percent20,
    /// Ocupa el 25% del ancho del contenedor (`flex-basis: 25%`).
    Percent25,
    /// Ocupa un tercio del ancho del contenedor (`flex-basis: 33.3333%`).
    Percent33,
    /// Ocupa el 40% del ancho del contenedor (`flex-basis: 40%`).
    Percent40,
    /// Ocupa la mitad del ancho del contenedor (`flex-basis: 50%`).
    Percent50,
    /// Ocupa el 60% del ancho del contenedor (`flex-basis: 60%`).
    Percent60,
    /// Ocupa dos tercios del ancho del contenedor (`flex-basis: 66.6667%`).
    Percent66,
    /// Ocupa el 75% del ancho del contenedor (`flex-basis: 75%`).
    Percent75,
    /// Ocupa el 80% del ancho del contenedor (`flex-basis: 80%`).
    Percent80,
    /// Ocupa el 90% del ancho del contenedor (`flex-basis: 90%`).
    Percent90,
    /// Ocupa el 100% del ancho del contenedor (`flex-basis: 100%`).
    Percent100,
    /// Cualquier otro valor, incluidas unidades absolutas (p. ej. un ancho fijo en píxeles).
    Custom(UnitValue),
}

impl ItemSize {
    // Devuelve el valor CSS de `flex-basis`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Default => "".into(),
            Self::Percent10 => "10%".into(),
            Self::Percent20 => "20%".into(),
            Self::Percent25 => "25%".into(),
            Self::Percent33 => "33.3333%".into(),
            Self::Percent40 => "40%".into(),
            Self::Percent50 => "50%".into(),
            Self::Percent60 => "60%".into(),
            Self::Percent66 => "66.6667%".into(),
            Self::Percent75 => "75%".into(),
            Self::Percent80 => "80%".into(),
            Self::Percent90 => "90%".into(),
            Self::Percent100 => "100%".into(),
            Self::Custom(value) => value.into(),
        }
    }
}
