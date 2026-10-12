use crate::BoxError;
use crate::actions;
use crate::async_trait;
use crate::config::LoadSettings;
use crate::core::AnyInfo;
use crate::core::action::ActionBox;
use crate::core::theme::ThemeRef;
use crate::locale::Lc;
use crate::web::Router;

/// Interfaz común que debe implementar cualquier extensión de PageTop.
///
/// Este trait es fácil de implementar, basta con declarar una estructura sin campos para la
/// extensión y sobrescribir los métodos que sean necesarios. Por ejemplo:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// pub struct MyExtension;
///
/// #[async_trait]
/// impl Extension for MyExtension {
///     fn name(&self) -> Lc {
///         Lc::n("My Extension")
///     }
///
///     fn description(&self) -> Lc {
///         Lc::n("Does something useful")
///     }
/// }
/// ```
#[async_trait]
pub trait Extension: AnyInfo + Send + Sync {
    /// Nombre de la extensión como *texto localizado* legible para el usuario.
    ///
    /// Predeterminado por el [`short_name()`](AnyInfo::short_name) del tipo asociado a la
    /// extensión.
    fn name(&self) -> Lc {
        Lc::n(self.short_name())
    }

    /// Descripción corta de la extensión como *texto localizado* para paneles, listados, etc.
    ///
    /// Por defecto devuelve un valor vacío (`Lc::default()`).
    fn description(&self) -> Lc {
        Lc::default()
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
    /// #[async_trait]
    /// impl Extension for MyTheme {
    ///     fn theme(&self) -> Option<ThemeRef> {
    ///         Some(&Self)
    ///     }
    /// }
    ///
    /// #[async_trait]
    /// impl Theme for MyTheme {}
    /// ```
    fn theme(&self) -> Option<ThemeRef> {
        None
    }

    /// Extensiones que deben inicializarse **antes** de esta.
    ///
    /// PageTop usa este método para construir un grafo de dependencias en tiempo de ejecución entre
    /// extensiones. Garantiza que todas las extensiones requeridas están presentes y se inicializan
    /// en el orden apropiado, es decir, las dependencias siempre antes que la extensión que las
    /// declara.
    ///
    /// Esta declaración es independiente de las dependencias en `Cargo.toml`, donde Rust gestiona
    /// las dependencias de los *crates* en tiempo de compilación.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # pub struct Database;
    /// # #[async_trait]
    /// # impl Extension for Database {}
    /// pub struct MyApp;
    ///
    /// #[async_trait]
    /// impl Extension for MyApp {
    ///     fn dependencies(&self) -> Vec<ExtensionRef> {
    ///         vec![&Database]
    ///     }
    /// }
    /// ```
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![]
    }

    /// Devuelve la lista de acciones que la extensión registra.
    ///
    /// Estas [acciones] se despachan por orden de registro o por [peso] (ver [`actions!`]),
    /// permitiendo personalizar el comportamiento de la aplicación en puntos específicos.
    ///
    /// PageTop lo invoca una sola vez por proceso, al registrar las extensiones y antes de
    /// [`initialize()`]. La lista queda fija a partir de ahí y no es posible añadir acciones más
    /// tarde.
    ///
    /// [acciones]: crate::core::action
    /// [peso]: crate::Weight
    /// [`actions!`]: crate::actions
    /// [`initialize()`]: Self::initialize
    fn actions(&self) -> Vec<ActionBox> {
        actions![]
    }

    /// Inicializa la extensión durante la fase de arranque de la aplicación.
    ///
    /// La firma extendida que muestra este método es el resultado de aplicar la macro
    /// [`pagetop::async_trait`] a la declaración:
    ///
    /// ```text
    /// async fn initialize(&self) -> Result<(), BoxError> { Ok(()) }
    /// ```
    ///
    /// Solo es necesario sobrescribir este método **cuando la extensión necesita ejecutar alguna
    /// lógica de inicialización**. PageTop lo invoca después de que todas las dependencias se han
    /// inicializado y antes de aceptar cualquier petición HTTP.
    ///
    /// Si devuelve un error, el arranque se detiene: PageTop lo registra en las trazas junto con
    /// sus causas, lo muestra en la salida de errores y termina el proceso con el código de salida
    /// `1`. El tipo [`BoxError`] permite propagar con `?` cualquier tipo de error.
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    ///
    /// pub struct MyExtension;
    ///
    /// #[async_trait]
    /// impl Extension for MyExtension {
    ///     async fn initialize(&self) -> Result<(), BoxError> {
    ///         // lógica de inicialización...
    ///         Ok(())
    ///     }
    /// }
    /// ```
    ///
    /// [`pagetop::async_trait`]: pagetop::async_trait
    /// [`BoxError`]: crate::BoxError
    async fn initialize(&self) -> Result<(), BoxError> {
        Ok(())
    }

