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

use crate::core::action::{ActionDispatcher, ActionKey, try_dispatch_actions};
use crate::locale::Lc;
use crate::response::ErrorPage;
use crate::web::HttpRequest;
use crate::{CowStr, UniqueId, Weight};

// **< CurrentUser >********************************************************************************

/// Identidad mínima del usuario que ejecuta la petición actual.
///
/// Se almacena automáticamente en el [`Context`] a partir de la petición HTTP. La identidad se
/// extrae de las extensiones de la petición, que una extensión de autenticación inyecta mediante su
/// middleware.
///
/// Se accede usando [`Contextual::current_user()`].
///
/// Los datos extendidos del usuario autenticado (roles, permisos, cuenta completa, ...) son
/// responsabilidad de la extensión de autenticación y se obtienen a través de
/// [`HttpRequest::extension`].
///
/// [`Context`]: crate::core::component::Context
/// [`Contextual::current_user()`]: crate::core::component::Contextual::current_user
/// [`HttpRequest::extension`]: crate::web::HttpRequest::extension
#[derive(Clone, Debug)]
pub enum CurrentUser {
    /// Usuario no autenticado.
    Anonymous,
    /// Usuario autenticado con su identificador y nombre visible.
    Authenticated {
        /// Identificador único del usuario en el sistema.
        id: i32,
        /// Nombre visible del usuario.
        display_name: String,
    },
}

impl CurrentUser {
    /// Devuelve `true` si el usuario no está autenticado.
    pub fn is_anonymous(&self) -> bool {
        matches!(self, CurrentUser::Anonymous)
    }

    /// Devuelve `true` si el usuario está autenticado.
    pub fn is_authenticated(&self) -> bool {
        matches!(self, CurrentUser::Authenticated { .. })
    }

    /// Devuelve el identificador del usuario, o `None` si es anónimo.
    pub fn id(&self) -> Option<i32> {
        match self {
            CurrentUser::Anonymous => None,
            CurrentUser::Authenticated { id, .. } => Some(*id),
        }
    }

    /// Devuelve el nombre visible del usuario, o `None` si es anónimo.
    pub fn display_name(&self) -> Option<&str> {
        match self {
            CurrentUser::Anonymous => None,
            CurrentUser::Authenticated { display_name, .. } => Some(display_name),
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
/// # use pagetop::auth::Permission;
/// # use pagetop::CowStr;
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

    // Despacha las acciones registradas con salida anticipada en cuanto una concede el permiso.
    #[inline]
    pub(crate) fn check(request: &HttpRequest, perm: PermissionRef) -> bool {
        let mut granted = false;
        try_dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, None),
            |action: &Self| {
                (action.f)(request, perm, &mut granted);
                if granted {
                    std::ops::ControlFlow::Break(())
                } else {
                    std::ops::ControlFlow::Continue(())
                }
            },
        );
        granted
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
    CheckPermission::check(request, perm)
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
// `ErrorPage` incluye `Option<HttpRequest>` en cada variante y es el tipo de error ya establecido
// para toda la respuesta HTTP; boxearlo aquí sólo para esta función no compensa.
#[allow(clippy::result_large_err)]
pub fn require_permission(request: &HttpRequest, perm: PermissionRef) -> Result<(), ErrorPage> {
    if has_permission(request, perm) {
        Ok(())
    } else {
        Err(ErrorPage::AccessDenied(Some(request.clone())))
    }
}
