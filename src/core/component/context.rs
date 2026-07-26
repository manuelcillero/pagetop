use crate::auth::CurrentUser;
use crate::core::TypeInfo;
use crate::core::component::{ChildOp, Component, MessageLevel, StatusMessage};
use crate::core::theme::all::DEFAULT_THEME;
use crate::core::theme::{ChildrenInRegions, CoreRegion, CoreTemplate};
use crate::core::theme::{RegionRef, TemplateRef, ThemeRef};
use crate::html::{Assets, Favicon, JavaScript, Preload, StyleSheet};
use crate::html::{Markup, Props, PropsOp, RoutePath, html};
use crate::locale::L10n;
use crate::locale::{LangId, LanguageIdentifier, RequestLocale};
use crate::web::HttpRequest;
use crate::{builder_fn, util};

use parking_lot::Mutex;
use thiserror::Error;

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Operaciones para modificar recursos asociados al [`Context`] de un documento.
pub enum AssetsOp {
    /// Define el *favicon* del documento. Sobrescribe cualquier valor anterior.
    SetFavicon(Option<Favicon>),
    /// Define el *favicon* solo si no se ha establecido previamente.
    SetFaviconIfNone(Favicon),

    /// Añade un recurso para precarga al documento.
    AddPreload(Preload),
    /// Elimina un recurso para precarga por su ruta.
    RemovePreload(&'static str),

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
#[derive(Debug, Error)]
pub enum ContextError {
    /// La clave no existe.
    #[error("parameter not found")]
    ParamNotFound,
    /// La clave existe, pero el valor guardado no coincide con el tipo solicitado. Incluye
    /// nombre de la clave (`key`), tipo esperado (`expected`) y tipo realmente guardado (`saved`)
    /// para facilitar el diagnóstico.
    #[error("type mismatch for parameter \"{key}\": expected \"{expected}\", found \"{saved}\"")]
    ParamTypeMismatch {
        key: &'static str,
        expected: &'static str,
        saved: &'static str,
    },
}

/// Interfaz para gestionar el **contexto de renderizado** de un documento HTML.
///
/// `Contextual` extiende [`LangId`] para establecer el idioma del documento y añade métodos para:
///
/// - Almacenar la **petición HTTP** de origen.
/// - Seleccionar el **tema** y la **plantilla** de renderizado.
/// - Administrar **recursos** del documento como el icono [`Favicon`], las hojas de estilo
///   [`StyleSheet`] o los scripts [`JavaScript`] mediante [`AssetsOp`].
/// - Leer y mantener **parámetros dinámicos tipados** de contexto.
///
/// Lo implementan, típicamente, estructuras que manejan el contexto de renderizado, como
/// [`Context`](crate::core::component::Context) o [`Page`](crate::response::Page).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_aliner::Aliner;
/// fn prepare_context<C: Contextual>(cx: C) -> C {
///     cx.with_langid(&Locale::resolve("es-ES"))
///       .with_theme(&Aliner)
///       .with_template(&CoreTemplate::Standard)
///       .with_assets(AssetsOp::SetFavicon(Some(Favicon::new().with_icon("/favicon.ico"))))
///       .with_assets(AssetsOp::AddStyleSheet(StyleSheet::from("/css/app.css")))
///       .with_assets(AssetsOp::AddJavaScript(JavaScript::defer("/js/app.js")))
///       .with_param("user_id", 42_i32)
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
    /// El valor se almacena junto con el nombre de su tipo, lo que permite generar mensajes de
    /// error precisos al recuperarlo con [`param`](Contextual::param) si el tipo solicitado no
    /// coincide.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let cx = Context::new(None)
    ///     .with_param("user_id", 42_i32)
    ///     .with_param("title", "Hello".to_string())
    ///     .with_param("flags", vec!["a", "b"]);
    /// ```
    #[builder_fn]
    fn with_param<T: Send + Sync + 'static>(self, key: &'static str, value: T) -> Self;

    /// Define los recursos del contexto usando [`AssetsOp`].
    #[builder_fn]
    fn with_assets(self, op: AssetsOp) -> Self;

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del elemento `<body>`.
    #[builder_fn]
    fn with_body_props(self, op: PropsOp) -> Self;

