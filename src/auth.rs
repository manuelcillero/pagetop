//! Identidad del usuario y sistema de autorización extensible.
//!
//! Define el tipo [`CurrentUser`] que PageTop inyecta en el [`Context`] con la información mínima
//! sobre el usuario que ejecuta la petición actual ([`HttpRequest`]).
//!
//! Incluye la acción [`CheckPermission`] para que las extensiones puedan implementar sus propios
//! modelos de permisos. Y también las funciones auxiliares [`has_permission()`] y
//! [`require_permission()`] para validar en el comienzo de cada handler, antes de construir ni
//! ejecutar nada, si la petición está autorizada.
//!
//! La resolución concreta del usuario (sesión en BD, LDAP, OAuth, ...) y la lógica de permisos
//! (RBAC, grupos LDAP, ...) son responsabilidad de las extensiones de autenticación. Un concepto
//! como "administrador" que tiene todos los permisos no es responsabilidad de PageTop: cada
//! extensión decide si existe y, si es así, lo aplica dentro de su propio handler
//! [`CheckPermission`].
//!
//! [`Context`]: crate::core::component::Context

use crate::core::action::{ActionDispatcher, try_dispatch_actions};
use crate::core::theme::{ThemeRef, theme_by_short_name};
use crate::datetime::{Timezone, Tz};
use crate::locale::{LanguageIdentifier, Lc, Locale};
use crate::response::ErrorPage;
use crate::web::HttpRequest;
use crate::{AutoDefault, CowStr, Getters, Weight, builder_impl, global};

use std::ops::ControlFlow;

// **< CurrentUser >********************************************************************************

/// Identidad mínima del usuario que ejecuta la petición actual.
///
/// Se almacena automáticamente en el [`Context`] a partir de la petición HTTP. La identidad se
/// extrae de las extensiones de la petición, que una extensión de autenticación inyecta mediante su
/// middleware. Sin extensión de autenticación, o si ésta no inyecta ninguna identidad, el usuario
/// es anónimo ([`CurrentUser::anonymous()`], que también es el valor por defecto).
///
/// Se accede usando [`Contextual::current_user()`].
///
/// Los usuarios pueden tener idioma, zona horaria y tema preferidos. Se asignan con su valor en
/// bruto y se validan al asignarlos: un idioma no soportado, una zona horaria desconocida o un tema
/// no habilitado en la aplicación se descartan y el dato queda sin valor, como si el usuario no
/// tuviera ninguno. Así, las preferencias de un `CurrentUser` son siempre válidas.
///
/// Los datos extendidos del usuario autenticado (roles, permisos, cuenta completa, ...) son
/// responsabilidad de la extensión de autenticación y se obtienen a través de
/// [`HttpRequest::extension`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let user = CurrentUser::authenticated(42, "Alice")
///     .with_language("es-ES")
///     .with_timezone("Europe/Madrid");
/// ```
///
/// [`Context`]: crate::core::component::Context
/// [`Contextual::current_user()`]: crate::core::component::Contextual::current_user
/// [`HttpRequest::extension`]: crate::web::HttpRequest::extension
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct CurrentUser {
    /// Devuelve el identificador del usuario, o `None` si es anónimo.
    #[getters(copy)]
    id: Option<i32>,
    // Siempre `Some` en un usuario autenticado y `None` en uno anónimo, igual que `id`.
    #[getters(skip)]
    display_name: Option<String>,
    /// Devuelve el idioma preferido del usuario, o `None` si no tiene ninguno.
    ///
    /// Lo tiene en cuenta [`RequestLocale`](crate::locale::RequestLocale) al decidir el idioma de
    /// la petición.
    #[getters(copy)]
    language: Option<&'static LanguageIdentifier>,
    // Ver `timezone()`, que devuelve la zona horaria efectiva.
    #[getters(skip)]
    timezone: Option<Tz>,
    /// Devuelve el tema preferido del usuario, o `None` si no tiene ninguno.
    ///
    /// Lo tiene en cuenta el [`Context`](crate::core::component::Context) de la petición al elegir
    /// el tema con el que se renderiza.
    #[getters(copy)]
    theme: Option<ThemeRef>,
}

#[builder_impl]
impl CurrentUser {
    /// Crea un usuario anónimo, sin idioma, zona horaria ni tema preferidos.
    pub fn anonymous() -> Self {
        Self::default()
    }

    /// Crea un usuario autenticado, sin idioma, zona horaria ni tema preferidos.
    pub fn authenticated(id: i32, display_name: impl Into<String>) -> Self {
        CurrentUser {
            id: Some(id),
            display_name: Some(display_name.into()),
            ..Self::default()
        }
    }

    // **< CurrentUser BUILDER >********************************************************************

