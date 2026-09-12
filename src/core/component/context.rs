use crate::auth::CurrentUser;
use crate::core::TypeInfo;
use crate::core::component::{ChildOp, Component, MessageLevel, StatusMessage};
use crate::core::theme::all::DEFAULT_THEME;
use crate::core::theme::{ChildrenInRegions, CoreRegions, CoreTemplates};
use crate::core::theme::{RegionRef, TemplateRef, ThemeRef};
use crate::html::{Assets, Favicon, JavaScript, Preload, ResponsiveStyles, StyleSheet};
use crate::html::{Markup, Props, PropsOp, RoutePath, html};
use crate::locale::Lc;
use crate::locale::{LangId, LanguageIdentifier, RequestLocale};
use crate::web::HttpRequest;
use crate::{builder_impl, util};

use parking_lot::Mutex;

use std::any::{Any, TypeId};
use std::collections::HashMap;

mod assets_op;
pub use assets_op::AssetsOp;

mod error;
pub use error::ContextError;

mod contextual;
pub use contextual::Contextual;

/// Implementa un **contexto de renderizado** para un documento HTML.
///
/// Se crea una sola vez por petición usando [`Context::new()`] (típicamente a través de
/// [`Page::new()`](crate::response::Page::new) o [`Page::admin()`](crate::response::Page::admin)),
/// y es la única vía por la que un componente, una acción o el tema activo conocen: la petición
/// HTTP de origen, el idioma negociado, el usuario autenticado
/// ([`current_user()`](Contextual::current_user)), la plantilla y el tema en uso, y los recursos
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
/// # fn new_context(request: HttpRequest) -> Context {
/// let cx = Context::new(request)
///     // Establece el idioma del documento a español.
///     .with_langid(&Locale::resolve("es-ES"))
///     // Establece el tema para renderizar.
///     .with_theme(&Aliner)
///     // Asigna un favicon.
///     .with_assets(Favicon::new().with_icon("/favicon.ico"))
///     // Añade una hoja de estilo externa.
///     .with_assets(StyleSheet::from("/css/style.css"))
///     // Añade un script JavaScript.
///     .with_assets(JavaScript::defer("/js/main.js"))
///     // Añade un parámetro dinámico al contexto.
///     .with_param("user_id", 42);
/// # cx }
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
    template    : TemplateRef,                    // Plantilla usada para renderizar.
    theme       : ThemeRef,                       // Referencia al tema usado para renderizar.
    favicon     : Option<Favicon>,                // Favicon, si se ha definido.
    preloads    : Assets<Preload>,                // Recursos para precarga.
    stylesheets : Assets<StyleSheet>,             // Hojas de estilo CSS.
    javascripts : Assets<JavaScript>,             // Scripts JavaScript.
    responsives : ResponsiveStyles,               // Estilos *responsive*.
    body_props  : Props,                          // Id, clases CSS y atributos del <body>.
    regions     : ChildrenInRegions,              // Regiones de componentes para renderizar.
    params      : HashMap<&'static str, (Box<dyn Any + Send + Sync>, &'static str)>, // Parámetros.
    id_counters : Mutex<HashMap<TypeId, usize>>,  // Mutex permite mutar desde build_id(&self).
    messages    : Vec<StatusMessage>,             // Mensajes de usuario acumulados.
}

impl Default for Context {
    fn default() -> Self {
        Self::base(None, &CoreTemplates::Standard)
    }
}

impl Context {
    // Construye el `Context` compartido por `new()`, `admin()` y `Default::default()`, evitando
    // duplicar la lista de campos entre ambos (y la recursión que tendría `new()` llamando a
    // `Default::default()`, o viceversa). Recibe la plantilla para que `admin()` no tenga que
    // construir con la plantilla estándar y sobrescribirla después.
    #[rustfmt::skip]
    fn base(request: Option<HttpRequest>, template: TemplateRef) -> Self {
        let locale = RequestLocale::from_request(request.as_ref());
        let current_user = Self::resolve_current_user(request.as_ref());
        Context {
            request,
            locale,
            current_user,
            template,
            theme      : *DEFAULT_THEME,
            favicon    : None,
            preloads   : Assets::<Preload>::new(),
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            responsives: ResponsiveStyles::new(),
            body_props : Props::default(),
            regions    : ChildrenInRegions::default(),
            params     : HashMap::default(),
            id_counters: Mutex::new(HashMap::new()),
            messages   : Vec::new(),
        }
    }

