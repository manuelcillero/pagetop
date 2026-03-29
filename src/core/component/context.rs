use crate::core::component::{ChildOp, MessageLevel, StatusMessage};
use crate::core::theme::all::DEFAULT_THEME;
use crate::core::theme::{ChildrenInRegions, DefaultRegion, RegionRef, TemplateRef, ThemeRef};
use crate::core::TypeInfo;
use crate::html::{html, Markup, RoutePath};
use crate::html::{Assets, Favicon, JavaScript, StyleSheet};
use crate::locale::L10n;
use crate::locale::{LangId, LanguageIdentifier, RequestLocale};
use crate::service::HttpRequest;
use crate::{builder_fn, util, CowStr};

use std::any::Any;
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;

/// Operaciones para modificar recursos asociados al [`Context`] de un documento.
pub enum AssetsOp {
    /// Define el *favicon* del documento. Sobrescribe cualquier valor anterior.
    SetFavicon(Option<Favicon>),
    /// Define el *favicon* solo si no se ha establecido previamente.
    SetFaviconIfNone(Favicon),

    /// Añade una hoja de estilos CSS al documento.
    AddStyleSheet(StyleSheet),
    /// Elimina una hoja de estilos por su ruta o identificador.
    RemoveStyleSheet(&'static str),

    /// Añade un script JavaScript al documento.
    AddJavaScript(JavaScript),
    /// Elimina un script por su ruta o identificador.
    RemoveJavaScript(&'static str),
}

/// Errores de acceso a parámetros dinámicos del contexto.
///
/// - [`ContextError::ParamNotFound`]: la clave no existe.
/// - [`ContextError::ParamTypeMismatch`]: la clave existe, pero el valor guardado no coincide con
///   el tipo solicitado. Incluye nombre de la clave (`key`), tipo esperado (`expected`) y tipo
///   realmente guardado (`saved`) para facilitar el diagnóstico.
#[derive(Debug)]
pub enum ContextError {
    ParamNotFound,
    ParamTypeMismatch {
        key: &'static str,
        expected: &'static str,
        saved: &'static str,
    },
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextError::ParamNotFound => {
                write!(f, "parameter not found")
            }
            ContextError::ParamTypeMismatch {
                key,
                expected,
                saved,
            } => write!(
                f,
                "type mismatch for parameter \"{key}\": expected \"{expected}\", found \"{saved}\""
            ),
        }
    }
}

impl std::error::Error for ContextError {}

/// Interfaz para gestionar el **contexto de renderizado** de un documento HTML.
///
/// `Contextual` extiende [`LangId`] para establecer el idioma del documento y añade métodos para:
///
/// - Almacenar la **petición HTTP** de origen.
/// - Seleccionar el **tema** y la **plantilla** de renderizado.
/// - Administrar **recursos** del documento como el icono [`Favicon`], las hojas de estilo
///   [`StyleSheet`] o los scripts [`JavaScript`] mediante [`AssetsOp`].
/// - Leer y mantener **parámetros dinámicos tipados** de contexto.
/// - Generar **identificadores únicos** por tipo de componente.
///
/// Lo implementan, típicamente, estructuras que manejan el contexto de renderizado, como
/// [`Context`](crate::core::component::Context) o [`Page`](crate::response::page::Page).
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_aliner::Aliner;
/// fn prepare_context<C: Contextual>(cx: C) -> C {
///     cx.with_langid(&Locale::resolve("es-ES"))
///       .with_theme(&Aliner)
///       .with_template(&DefaultTemplate::Standard)
///       .with_assets(AssetsOp::SetFavicon(Some(Favicon::new().with_icon("/favicon.ico"))))
///       .with_assets(AssetsOp::AddStyleSheet(StyleSheet::from("/css/app.css")))
///       .with_assets(AssetsOp::AddJavaScript(JavaScript::defer("/js/app.js")))
///       .with_param("usuario_id", 42_i32)
/// }
/// ```
pub trait Contextual: LangId {
    // **< Contextual BUILDER >*********************************************************************

    /// Establece el idioma del documento.
    #[builder_fn]
    fn with_langid(self, language: &impl LangId) -> Self;

