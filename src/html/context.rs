use crate::core::theme::all::{theme_by_short_name, DEFAULT_THEME};
use crate::core::theme::ThemeRef;
use crate::core::TypeInfo;
use crate::html::{html, Markup};
use crate::html::{Assets, Favicon, JavaScript, StyleSheet};
use crate::locale::{LangId, LangMatch, LanguageIdentifier, DEFAULT_LANGID, FALLBACK_LANGID};
use crate::service::HttpRequest;
use crate::{builder_fn, join};

use std::any::Any;
use std::collections::HashMap;

/// Operaciones para modificar el contexto ([`Context`]) del documento.
pub enum AssetsOp {
    // Favicon.
    /// Define el *favicon* del documento. Sobrescribe cualquier valor anterior.
    SetFavicon(Option<Favicon>),
    /// Define el *favicon* solo si no se ha establecido previamente.
    SetFaviconIfNone(Favicon),

    // Stylesheets.
    /// Añade una hoja de estilos CSS al documento.
    AddStyleSheet(StyleSheet),
    /// Elimina una hoja de estilos por su ruta o identificador.
    RemoveStyleSheet(&'static str),

    // JavaScripts.
    /// Añade un *script* JavaScript al documento.
    AddJavaScript(JavaScript),
    /// Elimina un *script* por su ruta o identificador.
    RemoveJavaScript(&'static str),
}

/// Errores de acceso a parámetros dinámicos del contexto.
///
/// - [`ErrorParam::NotFound`]: la clave no existe.
/// - [`ErrorParam::TypeMismatch`]: la clave existe, pero el valor guardado no coincide con el tipo
///   solicitado. Incluye nombre de la clave (`key`), tipo esperado (`expected`) y tipo realmente
///   guardado (`saved`) para facilitar el diagnóstico.
#[derive(Debug)]
pub enum ErrorParam {
    NotFound,
    TypeMismatch {
        key: &'static str,
        expected: &'static str,
        saved: &'static str,
    },
}

/// Representa el contexto de un documento HTML.
///
/// Se crea internamente para manejar información relevante del documento, como la solicitud HTTP de
/// origen, el idioma, tema y composición para el renderizado, los recursos *favicon* ([`Favicon`]),
/// hojas de estilo ([`StyleSheet`]) y *scripts* ([`JavaScript`]), así como *parámetros dinámicos
/// heterogéneos* de contexto definidos en tiempo de ejecución.
///
/// # Ejemplos
///
/// Crea un nuevo contexto asociado a una solicitud HTTP:
///
/// ```rust
/// use pagetop::prelude::*;
///
/// fn new_context(request: HttpRequest) -> Context {
///     Context::new(Some(request))
///         // Establece el idioma del documento a español.
///         .with_langid(&LangMatch::resolve("es-ES"))
///         // Selecciona un tema (por su nombre corto).
///         .with_theme("aliner")
///         // Asigna un favicon.
///         .with_assets(AssetsOp::SetFavicon(Some(Favicon::new().with_icon("/favicon.ico"))))
///         // Añade una hoja de estilo externa.
///         .with_assets(AssetsOp::AddStyleSheet(StyleSheet::from("/css/style.css")))
///         // Añade un script JavaScript.
///         .with_assets(AssetsOp::AddJavaScript(JavaScript::defer("/js/main.js")))
///         // Añade un parámetro dinámico al contexto.
///         .with_param("usuario_id", 42)
/// }
/// ```
///
/// Y hace operaciones con un contexto dado:
///
/// ```rust
/// use pagetop::prelude::*;
///
/// fn use_context(cx: &mut Context) {
///     // Recupera el tema seleccionado.
///     let active_theme = cx.theme();
///     assert_eq!(active_theme.short_name(), "aliner");
///
///     // Recupera el parámetro a su tipo original.
///     let id: i32 = *cx.get_param::<i32>("usuario_id").unwrap();
///     assert_eq!(id, 42);
///
///     // Genera un identificador para un componente de tipo `Menu`.
///     struct Menu;
///     let unique_id = cx.required_id::<Menu>(None);
///     assert_eq!(unique_id, "menu-1"); // Si es el primero generado.
/// }
/// ```
#[rustfmt::skip]
pub struct Context {
    request    : Option<HttpRequest>,           // Solicitud HTTP de origen.
    langid     : &'static LanguageIdentifier,   // Identificador de idioma.
    theme      : ThemeRef,                      // Referencia al tema para renderizar.
    layout     : &'static str,                  // Composición del documento para renderizar.
    favicon    : Option<Favicon>,               // Favicon, si se ha definido.
    stylesheets: Assets<StyleSheet>,            // Hojas de estilo CSS.
    javascripts: Assets<JavaScript>,            // Scripts JavaScript.
    params     : HashMap<&'static str, (Box<dyn Any>, &'static str)>, // Parámetros definidos en tiempo de ejecución.
    id_counter : usize,                         // Contador para generar identificadores únicos.
}

impl Default for Context {
    fn default() -> Self {
        Context::new(None)
    }
}

impl Context {
    /// Crea un nuevo contexto asociado a una solicitud HTTP.
    ///
    /// El contexto inicializa el idioma, tema y composición por defecto, sin favicon ni recursos
    /// cargados.
    #[rustfmt::skip]
    pub fn new(request: Option<HttpRequest>) -> Self {
        // Se intenta DEFAULT_LANGID.
        let langid = DEFAULT_LANGID
            // Si es None evalúa la cadena de extracción desde la cabecera HTTP.
            .or_else(|| {
                request
                    // Se usa `as_ref()` sobre `Option<HttpRequest>` para no mover el valor.
                    .as_ref()
                    .and_then(|req| req.headers().get("Accept-Language"))
                    .and_then(|value| value.to_str().ok())
                    .and_then(|language| LangMatch::resolve(language).as_option())
            })
            // Si todo falla, se recurre a &FALLBACK_LANGID.
            .unwrap_or(&FALLBACK_LANGID);

        Context {
            request,
            langid,
            theme      : *DEFAULT_THEME,
            layout     : "default",
            favicon    : None,
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            params     : HashMap::default(),
            id_counter : 0,
        }
    }

