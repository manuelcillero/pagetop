//! Acciones para que otras extensiones intervengan en el ciclo de vida de las cuentas.
//!
//! Todas son síncronas, como el resto de acciones de PageTop: la que necesite hacer E/S debe
//! lanzar su propia tarea o, si pinta algo, aportar un componente que la haga al renderizarse.
//!
//! La acción para declarar proveedores de autenticación,
//! [`DeclareAuthProviders`](crate::provider::DeclareAuthProviders), está en el módulo
//! [`provider`](crate::provider).

use pagetop::prelude::*;

use crate::account::{Account, UserStatus};

// **< AfterSessionResolved >***********************************************************************

/// Tipo de función de la acción [`AfterSessionResolved`].
pub type FnAfterSessionResolved = fn(request: &HttpRequest, account: &Account) -> Option<Response>;

/// Acción que se ejecuta en cada petición con una sesión iniciada.
///
/// Se ejecuta justo tras resolver la cuenta y antes de llegar al *handler*. Si devuelve una
/// respuesta, se responde con ella y la petición no llega al *handler* (p. ej. para llevar al
/// usuario a completar su perfil).
///
/// Responde la primera, por orden de peso, que devuelva una respuesta. Se ejecuta en todas las
/// peticiones de quien ha iniciado sesión, salvo las de archivos estáticos, así que debe decidir
/// muy deprisa y sin E/S; también debe dejar pasar la propia página a la que redirige, o el usuario
/// no podría llegar a ella.
///
/// Va dentro del *middleware* de sesión de `pagetop-user`, no en una capa propia: las capas de las
/// extensiones se ejecutan en orden y una propia podría hacerlo antes de que se resuelva la sesión.
pub struct AfterSessionResolved {
    f: FnAfterSessionResolved,
    weight: Weight,
}

impl ActionDispatcher for AfterSessionResolved {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl AfterSessionResolved {
    /// Registra una nueva acción.
    pub fn new(f: FnAfterSessionResolved) -> Self {
        AfterSessionResolved { f, weight: 0 }
    }

    /// Opcional. Las acciones de menor peso se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    // Despacha las acciones hasta que una responde. Primero las recoge, en su orden, y sólo si hay
    // alguna separa la petición para crear la `HttpRequest`: así no cuesta nada a las aplicaciones
    // que no la usan (separarla y volver a montarla añadía unos 180 ns a cada petición con sesión).
    pub(crate) fn dispatch(req: &mut web::Request, account: &Account) -> Option<Response> {
        let mut actions: Vec<FnAfterSessionResolved> = Vec::new();
        dispatch_actions(|action: &Self| actions.push(action.f));
        if actions.is_empty() {
            return None;
        }
        let (parts, body) = std::mem::take(req).into_parts();
        let request = HttpRequest::from_parts(&parts);
        let response = actions.iter().find_map(|f| f(&request, account));
        *req = web::Request::from_parts(parts, body);
        response
    }
}

// **< AfterUserStatusChanged >*********************************************************************

/// Tipo de función de la acción [`AfterUserStatusChanged`].
pub type FnAfterUserStatusChanged = fn(user_id: i32, before: UserStatus, after: UserStatus);

/// Acción que se ejecuta tras cambiar el estado de una cuenta.
///
/// Sirve para avisar a quien corresponda cuando un administrador la bloquea, la activa o la
/// aprueba, o cuando la cambia el proveedor que la gestiona.
///
/// No se ejecuta al verificar el email de una cuenta nueva.
pub struct AfterUserStatusChanged {
    f: FnAfterUserStatusChanged,
    weight: Weight,
}

impl ActionDispatcher for AfterUserStatusChanged {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl AfterUserStatusChanged {
    /// Registra una nueva acción.
    pub fn new(f: FnAfterUserStatusChanged) -> Self {
        AfterUserStatusChanged { f, weight: 0 }
    }

    /// Opcional. Las acciones de menor peso se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    pub(crate) fn dispatch(user_id: i32, before: UserStatus, after: UserStatus) {
        dispatch_actions(|action: &Self| (action.f)(user_id, before, after));
    }
}

// **< AdminUserFormExtras >************************************************************************

/// Tipo de función de la acción [`AdminUserFormExtras`].
pub type FnAdminUserFormExtras = fn(user_id: i32) -> Option<Child>;

/// Acción para añadir contenido a la ficha y a la edición de un usuario en la administración.
///
/// Por ejemplo, los datos que guarda un proveedor de autenticación sobre la cuenta. El componente
/// que devuelva se pinta tras los datos de la cuenta. Como la acción es síncrona, el que necesite
/// consultar la base de datos debe hacerlo en su `prepare()`.
pub struct AdminUserFormExtras {
    f: FnAdminUserFormExtras,
    weight: Weight,
}

impl ActionDispatcher for AdminUserFormExtras {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl AdminUserFormExtras {
    /// Registra una nueva acción.
    pub fn new(f: FnAdminUserFormExtras) -> Self {
        AdminUserFormExtras { f, weight: 0 }
    }

    /// Opcional. Las acciones de menor peso se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    pub(crate) fn collect(user_id: i32) -> Vec<Child> {
        let mut extras = Vec::new();
        dispatch_actions(|action: &Self| extras.extend((action.f)(user_id)));
        extras
    }
}
