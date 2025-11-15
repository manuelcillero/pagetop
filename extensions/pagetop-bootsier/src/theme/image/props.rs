use pagetop::prelude::*;

// **< Size >***************************************************************************************

/// Define las **dimensiones** de una imagen ([`Image`](crate::theme::Image)).
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

impl Size {
    // Devuelve el valor del atributo `style` en función del tamaño, o `None` si no aplica.
    #[inline]
    pub(crate) fn to_style(self) -> Option<String> {
        match self {
            Self::Auto => None,
            Self::Dimensions(w, h) => Some(format!("width: {w}; height: {h};")),
            Self::Width(w) => Some(format!("width: {w};")),
            Self::Height(h) => Some(format!("height: {h};")),
            Self::Both(v) => Some(format!("width: {v}; height: {v};")),
        }
    }
}

// **< Source >*************************************************************************************

/// Especifica la **fuente** para publicar una imagen ([`Image`](crate::theme::Image)).
#[derive(AutoDefault, Clone, Debug, PartialEq)]
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

impl Source {
    const IMG_FLUID: &str = "img-fluid";
    const IMG_THUMBNAIL: &str = "img-thumbnail";

    // Devuelve la clase base asociada a la imagen según la fuente.
    #[inline]
    fn as_str(&self) -> &'static str {
        match self {
            Source::Logo(_) | Source::Responsive(_) => Self::IMG_FLUID,
            Source::Thumbnail(_) => Self::IMG_THUMBNAIL,
            Source::Plain(_) => "",
        }
    }

    /* Añade la clase base asociada a la imagen según la fuente a la cadena de clases (reservado).
    #[inline]
    pub(crate) fn push_class(&self, classes: &mut String) {
        let s = self.as_str();
        if s.is_empty() {
            return;
        }
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(s);
    } */

    // Devuelve la clase asociada a la imagen según la fuente.
    #[inline]
    pub(crate) fn to_class(&self) -> String {
        let s = self.as_str();
        if s.is_empty() {
            String::new()
        } else {
            let mut class = String::with_capacity(s.len());
            class.push_str(s);
            class
        }
    }
}
