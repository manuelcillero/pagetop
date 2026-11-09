use super::{AssetsOp, Context, ContextError};

use crate::auth::CurrentUser;
use crate::core::TypeInfo;
use crate::core::component::{ChildOp, Component, MessageLevel, StatusMessage};
use crate::core::theme::{CoreRegions, RegionRef, TemplateRef, ThemeRef, default_theme};
use crate::datetime::{DateFormat, DatePrecision, RelativeFormat, TimeFormat};
use crate::datetime::{DateInputError, DateTime, NaiveDate, NaiveTime, Tz, Utc};
use crate::html::StyleSheet;
use crate::html::{Assets, Favicon, JavaScript, Props, PropsOp, ResponsiveStyles, RoutePath};
use crate::locale::{LangId, LanguageIdentifier, Lc, RequestLocale};
use crate::response::read_flash_messages;
use crate::web::HttpRequest;
use crate::{CowStr, builder_impl, util};

use std::any::TypeId;

// RFC 3339 con el offset de la zona horaria activa (no UTC fijo): igual en todos los idiomas, no
// es una clave Fluent. Usado sólo por `format_iso_datetime()`.
const ISO_DATETIME: &str = "%Y-%m-%dT%H:%M:%S%:z";

/// Interfaz para gestionar el **contexto de renderizado** de un documento HTML.
///
/// Todo lo que implementa `Contextual` es también proveedor de idioma ([`LangId`]), con el idioma
/// del documento, así que puede usarse directamente en [`Lc::lookup()`] o [`Lc::using()`]. Además,
/// ofrece métodos para:
///
/// - Establecer el **idioma** del documento ([`with_langid()`](Self::with_langid)).
/// - Almacenar la **petición HTTP** de origen ([`with_request()`](Self::with_request)).
/// - Conocer la **identidad del usuario actual** ([`current_user()`](Self::current_user)) y la
///   **zona horaria efectiva** del documento ([`timezone()`](Self::timezone)).
/// - Seleccionar el **tema** y la **plantilla** de renderizado.
/// - Definir el **título**, la **descripción** y los **metadatos** del documento.
/// - Administrar **recursos** del documento como el icono [`Favicon`], las hojas de estilo
///   [`StyleSheet`] o los scripts [`JavaScript`], directamente o mediante una operación
///   [`AssetsOp`].
/// - Leer y mantener **parámetros dinámicos tipados** de contexto.
/// - Construir **rutas** que preserven el idioma ([`route()`](Self::route)) e **identificadores**
///   únicos ([`build_id()`](Self::build_id)).
/// - Acumular **mensajes para el usuario** ([`push_message()`](Self::push_message)).
/// - Formatear **fechas y horas** ([`format_date()`](Self::format_date),
///   [`format_time()`](Self::format_time), [`format_datetime()`](Self::format_datetime) y demás)
///   para la zona horaria e idioma del documento.
///
/// Todos los métodos actúan sobre el [`Context`] que devuelven [`context()`](Self::context) y
/// [`context_mut()`](Self::context_mut), los únicos que hay que implementar. Lo implementan
/// [`Context`] (que se devuelve a sí mismo) y [`Page`](crate::response::Page) (que devuelve el
/// suyo), así que ambos ofrecen exactamente la misma interfaz.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_aliner::Aliner;
/// fn prepare_context<C: Contextual>(cx: C) -> C {
///     cx.with_langid(&Locale::resolve("es-ES"))
///       .with_theme(&Aliner)
///       .with_template(&CoreTemplates::Standard)
///       .with_assets(Favicon::new().with_icon("/favicon.ico"))
///       .with_assets(StyleSheet::from("/css/app.css"))
///       .with_assets(JavaScript::defer("/js/app.js"))
///       .with_param("user_id", 42_i32)
/// }
/// ```
#[builder_impl]
pub trait Contextual {
    /// Devuelve una referencia al contexto de renderizado.
    fn context(&self) -> &Context;

    /// Devuelve una referencia mutable al contexto de renderizado.
    ///
    /// Resulta útil para pasar el contexto a funciones o componentes que lo reciben como parámetro
    /// (`&mut Context`).
    fn context_mut(&mut self) -> &mut Context;

    // **< Contextual BUILDER >*********************************************************************

    /// Establece el idioma del documento.
    fn with_langid(self, language: &impl LangId) -> Self {
        self.context_mut().locale.with_langid(language);
        self
    }