    /// Añade un componente o aplica una operación [`ChildOp`] en la región por defecto del
    /// documento.
    #[builder_fn]
    fn with_child(self, op: impl Into<ChildOp>) -> Self;

    /// Añade un componente o aplica una operación [`ChildOp`] en una región específica del
    /// documento.
    #[builder_fn]
    fn with_child_in(self, region: RegionRef, op: impl Into<ChildOp>) -> Self;

    // **< Contextual GETTERS >*********************************************************************

    /// Devuelve una referencia a la petición HTTP asociada, si existe.
    fn request(&self) -> Option<&HttpRequest>;

    /// Devuelve la identidad del usuario actual.
    ///
    /// Si ninguna extensión de autenticación ha inyectado un
    /// [`CurrentUser`](crate::auth::CurrentUser) en las extensiones de la petición HTTP, devuelve
    /// `&CurrentUser::Anonymous`.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// async fn greet(request: HttpRequest) -> Result<Markup, ErrorPage> {
    ///     let mut page = Page::new(request);
    ///     if page.current_user().is_authenticated() {
    ///         // Personalizar la página para el usuario autenticado.
    ///     }
    ///     page.render().await
    /// }
    /// ```
    fn current_user(&self) -> &CurrentUser;

    /// Devuelve el tema que se usará para renderizar el documento.
    fn theme(&self) -> ThemeRef;

    /// Devuelve la plantilla configurada para renderizar el documento.
    fn template(&self) -> TemplateRef;

    /// Recupera una *referencia tipada* al parámetro solicitado.
    ///
    /// Devuelve:
    ///
    /// - `Ok(&T)` si la clave existe y el tipo coincide.
    /// - `Err(ContextError::ParamNotFound)` si la clave no existe.
    /// - `Err(ContextError::ParamTypeMismatch)` si la clave existe pero el tipo no coincide.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let cx = Context::new(None)
    ///     .with_param("user_id", 42_i32)
    ///     .with_param("title", "Hello".to_string());
    ///
    /// let id: i32 = *cx.param("user_id").unwrap();
    /// let title: &String = cx.param("title").unwrap();
    ///
    /// // Error de tipo:
    /// assert!(cx.param::<String>("user_id").is_err());
    /// ```
    fn param<T: 'static>(&self, key: &'static str) -> Result<&T, ContextError>;

    /// Devuelve el parámetro clonado o el **valor por defecto del tipo** (`T::default()`).
    fn param_or_default<T: Clone + Default + 'static>(&self, key: &'static str) -> T {
        self.param::<T>(key).ok().cloned().unwrap_or_default()
    }

    /// Devuelve el parámetro clonado o un **valor por defecto** si no existe.
    fn param_or<T: Clone + 'static>(&self, key: &'static str, default: T) -> T {
        self.param::<T>(key).ok().cloned().unwrap_or(default)
    }

    /// Devuelve el parámetro clonado o el **valor evaluado** por la función `f` si no existe.
    fn param_or_else<T: Clone + 'static, F: FnOnce() -> T>(&self, key: &'static str, f: F) -> T {
        self.param::<T>(key).ok().cloned().unwrap_or_else(f)
    }

    /// Devuelve el Favicon de los recursos del contexto.
    fn favicon(&self) -> Option<&Favicon>;

    /// Devuelve las hojas de estilo de los recursos del contexto.
    fn stylesheets(&self) -> &Assets<StyleSheet>;

    /// Devuelve los scripts JavaScript de los recursos del contexto.
    fn javascripts(&self) -> &Assets<JavaScript>;

    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del elemento `<body>`.
    fn body_props(&self) -> &Props;

    // **< Contextual HELPERS >*********************************************************************

    /// Elimina un parámetro del contexto. Devuelve `true` si la clave existía y se eliminó.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let mut cx = Context::new(None).with_param("temp", 1u8);
    /// assert!(cx.remove_param("temp"));
    /// assert!(!cx.remove_param("temp")); // ya no existe
    /// ```
    fn remove_param(&mut self, key: &'static str) -> bool;
}