    /// Almacena la petición HTTP de origen en el contexto.
    #[builder_fn]
    fn with_request(self, request: Option<HttpRequest>) -> Self;

    /// Especifica el tema para renderizar el documento.
    #[builder_fn]
    fn with_theme(self, theme: ThemeRef) -> Self;

    /// Especifica la plantilla para renderizar el documento.
    #[builder_fn]
    fn with_template(self, template: TemplateRef) -> Self;

    /// Añade o modifica un parámetro dinámico del contexto.
    ///
    /// El valor se guardará conservando el *nombre del tipo* real para mejorar los mensajes de
    /// error posteriores.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let cx = Context::new(None)
    ///     .with_param("usuario_id", 42_i32)
    ///     .with_param("titulo", "Hola".to_string())
    ///     .with_param("flags", vec!["a", "b"]);
    /// ```
    #[builder_fn]
    fn with_param<T: 'static>(self, key: &'static str, value: T) -> Self;

    /// Define los recursos del contexto usando [`AssetsOp`].
    #[builder_fn]
    fn with_assets(self, op: AssetsOp) -> Self;

    /// Añade un componente o aplica una operación [`ChildOp`] en la región por defecto del
    /// documento.
    #[builder_fn]
    fn with_child(self, op: impl Into<ChildOp>) -> Self;

    /// Añade un componente o aplica una operación [`ChildOp`] en una región específica del
    /// documento.
    #[builder_fn]
    fn with_child_in(self, region_ref: RegionRef, op: impl Into<ChildOp>) -> Self;

    // **< Contextual GETTERS >*********************************************************************

    /// Devuelve una referencia a la petición HTTP asociada, si existe.
    fn request(&self) -> Option<&HttpRequest>;

    /// Devuelve el tema que se usará para renderizar el documento.
    fn theme(&self) -> ThemeRef;

    /// Devuelve la plantilla configurada para renderizar el documento.
    fn template(&self) -> TemplateRef;

    /// Recupera un parámetro como [`Option`], simplificando el acceso.
    ///
    /// A diferencia de [`get_param`](Context::get_param), que devuelve un [`Result`] con
    /// información detallada de error, este método devuelve `None` tanto si la clave no existe como
    /// si el valor guardado no coincide con el tipo solicitado.
    ///
    /// Resulta útil en escenarios donde sólo interesa saber si el valor existe y es del tipo
    /// correcto, sin necesidad de diferenciar entre error de ausencia o de tipo.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let cx = Context::new(None).with_param("username", "Alice".to_string());
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
    fn param<T: 'static>(&self, key: &'static str) -> Option<&T>;

    /// Devuelve el parámetro clonado o el **valor por defecto del tipo** (`T::default()`).
    fn param_or_default<T: Default + Clone + 'static>(&self, key: &'static str) -> T {
        self.param::<T>(key).cloned().unwrap_or_default()
    }

    /// Devuelve el parámetro clonado o un **valor por defecto** si no existe.
    fn param_or<T: Clone + 'static>(&self, key: &'static str, default: T) -> T {
        self.param::<T>(key).cloned().unwrap_or(default)
    }

    /// Devuelve el parámetro clonado o el **valor evaluado** por la función `f` si no existe.
    fn param_or_else<T: Clone + 'static, F: FnOnce() -> T>(&self, key: &'static str, f: F) -> T {
        self.param::<T>(key).cloned().unwrap_or_else(f)
    }

    /// Devuelve el Favicon de los recursos del contexto.
    fn favicon(&self) -> Option<&Favicon>;

    /// Devuelve las hojas de estilo de los recursos del contexto.
    fn stylesheets(&self) -> &Assets<StyleSheet>;

    /// Devuelve los scripts JavaScript de los recursos del contexto.
    fn javascripts(&self) -> &Assets<JavaScript>;

    // **< Contextual HELPERS >*********************************************************************

    /// Devuelve el `id` proporcionado tal cual, o genera uno único para el tipo `T` si no se
    /// proporciona ninguno.
    ///
    /// Si `id` es `None`, construye un identificador en la forma `<tipo>-<n>`, donde `<tipo>` es el
    /// nombre corto del tipo en minúsculas y `<n>` un contador incremental interno del contexto. Es
    /// útil para asignar identificadores HTML predecibles cuando el componente no recibe uno
    /// explícito.
    fn required_id<T>(&self, id: Option<String>) -> String;

    /// Acumula un [`StatusMessage`] en el contexto para notificar al visitante.
    ///
    /// Pueden generarse en cualquier punto del ciclo de una petición web (manejadores, renderizado,
    /// lógica de negocio, etc.) que tengan acceso al contexto, y mostrarlos luego, por ejemplo, en
    /// la página final devuelta al usuario.
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// cx.push_message(MessageLevel::Warning, L10n::l("session-not-valid"));
    /// ```
    fn push_message(&mut self, level: MessageLevel, text: L10n);
}

