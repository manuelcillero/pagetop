use crate::prelude::*;

/// Tipo de función para alterar el renderizado de un componente.
///
/// Permite a un [tema](crate::base::action::theme) sobreescribir el renderizado predeterminado de
/// los componentes.
///
/// Recibe una referencia al componente `component` y una referencia mutable al contexto `cx`.
pub type FnPrepareRender<C> = fn(component: &C, cx: &mut Context) -> PrepareMarkup;

/// Ejecuta [`FnPrepareRender`] para preparar el renderizado de un componente.
///
/// Permite a un tema hacer una implementación nueva del renderizado de un componente.
pub struct PrepareRender<C: Component> {
    f: FnPrepareRender<C>,
    theme_type_id: Option<UniqueId>,
    referer_type_id: Option<UniqueId>,
}

/// Filtro para despachar [`FnPrepareRender`] que modifica el renderizado de un componente `C`.
impl<C: Component> ActionDispatcher for PrepareRender<C> {
    /// Devuelve el identificador de tipo ([`UniqueId`]) del tema.
    fn theme_type_id(&self) -> Option<UniqueId> {
        self.theme_type_id
    }

    /// Devuelve el identificador de tipo ([`UniqueId`]) del componente `C`.
    fn referer_type_id(&self) -> Option<UniqueId> {
        self.referer_type_id
    }
}

impl<C: Component> PrepareRender<C> {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnPrepareRender`] para un tema
    /// dado.
    pub fn new(theme: ThemeRef, f: FnPrepareRender<C>) -> Self {
        PrepareRender {
            f,
            theme_type_id: Some(theme.type_id()),
            referer_type_id: Some(UniqueId::of::<C>()),
        }
    }

    // Despacha las acciones. Se detiene en cuanto una renderiza.
    #[inline]
    pub(crate) fn dispatch(component: &C, cx: &mut Context) -> PrepareMarkup {
        let mut render_component = PrepareMarkup::None;
        dispatch_actions(
            &ActionKey::new(
                UniqueId::of::<Self>(),
                Some(cx.theme().type_id()),
                Some(UniqueId::of::<C>()),
                None,
            ),
            |action: &Self| {
                if render_component.is_empty() {
                    render_component = (action.f)(component, cx);
                }
            },
        );
        render_component
    }
}
