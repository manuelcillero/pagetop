use crate::core::component::Context;
use crate::html::assets::Asset;
use crate::html::{html, Markup, PreEscaped};
use crate::{join, join_pair, AutoDefault, Weight};

// Define el origen del recurso JavaScript y cómo debe cargarse en el navegador.
//
// Los distintos modos de carga permiten optimizar el rendimiento y controlar el comportamiento del
// script en relación con el análisis del documento HTML y la ejecución del resto de scripts.
//
// - [`From`]        – Carga estándar con la etiqueta `<script src="...">`.
// - [`Defer`]       – Igual que [`From`], pero con el atributo `defer`, descarga en paralelo y se
//                     ejecuta tras el análisis del documento HTML, respetando el orden de
//                     aparición.
// - [`Async`]       – Igual que [`From`], pero con el atributo `async`, descarga en paralelo y se
//                     ejecuta en cuanto esté listo, **sin garantizar** el orden relativo respecto a
//                     otros scripts.
// - [`Inline`]      – Inserta el código directamente en la etiqueta `<script>`.
// - [`OnLoad`]      – Inserta el código JavaScript y lo ejecuta tras el evento `DOMContentLoaded`.
// - [`OnLoadAsync`] – Igual que [`OnLoad`], pero con manejador asíncrono (`async`), útil si dentro
//                     del código JavaScript se utiliza `await`.
#[derive(AutoDefault)]
enum Source {
    #[default]
    From(String),
    Defer(String),
    Async(String),
    // `name`, `closure(Context) -> String`.
    Inline(String, Box<dyn Fn(&mut Context) -> String + Send + Sync>),
    // `name`, `closure(Context) -> String` (se ejecuta tras `DOMContentLoaded`).
    OnLoad(String, Box<dyn Fn(&mut Context) -> String + Send + Sync>),
    // `name`, `closure(Context) -> String` (manejador `async` tras `DOMContentLoaded`).
    OnLoadAsync(String, Box<dyn Fn(&mut Context) -> String + Send + Sync>),
}

