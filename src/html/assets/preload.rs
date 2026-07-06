use crate::core::component::Context;
use crate::html::assets::Asset;
use crate::html::{Markup, html};
use crate::{AutoDefault, CowStr, Weight, util};

// Tipo de recurso para el atributo `as` de un recurso para precargar.
// Informa al navegador de la naturaleza del recurso para que pueda asignarle la prioridad correcta
// y aplicar la política de caché adecuada.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub(crate) enum AsType {
    // Fuente web (`@font-face`). Implica `crossorigin` automático.
    #[default]
    Font,
    // Hoja de estilos CSS.
    Style,
    // Script JavaScript.
    Script,
    // Imagen.
    Image,
}

impl AsType {
    const fn as_str(self) -> &'static str {
        match self {
            AsType::Font => "font",
            AsType::Style => "style",
            AsType::Script => "script",
            AsType::Image => "image",
        }
    }
}

/// Define un recurso **Preload** para precargar en el documento.
///
/// Indica al navegador que descargue el recurso con alta prioridad antes de que el analizador del
/// HTML lo descubra de forma natural, reduciendo la latencia percibida.
///
/// > **Nota**
/// > Los recursos deben estar disponibles en el servidor web de la aplicación. Pueden servirse
/// > usando [`serve_static_files!`](crate::serve_static_files).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// // Precarga una fuente web (crossorigin se activa automáticamente).
/// let preload = Preload::font("/assets/fonts/inter.woff2").with_weight(-100);
///
/// // Precarga una imagen WebP crítica para la métrica "Largest Contentful Paint" (LCP).
/// let preload = Preload::webp("/assets/img/hero.webp");
/// ```
#[derive(AutoDefault)]
pub struct Preload {
    path: CowStr,      // Ruta del recurso (href).
    as_type: AsType,   // Tipo de recurso (atributo `as`).
    mime_type: CowStr, // Tipo MIME (atributo `type`), vacío si no se especifica.
    crossorigin: bool, // Activa el atributo `crossorigin`.
    version: CowStr,   // Versión del recurso para la caché del navegador.
    weight: Weight,    // Peso que determina el orden.
}

impl Preload {
    /// Precarga una fuente web.
    ///
    /// Equivale a `<link rel="preload" as="font" type="font/woff2" href="..." crossorigin>`.
    ///
    /// El atributo `crossorigin` se activa siempre porque `@font-face` usa CORS internamente; sin
    /// él el navegador descargaría el recurso dos veces.
    pub fn font(path: impl Into<CowStr>) -> Self {
        Self {
            path: path.into(),
            as_type: AsType::Font,
            mime_type: "font/woff2".into(),
            crossorigin: true,
            ..Default::default()
        }
    }

    /// Precarga una hoja de estilos CSS.
    ///
    /// Equivale a `<link rel="preload" as="style" href="...">`.
    pub fn style(path: impl Into<CowStr>) -> Self {
        Self {
            path: path.into(),
            as_type: AsType::Style,
            ..Default::default()
        }
    }

    /// Precarga un script JavaScript.
    ///
    /// Equivale a `<link rel="preload" as="script" href="...">`.
    pub fn script(path: impl Into<CowStr>) -> Self {
        Self {
            path: path.into(),
            as_type: AsType::Script,
            ..Default::default()
        }
    }

    /// Precarga una imagen.
    ///
    /// Equivale a `<link rel="preload" as="image" href="...">`. Adecuado para formatos con soporte
    /// universal (JPEG, PNG, GIF); para WebP o AVIF usar [`Self::webp()`] o [`Self::avif()`], que
    /// añaden el atributo `type` para que el navegador omita la descarga si no soporta el formato.
    pub fn image(path: impl Into<CowStr>) -> Self {
        Self {
            path: path.into(),
            as_type: AsType::Image,
            ..Default::default()
        }
    }

    /// Precarga una imagen en formato WebP.
    ///
    /// Equivale a `<link rel="preload" as="image" type="image/webp" href="...">`.
    ///
    /// El atributo `type` indica al navegador que omita la descarga si no soporta WebP.
    pub fn webp(path: impl Into<CowStr>) -> Self {
        Self {
            path: path.into(),
            as_type: AsType::Image,
            mime_type: "image/webp".into(),
            ..Default::default()
        }
    }

    /// Precarga una imagen en formato AVIF.
    ///
    /// Equivale a `<link rel="preload" as="image" type="image/avif" href="...">`.
    ///
    /// El atributo `type` indica al navegador que omita la descarga si no soporta AVIF.
    pub fn avif(path: impl Into<CowStr>) -> Self {
        Self {
            path: path.into(),
            as_type: AsType::Image,
            mime_type: "image/avif".into(),
            ..Default::default()
        }
    }

    // **< Preload BUILDER >************************************************************************

    /// Asocia una versión al recurso (usada para control de la caché del navegador).
    ///
    /// Si `version` está vacío, no se añade ningún parámetro a la URL.
    pub fn with_version(mut self, version: impl Into<CowStr>) -> Self {
        self.version = version.into();
        self
    }

    /// Modifica el peso del recurso.
    ///
    /// Los recursos se renderizan de menor a mayor peso. Por defecto es `0`, que respeta el orden
    /// de creación.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Activa el atributo `crossorigin`.
    ///
    /// Necesario para recursos servidos desde otro origen. Para fuentes (`as="font"`) se activa
    /// automáticamente.
    pub fn with_crossorigin(mut self) -> Self {
        self.crossorigin = true;
        self
    }

    /// Especifica el tipo MIME del recurso (atributo `type`).
    ///
    /// Permite que el navegador omita la descarga si no soporta el formato indicado. Los
    /// constructores [`Self::font()`], [`Self::webp()`] y [`Self::avif()`] lo establecen
    /// automáticamente; este método cubre otros formatos o permite cambiar el valor por defecto.
    pub fn with_mime_type(mut self, mime: impl Into<CowStr>) -> Self {
        self.mime_type = mime.into();
        self
    }
}

impl Asset for Preload {
    fn name(&self) -> &str {
        &self.path
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn render(&self, _cx: &mut Context) -> Markup {
        html! {
            link
                rel="preload"
                href=(util::join_pair!(&self.path, "?v=", &self.version))
                as=(self.as_type.as_str())
                type=[(!self.mime_type.is_empty()).then_some(self.mime_type.as_ref())]
                crossorigin[self.crossorigin];
        }
    }
}
