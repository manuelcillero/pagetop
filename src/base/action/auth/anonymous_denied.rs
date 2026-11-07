use crate::prelude::*;

use super::FnActionAnonymousDenied;

use std::ops::ControlFlow;

/// Acción para responder a un usuario anónimo sin acceso a una ruta.
///
/// La registra una extensión de autenticación para ofrecer el inicio de sesión en lugar de la
/// página de acceso denegado, y volver después a la ruta pedida. Responde la primera acción, por
/// orden de peso, que devuelva una respuesta. Ver [`deny_access()`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// fn offer_login(request: &HttpRequest) -> Option<Response> {
///     // Vuelve a la ruta pedida tras iniciar sesión.
///     let waypoint = Waypoint::new(request.uri().to_owned());
///     Some(Redirect::see_other(waypoint.append_to("/login")))
/// }
///
/// pub struct MyAuth;
///
/// #[async_trait]
/// impl Extension for MyAuth {
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![action::auth::AnonymousDenied::new(offer_login)]
///     }
/// }
/// ```
pub struct AnonymousDenied {
    f: FnActionAnonymousDenied,
    weight: Weight,
}

impl ActionDispatcher for AnonymousDenied {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl AnonymousDenied {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionAnonymousDenied`].
    pub fn new(f: FnActionAnonymousDenied) -> Self {
        AnonymousDenied { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones hasta que una responde, y devuelve su respuesta.
    #[inline]
    pub(crate) fn dispatch(request: &HttpRequest) -> Option<Response> {
        let mut response = None;
        try_dispatch_actions(|action: &Self| {
            response = (action.f)(request);
            if response.is_some() {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        response
    }
}
