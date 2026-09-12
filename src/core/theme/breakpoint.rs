use crate::AutoDefault;
use crate::core::component::{Context, Contextual};

// **< Breakpoint >*********************************************************************************

/// Puntos de corte *responsive*, *mobile-first* (se aplican a partir del ancho indicado).
///
/// `Breakpoint` no define ningún valor en píxeles por sí mismo; cada tema decide qué nombre y a qué
/// ancho mínimo corresponde cada variante en su especificación (ver [`Theme::breakpoint_entry()`]).
///
/// [`Theme::breakpoint_entry()`]: crate::core::theme::Theme::breakpoint_entry
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Breakpoint {
    /// Base *mobile-first*, equivale a "siempre aplica".
    #[default]
    Xs,
    /// Aplica a partir del ancho donde un tema suele pasar de móvil a tableta.
    Sm,
    /// Aplica a partir del ancho donde un tema suele pasar a un escritorio pequeño.
    Md,
    /// Aplica a partir del ancho donde un tema suele pasar a un escritorio normal.
    Lg,
    /// Aplica a partir del ancho donde un tema suele considerar el escritorio ancho.
    Xl,
    /// Aplica a partir del ancho donde un tema suele considerar el escritorio muy ancho.
    Xxl,
    /// Aplica a partir del ancho donde un tema suele considerar el escritorio extra ancho.
    Xxxl,
}

impl Breakpoint {
    // Todas las variantes, en orden mobile-first (de Xs a Xxxl).
    pub(crate) const ALL: [Breakpoint; 7] = [
        Breakpoint::Xs,
        Breakpoint::Sm,
        Breakpoint::Md,
        Breakpoint::Lg,
        Breakpoint::Xl,
        Breakpoint::Xxl,
        Breakpoint::Xxxl,
    ];

    /// Nombre del punto de corte resuelto a través del tema activo del contexto actual.
    ///
    /// Depende de [`Context`] y puede cambiar entre temas. Es un atajo de acceso al campo `name` de
    /// [`BreakpointEntry`] devuelto por [`Theme::breakpoint_entry()`] en el tema activo del
    /// contexto ([`Context::theme()`]).
    ///
    /// [`Theme::breakpoint_entry()`]: crate::core::theme::Theme::breakpoint_entry
    #[inline]
    pub fn name(&self, cx: &Context) -> &'static str {
        cx.theme().breakpoint_entry(*self).name
    }

    /// Ancho mínimo resuelto para el punto de corte a través del tema activo del contexto actual,
    /// como valor CSS ya formateado (p. ej. `"768px"`); o devuelve `""` si la variante se aplica
    /// siempre, sin un ancho real asociado.
    ///
    /// Normalmente se usará este método, aunque realmente es un atajo de acceso al campo
    /// `min_width` de [`BreakpointEntry`] que devuelve [`Theme::breakpoint_entry()`] en el tema
    /// activo del contexto ([`Context::theme()`]).
    ///
    /// [`Theme::breakpoint_entry()`]: crate::core::theme::Theme::breakpoint_entry
    #[inline]
    pub fn min_width(&self, cx: &Context) -> &'static str {
        cx.theme().breakpoint_entry(*self).min_width
    }

    /// Resuelve la variante en el tema activo del contexto ([`Context::theme()`]). Devuelve `None`
    /// si el valor de `min_width` está vacío por lo que no representa ningún ancho mínimo real para
    /// este tema. Devuelve el [`BreakpointEntry`] completo en caso contrario.
    ///
    /// Permite decidir si una variante debe tratarse como incondicional (sin envolver en `@media`
    /// y sin sufijo de punto de corte en nombres de clase), o como un punto de corte real. Se
    /// devuelve el `BreakpointEntry` completo para poder obtener el [`name`](BreakpointEntry::name)
    /// y el [`min_width`](BreakpointEntry::min_width) para el tema activo, sin tener que volver a
    /// consultar el tema.
    #[inline]
    pub fn resolved(self, cx: &Context) -> Option<BreakpointEntry> {
        let entry = cx.theme().breakpoint_entry(self);
        if entry.min_width.is_empty() {
            None
        } else {
            Some(entry)
        }
    }

    // **< Breakpoint HELPERS >*********************************************************************

    // Posición de esta variante en `Breakpoint::ALL`, para indexar `Responsive::values`. Válido
    // porque el orden de declaración del enum coincide con `ALL` (de `Xs` a `Xxxl`).
    fn index(self) -> usize {
        self as usize
    }
}

