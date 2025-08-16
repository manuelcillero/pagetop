use crate::html::{html, Markup, Render};
use crate::AutoDefault;

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
/// use pagetop::prelude::*;
///
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
pub struct Favicon(Vec<Markup>);

impl Favicon {
    /// Crea un nuevo `Favicon` vacío.
    ///
    /// Equivalente a `Favicon::default()`. Se recomienda iniciar la secuencia de configuración
    /// desde aquí.
    pub fn new() -> Self {
        Favicon::default()
    }

    // Favicon BUILDER *****************************************************************************

    /// Le añade un icono genérico apuntando a `image`. El tipo MIME se infiere automáticamente a
    /// partir de la extensión.
    pub fn with_icon(self, image: impl Into<String>) -> Self {
        self.add_icon_item("icon", image.into(), None, None)
    }

    /// Le añade un icono genérico con atributo `sizes`, útil para indicar resoluciones específicas.
    ///
    /// El atributo `sizes` informa al navegador de las dimensiones de la imagen para que seleccione
    /// el recurso más adecuado. Puede enumerar varias dimensiones separadas por espacios, p.ej.
    /// `"16x16 32x32 48x48"` o usar `any` para iconos escalables (SVG).
    ///
    /// No es imprescindible, pero puede mejorar la selección del icono más adecuado.
    pub fn with_icon_for_sizes(self, image: impl Into<String>, sizes: impl Into<String>) -> Self {
        self.add_icon_item("icon", image.into(), Some(sizes.into()), None)
    }

    /// Le añade un *Apple Touch Icon*, usado por dispositivos iOS para las pantallas de inicio.
    ///
    /// Se recomienda indicar también el tamaño, p.ej. `"256x256"`.
    pub fn with_apple_touch_icon(self, image: impl Into<String>, sizes: impl Into<String>) -> Self {
        self.add_icon_item("apple-touch-icon", image.into(), Some(sizes.into()), None)
    }

    /// Le añade un icono para el navegador Safari, con un color dinámico.
    ///
    /// El atributo `color` lo usa Safari para colorear el trazado SVG cuando el icono se muestra en
    /// modo *Pinned Tab*. Aunque Safari 12+ acepta *favicons normales*, este método garantiza
    /// compatibilidad con versiones anteriores.
    pub fn with_mask_icon(self, image: impl Into<String>, color: impl Into<String>) -> Self {
        self.add_icon_item("mask-icon", image.into(), None, Some(color.into()))
    }

    /// Define el color del tema (`<meta name="theme-color">`).
    ///
    /// Lo usan algunos navegadores para colorear la barra de direcciones o interfaces.
    pub fn with_theme_color(mut self, color: impl Into<String>) -> Self {
        self.0.push(html! {
            meta name="theme-color" content=(color.into());
        });
        self
    }

    /// Define el color del *tile* en Windows (`<meta name="msapplication-TileColor">`).
    pub fn with_ms_tile_color(mut self, color: impl Into<String>) -> Self {
        self.0.push(html! {
            meta name="msapplication-TileColor" content=(color.into());
        });
        self
    }

    /// Define la imagen del *tile* en Windows (`<meta name="msapplication-TileImage">`).
    pub fn with_ms_tile_image(mut self, image: impl Into<String>) -> Self {
        self.0.push(html! {
            meta name="msapplication-TileImage" content=(image.into());
        });
        self
    }

    // Función interna que centraliza la creación de las etiquetas `<link>`.
    //
    // - `icon_rel`: indica el tipo de recurso (`"icon"`, `"apple-touch-icon"`, etc.).
    // - `icon_source`: URL del recurso.
    // - `icon_sizes`: tamaños opcionales.
    // - `icon_color`: color opcional (solo relevante para `mask-icon`).
    //
    // También infiere automáticamente el tipo MIME (`type`) según la extensión del archivo.
    fn add_icon_item(
        mut self,
        icon_rel: &str,
        icon_source: String,
        icon_sizes: Option<String>,
        icon_color: Option<String>,
    ) -> Self {
        let icon_type = match icon_source.rfind('.') {
            Some(i) => match icon_source[i..].to_owned().to_lowercase().as_str() {
                ".avif" => Some("image/avif"),
                ".gif" => Some("image/gif"),
                ".ico" => Some("image/x-icon"),
                ".jpg" | ".jpeg" => Some("image/jpeg"),
                ".png" => Some("image/png"),
                ".svg" => Some("image/svg+xml"),
                ".webp" => Some("image/webp"),
                _ => None,
            },
            _ => None,
        };
        self.0.push(html! {
            link
                rel=(icon_rel)
                type=[(icon_type)]
                sizes=[(icon_sizes)]
                color=[(icon_color)]
                href=(icon_source);
        });
        self
    }
}

impl Render for Favicon {
    fn render(&self) -> Markup {
        html! {
            @for item in &self.0 {
                (item)
            }
        }
    }
}
