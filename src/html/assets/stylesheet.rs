use crate::core::component::Context;
use crate::html::assets::Asset;
use crate::html::{Markup, html};
use crate::{AutoDefault, CowStr, Weight, util};

/// Define el medio objetivo para una hoja de estilos.
///
/// Permite especificar en qué contexto se aplica el CSS, adaptándose a diferentes dispositivos o
/// situaciones de impresión.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum TargetMedia {
    /// Se aplica en todos los casos (el atributo `media` se omite).
    #[default]
    Default,
    /// Se aplica cuando el documento se imprime.
    Print,
    /// Se aplica en pantallas.
    Screen,
    /// Se aplica en dispositivos que convierten el texto a voz.
    Speech,
}

/// Devuelve el valor para el atributo `media` (`Some(...)`) o `None` para `Default`.
impl TargetMedia {
    const fn as_str(self) -> Option<&'static str> {
        match self {
            TargetMedia::Default => None,
            TargetMedia::Print => Some("print"),
            TargetMedia::Screen => Some("screen"),
            TargetMedia::Speech => Some("speech"),
        }
    }
}

/// Define un recurso **StyleSheet** para incluir en un documento HTML.
///
/// Este tipo permite incluir hojas de estilo CSS externas, con soporte para medios específicos
/// (`screen`, `print`, etc.) y [pesos](crate::Weight) que determinan el orden de inserción en el
/// documento.
///
/// Para declarar estilos embebidos en el documento (sin un archivo CSS externo), usar
/// [`AssetsOp::add_responsive_style()`](crate::core::component::AssetsOp::add_responsive_style) o
/// [`AssetsOp::add_responsive_styles()`](crate::core::component::AssetsOp::add_responsive_styles),
/// que asocian declaraciones de estilo `propiedad: valor` a una o varias clases CSS y las agrupan
/// en un único `<style>` en el `<head>` del documento.
///
/// > **Nota**
/// > Las hojas de estilo CSS deben estar disponibles en el servidor web de la aplicación. Pueden
/// > servirse usando [`serve_static_files!`](crate::serve_static_files).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// // Crea una hoja de estilos externa con control de versión y medio específico (`screen`).
/// let stylesheet = StyleSheet::from("/assets/css/main.css")
///     .with_version("2.0.1")
///     .for_media(TargetMedia::Screen)
///     .with_weight(-10);
/// ```
#[derive(AutoDefault)]
pub struct StyleSheet {
    path: CowStr,       // Ruta del recurso CSS externo.
    version: CowStr,    // Versión del recurso para la caché del navegador.
    media: TargetMedia, // Medio objetivo para los estilos (`print`, `screen`, ...).
    weight: Weight,     // Peso que determina el orden.
}

impl StyleSheet {
    /// Crea una hoja de estilos externa.
    ///
    /// Equivale a `<link rel="stylesheet" href="...">`.
    pub fn from(path: impl Into<CowStr>) -> Self {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }

    // **< StyleSheet BUILDER >*********************************************************************

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

    // **< StyleSheet HELPERS >*********************************************************************

    /// Especifica el medio donde se aplican los estilos.
    ///
    /// Según el argumento `media`:
    ///
    /// - `TargetMedia::Default` - Se aplica en todos los casos (medio por defecto).
    /// - `TargetMedia::Print`   - Se aplica cuando el documento se imprime.
    /// - `TargetMedia::Screen`  - Se aplica en pantallas.
    /// - `TargetMedia::Speech`  - Se aplica en dispositivos que convierten el texto a voz.
    pub fn for_media(mut self, media: TargetMedia) -> Self {
        self.media = media;
        self
    }
}

impl Asset for StyleSheet {
    /// Devuelve la ruta del recurso, utilizada como clave única.
    fn name(&self) -> &str {
        &self.path
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    // **< StyleSheet RENDER >**********************************************************************

    fn render(&self, _cx: &mut Context) -> Markup {
        html! {
            link
                rel="stylesheet"
                href=(util::join_pair!(&self.path, "?v=", &self.version))
                media=[self.media.as_str()];
        }
    }
}
