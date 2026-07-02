//! Identidad del usuario y sistema de autorización extensible.
//!
//! Define el tipo mínimo [`CurrentUser`] que PageTop inyecta en el [`Context`] de cada petición.
//! Incluye también la acción [`CheckPermission`] para que las extensiones puedan implementar sus
//! propios modelos de permisos, y la función auxiliar [`has_permission()`].
//!
//! La resolución concreta del usuario (sesión en BD, LDAP, OAuth, ...) y la lógica de permisos
//! (RBAC, grupos LDAP, ...) son responsabilidad de las extensiones de autenticación.

use crate::core::action::{ActionDispatcher, ActionKey, try_dispatch_actions};
use crate::core::component::Context;
use crate::{UniqueId, Weight};

// **< CurrentUser >********************************************************************************

/// Identidad mínima del usuario que ejecuta la petición actual.
///
/// Se almacena automáticamente en el [`Context`] a partir de la petición HTTP (ver
/// [`Context::new()`](crate::core::component::Context::new)). La identidad se extrae de las
/// extensiones de la petición, que una extensión de autenticación inyecta mediante su *middleware*.
///
/// Se accede con [`Contextual::current_user()`](crate::core::component::Contextual::current_user).
///
/// Los datos extendidos del usuario autenticado (roles, permisos, cuenta completa, ...) son
/// responsabilidad de la extensión de autenticación y se obtienen a través de
/// [`HttpRequest::extension`](crate::web::HttpRequest::extension).
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

// **< CheckPermission >****************************************************************************

/// Tipo de función para comprobar si el usuario actual tiene un permiso concreto.
///
/// Se invoca con:
///
/// - `cx`: el contexto de renderizado desde el que se puede acceder a la petición HTTP y a
///   cualquier dato inyectado por el *middleware* de autenticación.
/// - `key`: clave del permiso a comprobar (p. ej. `"myapp.edit_posts"`).
/// - `granted`: referencia mutable; el *handler* debe asignarla a `true` si concede el permiso.
pub type FnCheckPermission = fn(cx: &Context, key: &str, granted: &mut bool);

/// Acción para comprobar si el usuario actual tiene un permiso concreto.
///
/// Las extensiones de autenticación registran *handlers* de esta acción para implementar su modelo
/// de permisos. Los *handlers* son aditivos: si cualquiera de ellos asigna `granted = true`, el
/// permiso se concede.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// fn check_my_permissions(cx: &Context, key: &str, granted: &mut bool) {
///     // Leer datos extendidos de autenticación desde las extensiones de la petición.
///     // Si el usuario tiene el permiso, asignar `*granted = true`.
/// }
///
/// pub struct MyAuth;
///
/// impl Extension for MyAuth {
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![CheckPermission::new(check_my_permissions)]
///     }
/// }
/// ```
pub struct CheckPermission {
    f: FnCheckPermission,
    weight: Weight,
}

impl ActionDispatcher for CheckPermission {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl CheckPermission {
    /// Registra una nueva acción para la comprobación de permisos.
    pub fn new(f: FnCheckPermission) -> Self {
        CheckPermission { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    // Despacha las acciones registradas con salida anticipada en cuanto una concede el permiso.
    #[inline]
    pub(crate) fn check(cx: &Context, key: &str) -> bool {
        let mut granted = false;
        try_dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, None),
            |action: &Self| {
                (action.f)(cx, key, &mut granted);
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
/// asignando `granted = true` en su *handler*. Si no hay extensiones de autenticación activas,
/// devuelve `false` para cualquier usuario, incluido el anónimo.
///
/// La decisión de conceder o denegar permisos al usuario anónimo también es responsabilidad de cada
/// extensión.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// async fn my_handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     let mut page = Page::new(request.clone());
///     if !has_permission(page.context(), "myapp.edit") {
///         return Err(ErrorPage::NotFound(request));
///     }
///     page.render()
/// }
/// ```
pub fn has_permission(cx: &Context, key: &str) -> bool {
    CheckPermission::check(cx, key)
}
