use crate::core::component::Context;
use crate::html::{html, Markup};
use crate::{AutoDefault, CowStr};

/// Un **Favicon** es un recurso gráfico que usa el navegador como icono asociado al sitio.
///
/// Es universalmente aceptado para mostrar el icono del sitio (`.ico`, `.png`, `.svg`, ...) en
/// pestañas, marcadores o accesos directos.
///
/// Este tipo permite construir de forma fluida las distintas variantes de un *favicon*, ya sea un
/// icono estándar, un icono Apple para la pantalla de inicio, o un icono para Safari con color.
/// También puede aplicar colores al tema o configuraciones específicas para *tiles* de Windows.
///
/// > **Nota**
/// > Los archivos de los iconos deben estar disponibles en el servidor web de la aplicación. Pueden
/// > servirse usando [`static_files_service!`](crate::static_files_service).
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let favicon = Favicon::new()
///     // Estándar de facto admitido por todos los navegadores.
///     .with_icon("/icons/favicon.ico")
///
///     // Variante del favicon con tamaños explícitos: 32×32 y 16×16.
///     .with_icon_for_sizes("/icons/favicon-32.png", "32x32")
///     .with_icon_for_sizes("/icons/favicon-16.png", "16x16")
///
///     // Icono específico para accesos directos en la pantalla de inicio de iOS.
///     .with_apple_touch_icon("/icons/apple-touch-icon.png", "180x180")
///
///     // Icono vectorial con color dinámico para pestañas ancladas en Safari.
///     .with_mask_icon("/icons/safari-pinned-tab.svg", "#5bbad5")
///
///     // Personaliza la barra superior del navegador en Android Chrome (y soportado en otros).
///     .with_theme_color("#ffffff")
///
///     // Personalizaciones específicas para "tiles" en Windows.
///     .with_ms_tile_color("#da532c")
///     .with_ms_tile_image("/icons/mstile-144x144.png");
/// ```
#[derive(AutoDefault)]
pub struct Favicon(Vec<Item>);

/// Elementos que componen un favicon.
#[derive(Clone, Debug)]
enum Item {
    /// Etiqueta `<link>` para iconos.
    ///
    /// - `rel`: `"icon"`, `"apple-touch-icon"`, `"mask-icon"`, etc.
    /// - `href`: URL/ruta del recurso.
    /// - `sizes`: tamaños opcionales (p. ej. `"32x32"` o `"16x16 32x32"`).
    /// - `color`: color opcional (relevante para `mask-icon`).
    /// - `mime`: tipo MIME inferido por la extensión del recurso.
    Icon {
        rel: &'static str,
        href: CowStr,
        sizes: Option<CowStr>,
        color: Option<CowStr>,
        mime: Option<&'static str>,
    },

    /// Etiqueta `<meta>` para configuraciones del navegador/sistema.
    ///
    /// - `name`: `"theme-color"`, `"msapplication-TileColor"`, `"msapplication-TileImage"`, etc.
    /// - `content`: valor asociado.
    Meta { name: &'static str, content: CowStr },
}

impl Favicon {
    /// Crea un nuevo `Favicon` vacío.
    ///
    /// Equivalente a `Favicon::default()`. Se recomienda iniciar la secuencia de configuración
    /// desde aquí.
    pub fn new() -> Self {
        Self::default()
    }

    // **< Favicon BUILDER >************************************************************************

    /// Le añade un icono genérico apuntando a `image`. El tipo MIME se infiere automáticamente a
    /// partir de la extensión.
    pub fn with_icon(self, image: impl Into<CowStr>) -> Self {
        self.add_icon_item("icon", image.into(), None, None)
    }

    /// Le añade un icono genérico con atributo `sizes`, útil para indicar resoluciones específicas.
    ///
    /// El atributo `sizes` informa al navegador de las dimensiones de la imagen para que seleccione
    /// el recurso más adecuado. Puede enumerar varias dimensiones separadas por espacios, p. ej.
    /// `"16x16 32x32 48x48"` o usar `any` para iconos escalables (SVG).
    ///
    /// No es imprescindible, pero puede mejorar la selección del icono más adecuado.
    pub fn with_icon_for_sizes(self, image: impl Into<CowStr>, sizes: impl Into<CowStr>) -> Self {
        self.add_icon_item("icon", image.into(), Some(sizes.into()), None)
    }

    /// Le añade un *Apple Touch Icon*, usado por dispositivos iOS para las pantallas de inicio.
    ///
    /// Se recomienda indicar también el tamaño, p. ej. `"180x180"`.
    pub fn with_apple_touch_icon(self, image: impl Into<CowStr>, sizes: impl Into<CowStr>) -> Self {
        self.add_icon_item("apple-touch-icon", image.into(), Some(sizes.into()), None)
    }