    /// Devuelve los ajustes de configuración de la extensión, para cargarlos en el arranque de la
    /// aplicación.
    ///
    /// Se cargan en el orden en que se registran las extensiones y antes de llamar a
    /// [`initialize()`]. Así, un error de la configuración (como un valor no válido o una clave
    /// desconocida) detiene el arranque, igual que un error en [`initialize()`]. Cada extensión
    /// debe usar una sección propia: si dos extensiones reclaman la misma, el arranque también se
    /// detiene. Ver [`pagetop::config`] para saber cómo declarar los ajustes usando
    /// [`#[settings]`](crate::settings). La implementación por defecto devuelve `None`.
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// # use pagetop::prelude::*;
    /// pub mod config;
    ///
    /// pub struct Blog;
    ///
    /// #[async_trait]
    /// impl Extension for Blog {
    ///     fn settings(&self) -> Option<&'static dyn LoadSettings> {
    ///         Some(&config::SETTINGS)
    ///     }
    /// }
    /// ```
    ///
    /// [`initialize()`]: Self::initialize
    /// [`pagetop::config`]: crate::config
    fn settings(&self) -> Option<&'static dyn LoadSettings> {
        None
    }

    /// Registra rutas, servicios y capas de la extensión en el servidor web de la aplicación.
    ///
    /// Recibe las rutas acumuladas hasta ese momento, añade lo que la extensión necesite y retorna
    /// las rutas con las nuevas modificaciones. La implementación por defecto devuelve las rutas
    /// sin cambios.
    ///
    /// # Operaciones disponibles
    ///
    /// | Operación                        | Llamada sobre `router`                             |
    /// |----------------------------------|----------------------------------------------------|
    /// | Ruta HTTP                        | `.route("/path", web::get(handler))`               |
    /// | Rutas bajo prefijo común         | `.nest("/prefix", sub_router)`                     |
    /// | Estado compartido entre handlers | `.with_state(my_state)`                            |
    ///
    /// Los archivos estáticos se registran normalmente en [`configure_static_files`], donde no
    /// pasan por el middleware de las extensiones.
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
    /// #[async_trait]
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
    /// #[async_trait]
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
    /// ## Rutas con middleware acotado a esta extensión
    ///
    /// Cada `Router` mantiene su propia pila de middleware independiente. Cuando se crea un
    /// `Router::new()` separado y se llama a `.layer()` sobre él, esa capa sólo se aplica a las
    /// rutas de ese router concreto. Al fusionarlo con `.merge()` en el `router` principal, cada
    /// ruta se añade con su middleware asociado, sin tocar las demás.
    ///
    /// Otra cosa es llamar a `.layer()` directamente sobre el `router` principal que se recibe como
    /// parámetro, porque ese objeto ya contiene todas las rutas acumuladas por extensiones
    /// anteriores, y la capa se aplica a todas las rutas, no sólo a las propias.
    ///
    /// ```rust,ignore
    /// # use pagetop::prelude::*;
    /// pub struct Api;
    ///
    /// #[async_trait]
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
    /// Para el middleware que deba cubrir las rutas de **todas** las extensiones, usar
    /// [`configure_middleware`].
    ///
    /// [`configure_static_files`]: Self::configure_static_files
    /// [`configure_middleware`]: Self::configure_middleware
    fn configure_router(&self, router: Router) -> Router {
        router
    }

    /// Añade capas de middleware globales a las rutas de la aplicación.
    ///
    /// Se invoca **después** de que todas las extensiones hayan registrado sus rutas con
    /// [`configure_router`](Self::configure_router), de modo que las capas añadidas aquí se aplican
    /// a **todas** esas rutas, independientemente del orden de las extensiones. **No** se aplican a
    /// los archivos estáticos de [`configure_static_files`](Self::configure_static_files), que se
    /// registran después.
    ///
    /// Usar este método cuando el middleware deba interceptar todas las peticiones a rutas de la
    /// aplicación (p. ej. resolución de sesión, autenticación, etc.). Una capa que deba cubrir
    /// también los archivos estáticos (p. ej. ciertas cabeceras de seguridad) no puede añadirse
    /// aquí; una alternativa es aplicarla en el proxy inverso.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// # async fn session_middleware(req: web::Request, next: web::middleware::Next) -> Response {
    /// #     next.run(req).await
    /// # }
    /// pub struct MyAuth;
    ///
    /// #[async_trait]
    /// impl Extension for MyAuth {
    ///     fn configure_middleware(&self, router: Router) -> Router {
    ///         router.layer(web::middleware::from_fn(session_middleware))
    ///     }
    /// }
    /// ```
    fn configure_middleware(&self, router: Router) -> Router {
        router
    }

    /// Registra los archivos estáticos de la extensión (hojas de estilo, *scripts*, fuentes,
    /// imágenes, etc.) en el servidor web de la aplicación.
    ///
    /// Es un paso aparte de [`configure_router`](Self::configure_router) porque las capas de
    /// middleware sólo envuelven las rutas registradas antes de añadirlas. Este método se invoca
    /// **después** de [`configure_middleware`](Self::configure_middleware), así que estas rutas
    /// **no** pasan por las capas de middleware de las extensiones (p. ej. la resolución de
    /// sesión). Una página carga muchos recursos; sin esta separación, cada uno repetiría ese
    /// trabajo, que no necesita. La implementación por defecto devuelve las rutas sin cambios.
    ///
    /// Si algún archivo sí debe pasar por el middleware (p. ej. descargas que requieran una sesión
    /// iniciada), se registra en [`configure_router`](Self::configure_router).
    ///
    /// La macro [`serve_static_files!`](crate::serve_static_files) sombrea `router` internamente,
    /// por lo que el parámetro no necesita `mut`. Sí es necesario devolverlo al final.
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// # use pagetop::prelude::*;
    /// pub struct MyExtension;
    ///
    /// #[async_trait]
    /// impl Extension for MyExtension {
    ///     fn configure_static_files(&self, router: Router) -> Router {
    ///         serve_static_files!(router, [assets] => "/static");
    ///         router
    ///     }
    /// }
    /// ```
    fn configure_static_files(&self, router: Router) -> Router {
        router
    }
}

/// Representa una referencia a una extensión.
pub type ExtensionRef = &'static dyn Extension;
