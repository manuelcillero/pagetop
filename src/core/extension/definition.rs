use crate::actions;
use crate::core::AnyInfo;
use crate::core::action::ActionBox;
use crate::core::theme::ThemeRef;
use crate::locale::L10n;
use crate::web::Router;

/// Interfaz común que debe implementar cualquier extensión de PageTop.
///
/// Este *trait* es fácil de implementar, basta con declarar una estructura sin campos para la
/// extensión y sobrescribir los métodos que sean necesarios. Por ejemplo:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// pub struct MyExtension;
///
/// impl Extension for MyExtension {
///     fn name(&self) -> L10n {
///         L10n::n("My Extension")
///     }
///
///     fn description(&self) -> L10n {
///         L10n::n("Does something useful")
///     }
/// }
/// ```
pub trait Extension: AnyInfo + Send + Sync {
    /// Nombre de la extensión como *texto localizado* legible para el usuario.
    ///
    /// Predeterminado por el [`short_name()`](AnyInfo::short_name) del tipo asociado a la
    /// extensión.
    fn name(&self) -> L10n {
        L10n::n(self.short_name())
    }

    /// Descripción corta de la extensión como *texto localizado* para paneles, listados, etc.
    ///
    /// Por defecto devuelve un valor vacío (`L10n::default()`).
    fn description(&self) -> L10n {
        L10n::default()
    }

    /// Devuelve una referencia a esta misma extensión cuando actúa como un tema.
    ///
    /// Para ello, la implementación concreta debe ser una extensión que también implemente
    /// [`Theme`](crate::core::theme::Theme). Por defecto, asume que la extensión no es un tema y
    /// devuelve `None`.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// pub struct MyTheme;
    ///
    /// impl Extension for MyTheme {
    ///     fn theme(&self) -> Option<ThemeRef> {
    ///         Some(&Self)
    ///     }
    /// }
    ///
    /// impl Theme for MyTheme {}
    /// ```
    fn theme(&self) -> Option<ThemeRef> {
        None
    }

