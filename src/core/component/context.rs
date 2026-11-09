use crate::auth::CurrentUser;
use crate::core::component::{Component, StatusMessage};
use crate::core::theme::{ChildrenInRegions, CoreTemplates, default_theme};
use crate::core::theme::{RegionRef, TemplateRef, ThemeRef};
use crate::datetime::Tz;
use crate::html::{Assets, Favicon, JavaScript, Preload, ResponsiveStyles, StyleSheet};
use crate::html::{Markup, PreEscaped, Props, html};
use crate::locale::Lc;
use crate::locale::RequestLocale;
use crate::response::read_flash_messages;
use crate::web::HttpRequest;
use crate::{CowStr, util};

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;

mod assets_op;
pub use assets_op::AssetsOp;

mod error;
pub use error::ContextError;

mod contextual;
pub use contextual::Contextual;

/// Implementa un **contexto de renderizado** para un documento HTML.
///
/// Se crea una sola vez por petición usando [`Context::new()`] (típicamente a través de
/// [`Page::new()`] o [`Page::admin()`]), y es la única vía por la que un componente, una acción o
/// el tema activo conocen la petición HTTP de origen, el idioma negociado y la zona horaria
/// efectiva, el usuario autenticado ([`current_user()`]), el tema y la plantilla en uso, y los
/// recursos (favicon, hojas de estilo, scripts) acumulados hasta ese momento. Otros datos que los
/// componentes necesiten durante el renderizado pueden ser parámetros dinámicos tipados con
/// [`with_param()`]/[`param()`].
///
/// Su interfaz la aporta [`Contextual`]: los métodos *builder* (`with_*`), los *getters* y las
/// utilidades como [`route()`], [`build_id()`] o [`push_message()`]. Como también lo implementa
/// [`Page`], sobre el contexto que contiene, ambos se usan exactamente igual. Además de sus
/// constructores, `Context` sólo añade [`render_assets()`] y [`render_region()`], con los que los
/// temas y la página producen el HTML final del documento.
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
///
/// [`Page`]: crate::response::Page
/// [`Page::new()`]: crate::response::Page::new
/// [`Page::admin()`]: crate::response::Page::admin
/// [`current_user()`]: Contextual::current_user
/// [`with_param()`]: Contextual::with_param
/// [`param()`]: Contextual::param
/// [`route()`]: Contextual::route
/// [`build_id()`]: Contextual::build_id
/// [`push_message()`]: Contextual::push_message
/// [`render_assets()`]: Self::render_assets
/// [`render_region()`]: Self::render_region
#[rustfmt::skip]
pub struct Context {
    request     : Option<HttpRequest>,            // Petición HTTP de origen.
    locale      : RequestLocale,                  // Idioma asociado a la petición.
    current_user: CurrentUser,                    // Identidad del usuario actual.
    timezone    : Tz,                             // Zona horaria efectiva del documento.
    theme       : ThemeRef,                       // Referencia al tema usado para renderizar.
    template    : TemplateRef,                    // Plantilla usada para renderizar.
    favicon     : Option<Favicon>,                // Favicon, si se ha definido.
    preloads    : Assets<Preload>,                // Recursos para precarga.
    stylesheets : Assets<StyleSheet>,             // Hojas de estilo CSS.
    javascripts : Assets<JavaScript>,             // Scripts JavaScript.
    responsives : ResponsiveStyles,               // Estilos *responsive*.
    title       : Lc,                             // Título del documento.
    description : Lc,                             // Descripción del documento.
    metadata    : Vec<(&'static str, CowStr)>,    // Entradas <meta name=...>.
    properties  : Vec<(&'static str, CowStr)>,    // Entradas <meta property=...>.
    body_props  : Props,                          // Id, clases CSS y atributos del <body>.
    regions     : ChildrenInRegions,              // Regiones de componentes para renderizar.
    deferred    : Option<Deferred>,               // Componentes de renderizado aplazado.
    params      : HashMap<&'static str, (Box<dyn Any + Send + Sync>, &'static str)>, // Parámetros.
    id_counters : Mutex<HashMap<TypeId, usize>>,  // Mutex permite mutar desde build_id(&self).
    messages    : Vec<StatusMessage>,             // Mensajes de usuario acumulados.
}

// Componentes aplazados mientras se renderiza el `<body>` de una página, cada uno con la marca que
// ocupa su lugar. El valor aleatorio de las marcas las distingue de cualquier comentario HTML que
// traiga el contenido.
struct Deferred {
    nonce: u32,
    components: Vec<(String, Box<dyn Component>)>,
}

impl Default for Context {
    fn default() -> Self {
        Self::base(None, &CoreTemplates::Standard)
    }
}

impl Context {
    // Construye el `Context` compartido por `new()`, `admin()` y `Default::default()`, evitando
    // duplicar la lista de campos entre ellos (y la recursión que tendría `new()` llamando a
    // `Default::default()`, o viceversa). Recibe la plantilla para que `admin()` no tenga que
    // construir con la plantilla estándar y sobrescribirla después.
    #[rustfmt::skip]
    fn base(request: Option<HttpRequest>, template: TemplateRef) -> Self {
        let locale = RequestLocale::from_request(request.as_ref());
        let current_user = Self::resolve_current_user(request.as_ref());
        let timezone = current_user.timezone();
        let theme = current_user.theme().unwrap_or_else(default_theme);
        let messages = request
            .as_ref()
            .map(|request| read_flash_messages(request.headers()))
            .unwrap_or_default();
        Context {
            request,
            locale,
            current_user,
            timezone,
            theme,
            template,
            favicon    : None,
            preloads   : Assets::<Preload>::new(),
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            responsives: ResponsiveStyles::new(),
            title      : Lc::none(),
            description: Lc::none(),
            metadata   : Vec::new(),
            properties : Vec::new(),
            body_props : Props::default(),
            regions    : ChildrenInRegions::default(),
            deferred   : None,
            params     : HashMap::default(),
            id_counters: Mutex::new(HashMap::new()),
            messages,
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
    // un usuario anónimo si no hay petición o ninguna extensión de autenticación está activa.
    fn resolve_current_user(request: Option<&HttpRequest>) -> CurrentUser {
        request
            .and_then(|r| r.extension::<CurrentUser>())
            .cloned()
            .unwrap_or_default()
    }

    // **< Context RENDER >*************************************************************************

    /// Renderiza los recursos del contexto.
    ///
    /// Si no se ha definido ningún [`Favicon`], declara un icono vacío para evitar que el navegador
    /// pida `/favicon.ico` en cada página, una ruta que normalmente no existe y que respondería con
    /// una página de error completa. Con el icono vacío no hace ninguna petición y muestra su icono
    /// genérico.
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
            } @else {
                link rel="icon" href="data:,";
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

    // Empieza a aplazar los componentes que lo pidan (`Component::is_deferred()`), mientras
    // `Page::render()` renderiza el `<body>`.
    pub(crate) fn start_deferring(&mut self) {
        use std::hash::{BuildHasher, Hasher, RandomState};

        self.deferred = Some(Deferred {
            nonce: RandomState::new().build_hasher().finish() as u32,
            components: Vec::new(),
        });
    }

    // Guarda un clon del componente para renderizarlo cuando termine el resto del `<body>`, y
    // devuelve la marca que ocupa su lugar, o `None` si no se está renderizando una página.
    pub(crate) fn defer(&mut self, component: &dyn Component) -> Option<Markup> {
        let deferred = self.deferred.as_mut()?;
        let mark = format!(
            "<!-- placeholder {} {:08x}-{} -->",
            component.short_name(),
            deferred.nonce,
            deferred.components.len()
        );
        // Se guarda una copia de la marca para buscarla después: clonarla cuesta menos que volver
        // a formatearla.
        deferred
            .components
            .push((mark.clone(), component.clone_box()));
        Some(PreEscaped(mark))
    }

    // Deja de aplazar, renderiza los componentes aplazados y sustituye sus marcas en `body`. Un
    // componente aplazado que contenga otro lo renderiza ya en el momento.
    pub(crate) async fn render_deferred(&mut self, body: Markup) -> Markup {
        let Some(deferred) = self.deferred.take() else {
            return body;
        };
        let mut body = body.into_string();
        for (mark, mut component) in deferred.components {
            let markup = component.render(self).await.into_string();
            // Sin nada que mostrar, la marca se queda: es un comentario HTML, y así no hay que
            // buscarla ni copiar el cuerpo, que es lo habitual (p. ej. `Messages` sin mensajes).
            if markup.is_empty() {
                continue;
            }
            // Una transformación del marcado que quite los comentarios (p. ej. un minificador) la
            // haría desaparecer, y sin este aviso el componente no se vería.
            let Some(start) = body.find(&mark) else {
                crate::trace::warn!(
                    path = self.request().map(|r| r.path()).unwrap_or("<unknown>"),
                    component = component.name(),
                    "deferred component mark not found in the page body, component not shown"
                );
                continue;
            };
            // Una sola reserva con la longitud exacta. `String::replace_range()` sería más lento,
            // porque copia la inserción byte a byte.
            body = util::join!(&body[..start], markup, &body[start + mark.len()..]);
        }
        PreEscaped(body)
    }

    // Retira los mensajes acumulados, para que el componente `Messages` los muestre una sola vez.
    pub(crate) fn take_messages(&mut self) -> Vec<StatusMessage> {
        std::mem::take(&mut self.messages)
    }
}

impl Contextual for Context {
    #[inline]
    fn context(&self) -> &Context {
        self
    }

    #[inline]
    fn context_mut(&mut self) -> &mut Context {
        self
    }
}
