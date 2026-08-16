use crate::prelude::*;

// **< Size >***************************************************************************************

/// Define las **dimensiones** de una imagen ([`Image`](super::Image)).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Size {
    /// Ajuste automático por defecto.
    ///
    /// La imagen usa su tamaño natural o se ajusta al contenedor donde se publica.
    #[default]
    Auto,
    /// Establece explícitamente el **ancho y alto** de la imagen.
    ///
    /// Útil cuando se desea fijar ambas dimensiones de forma exacta. Ten en cuenta que la imagen
    /// puede distorsionarse si no se mantiene la proporción original.
    Dimensions(UnitValue, UnitValue),
    /// Establece sólo el **ancho** de la imagen.
    ///
    /// La altura se ajusta proporcionalmente de manera automática.
    Width(UnitValue),
    /// Establece sólo la **altura** de la imagen.
    ///
    /// El ancho se ajusta proporcionalmente de manera automática.
    Height(UnitValue),
    /// Establece **el mismo valor** para el ancho y el alto de la imagen.
    ///
    /// Práctico para forzar rápidamente un área cuadrada. Ten en cuenta que la imagen puede
    /// distorsionarse si la original no es cuadrada.
    Both(UnitValue),
}

// **< Source >*************************************************************************************

/// Especifica la **fuente** para publicar una imagen ([`Image`](super::Image)).
///
/// Las variantes son puramente semánticas. El componente aplica una clase CSS base según la
/// variante en su propio `setup()`; los temas pueden sobrescribirla interceptando el renderizado
/// del componente.
#[derive(AutoDefault, Clone, Debug, PartialEq)]
pub enum Source {
    /// Imagen con el logotipo de PageTop.
    #[default]
    Logo(PageTopSvg),
    /// Imagen que se adapta automáticamente a su contenedor.
    ///
    /// Lleva asociada la URL (o ruta) de la imagen.
    Responsive(CowStr),
    /// Imagen que aplica un estilo de miniatura.
    ///
    /// Lleva asociada la URL (o ruta) de la imagen.
    Thumbnail(CowStr),
    /// Imagen sin modificadores adicionales de estilo, útil para controlar la apariencia con CSS
    /// propio.
    ///
    /// Lleva asociada la URL (o ruta) de la imagen.
    Plain(CowStr),
}

impl Source {
    /// Imagen con el logotipo de PageTop.
    #[inline]
    pub fn logo(svg: PageTopSvg) -> Self {
        Self::Logo(svg)
    }

    /// Imagen responsive.
    #[inline]
    pub fn responsive(url: impl Into<CowStr>) -> Self {
        Self::Responsive(url.into())
    }

    /// Imagen miniatura.
    #[inline]
    pub fn thumbnail(url: impl Into<CowStr>) -> Self {
        Self::Thumbnail(url.into())
    }

    /// Imagen sin modificadores adicionales de estilo.
    #[inline]
    pub fn plain(url: impl Into<CowStr>) -> Self {
        Self::Plain(url.into())
    }
}