    /// Otras extensiones que deben habilitarse **antes** de esta.
    ///
    /// PageTop resolverá automáticamente estas dependencias respetando el orden durante el arranque
    /// de la aplicación.
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![]
    }

    /// Devuelve la lista de acciones que la extensión registra.
    ///
    /// Estas [acciones](crate::core::action) se despachan por orden de registro o por
    /// [peso](crate::Weight) (ver [`actions!`](crate::actions)), permitiendo
    /// personalizar el comportamiento de la aplicación en puntos específicos.
    fn actions(&self) -> Vec<ActionBox> {
        actions![]
    }

    /// Inicializa la extensión durante la fase de arranque de la aplicación.
    ///
    /// Se llama una sola vez, después de que todas las dependencias se han inicializado y antes de
    /// aceptar cualquier petición HTTP.
    fn initialize(&self) {}

    /// Registra rutas, servicios y capas de la extensión en el servidor web de la aplicación.
    ///
    /// Recibe las rutas acumuladas hasta ese momento, añade lo que la extensión necesite y retorna
    /// las rutas con las nuevas modificaciones. La implementación por defecto devuelve las rutas
    /// sin cambios.
    ///
    /// # Operaciones disponibles
    ///
    /// | Operación                          | Llamada sobre `router`                          |
    /// |------------------------------------|-------------------------------------------------|
    /// | Ruta HTTP                          | `.route("/path", web::get(handler))`            |
    /// | Rutas bajo prefijo común           | `.nest("/prefix", sub_router)`                  |
    /// | Archivos estáticos                 | `serve_static_files!(router, [...] => "/path")` |
    /// | Estado compartido entre *handlers* | `.with_state(my_state)`                         |
    ///
    /// # Ejemplos
    ///
    /// ## Rutas HTTP básicas
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # async fn list_posts() -> &'static str { "" }
    /// # async fn view_post() -> &'static str { "" }
    /// # async fn create_post() -> &'static str { "" }
    /// pub struct Blog;
    ///
    /// impl Extension for Blog {
    ///     fn configure_router(&self, router: Router) -> Router {
    ///         router
    ///             .route("/posts",      web::get(list_posts))
    ///             .route("/posts/{id}", web::get(view_post))
    ///             .route("/posts/new",  web::post(create_post))
    ///     }
    /// }
    /// ```
    ///
    /// ## Rutas agrupadas bajo un prefijo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # async fn dashboard() -> &'static str { "" }
    /// # async fn list_users() -> &'static str { "" }
    /// pub struct Admin;
    ///
    /// impl Extension for Admin {
    ///     fn configure_router(&self, router: Router) -> Router {
    ///         let admin = Router::new()
    ///             .route("/dashboard", web::get(dashboard))
    ///             .route("/users",     web::get(list_users));
    ///
    ///         router.nest("/admin", admin)
    ///     }
    /// }
    /// ```
    ///
    /// ## Rutas con *middleware* acotado a esta extensión
    ///
    /// Cada `Router` mantiene su propia pila de *middleware* independiente. Cuando se crea un
    /// `Router::new()` separado y se llama a `.layer()` sobre él, esa capa sólo se aplica a las
    /// rutas de ese *router* concreto. Al fusionarlo con `.merge()` en el `router` principal, cada
    /// ruta se añade con su *middleware* asociado, sin tocar las demás.
    ///
    /// Otra cosa es llamar a `.layer()` directamente sobre el `router` principal que se recibe como
    /// parámetro, porque ese objeto ya contiene todas las rutas acumuladas por extensiones
    /// anteriores, y la capa se aplica a todas las rutas, no sólo a las propias.
    ///
    /// ```rust,ignore
    /// # use pagetop::prelude::*;
    /// pub struct Api;
    ///
    /// impl Extension for Api {
    ///     fn configure_router(&self, router: Router) -> Router {
    ///         let api = Router::new()
    ///             .route("/api/data", web::get(get_data))
    ///             .layer(auth_layer());
    ///         router.merge(api)
    ///     }
    /// }
    /// ```
    ///
    /// Para *middleware* que deba cubrir **todas** las rutas, usar
    /// [`configure_middleware`](Self::configure_middleware).
    ///
    /// ## Archivos estáticos
    ///
    /// La macro [`serve_static_files!`](crate::serve_static_files) sombrea `router` internamente,
    /// por lo que el parámetro no necesita `mut`. Sí es necesario devolverlo al final.
    ///
    /// ```rust,ignore
    /// # use pagetop::prelude::*;
    /// pub struct MyExtension;
    ///
    /// impl Extension for MyExtension {
    ///     fn configure_router(&self, router: Router) -> Router {
    ///         serve_static_files!(router, [assets] => "/static");
    ///         router
    ///     }
    /// }
    /// ```
    fn configure_router(&self, router: Router) -> Router {
        router
    }

    /// Añade capas de *middleware* globales al *router* ya completamente preparado.
    ///
    /// Se invoca **después** de que todas las extensiones hayan registrado sus rutas con
    /// [`configure_router`](Self::configure_router), de modo que las capas añadidas aquí se aplican
    /// a **todas** las rutas de la aplicación, independientemente del orden de las extensiones.
    ///
    /// Usar este método cuando el *middleware* deba interceptar cualquier petición entrante (p. ej.
    /// resolución de sesión, autenticación, cabeceras de seguridad, ...).
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # use axum::middleware;
    /// # async fn session_middleware(
    /// #     req: axum::extract::Request,
    /// #     next: middleware::Next,
    /// # ) -> axum::response::Response { next.run(req).await }
    /// pub struct MyAuth;
    ///
    /// impl Extension for MyAuth {
    ///     fn configure_middleware(&self, router: Router) -> Router {
    ///         router.layer(middleware::from_fn(session_middleware))
    ///     }
    /// }
    /// ```
    fn configure_middleware(&self, router: Router) -> Router {
        router
    }
}

/// Representa una referencia a una extensión.
pub type ExtensionRef = &'static dyn Extension;