/// Implementa un **contexto de renderizado** para un documento HTML.
///
/// Extiende [`Contextual`] con métodos para **instanciar** y configurar un nuevo contexto,
/// **renderizar los recursos** del documento (incluyendo el [`Favicon`], las hojas de estilo
/// [`StyleSheet`] y los scripts [`JavaScript`]), o extender el uso de **parámetros dinámicos
/// tipados** con nuevos métodos.
///
/// # Ejemplos
///
/// Crea un nuevo contexto asociado a una petición HTTP:
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_aliner::Aliner;
/// fn new_context(request: HttpRequest) -> Context {
///     Context::new(Some(request))
///         // Establece el idioma del documento a español.
///         .with_langid(&Locale::resolve("es-ES"))
///         // Establece el tema para renderizar.
///         .with_theme(&Aliner)
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
/// # use pagetop::prelude::*;
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
    request    : Option<HttpRequest>,           // Petición HTTP de origen.
    locale     : RequestLocale,                 // Idioma asociado a la petición.
    theme      : ThemeRef,                      // Referencia al tema usado para renderizar.
    template   : TemplateRef,                   // Plantilla usada para renderizar.
    favicon    : Option<Favicon>,               // Favicon, si se ha definido.
    stylesheets: Assets<StyleSheet>,            // Hojas de estilo CSS.
    javascripts: Assets<JavaScript>,            // Scripts JavaScript.
    regions    : ChildrenInRegions,             // Regiones de componentes para renderizar.
    params     : HashMap<&'static str, (Box<dyn Any>, &'static str)>, // Parámetros en ejecución.
    id_counter : Cell<usize>,  // Cell permite incrementarlo desde &self en required_id().
    messages   : Vec<StatusMessage>,            // Mensajes de usuario acumulados.
}

impl Default for Context {
    fn default() -> Self {
        Context::new(None)
    }
}

impl Context {
    /// Crea un nuevo contexto asociado a una petición HTTP.
    ///
    /// El contexto inicializa el idioma, el tema y la plantilla por defecto, sin favicon ni otros
    /// recursos cargados.
    #[rustfmt::skip]
    pub fn new(request: Option<HttpRequest>) -> Self {
        let locale = RequestLocale::from_request(request.as_ref());
        Context {
            request,
            locale,
            theme      : *DEFAULT_THEME,
            template   : DEFAULT_THEME.default_template(),
            favicon    : None,
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            regions    : ChildrenInRegions::default(),
            params     : HashMap::default(),
            id_counter : Cell::new(0),
            messages   : Vec::new(),
        }
    }

    // **< Context RENDER >*************************************************************************

    /// Renderiza los recursos del contexto.
    pub fn render_assets(&mut self) -> Markup {
        use std::mem::take as mem_take;

        // Extrae temporalmente los recursos.
        let favicon = mem_take(&mut self.favicon); // Deja valor por defecto (None) en self.
        let stylesheets = mem_take(&mut self.stylesheets); // Assets<StyleSheet>::default() en self.
        let javascripts = mem_take(&mut self.javascripts); // Assets<JavaScript>::default() en self.

        // Renderiza con `&mut self` como contexto.
        let markup = html! {
            @if let Some(fi) = &favicon {
                (fi.render(self))
            }
            (stylesheets.render(self))
            (javascripts.render(self))
        };

        // Restaura los campos tal y como estaban.
        self.favicon = favicon;
        self.stylesheets = stylesheets;
        self.javascripts = javascripts;

        markup
    }

