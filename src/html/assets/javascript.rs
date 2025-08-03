use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup, Render};
use crate::{join, join_pair, AutoDefault, Weight};

// Define el origen del recurso JavaScript y cómo debe cargarse en el navegador.
//
// Los distintos modos de carga permiten optimizar el rendimiento y controlar el comportamiento del
// script.
//
// - [`From`]   – Carga el script de forma estándar con la etiqueta `<script src="...">`.
// - [`Defer`]  – Igual que [`From`], pero con el atributo `defer`.
// - [`Async`]  – Igual que [`From`], pero con el atributo `async`.
// - [`Inline`] – Inserta el código directamente en la etiqueta `<script>`.
// - [`OnLoad`] – Inserta el código JavaScript y lo ejecuta tras el evento `DOMContentLoaded`.
#[derive(AutoDefault)]
enum Source {
    #[default]
    From(String),
    Defer(String),
    Async(String),
    Inline(String, String),
    OnLoad(String, String),
}

/// Define un recurso **JavaScript** para incluir en un documento HTML.
///
/// Este tipo permite añadir *scripts* externos o embebidos con distintas estrategias de carga
/// (`defer`, `async`, *inline*, etc.) y [pesos](crate::Weight) para controlar el orden de inserción
/// en el documento.
///
/// > **Nota**
/// > Los archivos de los *scripts* deben estar disponibles en el servidor web de la aplicación.
/// > Pueden incluirse en el proyecto utilizando [`include_files!`](crate::include_files) y servirse
/// > con [`include_files_service!`](crate::include_files_service).
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Script externo con carga diferida, versión para control de caché y prioriza el renderizado.
/// let script = JavaScript::defer("/assets/js/app.js")
///     .with_version("1.2.3")
///     .with_weight(-10);
///
/// // Script embebido que se ejecuta tras la carga del documento.
/// let script = JavaScript::on_load("init_tooltips", r#"
///     const tooltips = document.querySelectorAll('[data-tooltip]');
///     for (const el of tooltips) {
///         el.addEventListener('mouseenter', showTooltip);
///     }
/// "#);
/// ```
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct JavaScript {
    source : Source, // Fuente y modo de carga del script.
    version: String, // Versión del recurso para la caché del navegador.
    weight : Weight, // Peso que determina el orden.
}

impl JavaScript {
    /// Crea un **script externo** que se carga y ejecuta de forma inmediata, en orden con el resto
    /// del documento HTML.
    ///
    /// Equivale a `<script src="...">`.
    pub fn from(path: impl Into<String>) -> Self {
        JavaScript {
            source: Source::From(path.into()),
            ..Default::default()
        }
    }

    /// Crea un **script externo** con el atributo `defer`, que se carga en segundo plano y se
    /// ejecuta tras analizar completamente el documento HTML.
    ///
    /// Equivale a `<script src="..." defer>`. Útil para mantener el orden de ejecución y evitar
    /// bloquear el análisis del documento HTML.
    pub fn defer(path: impl Into<String>) -> Self {
        JavaScript {
            source: Source::Defer(path.into()),
            ..Default::default()
        }
    }

    /// Crea un **script externo** con el atributo `async`, que se carga y ejecuta de forma
    /// asíncrona tan pronto como esté disponible.
    ///
    /// Equivale a `<script src="..." async>`. La ejecución puede producirse fuera de orden respecto
    /// a otros *scripts*.
    pub fn asynchronous(path: impl Into<String>) -> Self {
        JavaScript {
            source: Source::Async(path.into()),
            ..Default::default()
        }
    }

    /// Crea un **script embebido** directamente en el documento HTML.
    ///
    /// Equivale a `<script>...</script>`. El parámetro `name` se usa como identificador interno del
    /// *script*.
    pub fn inline(name: impl Into<String>, script: impl Into<String>) -> Self {
        JavaScript {
            source: Source::Inline(name.into(), script.into()),
            ..Default::default()
        }
    }

    /// Crea un **script embebido** que se ejecuta automáticamente al terminar de cargarse el
    /// documento HTML.
    ///
    /// El código se envuelve automáticamente en un `addEventListener('DOMContentLoaded', ...)`. El
    /// parámetro `name` se usa como identificador interno del *script*.
    pub fn on_load(name: impl Into<String>, script: impl Into<String>) -> Self {
        JavaScript {
            source: Source::OnLoad(name.into(), script.into()),
            ..Default::default()
        }
    }

    // JavaScript BUILDER **************************************************************************

    /// Asocia una versión al recurso (usada para control de la caché del navegador).
    ///
    /// Si `version` está vacío, no se añade ningún parámetro a la URL.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
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
}

impl AssetsTrait for JavaScript {
    // Para *scripts* externos es la ruta; para *scripts* embebidos, un identificador.
    fn name(&self) -> &str {
        match &self.source {
            Source::From(path) => path,
            Source::Defer(path) => path,
            Source::Async(path) => path,
            Source::Inline(name, _) => name,
            Source::OnLoad(name, _) => name,
        }
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl Render for JavaScript {
    fn render(&self) -> Markup {
        match &self.source {
            Source::From(path) => html! {
                script src=(join_pair!(path, "?v=", self.version.as_str())) {};
            },
            Source::Defer(path) => html! {
                script src=(join_pair!(path, "?v=", self.version.as_str())) defer {};
            },
            Source::Async(path) => html! {
                script src=(join_pair!(path, "?v=", self.version.as_str())) async {};
            },
            Source::Inline(_, code) => html! {
                script { (code) };
            },
            Source::OnLoad(_, code) => html! { (join!(
                "document.addEventListener('DOMContentLoaded',function(){", code, "});"
            )) },
        }
    }
}