    // Context BUILDER *****************************************************************************

    /// Modifica la fuente de idioma del documento.
    #[builder_fn]
    pub fn with_langid(mut self, language: &impl LangId) -> Self {
        self.langid = language.langid();
        self
    }

    /// Modifica el tema que se usará para renderizar el documento.
    ///
    /// Localiza el tema por su [`short_name()`](crate::core::AnyInfo::short_name), y si no aplica
    /// ninguno entonces usará el tema por defecto.
    #[builder_fn]
    pub fn with_theme(mut self, theme_name: &'static str) -> Self {
        self.theme = theme_by_short_name(theme_name).unwrap_or(*DEFAULT_THEME);
        self
    }

    /// Modifica la composición para renderizar el documento.
    #[builder_fn]
    pub fn with_layout(mut self, layout_name: &'static str) -> Self {
        self.layout = layout_name;
        self
    }

    /// Define los recursos del contexto usando [`AssetsOp`].
    #[builder_fn]
    pub fn with_assets(mut self, op: AssetsOp) -> Self {
        match op {
            // Favicon.
            AssetsOp::SetFavicon(favicon) => {
                self.favicon = favicon;
            }
            AssetsOp::SetFaviconIfNone(icon) => {
                if self.favicon.is_none() {
                    self.favicon = Some(icon);
                }
            }
            // Stylesheets.
            AssetsOp::AddStyleSheet(css) => {
                self.stylesheets.add(css);
            }
            AssetsOp::RemoveStyleSheet(path) => {
                self.stylesheets.remove(path);
            }
            // JavaScripts.
            AssetsOp::AddJavaScript(js) => {
                self.javascripts.add(js);
            }
            AssetsOp::RemoveJavaScript(path) => {
                self.javascripts.remove(path);
            }
        }
        self
    }