/// Implementa un **contexto de renderizado** para un documento HTML.
///
/// Se crea una sola vez por petición usando [`Context::new()`] (típicamente a través de
/// [`Page::new()`](crate::response::Page::new) o [`Page::admin()`](crate::response::Page::admin)),
/// y es la única vía por la que un componente, una acción o el tema activo conocen: la petición
/// HTTP de origen, el idioma negociado, el usuario autenticado
/// ([`current_user()`](Contextual::current_user)), el tema y la plantilla en uso, y los recursos
/// (favicon, hojas de estilo, scripts) acumulados hasta ese momento. Otros datos que los
/// componentes necesiten durante el renderizado pueden ser parámetros dinámicos tipados con
/// [`with_param()`](Contextual::with_param)/[`param()`](Contextual::param).
///
/// La implementación extiende [`Contextual`], que aporta los métodos *builder* (`with_*`) y los
/// *getters* comunes a cualquier estructura que gestione un contexto de renderizado (también los
/// implementa [`Page`](crate::response::Page)). Además, `Context` añade:
///
/// - [`route()`](Self::route) para construir URLs que preserven `?lang=...` cuando corresponda.
/// - [`build_id()`](Self::build_id)/[`required_id()`](Self::required_id) para generar
///   identificadores HTML únicos por tipo de componente.
/// - [`push_message()`](Self::push_message)/[`messages()`](Self::messages) para acumular
///   [`StatusMessage`] que mostrar en algún momento del renderizado.
/// - [`render_assets()`](Self::render_assets)/[`render_region()`](Self::render_region), usados
///   internamente por [`Page`](crate::response::Page) para producir el HTML final del documento.
///
/// # Ejemplos
///
/// Crea un nuevo contexto asociado a una petición HTTP:
///
/// ```rust,no_run
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
///         .with_param("user_id", 42)
/// }
/// ```
///
/// Y hace operaciones con un contexto dado:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # #[derive(AutoDefault, Clone, Debug)]
/// # struct Menu;
/// # impl Component for Menu {
/// #     fn new() -> Self { Self::default() }
/// # }
/// fn use_context(cx: &mut Context) {
///     // Recupera el tema seleccionado.
///     let _theme = cx.theme(); // short_name() => "basic" por defecto
///
///     // Recupera el parámetro a su tipo original.
///     let _id: i32 = *cx.param::<i32>("user_id").unwrap(); // => 42
///
///     // Genera un identificador para un componente de tipo `Menu`.
///     let _unique_id = cx.build_id::<Menu>(1); // => "menu-1" si es el primero
/// }
/// ```
#[rustfmt::skip]
pub struct Context {
    request     : Option<HttpRequest>,            // Petición HTTP de origen.
    locale      : RequestLocale,                  // Idioma asociado a la petición.
    current_user: CurrentUser,                    // Identidad del usuario actual.
    theme       : ThemeRef,                       // Referencia al tema usado para renderizar.
    template    : TemplateRef,                    // Plantilla usada para renderizar.
    favicon     : Option<Favicon>,                // Favicon, si se ha definido.
    preloads    : Assets<Preload>,                // Recursos para precarga.
    stylesheets : Assets<StyleSheet>,             // Hojas de estilo CSS.
    javascripts : Assets<JavaScript>,             // Scripts JavaScript.
    body_props  : Props,                          // Id, clases CSS y atributos del <body>.
    regions     : ChildrenInRegions,              // Regiones de componentes para renderizar.
    params      : HashMap<&'static str, (Box<dyn Any + Send + Sync>, &'static str)>, // Parámetros.
    id_counters : Mutex<HashMap<TypeId, usize>>,  // Mutex permite mutar desde build_id(&self).
    messages    : Vec<StatusMessage>,             // Mensajes de usuario acumulados.
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
        let current_user = Self::resolve_current_user(request.as_ref());
        Context {
            request,
            locale,
            current_user,
            theme      : *DEFAULT_THEME,
            template   : &CoreTemplate::Standard,
            favicon    : None,
            preloads   : Assets::<Preload>::new(),
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            body_props : Props::default(),
            regions    : ChildrenInRegions::default(),
            params     : HashMap::default(),
            id_counters: Mutex::new(HashMap::new()),
            messages   : Vec::new(),
        }
    }

    // Extrae el `CurrentUser` inyectado por middleware en las extensiones de la petición, o
    // `CurrentUser::Anonymous` si no hay petición o ninguna extensión de autenticación está activa.
    fn resolve_current_user(request: Option<&HttpRequest>) -> CurrentUser {
        request
            .and_then(|r| r.extension::<CurrentUser>())
            .cloned()
            .unwrap_or(CurrentUser::Anonymous)
    }

    // **< Context RENDER >*************************************************************************

    /// Renderiza los recursos del contexto.
    pub fn render_assets(&mut self) -> Markup {
        use std::mem::take as mem_take;

        // Extrae temporalmente los recursos.
        let favicon = mem_take(&mut self.favicon); // Deja valor por defecto (None) en self.
        let preloads = mem_take(&mut self.preloads); // Assets<Preload>::default() en self.
        let stylesheets = mem_take(&mut self.stylesheets); // Assets<StyleSheet>::default() en self.
        let javascripts = mem_take(&mut self.javascripts); // Assets<JavaScript>::default() en self.

        // Renderiza con `&mut self` como contexto.
        let markup = html! {
            @if let Some(fi) = &favicon {
                (fi.render(self))
            }
            // Primero los recursos para precarga para iniciar las descargas inmediatamente.
            (preloads.render(self))
            (stylesheets.render(self))
            (javascripts.render(self))
        };

        // Restaura los campos tal y como estaban.
        self.favicon = favicon;
        self.preloads = preloads;
        self.stylesheets = stylesheets;
        self.javascripts = javascripts;

        markup
    }

    /// Renderiza los componentes de una región.
    ///
    /// Combina los componentes registrados para esta región en la petición actual con los
    /// prototipos globales añadidos vía [`InRegion`](crate::core::theme::InRegion) (comunes o
    /// específicos del tema activo).
    pub async fn render_region(&mut self, region: RegionRef) -> Markup {
        self.regions
            .assemble_region(self.theme, region)
            .render(self)
            .await
    }

    // **< Context HELPERS >************************************************************************

    /// Construye una ruta aplicada al contexto actual.
    ///
    /// Acepta cualquier tipo convertible a [`RoutePath`] (un literal, un `String`, un `&str` de
    /// cualquier vida, o un [`RoutePath`] ya construido con sus propios parámetros). Si la política
    /// de negociación del idioma ([`LangNegotiation`](crate::global::LangNegotiation)) indica que
    /// debe propagarse el idioma para esta petición, se añade o actualiza automáticamente el
    /// parámetro de *query* `lang=...` con el identificador de idioma definido en el contexto.
    ///
    /// Esto garantiza que los enlaces generados desde el contexto preservan la preferencia de
    /// idioma del usuario durante la navegación. Si `path` **parece** una URL externa (ver
    /// [`util::url_looks_external()`](crate::util::url_looks_external)), nunca se le añade `lang`.
    ///
    /// Este método asume que ya tienes `cx` a mano en el momento de construir la ruta (dentro de
    /// `prepare()`, un *handler* HTTP, etc.). Si lo que estás definiendo es un campo de componente
    /// que se construye una sola vez y se reutiliza en peticiones futuras (un menú, un botón,
    /// etc.), usa [`Route`](crate::core::component::Route) en su lugar (su documentación explica el
    /// criterio completo para elegir entre ambos).
    pub fn route(&self, path: impl Into<RoutePath>) -> RoutePath {
        let mut route = path.into();
        if !route.is_external() && self.locale.needs_lang_query() {
            route.alter_param("lang", self.locale.langid().to_string());
        }
        route
    }

    /// Construye un identificador HTML único para el tipo de componente `C`.
    ///
    /// Toma los `segments` finales del *path* completo del tipo, los une con `-` y los convierte a
    /// minúsculas, y añade un contador independiente por tipo. Por ejemplo, para `MyApp::ui::Menu`
    /// con `segments = 2` devuelve `ui-menu-1` la primera vez que se invoca para ese tipo,
    /// `ui-menu-2` la segunda, etc.
    ///
    /// Con `segments = 1` se usa sólo el nombre corto del tipo. Si `segments` es `0` o supera el
    /// número de segmentos del *path*, se usan todos.
    ///
    /// Es útil para asignar identificadores cuando el componente no recibe uno explícito. El
    /// contador es local a este contexto y se reinicia para cada nueva petición.
    pub fn build_id<C: Component>(&self, segments: usize) -> String {
        let path: Vec<&str> = TypeInfo::FullName.of::<C>().split("::").collect();
        let segments = if segments == 0 || segments >= path.len() {
            path.len()
        } else {
            segments
        };
        let count = {
            let mut map = self.id_counters.lock();
            let n = map.entry(TypeId::of::<C>()).or_insert(0);
            *n += 1;
            *n
        };
        let prefix = path[path.len() - segments..].join("-").to_lowercase();
        util::join!(prefix, "-", count.to_string())
    }

    /// Devuelve `id` si contiene un valor, o genera uno único con [`build_id`](Self::build_id)
    /// si es `None`.
    pub fn required_id<C: Component>(&self, id: Option<String>, segments: usize) -> String {
        match id {
            Some(id) => id,
            None => self.build_id::<C>(segments),
        }
    }

    /// Acumula un [`StatusMessage`] en el contexto para notificar al usuario.
    ///
    /// Pueden generarse en cualquier punto del ciclo de una petición web (handlers, renderizado,
    /// lógica de negocio, etc.) que tengan acceso al contexto, y mostrarlos luego, por ejemplo, en
    /// la página final devuelta al usuario.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # let mut cx = Context::new(None);
    /// cx.push_message(MessageLevel::Warning, L10n::n("Session is not valid"));
    /// ```
    pub fn push_message(&mut self, level: MessageLevel, text: L10n) {
        self.messages.push(StatusMessage::new(level, text));
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
        // Recalcula el *locale* y el usuario actual según la nueva petición y la política de
        // negociación configurada.
        self.locale = RequestLocale::from_request(self.request.as_ref());
        self.current_user = Self::resolve_current_user(self.request.as_ref());
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
    fn with_param<T: Send + Sync + 'static>(mut self, key: &'static str, value: T) -> Self {
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
            // Preloads.
            AssetsOp::AddPreload(preload) => {
                self.preloads.add(preload);
            }
            AssetsOp::RemovePreload(path) => {
                self.preloads.remove(path);
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
    fn with_body_props(mut self, op: PropsOp) -> Self {
        self.body_props.alter_prop(op);
        self
    }

    #[builder_fn]
    fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.regions.alter_child_in(&CoreRegion::Content, op.into());
        self
    }

    #[builder_fn]
    fn with_child_in(mut self, region: RegionRef, op: impl Into<ChildOp>) -> Self {
        self.regions.alter_child_in(region, op.into());
        self
    }

    // **< Contextual GETTERS >*********************************************************************

    fn request(&self) -> Option<&HttpRequest> {
        self.request.as_ref()
    }

    fn current_user(&self) -> &CurrentUser {
        &self.current_user
    }

    fn theme(&self) -> ThemeRef {
        self.theme
    }

    fn template(&self) -> TemplateRef {
        self.template
    }

    fn param<T: 'static>(&self, key: &'static str) -> Result<&T, ContextError> {
        let (any, type_name) = self.params.get(key).ok_or(ContextError::ParamNotFound)?;
        any.downcast_ref::<T>()
            .ok_or_else(|| ContextError::ParamTypeMismatch {
                key,
                expected: TypeInfo::FullName.of::<T>(),
                saved: type_name,
            })
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

    fn body_props(&self) -> &Props {
        &self.body_props
    }

    // **< Contextual HELPERS >*********************************************************************

    fn remove_param(&mut self, key: &'static str) -> bool {
        self.params.remove(key).is_some()
    }
}