    /// Le añade un icono para el navegador Safari, con un color dinámico.
    ///
    /// El atributo `color` lo usa Safari para colorear el trazado SVG cuando el icono se muestra en
    /// modo *Pinned Tab*. Aunque Safari 12+ acepta *favicons normales*, este método garantiza
    /// compatibilidad con versiones anteriores.
    pub fn with_mask_icon(self, image: impl Into<CowStr>, color: impl Into<CowStr>) -> Self {
        self.add_icon_item("mask-icon", image.into(), None, Some(color.into()))
    }

    /// Define el color del tema (`<meta name="theme-color">`).
    ///
    /// Lo usan algunos navegadores para colorear la barra de direcciones o interfaces.
    pub fn with_theme_color(mut self, color: impl Into<CowStr>) -> Self {
        self.0.push(Item::Meta {
            name: "theme-color",
            content: color.into(),
        });
        self
    }

    /// Define el color del *tile* en Windows (`<meta name="msapplication-TileColor">`).
    pub fn with_ms_tile_color(mut self, color: impl Into<CowStr>) -> Self {
        self.0.push(Item::Meta {
            name: "msapplication-TileColor",
            content: color.into(),
        });
        self
    }

    /// Define la imagen del *tile* en Windows (`<meta name="msapplication-TileImage">`).
    pub fn with_ms_tile_image(mut self, image: impl Into<CowStr>) -> Self {
        self.0.push(Item::Meta {
            name: "msapplication-TileImage",
            content: image.into(),
        });
        self
    }

    // **< Favicon HELPERS >************************************************************************

    /// Infiere el tipo MIME (`type="..."`) a partir de la extensión del recurso.
    #[inline]
    fn infer_mime(href: &str) -> Option<&'static str> {
        // Ignora query/fragment sin asignaciones (p. ej. ".png?v=1" o ".svg#v2").
        let href = href.split_once('#').map(|(s, _)| s).unwrap_or(href);
        let href = href.split_once('?').map(|(s, _)| s).unwrap_or(href);

        let (_, ext) = href.rsplit_once('.')?;

        match ext.len() {
            3 if ext.eq_ignore_ascii_case("gif") => Some("image/gif"),
            3 if ext.eq_ignore_ascii_case("ico") => Some("image/x-icon"),
            3 if ext.eq_ignore_ascii_case("jpg") => Some("image/jpeg"),
            3 if ext.eq_ignore_ascii_case("png") => Some("image/png"),
            3 if ext.eq_ignore_ascii_case("svg") => Some("image/svg+xml"),
            4 if ext.eq_ignore_ascii_case("avif") => Some("image/avif"),
            4 if ext.eq_ignore_ascii_case("jpeg") => Some("image/jpeg"),
            4 if ext.eq_ignore_ascii_case("webp") => Some("image/webp"),
            _ => None,
        }
    }

    /// Centraliza la creación de los elementos `<link>`.
    ///
    /// - `icon_rel`: indica el tipo de recurso (`"icon"`, `"apple-touch-icon"`, etc.).
    /// - `href`: URL del recurso.
    /// - `sizes`: tamaños opcionales.
    /// - `color`: color opcional (solo relevante para `mask-icon`).
    ///
    /// También infiere automáticamente el tipo MIME (`type`) según la extensión del archivo.
    fn add_icon_item(
        mut self,
        icon_rel: &'static str,
        icon_source: CowStr,
        icon_sizes: Option<CowStr>,
        icon_color: Option<CowStr>,
    ) -> Self {
        let mime = Self::infer_mime(icon_source.as_ref());
        self.0.push(Item::Icon {
            rel: icon_rel,
            href: icon_source,
            sizes: icon_sizes,
            color: icon_color,
            mime,
        });
        self
    }

    // **< Favicon RENDER >*************************************************************************

    /// Renderiza el **Favicon** completo con todas las etiquetas declaradas.
    ///
    /// El parámetro `Context` se acepta por coherencia con el resto de *assets*, aunque en este
    /// caso es ignorado.
    pub fn render(&self, _cx: &mut Context) -> Markup {
        html! {
            @for item in &self.0 {
                @match item {
                    Item::Icon { rel, href, sizes, color, mime } => {
                        link
                            rel=(rel)
                            type=[*mime]
                            sizes=[sizes.as_deref()]
                            color=[color.as_deref()]
                            href=(href.as_ref());
                    }
                    Item::Meta { name, content } => {
                        meta name=(name) content=(content.as_ref());
                    }
                }
            }
        }
    }
}