    /// Asigna el idioma preferido a partir de su identificador (p. ej. `"es-ES"` o `"es"`).
    ///
    /// Se resuelve con [`Locale::resolve()`](crate::locale::Locale::resolve); si el idioma no está
    /// soportado por la aplicación, o es `None`, el usuario queda sin idioma preferido.
    pub fn with_language<'a>(mut self, language: impl Into<Option<&'a str>>) -> Self {
        self.language = language
            .into()
            .and_then(|language| Locale::resolve(language).as_option());
        self
    }

    /// Asigna la zona horaria a partir de su nombre IANA (p. ej. `"Europe/Madrid"`).
    ///
    /// Si el nombre no corresponde a ninguna zona horaria conocida, o es `None`, el usuario queda
    /// sin zona horaria propia.
    pub fn with_timezone<'a>(mut self, timezone: impl Into<Option<&'a str>>) -> Self {
        self.timezone = timezone.into().and_then(|timezone| timezone.parse().ok());
        self
    }

    /// Asigna el tema preferido a partir de su nombre corto (p. ej. `"basic"`).
    ///
    /// Se busca con [`theme_by_short_name()`](crate::core::theme::theme_by_short_name); si el tema
    /// no está habilitado en la aplicación, o es `None`, el usuario queda sin tema preferido.
    pub fn with_theme<'a>(mut self, theme: impl Into<Option<&'a str>>) -> Self {
        self.theme = theme.into().and_then(theme_by_short_name);
        self
    }

    // **< CurrentUser GETTERS >********************************************************************

    /// Devuelve `true` si el usuario no está autenticado.
    pub fn is_anonymous(&self) -> bool {
        self.id.is_none()
    }

    /// Devuelve `true` si el usuario está autenticado.
    pub fn is_authenticated(&self) -> bool {
        self.id.is_some()
    }

    /// Devuelve el nombre visible del usuario, o `None` si es anónimo.
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// Devuelve la zona horaria efectiva del usuario.
    ///
    /// Devuelve su zona horaria si tiene una y [`global::SETTINGS.app.timezone_per_user`] lo
    /// permite; en otro caso, devuelve [`Timezone::default_tz()`].
    ///
    /// Normalmente se resuelve una sola vez, al construir el [`Context`] de la petición. A partir
    /// de ese momento el renderizado del documento no vuelve a llamarlo porque usa el valor ya
    /// resuelto vía [`Contextual::timezone()`].
    ///
    /// [`global::SETTINGS.app.timezone_per_user`]: crate::global::App::timezone_per_user
    /// [`Context`]: crate::core::component::Context
    /// [`Contextual::timezone()`]: crate::core::component::Contextual::timezone
    pub fn timezone(&self) -> Tz {
        match self.timezone {
            Some(tz) if global::SETTINGS.app.timezone_per_user => tz,
            _ => Timezone::default_tz(),
        }
    }
}

// **< Permission >*********************************************************************************

/// Clave tipada de un permiso de acceso.
///
/// Cada extensión que lo requiera puede definir su propio enum de permisos e implementar este trait
/// para obtener la clave textual que finalmente se compara contra su modelo de permisos (RBAC en
/// base de datos, grupos LDAP, ...).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// #[derive(Clone, Copy, Debug)]
/// pub enum MyPermission {
///     EditPosts,
///     DeletePosts,
/// }
///
/// impl Permission for MyPermission {
///     fn key(&self) -> CowStr {
///         match self {
///             Self::EditPosts => "my_extension.edit_posts".into(),
///             Self::DeletePosts => "my_extension.delete_posts".into(),
///         }
///     }
/// }
/// ```
pub trait Permission: Send + Sync {
    /// Clave única del permiso (p. ej. `"my_extension.edit_posts"`).
    fn key(&self) -> CowStr;

    /// Descripción breve para humanos (p. ej. en una pantalla de asignación de permisos a roles).
    ///
    /// Por defecto devuelve la propia clave; una extensión que registre sus permisos en un catálogo
    /// visible debería sobrescribirlo con un texto traducible.
    fn label(&self) -> Lc {
        Lc::n(self.key())
    }

    /// Identificador estable de la categoría del permiso, usado para agrupar en un catálogo (p.
    /// ej. `"administration"`). Por defecto no pertenece a ningún grupo.
    fn group(&self) -> &'static str {
        ""
    }

    /// Título traducible de [`group()`](Self::group), mostrado en la UI de administración.
    ///
    /// Por defecto reutiliza el propio identificador del grupo como texto fijo.
    fn group_label(&self) -> Lc {
        Lc::n(self.group())
    }
}

/// Referencia estática a un permiso de acceso.
///
/// Es el tipo que recorre toda la API de autorización ([`has_permission()`],
/// [`require_permission()`] o [`CheckPermission`]).
pub type PermissionRef = &'static dyn Permission;