/// Define un recurso **JavaScript** para incluir en un documento HTML.
///
/// Este tipo permite añadir scripts externos o embebidos con distintas estrategias de carga
/// (`defer`, `async`, *inline*, etc.) y [pesos](crate::Weight) para controlar el orden de inserción
/// en el documento.
///
/// > **Nota**
/// > Los archivos de los scripts deben estar disponibles en el servidor web de la aplicación.
/// > Pueden servirse usando [`static_files_service!`](crate::static_files_service).
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Script externo con carga diferida, versión de caché y prioridad en el renderizado.
/// let script = JavaScript::defer("/assets/js/app.js")
///     .with_version("1.2.3")
///     .with_weight(-10);
///
/// // Script embebido que se ejecuta tras la carga del documento.
/// let script = JavaScript::on_load("init_tooltips", |_| r#"
///     const tooltips = document.querySelectorAll('[data-tooltip]');
///     for (const el of tooltips) {
///         el.addEventListener('mouseenter', showTooltip);
///     }
/// "#.to_string());
///
/// // Script embebido con manejador asíncrono (`async`) que puede usar `await`.
/// let mut cx = Context::new(None).with_param("user_id", 7u32);
///
/// let js = JavaScript::on_load_async("hydrate", |cx| {
///     // Ejemplo: lectura de un parámetro del contexto para inyectarlo en el código.
///     let uid: u32 = cx.param_or_default("user_id");
///     format!(r#"
///         const USER_ID = {};
///         await Promise.resolve(USER_ID);
///         // Aquí se podría hidratar la interfaz o cargar módulos dinámicos:
///         // await import('/assets/js/hydrate.js');
///     "#, uid)
/// });
/// ```
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct JavaScript {
    source : Source, // Fuente y estrategia de carga del script.
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

    /// Crea un **script externo** con el atributo `defer`, que se descarga en paralelo y se ejecuta
    /// tras analizar completamente el documento HTML, **respetando el orden** de inserción.
    ///
    /// Equivale a `<script src="..." defer>`. Suele ser la opción recomendada para scripts no
    /// críticos.
    pub fn defer(path: impl Into<String>) -> Self {
        JavaScript {
            source: Source::Defer(path.into()),
            ..Default::default()
        }
    }

    /// Crea un **script externo** con el atributo `async`, que se descarga en paralelo y se ejecuta
    /// tan pronto como esté disponible.
    ///
    /// Equivale a `<script src="..." async>`. **No garantiza** el orden relativo con otros scripts.
    pub fn asynchronous(path: impl Into<String>) -> Self {
        JavaScript {
            source: Source::Async(path.into()),
            ..Default::default()
        }
    }

    /// Crea un **script embebido** directamente en el documento HTML.
    ///
    /// Equivale a `<script>...</script>`. El parámetro `name` se usa como identificador interno del
    /// script.
    ///
    /// La función *closure* recibirá el [`Context`] por si se necesita durante el renderizado.
    pub fn inline<F>(name: impl Into<String>, f: F) -> Self
    where
        F: Fn(&mut Context) -> String + Send + Sync + 'static,
    {
        JavaScript {
            source: Source::Inline(name.into(), Box::new(f)),
            ..Default::default()
        }
    }

    /// Crea un **script embebido** que se ejecuta cuando **el DOM está listo**.
    ///
    /// El código se envuelve en un `addEventListener('DOMContentLoaded',function(){...})` que lo
    /// ejecuta tras analizar el documento HTML, **no** espera imágenes ni otros recursos externos.
    /// Útil para inicializaciones que no dependen de `await`. El parámetro `name` se usa como
    /// identificador interno del script.
    ///
    /// Los scripts con `defer` se ejecutan antes de `DOMContentLoaded`.
    ///
    /// La función *closure* recibirá el [`Context`] por si se necesita durante el renderizado.
    pub fn on_load<F>(name: impl Into<String>, f: F) -> Self
    where
        F: Fn(&mut Context) -> String + Send + Sync + 'static,
    {
        JavaScript {
            source: Source::OnLoad(name.into(), Box::new(f)),
            ..Default::default()
        }
    }

    /// Crea un **script embebido** con un **manejador asíncrono**.
    ///
    /// El código se envuelve en un `addEventListener('DOMContentLoaded',async()=>{...})`, que
    /// emplea una función `async` para que el cuerpo devuelto por la función *closure* pueda usar
    /// `await`. Ideal para hidratar la interfaz, cargar módulos dinámicos o realizar lecturas
    /// iniciales.
    ///
    /// La función *closure* recibirá el [`Context`] por si se necesita durante el renderizado.
    pub fn on_load_async<F>(name: impl Into<String>, f: F) -> Self
    where
        F: Fn(&mut Context) -> String + Send + Sync + 'static,
    {
        JavaScript {
            source: Source::OnLoadAsync(name.into(), Box::new(f)),
            ..Default::default()
        }
    }

    // **< JavaScript BUILDER >*********************************************************************

    /// Asocia una **versión** al recurso (usada para control de la caché del navegador).
    ///
    /// Si `version` está vacío, **no** se añade ningún parámetro a la URL.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Modifica el **peso** del recurso.
    ///
    /// Los recursos se renderizan de menor a mayor peso. Por defecto es `0`, que respeta el orden
    /// de creación.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}

impl Asset for JavaScript {
    /// Devuelve el nombre del recurso, utilizado como clave única.
    ///
    /// Para scripts externos es la ruta del recurso; para scripts embebidos, un identificador.
    fn name(&self) -> &str {
        match &self.source {
            Source::From(path) => path,
            Source::Defer(path) => path,
            Source::Async(path) => path,
            Source::Inline(name, _) => name,
            Source::OnLoad(name, _) => name,
            Source::OnLoadAsync(name, _) => name,
        }
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    // **< JavaScript RENDER >**********************************************************************

    fn render(&self, cx: &mut Context) -> Markup {
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
            Source::Inline(_, f) => html! {
                script { (PreEscaped((f)(cx))) };
            },
            Source::OnLoad(_, f) => html! { script { (PreEscaped(join!(
                "document.addEventListener(\"DOMContentLoaded\",function(){", (f)(cx), "});"
            ))) } },
            Source::OnLoadAsync(_, f) => html! { script { (PreEscaped(join!(
                "document.addEventListener(\"DOMContentLoaded\",async()=>{", (f)(cx), "});"
            ))) } },
        }
    }
}
