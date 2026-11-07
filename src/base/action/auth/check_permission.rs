use crate::prelude::*;

use super::FnActionCheckPerm;

use std::ops::ControlFlow;

/// Acción para comprobar si el usuario actual tiene un permiso concreto.
///
/// Las extensiones de autenticación pueden registrar su handler sobre esta acción para implementar
/// su modelo de permisos. Los handlers son aditivos de tal forma que si cualquiera de ellos asigna
/// `granted = true`, el permiso se concede. La consultan [`has_permission()`] y el acceso de las
/// rutas ([`Access::Permission`]).
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
///         actions![action::auth::CheckPermission::new(check_my_permissions)]
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
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionCheckPerm`].
    pub fn new(f: FnActionCheckPerm) -> Self {
        CheckPermission { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones hasta que una concede el permiso, y devuelve si alguna lo ha hecho.
    #[inline]
    pub(crate) fn dispatch(request: &HttpRequest, perm: PermissionRef) -> bool {
        let mut granted = false;
        try_dispatch_actions(|action: &Self| {
            (action.f)(request, perm, &mut granted);
            if granted {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        granted
    }
}
