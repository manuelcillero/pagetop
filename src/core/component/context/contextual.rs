use super::{AssetsOp, ContextError};

use crate::auth::CurrentUser;
use crate::builder_impl;
use crate::core::component::ChildOp;
use crate::core::theme::{RegionRef, TemplateRef, ThemeRef};
use crate::datetime::{DateFormat, DatePrecision, RelativeFormat, TimeFormat};
use crate::datetime::{DateTime, NaiveDate, Tz, Utc};
use crate::html::{Assets, Favicon, JavaScript, Props, PropsOp, ResponsiveStyles, StyleSheet};
use crate::locale::{LangId, Lc};
use crate::web::HttpRequest;

// RFC 3339 con el offset de la zona horaria activa (no UTC fijo): igual en todos los idiomas, no
// es una clave Fluent. Usado sólo por `format_iso_datetime()`.
const ISO_DATETIME: &str = "%Y-%m-%dT%H:%M:%S%:z";

/// Interfaz para gestionar el **contexto de renderizado** de un documento HTML.
///
/// `Contextual` extiende [`LangId`] para establecer el idioma del documento y añade métodos para:
///
/// - Almacenar la **petición HTTP** de origen.
/// - Conocer la **identidad del usuario actual** ([`current_user()`](Self::current_user)) y la
///   **zona horaria efectiva** del documento ([`timezone()`](Self::timezone)).
/// - Seleccionar la **plantilla** y el **tema** de renderizado.
/// - Administrar **recursos** del documento como el icono [`Favicon`], las hojas de estilo
///   [`StyleSheet`] o los scripts [`JavaScript`], directamente o mediante una operación
///   [`AssetsOp`].
/// - Leer y mantener **parámetros dinámicos tipados** de contexto.
/// - Formatear **fechas y horas** ([`format_date()`](Self::format_date),
///   [`format_time()`](Self::format_time), [`format_datetime()`](Self::format_datetime) y demás)
///   para la zona horaria e idioma del documento.
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
///       .with_template(&CoreTemplates::Standard)
///       .with_theme(&Aliner)
///       .with_assets(Favicon::new().with_icon("/favicon.ico"))
///       .with_assets(StyleSheet::from("/css/app.css"))
///       .with_assets(JavaScript::defer("/js/app.js"))
///       .with_param("user_id", 42_i32)
/// }
/// ```
#[builder_impl]
pub trait Contextual: LangId {
    // **< Contextual BUILDER >*********************************************************************

    /// Establece el idioma del documento.
    fn with_langid(self, language: &impl LangId) -> Self;

    /// Fuerza la zona horaria que se utilizará para mostrar fechas y horas en el documento.
    ///
    /// Sustituye la zona horaria aplicada (por el usuario actual o por configuración de la
    /// aplicación) por otra explícita. Ver [`CurrentUser::timezone()`].
    ///
    /// [`CurrentUser::timezone()`]: crate::auth::CurrentUser::timezone
    fn with_timezone(self, tz: Tz) -> Self;

    /// Almacena la petición HTTP de origen en el contexto.
    ///
    /// Al asociar la petición, recalcula el idioma ([`RequestLocale::from_request()`]), establece
    /// el usuario actual ([`current_user()`]) y, a partir de éste, asigna la zona horaria efectiva
    /// ([`timezone()`]), descartando en el proceso cualquier idioma o zona horaria anteriores.
    ///
    /// Si sabes que vas a forzar el idioma o la zona horaria, llama a `with_request()` primero en
    /// la cadena de construcción, nunca después.
    ///
    /// [`RequestLocale::from_request()`]: crate::locale::RequestLocale::from_request
    /// [`current_user()`]: Self::current_user
    /// [`timezone()`]: Self::timezone
    fn with_request(self, request: Option<HttpRequest>) -> Self;

    /// Especifica la plantilla para renderizar el documento.
    fn with_template(self, template: TemplateRef) -> Self;

    /// Especifica el tema para renderizar el documento.
    fn with_theme(self, theme: ThemeRef) -> Self;

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
    fn with_param<T: Send + Sync + 'static>(self, key: &'static str, value: T) -> Self;

    /// Añade un recurso ([`Favicon`], [`StyleSheet`], [`JavaScript`] o
    /// [`Preload`](crate::html::Preload)) directamente, o aplica una operación [`AssetsOp`] sobre
    /// los recursos del contexto.
    fn with_assets(self, op: impl Into<AssetsOp>) -> Self;

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del elemento `<body>`.
    fn with_body_props(self, op: PropsOp) -> Self;

    /// Añade un componente o aplica una operación [`ChildOp`] en la región por defecto del
    /// documento.
    fn with_child(self, op: impl Into<ChildOp>) -> Self;

    /// Añade un componente o aplica una operación [`ChildOp`] en una región específica del
    /// documento.
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
    fn timezone(&self) -> Tz;

    /// Devuelve la plantilla configurada para renderizar el documento.
    fn template(&self) -> TemplateRef;

    /// Devuelve el tema que se usará para renderizar el documento.
    fn theme(&self) -> ThemeRef;

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

    /// Devuelve los estilos *responsive* acumulados en el contexto.
    fn responsive_styles(&self) -> &ResponsiveStyles;

    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del elemento `<body>`.
    fn body_props(&self) -> &Props;

    // **< Contextual HELPERS >*********************************************************************

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
    fn remove_param(&mut self, key: &'static str) -> bool;
}