    /// Renderiza los componentes de una región.
    pub fn render_region(&mut self, region_ref: RegionRef) -> Markup {
        self.regions
            .children_for(self.theme, region_ref)
            .render(self)
    }

    // **< Context PARAMS >*************************************************************************

    /// Recupera una *referencia tipada* al parámetro solicitado.
    ///
    /// Devuelve:
    ///
    /// - `Ok(&T)` si la clave existe y el tipo coincide.
    /// - `Err(ContextError::ParamNotFound)` si la clave no existe.
    /// - `Err(ContextError::ParamTypeMismatch)` si la clave existe pero el tipo no coincide.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let cx = Context::new(None)
    ///     .with_param("usuario_id", 42_i32)
    ///     .with_param("titulo", "Hola".to_string());
    ///
    /// let id: &i32 = cx.get_param("usuario_id").unwrap();
    /// let titulo: &String = cx.get_param("titulo").unwrap();
    ///
    /// // Error de tipo:
    /// assert!(cx.get_param::<String>("usuario_id").is_err());
    /// ```
    pub fn get_param<T: 'static>(&self, key: &'static str) -> Result<&T, ContextError> {
        let (any, type_name) = self.params.get(key).ok_or(ContextError::ParamNotFound)?;
        any.downcast_ref::<T>()
            .ok_or_else(|| ContextError::ParamTypeMismatch {
                key,
                expected: TypeInfo::FullName.of::<T>(),
                saved: type_name,
            })
    }

    /// Recupera el parámetro solicitado y lo elimina del contexto.
    ///
    /// Devuelve:
    ///
    /// - `Ok(T)` si la clave existía y el tipo coincide.
    /// - `Err(ContextError::ParamNotFound)` si la clave no existe.
    /// - `Err(ContextError::ParamTypeMismatch)` si el tipo no coincide.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let mut cx = Context::new(None)
    ///     .with_param("contador", 7_i32)
    ///     .with_param("titulo", "Hola".to_string());
    ///
    /// let n: i32 = cx.take_param("contador").unwrap();
    /// assert!(cx.get_param::<i32>("contador").is_err()); // ya no está
    ///
    /// // Error de tipo:
    /// assert!(cx.take_param::<i32>("titulo").is_err());
    /// ```
    pub fn take_param<T: 'static>(&mut self, key: &'static str) -> Result<T, ContextError> {
        let (boxed, saved) = self.params.remove(key).ok_or(ContextError::ParamNotFound)?;
        boxed
            .downcast::<T>()
            .map(|b| *b)
            .map_err(|_| ContextError::ParamTypeMismatch {
                key,
                expected: TypeInfo::FullName.of::<T>(),
                saved,
            })
    }

    /// Elimina un parámetro del contexto. Devuelve `true` si la clave existía y se eliminó.
    ///
    /// Devuelve `false` en caso contrario. Usar cuando sólo interesa borrar la entrada.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let mut cx = Context::new(None).with_param("temp", 1u8);
    /// assert!(cx.remove_param("temp"));
    /// assert!(!cx.remove_param("temp")); // ya no existe
    /// ```
    pub fn remove_param(&mut self, key: &'static str) -> bool {
        self.params.remove(key).is_some()
    }

    // **< Context HELPERS >************************************************************************

    /// Construye una ruta aplicada al contexto actual.
    ///
    /// La ruta resultante se envuelve en un [`RoutePath`], que permite añadir parámetros de
    /// consulta de forma tipada. Si la política de negociación de idioma actual
    /// [`LangNegotiation`](crate::global::LangNegotiation) indica que debe propagarse el idioma
    /// para esta petición, se añade o actualiza el parámetro de *query* `lang=...` con el
    /// identificador de idioma efectivo del contexto.
    ///
    /// Esto garantiza que los enlaces generados desde el contexto preservan la preferencia de
    /// idioma del usuario cuando procede.
    pub fn route(&self, path: impl Into<CowStr>) -> RoutePath {
        let mut route = RoutePath::new(path);
        if self.locale.needs_lang_query() {
            route.alter_param("lang", self.locale.langid().to_string());
        }
        route
    }

    /// Devuelve todos los mensajes de usuario acumulados.
    pub fn messages(&self) -> &[StatusMessage] {
        &self.messages
    }

    /// Indica si hay mensajes de usuario acumulados.
    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }
}

