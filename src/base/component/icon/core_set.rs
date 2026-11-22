use crate::prelude::*;

/// Pequeño conjunto de **iconos SVG** incluido en PageTop.
///
/// Son iconos de trazo sencillos, sin dependencias ni recursos externos, que funcionan con
/// cualquier tema. Incluye los que usa el panel de administración y otros de uso general, y sirve
/// de ejemplo de cómo integrar un conjunto de iconos (ver [`icon::Source`](super::Source)).
///
/// Como el icono conserva su identidad hasta el renderizado ([`icon::Source::Core`]), un tema puede
/// sustituirlo por el equivalente de su propio conjunto. Así una extensión puede usarlos sin
/// depender de ningún tema, y verlos con el aspecto del que esté activo.
///
/// [`icon::Source::Core`]: super::Source::Core
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let check = Icon::with(icon::Core::Check).with_label(Lc::n("Done"));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Core {
    /// Gráfico de barras, para informes.
    BarChart,
    /// Marca de verificación.
    Check,
    /// Flecha angular hacia abajo.
    ChevronDown,
    /// Flecha angular hacia la izquierda.
    ChevronLeft,
    /// Flecha angular hacia la derecha.
    ChevronRight,
    /// Flecha angular hacia arriba.
    ChevronUp,
    /// Aspa, para cerrar o descartar.
    Close,
    /// Engranaje, para la administración.
    Gear,
    /// Casa, para la página de inicio.
    Home,
    /// Llave, para permisos o accesos.
    Key,
    /// Capas apiladas, para la estructura.
    Layers,
    /// Signo menos, para quitar.
    Minus,
    /// Grupo de personas.
    People,
    /// Una persona.
    Person,
    /// Signo más, para añadir.
    Plus,
    /// Interrogación en un círculo, para la ayuda.
    QuestionCircle,
    /// Escudo, para roles o seguridad.
    Shield,
    /// Flecha que sale de una puerta, para cerrar sesión.
    SignOut,
    /// Controles deslizantes, para la configuración o los ajustes.
    Sliders,
}

// Envuelve las formas de un icono, dibujadas en una cuadrícula de 16 x 16, en el trazo común del
// conjunto. El resultado es una cadena estática, que se copia tal cual al pintar el icono.
macro_rules! stroked {
    ( $($shape:expr),+ $(,)? ) => {
        concat!(
            r#"<g fill="none" stroke="currentColor" stroke-width="1.5" "#,
            r#"stroke-linecap="round" stroke-linejoin="round">"#,
            $($shape,)+
            "</g>",
        )
    };
}

impl Core {
    // Contenido del `<svg>` del icono, para el `viewBox` común `0 0 16 16`.
    pub(super) const fn svg(&self) -> &'static str {
        match self {
            Self::BarChart => stroked!(r#"<path d="M2 14h12M4 14V9M8 14V3.5M12 14V6.5"/>"#),
            Self::Check => stroked!(r#"<path d="M3 8.5l3 3 7-7"/>"#),
            Self::ChevronDown => stroked!(r#"<path d="M3 5.5l5 5 5-5"/>"#),
            Self::ChevronLeft => stroked!(r#"<path d="M10.5 3l-5 5 5 5"/>"#),
            Self::ChevronRight => stroked!(r#"<path d="M5.5 3l5 5-5 5"/>"#),
            Self::ChevronUp => stroked!(r#"<path d="M3 10.5l5-5 5 5"/>"#),
            Self::Close => stroked!(r#"<path d="M4 4l8 8M12 4l-8 8"/>"#),
            Self::Gear => stroked!(
                r#"<path d="M12.9 6.9 14.7 7.1 14.7 8.9 12.9 9.1 12.2 10.7 13.4 12.1 12.1 13.4 "#,
                r#"10.7 12.2 9.1 12.9 8.9 14.7 7.1 14.7 6.9 12.9 5.3 12.2 3.9 13.4 2.6 12.1 "#,
                r#"3.8 10.7 3.1 9.1 1.3 8.9 1.3 7.1 3.1 6.9 3.8 5.3 2.6 3.9 3.9 2.6 5.3 3.8 "#,
                r#"6.9 3.1 7.1 1.3 8.9 1.3 9.1 3.1 10.7 3.8 12.1 2.6 13.4 3.9 12.2 5.3z"/>"#,
                r#"<circle cx="8" cy="8" r="2"/>"#,
            ),
            Self::Home => {
                stroked!(r#"<path d="M2 7.5l6-5 6 5M3.5 6.5v7h9v-7M6.5 13.5V10h3v3.5"/>"#)
            }
            Self::Key => stroked!(
                r#"<circle cx="5" cy="11" r="2.5"/>"#,
                r#"<path d="M6.8 9.2l6.7-6.7M11 5l2 2M12.5 3.5L14 5"/>"#,
            ),
            Self::Layers => stroked!(
                r#"<path d="M8 2l6 3.25-6 3.25-6-3.25zM2 8.25l6 3.25 6-3.25"#,
                r#"M2 11.25l6 3.25 6-3.25"/>"#,
            ),
            Self::Minus => stroked!(r#"<path d="M3 8h10"/>"#),
            Self::People => stroked!(
                r#"<circle cx="6" cy="5.5" r="2.25"/>"#,
                r#"<path d="M1.5 13.5c.4-2.6 2.1-3.75 4.5-3.75s4.1 1.15 4.5 3.75"/>"#,
                r#"<circle cx="11.25" cy="5" r="1.75"/>"#,
                r#"<path d="M11.5 9.25c1.7.2 2.75 1.4 3 3.25"/>"#,
            ),
            Self::Person => stroked!(
                r#"<circle cx="8" cy="5" r="2.75"/>"#,
                r#"<path d="M2.5 14c.5-3 2.7-4.5 5.5-4.5s5 1.5 5.5 4.5"/>"#,
            ),
            Self::Plus => stroked!(r#"<path d="M8 3v10M3 8h10"/>"#),
            Self::QuestionCircle => stroked!(
                r#"<circle cx="8" cy="8" r="6.5"/>"#,
                r#"<path d="M6.25 6.25a1.75 1.75 0 1 1 2.5 1.6c-.5.25-.75.6-.75 1.15v.5"#,
                r#"M8 11.5v.01"/>"#,
            ),
            Self::Shield => stroked!(
                r#"<path d="M8 1.5l5.5 2v4c0 3.3-2.3 5.9-5.5 7-3.2-1.1-5.5-3.7-5.5-7v-4z"/>"#,
            ),
            Self::SignOut => stroked!(r#"<path d="M6.5 2.5h-3v11h3M10 5l3 3-3 3M13 8H6.5"/>"#),
            Self::Sliders => stroked!(
                r#"<path d="M2 4h2M6.5 4H14M2 8h7M11.5 8H14M2 12h3.5M8 12h6"/>"#,
                r#"<circle cx="5.25" cy="4" r="1.25"/>"#,
                r#"<circle cx="10.25" cy="8" r="1.25"/>"#,
                r#"<circle cx="6.75" cy="12" r="1.25"/>"#,
            ),
        }
    }
}

impl From<Core> for icon::Source {
    fn from(icon: Core) -> Self {
        icon::Source::Core(icon)
    }
}
