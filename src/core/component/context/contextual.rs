use super::{AssetsOp, ContextError};

use crate::auth::CurrentUser;
use crate::builder_impl;
use crate::core::component::ChildOp;
use crate::core::theme::{RegionRef, TemplateRef, ThemeRef};
use crate::html::{Assets, Favicon, JavaScript, Props, PropsOp, ResponsiveStyles, StyleSheet};
use crate::locale::LangId;
use crate::web::HttpRequest;

/// Interfaz para gestionar el **contexto de renderizado** de un documento HTML.
///
/// `Contextual` extiende [`LangId`] para establecer el idioma del documento y añade métodos para:
///
/// - Almacenar la **petición HTTP** de origen.
/// - Seleccionar la **plantilla** y el **tema** de renderizado.
/// - Administrar **recursos** del documento como el icono [`Favicon`], las hojas de estilo
///   [`StyleSheet`] o los scripts [`JavaScript`], directamente o mediante una operación
///   [`AssetsOp`].
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

    /// Almacena la petición HTTP de origen en el contexto.
    ///
    /// También recalcula el idioma ([`RequestLocale::from_request()`]) y
    /// [`current_user()`](Self::current_user) a partir de la petición indicada, descartando
    /// cualquier idioma forzado antes con [`with_langid()`](Self::with_langid) o el usuario ya
    /// resuelto. Si necesitas forzar el idioma o el usuario, llama a `with_request()` primero en
    /// la cadena de construcción, nunca después.
    ///
    /// [`RequestLocale::from_request()`]: crate::locale::RequestLocale::from_request
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
