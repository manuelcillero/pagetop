use crate::prelude::*;

/// Especifica el **origen** de un icono ([`Icon`](super::Icon)).
///
/// Es el punto de integración de cualquier conjunto de iconos: basta con implementar
/// `From<T> for icon::Source` para que su tipo pueda pasarse directamente a
/// [`Icon::with()`](super::Icon::with).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// // Un conjunto propio de iconos SVG.
/// enum MyIcons {
///     Dot,
/// }
///
/// impl From<MyIcons> for icon::Source {
///     fn from(icon: MyIcons) -> Self {
///         match icon {
///             MyIcons::Dot => {
///                 icon::Source::svg("0 0 16 16", html! { circle cx="8" cy="8" r="4" {} })
///             }
///         }
///     }
/// }
///
/// let dot = Icon::with(MyIcons::Dot);
///
/// // Un glifo de una fuente de iconos cuyas hojas de estilo ya se han cargado en la página.
/// let star = Icon::with(icon::Source::font("my-icons my-icons-star"));
/// ```
#[derive(Clone, Debug)]
pub enum Source {
    /// Icono SVG en línea.
    ///
    /// Se renderiza como un elemento `<svg>` de `1em` de ancho y alto, que escala con el tamaño de
    /// la fuente, y rellena con el color del texto (`fill="currentColor"`), salvo que las formas
    /// indiquen otra cosa.
    Svg {
        /// Valor del atributo `viewBox` del `<svg>`, p. ej. `"0 0 16 16"`.
        viewbox: CowStr,
        /// Formas del icono (`path`, `circle`...), insertadas tal cual dentro del `<svg>`.
        shapes: Markup,
    },
    /// Icono del conjunto incluido en PageTop.
    ///
    /// Se renderiza como un icono SVG en línea, salvo que el tema activo lo sustituya por el
    /// equivalente de su propio conjunto en su `setup_component()`. Como el `setup()` del icono ya
    /// se ha ejecutado, el tema debe añadir también las clases CSS del nuevo origen (p. ej. las del
    /// glifo, si lo cambia por [`Font`](Self::Font)).
    Core(icon::Core),
    /// Glifo de una fuente de iconos, identificado por sus clases CSS.
    ///
    /// Se renderiza como un elemento `<i>` con esas clases. Las hojas de estilo y la fuente deben
    /// cargarse aparte, normalmente desde el tema.
    Font(CowStr),
}

impl Source {
    /// Icono SVG en línea con el `viewBox` y las formas indicadas.
    #[inline]
    pub fn svg(viewbox: impl Into<CowStr>, shapes: Markup) -> Self {
        Self::Svg {
            viewbox: viewbox.into(),
            shapes,
        }
    }

    /// Glifo de una fuente de iconos con las clases CSS indicadas.
    #[inline]
    pub fn font(classes: impl Into<CowStr>) -> Self {
        Self::Font(classes.into())
    }
}