// **< CheckPermission >****************************************************************************

/// Tipo de función para comprobar si el usuario actual tiene un permiso concreto.
///
/// Se invoca con:
///
/// - `request`: petición HTTP desde la que se accede a los datos inyectados por el middleware de
///   autenticación.
/// - `perm`: permiso a comprobar; el handler usará [`Permission::key()`] para identificarlo contra
///   su propio modelo de permisos.
/// - `granted`: referencia mutable; el handler debe asignarla a `true` si concede el permiso.
pub type FnActionCheckPerm = fn(request: &HttpRequest, perm: PermissionRef, granted: &mut bool);

/// Acción para comprobar si el usuario actual tiene un permiso concreto.
///
/// Las extensiones de autenticación pueden registrar su handler sobre esta acción para implementar
/// su modelo de permisos. Los handlers son aditivos de tal forma que si cualquiera de ellos asigna
/// `granted = true`, el permiso se concede.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// fn check_my_permissions(request: &HttpRequest, perm: PermissionRef, granted: &mut bool) {
///     // Leer los datos extendidos de autenticación inyectados en la petición.
///     // Comparar `perm.key()` contra el modelo propio.
///     // Si concede el permiso, asignar `*granted = true`.
/// }
///
/// pub struct MyAuth;
///
/// #[async_trait]
/// impl Extension for MyAuth {
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![CheckPermission::new(check_my_permissions)]
///     }
/// }
/// ```
pub struct CheckPermission {
    f: FnActionCheckPerm,
    weight: Weight,
}

impl ActionDispatcher for CheckPermission {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl CheckPermission {
    /// Registra una nueva acción para la comprobación de permisos.
    pub fn new(f: FnActionCheckPerm) -> Self {
        CheckPermission { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}

// **< has_permission >*****************************************************************************

/// Comprueba si el usuario actual tiene el permiso indicado.
///
/// Despacha la acción [`CheckPermission`]: cualquier extensión registrada puede conceder el permiso
/// asignando `granted = true` en su handler. Si no hay extensiones de autenticación activas,
/// devuelve `false` para cualquier usuario, incluido el anónimo.
///
/// La decisión de conceder o denegar permisos al usuario anónimo también es responsabilidad de cada
/// extensión.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # #[derive(Clone, Copy, Debug)]
/// # enum MyPermission { Edit }
/// # impl Permission for MyPermission {
/// #     fn key(&self) -> CowStr { "myapp.edit".into() }
/// # }
/// async fn my_handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     if !has_permission(&request, &MyPermission::Edit) {
///         return Err(ErrorPage::NotFound(Some(request)));
///     }
///     Page::new(request).render().await
/// }
/// ```
pub fn has_permission(request: &HttpRequest, perm: PermissionRef) -> bool {
    // Despacha las acciones registradas con salida anticipada en cuanto una concede el permiso.
    let mut granted = false;
    try_dispatch_actions(|action: &CheckPermission| {
        (action.f)(request, perm, &mut granted);
        if granted {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    granted
}

// **< require_permission >*************************************************************************

/// Comprueba un permiso y devuelve `Err(ErrorPage::AccessDenied)` si se deniega.
///
/// Ejecuta [`has_permission()`] para el caso más habitual: detener un handler con una respuesta 403
/// en cuanto falta el permiso, sin repetir el `if`/`return` en cada punto de comprobación. Se hace
/// directamente sobre la petición, antes de construir ni ejecutar nada (`Context`, `Page`,
/// consultas a datos, etc.), para no hacer ningún trabajo si la petición no está autorizada.
///
/// Si la aplicación necesita ocultar la existencia del recurso a quien no tiene permiso (devolver
/// un 404 en vez de un 403), no se puede reutilizar esta función: hay que llamar a
/// `has_permission()` directamente, como en su propio ejemplo.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # #[derive(Clone, Copy, Debug)]
/// # enum MyPermission { Edit }
/// # impl Permission for MyPermission {
/// #     fn key(&self) -> CowStr { "myapp.edit".into() }
/// # }
/// async fn my_handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     // Comprueba si la petición está autorizada.
///     require_permission(&request, &MyPermission::Edit)?;
///
///     // Ejecuta las instrucciones propias de la petición.
///     Page::new(request)
///         .with_child(Html::with(|_| html! { p { "You have permission!" } }))
///         .render()
///         .await
/// }
/// ```
pub fn require_permission(request: &HttpRequest, perm: PermissionRef) -> Result<(), ErrorPage> {
    if has_permission(request, perm) {
        Ok(())
    } else {
        Err(ErrorPage::AccessDenied(Some(request.clone())))
    }
}