/// Permite a [`Context`](crate::core::component::Context) actuar como proveedor de idioma.
///
/// Internamente delega en [`RequestLocale`], que tiene en cuenta la petición HTTP, la configuración
/// global de idioma de la aplicación, la cabecera `Accept-Language` y/o el idioma de respaldo.
///
/// Todo ello según la negociación indicada en [`global::SETTINGS.app.lang_negotiation`]. Esto
/// permite que el [`Context`] se use como fuente de idioma coherente en
/// [`L10n::lookup()`](crate::locale::L10n::lookup) o [`L10n::using()`](crate::locale::L10n::using).
impl LangId for Context {
    #[inline]
    fn langid(&self) -> &'static LanguageIdentifier {
        self.locale.langid()
    }
}

impl Contextual for Context {
    // **< Contextual BUILDER >*********************************************************************

    #[builder_fn]
    fn with_request(mut self, request: Option<HttpRequest>) -> Self {
        self.request = request;
        // Recalcula el locale según la nueva petición y la política de negociación configurada.
        self.locale = RequestLocale::from_request(self.request.as_ref());
        self
    }

    #[builder_fn]
    fn with_langid(mut self, language: &impl LangId) -> Self {
        self.locale.with_langid(language);
        self
    }

    #[builder_fn]
    fn with_theme(mut self, theme: ThemeRef) -> Self {
        self.theme = theme;
        self
    }

    #[builder_fn]
    fn with_template(mut self, template: TemplateRef) -> Self {
        self.template = template;
        self
    }

    #[builder_fn]
    fn with_param<T: 'static>(mut self, key: &'static str, value: T) -> Self {
        let type_name = TypeInfo::FullName.of::<T>();
        self.params.insert(key, (Box::new(value), type_name));
        self
    }

    #[builder_fn]
    fn with_assets(mut self, op: AssetsOp) -> Self {
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
            // Scripts JavaScript.
            AssetsOp::AddJavaScript(js) => {
                self.javascripts.add(js);
            }
            AssetsOp::RemoveJavaScript(path) => {
                self.javascripts.remove(path);
            }
        }
        self
    }

    #[builder_fn]
    fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.regions
            .alter_child_in(&DefaultRegion::Content, op.into());
        self
    }

    #[builder_fn]
    fn with_child_in(mut self, region_ref: RegionRef, op: impl Into<ChildOp>) -> Self {
        self.regions.alter_child_in(region_ref, op.into());
        self
    }

    // **< Contextual GETTERS >*********************************************************************

    fn request(&self) -> Option<&HttpRequest> {
        self.request.as_ref()
    }

    fn theme(&self) -> ThemeRef {
        self.theme
    }

    fn template(&self) -> TemplateRef {
        self.template
    }

    fn param<T: 'static>(&self, key: &'static str) -> Option<&T> {
        self.get_param::<T>(key).ok()
    }

    fn favicon(&self) -> Option<&Favicon> {
        self.favicon.as_ref()
    }

    fn stylesheets(&self) -> &Assets<StyleSheet> {
        &self.stylesheets
    }

    fn javascripts(&self) -> &Assets<JavaScript> {
        &self.javascripts
    }

    // **< Contextual HELPERS >*********************************************************************

    fn required_id<T>(&self, id: Option<String>) -> String {
        if let Some(id) = id {
            id
        } else {
            let prefix = TypeInfo::ShortName
                .of::<T>()
                .trim()
                .replace(' ', "_")
                .to_lowercase();
            let prefix = if prefix.is_empty() {
                "prefix".to_string()
            } else {
                prefix
            };
            self.id_counter.set(self.id_counter.get() + 1);
            util::join!(prefix, "-", self.id_counter.get().to_string())
        }
    }

    fn push_message(&mut self, level: MessageLevel, text: L10n) {
        self.messages.push(StatusMessage::new(level, text));
    }
}
