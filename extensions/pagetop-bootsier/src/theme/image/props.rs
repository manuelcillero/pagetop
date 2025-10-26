use pagetop::prelude::*;

// **< Size >***************************************************************************************

/// Define las **dimensiones** de una imagen ([`Image`](crate::theme::Image)).
#[derive(AutoDefault)]
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

/// Especifica la **fuente** para publicar una imagen ([`Image`](crate::theme::Image)).
#[derive(AutoDefault)]
pub enum Source {
    /// Imagen con el logotipo de PageTop.
    #[default]
    Logo(PageTopSvg),
    /// Imagen que se adapta automáticamente a su contenedor.
    ///
    /// El `String` asociado es la URL (o ruta) de la imagen.
    Responsive(String),
    /// Imagen que aplica el estilo **miniatura** de Bootstrap.
    ///
    /// El `String` asociado es la URL (o ruta) de la imagen.
    Thumbnail(String),
    /// Imagen sin clases específicas de Bootstrap, útil para controlar con CSS propio.
    ///
    /// El `String` asociado es la URL (o ruta) de la imagen.
    Plain(String),
}
