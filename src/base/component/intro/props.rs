use crate::prelude::*;

// **< Kind >***************************************************************************************

/// Tipo de introducción de un componente [`Intro`](super::Intro).
///
/// Distingue entre una presentación estándar sobre PageTop (la que usa, por ejemplo, la página de
/// bienvenida [`Welcome`]) y una introducción completamente personalizada.
///
/// Se fija al construir la introducción, con [`Intro::default()`] o [`Intro::new()`] para
/// [`Kind::PageTop`], o con [`Intro::custom()`] para [`Kind::Custom`], y se consulta con
/// [`Intro::kind()`].
///
/// [`Welcome`]: crate::base::extension::Welcome
/// [`Intro::default()`]: super::Intro::default
/// [`Intro::new()`]: super::Intro::new
/// [`Intro::custom()`]: super::Intro::custom
/// [`Intro::kind()`]: super::Intro::kind
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Presentación predeterminada. Muestra una introducción estándar de PageTop e incluye
    /// automáticamente *badges* con información sobre la última versión liberada, fecha del último
    /// lanzamiento y licencia de uso.
    #[default]
    PageTop,
    /// Modo personalizado. Usa la imagen de PageTop pero sin contenido de ningún tipo.
    Custom,
}

// **< Width >**************************************************************************************

/// Ancho máximo para el área de contenidos de un componente [`Intro`](super::Intro).
///
/// Sólo afecta a partir del punto de corte [`Breakpoint::Lg`], donde el área de contenidos se
/// muestra como un panel centrado. Por debajo ocupa siempre todo el ancho disponible. En pantallas
/// que no alcanzan el ancho máximo elegido, el panel se reduce para dejar un margen lateral.
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Width {
    /// Ancho estrecho.
    Narrow,
    /// Ancho por defecto.
    #[default]
    Normal,
    /// Ancho amplio.
    Wide,
    /// Ancho más amplio de los disponibles.
    Full,
}

impl Width {
    // Clases CSS del área de contenidos: la base más la modificadora, si el ancho no es el normal.
    pub(super) fn classes(self) -> &'static str {
        match self {
            Width::Narrow => "intro-text intro-text-narrow",
            Width::Normal => "intro-text",
            Width::Wide => "intro-text intro-text-wide",
            Width::Full => "intro-text intro-text-full",
        }
    }
}
