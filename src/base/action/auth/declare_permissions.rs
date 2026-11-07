use crate::prelude::*;

use super::FnActionDeclarePermissions;

/// Acción para declarar los permisos de una extensión en el catálogo de permisos.
///
/// Cada extensión añade una instancia en [`Extension::actions()`]. PageTop despacha todas al
/// arrancar, después de registrar las acciones y antes de inicializar las extensiones, y construye
/// el catálogo global ([`PermissionRegistry::global()`]). El texto y el grupo de cada permiso los
/// aporta el propio [`Permission`] ([`label()`], [`group()`] y [`group_label()`]).
///
/// Declarar un permiso no lo concede a nadie: sólo lo da a conocer, para que una extensión de
/// autenticación pueda ofrecerlo (p. ej. en una pantalla de asignación de permisos a roles). Sin
/// ninguna, el permiso existe en el catálogo pero [`has_permission()`] lo deniega siempre.
///
/// Un permiso que exige una ruta ([`Access::Permission`]) o una pieza del panel de administración
/// sin estar declarado se avisa en las trazas al arrancar: nadie podría concederlo desde una
/// pantalla así.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// #[derive(Clone, Copy, Debug)]
/// enum MyPermission {
///     DoSomething,
/// }
///
/// impl Permission for MyPermission {
///     fn key(&self) -> CowStr {
///         match self {
///             Self::DoSomething => "myext:do_something".into(),
///         }
///     }
/// }
///
/// fn declare_permissions(registry: &mut PermissionRegistry) {
///     registry.register(&MyPermission::DoSomething);
/// }
///
/// pub struct MyExtension;
///
/// #[async_trait]
/// impl Extension for MyExtension {
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![action::auth::DeclarePermissions::new(declare_permissions)]
///     }
/// }
/// ```
///
/// [`label()`]: Permission::label
/// [`group()`]: Permission::group
/// [`group_label()`]: Permission::group_label
pub struct DeclarePermissions {
    f: FnActionDeclarePermissions,
    weight: Weight,
}

impl ActionDispatcher for DeclarePermissions {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclarePermissions {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionDeclarePermissions`].
    pub fn new(f: FnActionDeclarePermissions) -> Self {
        DeclarePermissions { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    ///
    /// Determina el orden de registro en el catálogo y, con él, el de los permisos de un mismo
    /// grupo en [`PermissionRegistry::by_group()`].
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones sobre el catálogo que se está construyendo.
    #[inline]
    pub(crate) fn dispatch(registry: &mut PermissionRegistry) {
        dispatch_actions(|action: &Self| (action.f)(registry));
    }
}