    // Context GETTERS *****************************************************************************

    /// Devuelve una referencia a la solicitud HTTP asociada, si existe.
    pub fn request(&self) -> Option<&HttpRequest> {
        self.request.as_ref()
    }

    /// Devuelve el tema que se usará para renderizar el documento.
    pub fn theme(&self) -> ThemeRef {
        self.theme
    }

    /// Devuelve la composición para renderizar el documento. Por defecto es `"default"`.
    pub fn layout(&self) -> &str {
        self.layout
    }

    // Context RENDER ******************************************************************************

    /// Renderiza los recursos del contexto.
    pub fn render_assets(&self) -> Markup {
        html! {
            @if let Some(favicon) = &self.favicon {
                (favicon)
            }
            (self.stylesheets)
            (self.javascripts)
        }
    }

    // Context PARAMS ******************************************************************************

    /// Añade o modifica un parámetro dinámico del contexto.
    ///
    /// El valor se guarda conservando el *nombre del tipo* real para mejorar los mensajes de error
    /// posteriores.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// use pagetop::prelude::*;
    ///
    /// let cx = Context::new(None)
    ///     .with_param("usuario_id", 42_i32)
    ///     .with_param("titulo", String::from("Hola"))
    ///     .with_param("flags", vec!["a", "b"]);
    /// ```
    #[builder_fn]
    pub fn with_param<T: 'static>(mut self, key: &'static str, value: T) -> Self {
        let type_name = TypeInfo::FullName.of::<T>();
        self.params.insert(key, (Box::new(value), type_name));
        self
    }