    /// Fuerza la zona horaria que se utilizará para mostrar fechas y horas en el documento.
    ///
    /// Sustituye la zona horaria aplicada (por el usuario actual o por configuración de la
    /// aplicación) por otra explícita. Ver [`CurrentUser::timezone()`].
    ///
    /// [`CurrentUser::timezone()`]: crate::auth::CurrentUser::timezone
    fn with_timezone(self, tz: Tz) -> Self {
        self.context_mut().timezone = tz;
        self
    }

    /// Almacena la petición HTTP de origen en el contexto.
    ///
    /// Al asociar la petición, recalcula el idioma ([`RequestLocale::from_request()`]), establece
    /// el usuario actual ([`current_user()`]) y, a partir de éste, asigna la zona horaria efectiva
    /// ([`timezone()`]) y el tema ([`theme()`]), descartando en el proceso cualquier idioma, zona
    /// horaria o tema anteriores.
    ///
    /// Si sabes que vas a forzar el idioma, la zona horaria o el tema, llama a `with_request()`
    /// primero en la cadena de construcción, nunca después.
    ///
    /// [`RequestLocale::from_request()`]: crate::locale::RequestLocale::from_request
    /// [`current_user()`]: Self::current_user
    /// [`timezone()`]: Self::timezone
    /// [`theme()`]: Self::theme
    fn with_request(self, request: Option<HttpRequest>) -> Self {
        let cx = self.context_mut();
        // Los mensajes para la página siguiente se leen una sola vez, con la primera petición (las
        // páginas de error, p. ej., parten de un contexto sin petición).
        if cx.request.is_none()
            && let Some(request) = &request
        {
            cx.messages.extend(read_flash_messages(request.headers()));
        }
        cx.request = request;
        // Recalcula el *locale*, el usuario actual, la zona horaria y el tema según la nueva
        // petición y la política de negociación configurada.
        cx.locale = RequestLocale::from_request(cx.request.as_ref());
        cx.current_user = Context::resolve_current_user(cx.request.as_ref());
        cx.timezone = cx.current_user.timezone();
        cx.theme = cx.current_user.theme().unwrap_or_else(default_theme);
        self
    }

    /// Especifica el tema para renderizar el documento.
    fn with_theme(self, theme: ThemeRef) -> Self {
        self.context_mut().theme = theme;
        self
    }