    /// Crea un nuevo contexto asociado a una petición HTTP.
    ///
    /// El contexto inicializa el idioma, el tema y la plantilla por defecto, sin favicon ni otros
    /// recursos cargados.
    ///
    /// Para un contexto sin petición (renderizar un componente de forma aislada, en tests o fuera
    /// del ciclo de una petición web), usa [`Context::default()`].
    pub fn new(request: HttpRequest) -> Self {
        Self::base(Some(request), &CoreTemplates::Standard)
    }

    /// Crea un nuevo contexto asociado a una petición HTTP, con la plantilla de administración.
    ///
    /// El contexto inicializa el idioma, el tema y la plantilla [`CoreTemplates::Admin`], sin
    /// favicon ni otros recursos cargados.
    pub fn admin(request: HttpRequest) -> Self {
        Self::base(Some(request), &CoreTemplates::Admin)
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
            // Después los estilos *responsive*, para poder sobrescribir sus clases.
            @if !self.responsives.is_empty() {
                style { (self.responsives.render(self)) }
            }
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
            route.alter_param("lang", self.locale.lang_query_value());
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
    /// # let mut cx = Context::default();
    /// cx.push_message(MessageLevel::Warning, Lc::n("Session is not valid"));
    /// ```
    pub fn push_message(&mut self, level: MessageLevel, text: Lc) {
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

/// Permite a [`Context`] actuar como proveedor de idioma.
///
/// Internamente delega en [`RequestLocale`], que tiene en cuenta la petición HTTP, la configuración
/// global de idioma de la aplicación, la cabecera `Accept-Language` y/o el idioma de respaldo.
///
/// Todo ello según la negociación indicada en [`global::SETTINGS.app.lang_negotiation`]. Esto
/// permite que el [`Context`] se use como fuente de idioma coherente en [`Lc::lookup()`] o
/// [`Lc::using()`].
///
/// [`Context`]: crate::core::component::Context
/// [`global::SETTINGS.app.lang_negotiation`]: crate::global::App::lang_negotiation
/// [`Lc::lookup()`]: crate::locale::Lc::lookup
/// [`Lc::using()`]: crate::locale::Lc::using
impl LangId for Context {
    #[inline]
    fn langid(&self) -> &'static LanguageIdentifier {
        self.locale.langid()
    }
}

#[builder_impl]
impl Contextual for Context {
    // **< Contextual BUILDER >*********************************************************************

    fn with_request(mut self, request: Option<HttpRequest>) -> Self {
        self.request = request;
        // Recalcula el *locale* y el usuario actual según la nueva petición y la política de
        // negociación configurada.
        self.locale = RequestLocale::from_request(self.request.as_ref());
        self.current_user = Self::resolve_current_user(self.request.as_ref());
        self
    }

    fn with_langid(mut self, language: &impl LangId) -> Self {
        self.locale.with_langid(language);
        self
    }

    fn with_template(mut self, template: TemplateRef) -> Self {
        self.template = template;
        self
    }

    fn with_theme(mut self, theme: ThemeRef) -> Self {
        self.theme = theme;
        self
    }

    fn with_param<T: Send + Sync + 'static>(mut self, key: &'static str, value: T) -> Self {
        let type_name = TypeInfo::FullName.of::<T>();
        self.params.insert(key, (Box::new(value), type_name));
        self
    }

    fn with_assets(mut self, op: impl Into<AssetsOp>) -> Self {
        match op.into() {
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
            // Estilos responsive.
            AssetsOp::AddResponsiveStyle(breakpoint, classes, property, value) => {
                self.responsives
                    .add_style(breakpoint, classes, property, value);
            }
            AssetsOp::AddResponsiveStyles(breakpoint, classes, styles) => {
                self.responsives.add_styles(breakpoint, classes, styles);
            }
        }
        self
    }

    fn with_body_props(mut self, op: PropsOp) -> Self {
        self.body_props.alter_prop(op);
        self
    }

    fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.regions
            .alter_child_in(&CoreRegions::Content, op.into());
        self
    }

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

    fn template(&self) -> TemplateRef {
        self.template
    }

    fn theme(&self) -> ThemeRef {
        self.theme
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

    fn responsive_styles(&self) -> &ResponsiveStyles {
        &self.responsives
    }

    fn body_props(&self) -> &Props {
        &self.body_props
    }

    // **< Contextual HELPERS >*********************************************************************

    fn remove_param(&mut self, key: &'static str) -> bool {
        self.params.remove(key).is_some()
    }
}