    /// Recupera un parámetro como [`Option`], simplificando el acceso.
    ///
    /// A diferencia de [`get_param`](Self::get_param), que devuelve un [`Result`] con información
    /// detallada de error, este método devuelve `None` tanto si la clave no existe como si el valor
    /// guardado no coincide con el tipo solicitado.
    ///
    /// Resulta útil en escenarios donde sólo interesa saber si el valor existe y es del tipo
    /// correcto, sin necesidad de diferenciar entre error de ausencia o de tipo.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use pagetop::prelude::*;
    ///
    /// let cx = Context::new(None).with_param("username", String::from("Alice"));
    ///
    /// // Devuelve Some(&String) si existe y coincide el tipo.
    /// assert_eq!(cx.param::<String>("username").map(|s| s.as_str()), Some("Alice"));
    ///
    /// // Devuelve None si no existe o si el tipo no coincide.
    /// assert!(cx.param::<i32>("username").is_none());
    /// assert!(cx.param::<String>("missing").is_none());
    ///
    /// // Acceso con valor por defecto.
    /// let user = cx.param::<String>("missing")
    ///     .cloned()
    ///     .unwrap_or_else(|| "visitor".to_string());
    /// assert_eq!(user, "visitor");
    /// ```
    pub fn param<T: 'static>(&self, key: &'static str) -> Option<&T> {
        self.get_param::<T>(key).ok()
    }

    /// Recupera una *referencia tipada* al parámetro solicitado.
    ///
    /// Devuelve:
    ///
    /// - `Ok(&T)` si la clave existe y el tipo coincide.
    /// - `Err(ErrorParam::NotFound)` si la clave no existe.
    /// - `Err(ErrorParam::TypeMismatch)` si la clave existe pero el tipo no coincide.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// use pagetop::prelude::*;
    ///
    /// let cx = Context::new(None)
    ///     .with_param("usuario_id", 42_i32)
    ///     .with_param("titulo", String::from("Hola"));
    ///
    /// let id: &i32 = cx.get_param("usuario_id").unwrap();
    /// let titulo: &String = cx.get_param("titulo").unwrap();
    ///
    /// // Error de tipo:
    /// assert!(cx.get_param::<String>("usuario_id").is_err());
    /// ```
    pub fn get_param<T: 'static>(&self, key: &'static str) -> Result<&T, ErrorParam> {
        let (any, type_name) = self.params.get(key).ok_or(ErrorParam::NotFound)?;
        any.downcast_ref::<T>()
            .ok_or_else(|| ErrorParam::TypeMismatch {
                key,
                expected: TypeInfo::FullName.of::<T>(),
                saved: *type_name,
            })
    }

    /// Elimina un parámetro del contexto. Devuelve `true` si la clave existía y se eliminó.
    ///
    /// Devuelve `false` en caso contrario. Usar cuando solo interesa borrar la entrada.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// use pagetop::prelude::*;
    ///
    /// let mut cx = Context::new(None).with_param("temp", 1u8);
    /// assert!(cx.remove_param("temp"));
    /// assert!(!cx.remove_param("temp")); // ya no existe
    /// ```
    pub fn remove_param(&mut self, key: &'static str) -> bool {
        self.params.remove(key).is_some()
    }

    /// Recupera el parámetro solicitado y lo elimina del contexto.
    ///
    /// Devuelve:
    ///
    /// - `Ok(T)` si la clave existía y el tipo coincide.
    /// - `Err(ErrorParam::NotFound)` si la clave no existe.
    /// - `Err(ErrorParam::TypeMismatch)` si el tipo no coincide.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// use pagetop::prelude::*;
    ///
    /// let mut cx = Context::new(None)
    ///     .with_param("contador", 7_i32)
    ///     .with_param("titulo", String::from("Hola"));
    ///
    /// let n: i32 = cx.take_param("contador").unwrap();
    /// assert!(cx.get_param::<i32>("contador").is_err()); // ya no está
    ///
    /// // Error de tipo:
    /// assert!(cx.take_param::<i32>("titulo").is_err());
    /// ```
    pub fn take_param<T: 'static>(&mut self, key: &'static str) -> Result<T, ErrorParam> {
        let (boxed, saved) = self.params.remove(key).ok_or(ErrorParam::NotFound)?;
        boxed
            .downcast::<T>()
            .map(|b| *b)
            .map_err(|_| ErrorParam::TypeMismatch {
                key,
                expected: TypeInfo::FullName.of::<T>(),
                saved,
            })
    }

    // Context EXTRAS ******************************************************************************

    /// Genera un identificador único si no se proporciona uno explícito.
    ///
    /// Si no se proporciona un `id`, se genera un identificador único en la forma `<tipo>-<número>`
    /// donde `<tipo>` es el nombre corto del tipo en minúsculas (sin espacios) y `<número>` es un
    /// contador interno incremental.
    pub fn required_id<T>(&mut self, id: Option<String>) -> String {
        if let Some(id) = id {
            id
        } else {
            let prefix = TypeInfo::ShortName
                .of::<T>()
                .trim()
                .replace(' ', "_")
                .to_lowercase();
            let prefix = if prefix.is_empty() {
                "prefix".to_owned()
            } else {
                prefix
            };
            self.id_counter += 1;
            join!(prefix, "-", self.id_counter.to_string())
        }
    }
}

/// Permite a [`Context`](crate::html::Context) actuar como proveedor de idioma.
///
/// Devuelve un [`LanguageIdentifier`] siguiendo este orden de prioridad:
///
/// 1. Un idioma válido establecido explícitamente con [`Context::with_langid`].
/// 2. El idioma por defecto configurado para la aplicación.
/// 3. Un idioma válido extraído de la cabecera `Accept-Language` del navegador.
/// 4. Y si ninguna de las opciones anteriores aplica, se usa el idioma de respaldo (`"en-US"`).
///
/// Resulta útil para usar un contexto ([`Context`]) como fuente de traducción en
/// [`L10n::lookup()`](crate::locale::L10n::lookup) o [`L10n::using()`](crate::locale::L10n::using).
impl LangId for Context {
    fn langid(&self) -> &'static LanguageIdentifier {
        self.langid
    }
}
