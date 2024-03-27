use crate::prelude::*;

pub type FnRenderComponent<C> = fn(component: &C, cx: &mut Context) -> Option<Markup>;

pub struct RenderComponent<C: ComponentTrait> {
    f: FnRenderComponent<C>,
    theme_type_id: Option<TypeId>,
    referer_type_id: Option<TypeId>,
}

impl<C: ComponentTrait> ActionTrait for RenderComponent<C> {
    fn theme_type_id(&self) -> Option<TypeId> {
        self.theme_type_id
    }

    fn referer_type_id(&self) -> Option<TypeId> {
        self.referer_type_id
    }
}

impl<C: ComponentTrait> RenderComponent<C> {
    pub fn new(theme: ThemeRef, f: FnRenderComponent<C>) -> Self {
        RenderComponent {
            f,
            theme_type_id: Some(theme.type_id()),
            referer_type_id: Some(TypeId::of::<C>()),
        }
    }

    #[inline(always)]
    pub(crate) fn dispatch(component: &C, cx: &mut Context) -> Option<Markup> {
        let mut render_component: Option<Markup> = None;
        dispatch_actions(
            ActionKey::new(
                TypeId::of::<Self>(),
                Some(cx.theme().type_id()),
                Some(TypeId::of::<C>()),
                None,
            ),
            |action: &Self| {
                if render_component.is_none() {
                    render_component = (action.f)(component, cx);
                }
            },
        );
        render_component
    }
}