    /// Especifica la plantilla para renderizar el documento.
    fn with_template(self, template: TemplateRef) -> Self {
        self.context_mut().template = template;
        self
    }

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
    /// let cx = Context::default()
    ///     .with_param("user_id", 42_i32)
    ///     .with_param("title", "Hello".to_string())
    ///     .with_param("flags", vec!["a", "b"]);
    /// ```
    fn with_param<T: Send + Sync + 'static>(self, key: &'static str, value: T) -> Self {
        let type_name = TypeInfo::FullName.of::<T>();
        self.context_mut()
            .params
            .insert(key, (Box::new(value), type_name));
        self
    }

    /// Añade un recurso ([`Favicon`], [`StyleSheet`], [`JavaScript`] o
    /// [`Preload`](crate::html::Preload)) directamente, o aplica una operación [`AssetsOp`] sobre
    /// los recursos del contexto.
    fn with_assets(self, op: impl Into<AssetsOp>) -> Self {
        let cx = self.context_mut();
        match op.into() {
            // Favicon.
            AssetsOp::SetFavicon(favicon) => {
                cx.favicon = favicon;
            }
            AssetsOp::SetFaviconIfNone(icon) => {
                if cx.favicon.is_none() {
                    cx.favicon = Some(icon);
                }
            }
            // Preloads.
            AssetsOp::AddPreload(preload) => {
                cx.preloads.add(preload);
            }
            AssetsOp::RemovePreload(path) => {
                cx.preloads.remove(path);
            }
            // Stylesheets.
            AssetsOp::AddStyleSheet(css) => {
                cx.stylesheets.add(css);
            }
            AssetsOp::RemoveStyleSheet(path) => {
                cx.stylesheets.remove(path);
            }
            // Scripts JavaScript.
            AssetsOp::AddJavaScript(js) => {
                cx.javascripts.add(js);
            }
            AssetsOp::RemoveJavaScript(path) => {
                cx.javascripts.remove(path);
            }
            // Estilos responsive.
            AssetsOp::AddResponsiveStyle(breakpoint, classes, property, value) => {
                cx.responsives
                    .add_style(breakpoint, classes, property, value);
            }
            AssetsOp::AddResponsiveStyles(breakpoint, classes, styles) => {
                cx.responsives.add_styles(breakpoint, classes, styles);
            }
        }
        self
    }

    /// Establece el título del documento como un valor traducible.
    ///
    /// Además de usarse en el `<head>`, lo ven las plantillas y los componentes al renderizarse,
    /// como el marco de las páginas del panel de administración
    /// ([`AdminFrame`](crate::base::component::AdminFrame)), que lo muestra como encabezado.
    fn with_title(self, title: Lc) -> Self {
        self.context_mut().title = title;
        self
    }

    /// Establece la descripción del documento como un valor traducible.
    ///
    /// Se usa en la etiqueta `<meta name="description">` del `<head>`.
    fn with_description(self, description: Lc) -> Self {
        self.context_mut().description = description;
        self
    }

    /// Añade una entrada `<meta name="..." content="...">` al `<head>`.
    fn with_metadata(self, name: &'static str, content: impl Into<CowStr>) -> Self {
        self.context_mut().metadata.push((name, content.into()));
        self
    }

    /// Añade una entrada `<meta property="..." content="...">` al `<head>`.
    fn with_property(self, property: &'static str, content: impl Into<CowStr>) -> Self {
        self.context_mut()
            .properties
            .push((property, content.into()));
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del elemento `<body>`.
    fn with_body_props(self, op: PropsOp) -> Self {
        self.context_mut().body_props.alter_prop(op);
        self
    }

    /// Añade un componente o aplica una operación [`ChildOp`] en la región por defecto del
    /// documento.
    fn with_child(self, op: impl Into<ChildOp>) -> Self {
        self.context_mut()
            .regions
            .alter_child_in(&CoreRegions::Content, op.into());
        self
    }

    /// Añade un componente o aplica una operación [`ChildOp`] en una región específica del
    /// documento.
    fn with_child_in(self, region: RegionRef, op: impl Into<ChildOp>) -> Self {
        self.context_mut().regions.alter_child_in(region, op.into());
        self
    }

    // **< Contextual GETTERS >*********************************************************************

    /// Devuelve una referencia a la petición HTTP asociada, si existe.
    fn request(&self) -> Option<&HttpRequest> {
        self.context().request.as_ref()
    }

    /// Devuelve la identidad del usuario actual.
    ///
    /// Si ninguna extensión de autenticación ha inyectado un
    /// [`CurrentUser`](crate::auth::CurrentUser) en las extensiones de la petición HTTP, devuelve
    /// un usuario anónimo ([`CurrentUser::anonymous()`](crate::auth::CurrentUser::anonymous)).
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
    fn current_user(&self) -> &CurrentUser {
        &self.context().current_user
    }

    /// Devuelve la zona horaria efectiva para mostrar fechas y horas en el documento.
    ///
    /// Se resuelve una sola vez, al construir el `Context` o al llamar a [`with_request()`], y
    /// queda guardado para consultar. Llamarlo dentro de un bucle que renderiza miles de filas (p.
    /// ej. `format_date()`/`format_time()`/`format_datetime()` en cada celda de una tabla) no
    /// repite esa resolución.
    ///
    /// Ver [`CurrentUser::timezone()`] para el orden de resolución.
    ///
    /// [`with_request()`]: Self::with_request
    /// [`CurrentUser::timezone()`]: crate::auth::CurrentUser::timezone
    fn timezone(&self) -> Tz {
        self.context().timezone
    }

    /// Devuelve el tema que se usará para renderizar el documento.
    fn theme(&self) -> ThemeRef {
        self.context().theme
    }

    /// Devuelve la plantilla configurada para renderizar el documento.
    fn template(&self) -> TemplateRef {
        self.context().template
    }

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
    /// let cx = Context::default()
    ///     .with_param("user_id", 42_i32)
    ///     .with_param("title", "Hello".to_string());
    ///
    /// let id: i32 = *cx.param("user_id").unwrap();
    /// let title: &String = cx.param("title").unwrap();
    ///
    /// // Error de tipo:
    /// assert!(cx.param::<String>("user_id").is_err());
    /// ```
    fn param<T: 'static>(&self, key: &'static str) -> Result<&T, ContextError> {
        let (any, type_name) = self
            .context()
            .params
            .get(key)
            .ok_or(ContextError::ParamNotFound)?;
        any.downcast_ref::<T>()
            .ok_or_else(|| ContextError::ParamTypeMismatch {
                key,
                expected: TypeInfo::FullName.of::<T>(),
                saved: type_name,
            })
    }

    /// Devuelve el parámetro clonado o el **valor por defecto del tipo** (`T::default()`) si no
    /// existe.
    ///
    /// En desarrollo y en pruebas falla si existe con otro tipo; en producción devuelve también el
    /// valor por defecto.
    fn param_or_default<T: Clone + Default + 'static>(&self, key: &'static str) -> T {
        checked_param(self.param::<T>(key)).unwrap_or_default()
    }

    /// Devuelve el parámetro clonado o un **valor por defecto** si no existe.
    ///
    /// En desarrollo y en pruebas falla si existe con otro tipo; en producción devuelve también el
    /// valor por defecto.
    fn param_or<T: Clone + 'static>(&self, key: &'static str, default: T) -> T {
        checked_param(self.param::<T>(key)).unwrap_or(default)
    }

    /// Devuelve el parámetro clonado o el **valor evaluado** por la función `f` si no existe.
    ///
    /// En desarrollo y en pruebas falla si existe con otro tipo; en producción evalúa también `f`.
    fn param_or_else<T: Clone + 'static, F: FnOnce() -> T>(&self, key: &'static str, f: F) -> T {
        checked_param(self.param::<T>(key)).unwrap_or_else(f)
    }

    /// Devuelve el Favicon de los recursos del contexto.
    fn favicon(&self) -> Option<&Favicon> {
        self.context().favicon.as_ref()
    }

    /// Devuelve las hojas de estilo de los recursos del contexto.
    fn stylesheets(&self) -> &Assets<StyleSheet> {
        &self.context().stylesheets
    }

    /// Devuelve los scripts JavaScript de los recursos del contexto.
    fn javascripts(&self) -> &Assets<JavaScript> {
        &self.context().javascripts
    }

    /// Devuelve los estilos *responsive* acumulados en el contexto.
    fn responsive_styles(&self) -> &ResponsiveStyles {
        &self.context().responsives
    }

    /// Devuelve el título del documento, o [`Lc::none()`] si no tiene.
    fn title(&self) -> &Lc {
        &self.context().title
    }

    /// Devuelve la descripción del documento, o [`Lc::none()`] si no tiene.
    fn description(&self) -> &Lc {
        &self.context().description
    }

    /// Devuelve la lista de metadatos `<meta name=...>`.
    fn metadata(&self) -> &[(&'static str, CowStr)] {
        &self.context().metadata
    }

    /// Devuelve la lista de propiedades `<meta property=...>`.
    fn properties(&self) -> &[(&'static str, CowStr)] {
        &self.context().properties
    }

    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del elemento `<body>`.
    fn body_props(&self) -> &Props {
        &self.context().body_props
    }

    /// Devuelve los mensajes de usuario acumulados que aún no ha mostrado ningún componente
    /// [`Messages`](crate::base::component::Messages).
    fn messages(&self) -> &[StatusMessage] {
        &self.context().messages
    }

    /// Indica si hay mensajes de usuario acumulados que aún no ha mostrado ningún componente
    /// [`Messages`](crate::base::component::Messages).
    fn has_messages(&self) -> bool {
        !self.context().messages.is_empty()
    }

    // **< Contextual HELPERS >*********************************************************************

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
    fn route(&self, path: impl Into<RoutePath>) -> RoutePath {
        let locale = &self.context().locale;
        let mut route = path.into();
        if !route.is_external() && locale.needs_lang_query() {
            route.alter_param("lang", locale.lang_query_value());
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
    fn build_id<C: Component>(&self, segments: usize) -> String {
        let path: Vec<&str> = TypeInfo::FullName.of::<C>().split("::").collect();
        let segments = if segments == 0 || segments >= path.len() {
            path.len()
        } else {
            segments
        };
        let count = {
            let mut map = self.context().id_counters.lock().unwrap();
            let n = map.entry(TypeId::of::<C>()).or_insert(0);
            *n += 1;
            *n
        };
        let prefix = path[path.len() - segments..].join("-").to_lowercase();
        util::join!(prefix, "-", count.to_string())
    }

    /// Devuelve `id` si contiene un valor, o genera uno único con [`build_id`](Self::build_id)
    /// si es `None`.
    fn required_id<C: Component>(&self, id: Option<String>, segments: usize) -> String {
        match id {
            Some(id) => id,
            None => self.build_id::<C>(segments),
        }
    }

    /// Acumula un [`StatusMessage`] en el contexto para notificar al usuario.
    ///
    /// Pueden generarse en cualquier punto del ciclo de una petición web (handlers, renderizado,
    /// lógica de negocio, etc.) que tengan acceso al contexto. Los muestra, y los retira, el primer
    /// componente [`Messages`](crate::base::component::Messages) que se renderice después de
    /// añadirlos: uno propio de la página, si lo hay, o el que encabeza la región
    /// [`CoreRegions::Content`], que se renderiza una vez renderizado el resto del `<body>` y
    /// recoge los que queden.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # let mut cx = Context::default();
    /// cx.push_message(MessageLevel::Warning, Lc::n("Session is not valid"));
    /// ```
    fn push_message(&mut self, level: MessageLevel, text: Lc) {
        self.context_mut()
            .messages
            .push(StatusMessage::new(level, text));
    }

    /// Elimina un parámetro del contexto. Devuelve `true` si la clave existía y se eliminó.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let mut cx = Context::default().with_param("temp", 1u8);
    /// assert!(cx.remove_param("temp"));
    /// assert!(!cx.remove_param("temp")); // ya no existe
    /// ```
    fn remove_param(&mut self, key: &'static str) -> bool {
        self.context_mut().params.remove(key).is_some()
    }

    /// Formatea una fecha y hora completas en la zona horaria efectiva del documento
    /// ([`timezone()`](Self::timezone)), combinando un [`DateFormat`] y un [`TimeFormat`]
    /// independientes (posiblemente distintos entre sí, como una fecha larga con la hora corta),
    /// con el separador de la clave Fluent `datetime_join` del idioma efectivo.
    ///
    /// `dt` se graba siempre en UTC. Aquí la conversión a la zona horaria de visualización se hace
    /// una sola vez, antes de aplicar `date`/`time` por separado (evita convertir la misma fecha y
    /// hora dos veces).
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # fn show(cx: &Context, dt: DateTime<Utc>) {
    /// let text = cx.format_datetime(dt, DateFormat::Medium, TimeFormat::Short);
    /// # }
    /// ```
    fn format_datetime(
        &self,
        dt: DateTime<Utc>,
        date: DateFormat<'_>,
        time: TimeFormat<'_>,
    ) -> String
    where
        Self: Sized,
    {
        let local = dt.with_timezone(&self.timezone());
        let date = date.apply(local.date_naive(), self);
        let time = time.apply(local.time(), self);
        Lc::l("datetime_join")
            .with_arg("date", date.clone())
            .with_arg("time", time.clone())
            .lookup(self)
            .unwrap_or_else(|| format!("{date} {time}"))
    }

    /// Formatea una fecha sin hora ([`NaiveDate`]).
    ///
    /// A diferencia de [`format_datetime()`] y [`format_time()`], que reciben [`DateTime<Utc>`] y
    /// convierten a la zona horaria efectiva, aquí no hay ninguna conversión que aplicar. El
    /// argumento `date` ya es la fecha civil a mostrar. No es una inconsistencia de tipos entre
    /// métodos de la misma familia. Un `DateTime<Utc>` es un instante grabado en UTC ("qué día es"
    /// depende de la zona horaria de quien mira); un `NaiveDate` es una fecha civil sin hora ni
    /// zona horaria asociada (una fecha de nacimiento, "socio desde junio de 2020", etc.), es el
    /// mismo dato en cualquier sitio desde el que se mire. Forzar aquí un `DateTime<Utc>` obligaría
    /// a inventar una hora falsa para encajar en el tipo, y expondría el dato al error que se
    /// quiere evitar. Si se convierte a un huso horario negativo, esa hora inventada puede hacer
    /// caer la fecha en el día anterior, por lo que dos usuarios en zonas distintas verían un día
    /// distinto para lo que en el dominio es un único hecho fijo.
    ///
    /// Usa [`DateFormat`], cuyos patrones por defecto no incluyen componentes de hora.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # fn show(cx: &Context, date: NaiveDate) {
    /// let text = cx.format_date(date, DateFormat::Medium);
    /// # }
    /// ```
    ///
    /// Si el dato de origen es un instante (`DateTime<Utc>`) y sólo hace falta mostrar su fecha,
    /// conviértelo explícitamente a la zona horaria efectiva antes de llamar:
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # fn show(cx: &Context, dt: DateTime<Utc>) {
    /// // `dt`, p. ej. `created_at`: un instante real, con hora, no una fecha civil fabricada.
    /// let local_date = dt.with_timezone(&cx.timezone()).date_naive();
    /// let text = cx.format_date(local_date, DateFormat::Medium);
    /// # }
    /// ```
    ///
    /// [`format_datetime()`]: Self::format_datetime
    /// [`format_time()`]: Self::format_time
    /// [`DateTime<Utc>`]: chrono::DateTime
    fn format_date(&self, date: NaiveDate, format: DateFormat<'_>) -> String
    where
        Self: Sized,
    {
        format.apply(date, self)
    }

    /// Formatea sólo la hora del día de `dt`, convertida a la zona horaria efectiva
    /// ([`timezone()`](Self::timezone)). Usa [`TimeFormat`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # fn show(cx: &Context, dt: DateTime<Utc>) {
    /// let text = cx.format_time(dt, TimeFormat::Short);
    /// # }
    /// ```
    fn format_time(&self, dt: DateTime<Utc>, format: TimeFormat<'_>) -> String
    where
        Self: Sized,
    {
        format.apply(dt.with_timezone(&self.timezone()).time(), self)
    }

    /// Formatea `dt` como ISO 8601 (RFC 3339), con el offset de la zona horaria efectiva
    /// ([`timezone()`](Self::timezone)); igual para todos los idiomas.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # fn show(cx: &Context, dt: DateTime<Utc>) {
    /// let text = cx.format_iso_datetime(dt);
    /// # }
    /// ```
    fn format_iso_datetime(&self, dt: DateTime<Utc>) -> String
    where
        Self: Sized,
    {
        dt.with_timezone(&self.timezone())
            .format(ISO_DATETIME)
            .to_string()
    }

    /// Formatea `dt` en relación al momento actual ("hace 3 años", "dentro de 5 días"), en la zona
    /// horaria efectiva del documento ([`timezone()`](Self::timezone)). Usa [`RelativeFormat`].
    ///
    /// La comparación es entre fechas civiles (la de `dt` y la de "ahora", ambas convertidas a la
    /// zona horaria efectiva), no entre instantes. Un valor de hace unos minutos o dentro de unos
    /// minutos se muestra como "hoy" si cae en la misma fecha civil que "ahora".
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # fn show(cx: &Context, dt: DateTime<Utc>) {
    /// let text = cx.format_relative(dt, RelativeFormat::Medium);
    /// # }
    /// ```
    fn format_relative(&self, dt: DateTime<Utc>, format: RelativeFormat) -> String
    where
        Self: Sized,
    {
        let today = Utc::now().with_timezone(&self.timezone()).date_naive();
        let target = dt.with_timezone(&self.timezone()).date_naive();
        format.apply(target, today, self)
    }

    /// Formatea `date` como fecha de **inicio**, con la precisión indicada ("desde junio", "desde
    /// junio de 2026", "desde el 3 de junio de 2026"). Usa [`DatePrecision`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # fn show(cx: &Context, date: NaiveDate) {
    /// let text = cx.format_since(date, DatePrecision::Medium);
    /// # }
    /// ```
    fn format_since(&self, date: NaiveDate, precision: DatePrecision) -> String
    where
        Self: Sized,
    {
        precision.apply_since(date, self)
    }

    /// Formatea `date` como fecha de **fin**, con la precisión indicada ("hasta junio", "hasta
    /// junio de 2026", "hasta el 3 de junio de 2026"). Usa [`DatePrecision`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # fn show(cx: &Context, date: NaiveDate) {
    /// let text = cx.format_until(date, DatePrecision::Medium);
    /// # }
    /// ```
    fn format_until(&self, date: NaiveDate, precision: DatePrecision) -> String
    where
        Self: Sized,
    {
        precision.apply_until(date, self)
    }

    /// Lee una fecha sin hora escrita por el usuario (p. ej. en un
    /// [`form::date::Field::date()`](crate::base::component::form::date::Field::date)).
    ///
    /// Acepta el formato de fecha del idioma efectivo del documento (p. ej. `dd/mm/aaaa` en
    /// español o `mm/dd/yyyy` en inglés de Estados Unidos), con día y mes de una o dos cifras y el
    /// año siempre con cuatro, y también el formato ISO 8601 (`aaaa-mm-dd`). Es el mismo formato
    /// con el que el campo muestra el valor. Una fecha sin hora no se convierte de zona horaria
    /// (ver [`format_date()`](Self::format_date)).
    ///
    /// Devuelve `Ok(None)` si el texto está en blanco; comprobar si el campo es obligatorio queda
    /// a cargo de quien llama.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # fn read(cx: &Context, text: &str) -> Result<(), Lc> {
    /// let expires: Option<NaiveDate> = cx.parse_date(text).map_err(|e| e.message())?;
    /// # Ok(())
    /// # }
    /// ```
    fn parse_date(&self, text: &str) -> Result<Option<NaiveDate>, DateInputError>
    where
        Self: Sized,
    {
        crate::datetime::input::parse_date(text, self)
    }

    /// Lee una hora del día escrita por el usuario (p. ej. en un
    /// [`form::date::Field::time()`](crate::base::component::form::date::Field::time)).
    ///
    /// Acepta el formato de hora del idioma efectivo (p. ej. `hh:mm`), con la hora y los minutos de
    /// una o dos cifras, y también `hh:mm` en ISO 8601. No se convierte de zona horaria: es una
    /// hora civil. Devuelve `Ok(None)` si el texto está en blanco.
    fn parse_time(&self, text: &str) -> Result<Option<NaiveTime>, DateInputError>
    where
        Self: Sized,
    {
        crate::datetime::input::parse_time(text, self)
    }

    /// Lee una fecha y hora escrita por el usuario en su hora local (p. ej. en un
    /// [`form::date::Field::datetime()`](crate::base::component::form::date::Field::datetime)) y
    /// la convierte a UTC, que es como se graban siempre los instantes.
    ///
    /// Acepta el formato del idioma efectivo (la fecha y la hora separadas por un espacio, p. ej.
    /// `dd/mm/aaaa hh:mm`) y también ISO 8601 (`aaaa-mm-ddThh:mm`, con `T` o con un espacio entre
    /// la fecha y la hora), y lo interpreta en la zona horaria efectiva del documento
    /// ([`timezone()`](Self::timezone)). Devuelve `Ok(None)` si el texto está en blanco.
    ///
    /// En los cambios de hora:
    ///
    /// - Una hora que no existe, porque el reloj se adelanta (p. ej. las 02:30 del último domingo
    ///   de marzo en Madrid), devuelve [`DateInputError::NonexistentTime`].
    /// - Una hora que existe dos veces, porque el reloj se retrasa (p. ej. las 02:30 del último
    ///   domingo de octubre en Madrid), se interpreta como la primera de las dos.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # fn read(cx: &Context, text: &str) -> Result<(), Lc> {
    /// let extracted: Option<DateTime<Utc>> = cx.parse_datetime(text).map_err(|e| e.message())?;
    /// # Ok(())
    /// # }
    /// ```
    fn parse_datetime(&self, text: &str) -> Result<Option<DateTime<Utc>>, DateInputError>
    where
        Self: Sized,
    {
        crate::datetime::input::parse_datetime(text, self, self.timezone())
    }
}

/// Cualquier contexto de renderizado es proveedor de idioma, con el idioma del documento.
///
/// El idioma lo resuelve [`RequestLocale`], que tiene en cuenta la petición HTTP (parámetro `?lang`
/// e idioma preferido del usuario), la configuración global de idioma de la aplicación, la cabecera
/// `Accept-Language` y/o el idioma de respaldo, según la negociación indicada en
/// [`global::SETTINGS.app.lang_negotiation`].
///
/// [`global::SETTINGS.app.lang_negotiation`]: crate::global::AppSettings::lang_negotiation
impl<T: Contextual> LangId for T {
    #[inline]
    fn langid(&self) -> &'static LanguageIdentifier {
        self.context().locale.langid()
    }
}

// Un parámetro ausente es normal; pedirlo con un tipo distinto al guardado es error de desarrollo.
fn checked_param<T: Clone>(param: Result<&T, ContextError>) -> Option<T> {
    param
        .inspect_err(|err| {
            debug_assert!(
                !matches!(err, ContextError::ParamTypeMismatch { .. }),
                "{err}"
            );
        })
        .ok()
        .cloned()
}
