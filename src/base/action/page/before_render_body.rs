use crate::prelude::*;

use crate::base::action::page::FnActionWithPage;

/// Ejecuta [`FnActionWithPage`](crate::base::action::page::FnActionWithPage) antes de renderizar
/// el cuerpo de la página.
///
/// Este tipo de acción se despacha antes de renderizar el contenido principal de la página
/// (`<body>`), permitiendo ajustes sobre la instancia [`Page`].
///
/// Las acciones se ejecutan en orden según el [`Weight`] asignado.
pub struct BeforeRenderBody {
    f: FnActionWithPage,
    weight: Weight,
}

impl ActionDispatcher for BeforeRenderBody {
    /// Devuelve el peso para definir el orden de ejecución.
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl BeforeRenderBody {
    /// Permite [registrar](Extension::actions) una nueva acción
    /// [`FnActionWithPage`](crate::base::action::page::FnActionWithPage).
    pub fn new(f: FnActionWithPage) -> Self {
        BeforeRenderBody { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones.
    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, None, None),
            |action: &Self| (action.f)(page),
        );
    }
}