// **< BreakpointEntry >****************************************************************************

/// Punto de corte [`Breakpoint`] con su nombre y ancho mínimo *responsive* en un tema.
///
/// Ver [`Theme::breakpoint_entry()`](crate::core::theme::Theme::breakpoint_entry) para entender
/// cómo definir los puntos de corte en la implementación de un tema dado.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BreakpointEntry {
    /// Variante de la que procede esta entrada. Coincide siempre con el [`Breakpoint`] pasado a
    /// [`Theme::breakpoint_entry()`](crate::core::theme::Theme::breakpoint_entry), incluso si el
    /// tema delega en su padre. Se incluye para que el valor siga siendo identificable aunque se
    /// conozca de partida.
    pub breakpoint: Breakpoint,
    /// Nombre del punto de corte según el tema (p. ej. `"md"` o `"tablet"` podrían ser nombres para
    /// `Breakpoint::Md` en dos temas diferentes).
    pub name: &'static str,
    /// Ancho mínimo *responsive*, como valor CSS ya formateado, por ejemplo `"768px"`. Se usará una
    /// cadena vacía `""` si la variante se aplica siempre (para cualquier ancho).
    pub min_width: &'static str,
}

// **< Responsive >*********************************************************************************

/// Encapsula valores para cada punto de corte, aplicados en cascada *mobile-first*.
///
/// Guarda un valor opcional para cada variante de [`Breakpoint`]. No decide por sí mismo cómo se
/// interpreta cada punto de corte. Con [`by_breakpoint()`] se pueden devolver los valores
/// establecidos para cada variante, sin resolver ningún ancho ni consultar el tema activo. Traducir
/// eso a CSS, incluida la decisión de envolver en `@media` según [`Breakpoint::min_width()`], es
/// responsabilidad de quien consuma [`by_breakpoint()`], normalmente para acabar registrado en
/// [`ResponsiveStyles`].
///
/// Uso típico: los campos de [`Flex`]/[`FlexItem`] para el posicionamiento Flexbox de componentes.
///
/// [`by_breakpoint()`]: Self::by_breakpoint
/// [`ResponsiveStyles`]: crate::html::ResponsiveStyles
/// [`Flex`]: crate::html::Flex
/// [`FlexItem`]: crate::html::FlexItem
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct Responsive<T> {
    values: [Option<T>; 7],
}

impl<T: Copy> Responsive<T> {
    // **< Responsive BUILDER >*********************************************************************

    /// Establece el valor base (sin punto de corte, activo siempre).
    pub fn set(mut self, value: T) -> Self {
        self.values[0] = Some(value);
        self
    }

    /// Establece el valor a partir del punto de corte indicado.
    pub fn set_at(mut self, bp: Breakpoint, value: T) -> Self {
        self.values[bp.index()] = Some(value);
        self
    }

    /// Combina con otro `Responsive<T>`, punto de corte a punto de corte. Donde `other` tenga un
    /// valor, sustituye al de `self`; donde no, se conserva el de `self`.
    pub fn merge(mut self, other: Self) -> Self {
        for (slot, value) in self.values.iter_mut().zip(other.values) {
            if value.is_some() {
                *slot = value;
            }
        }
        self
    }

    // **< Responsive GETTERS >*********************************************************************

    /// Devuelve el valor establecido para el punto de corte exacto indicado, si existe.
    pub fn get_at(&self, bp: Breakpoint) -> Option<T> {
        self.values[bp.index()]
    }

    // **< Responsive HELPERS >*********************************************************************

    /// Recorre los valores establecidos, en orden, como pares `(punto de corte, valor)`. Decidir si
    /// su ancho mínimo resuelto la hace incondicional, el caso por defecto, es responsabilidad de
    /// quien consuma este iterador, vía [`Breakpoint::resolved()`].
    pub fn by_breakpoint(&self) -> impl Iterator<Item = (Breakpoint, T)> + '_ {
        Breakpoint::ALL
            .iter()
            .zip(self.values.iter())
            .filter_map(|(bp, value)| value.map(|value| (*bp, value)))
    }
}
