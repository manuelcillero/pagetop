use crate::prelude::*;

pub type FnRenderComponent<C> = fn(component: &C, cx: &mut Context) -> Option<Markup>;

pub struct RenderComponent<C: ComponentTrait> {
    f: FnRenderComponent<C>,
    layout_type_id: Option<UniqueId>,
    referer_type_id: Option<UniqueId>,
}

impl<C: ComponentTrait> ActionTrait for RenderComponent<C> {
    fn layout_type_id(&self) -> Option<UniqueId> {
        self.layout_type_id
    }

    fn referer_type_id(&self) -> Option<UniqueId> {
        self.referer_type_id
    }
}

impl<C: ComponentTrait> RenderComponent<C> {
    pub fn new(layout: LayoutRef, f: FnRenderComponent<C>) -> Self {
        RenderComponent {
            f,
            layout_type_id: Some(layout.type_id()),
            referer_type_id: Some(UniqueId::of::<C>()),
        }
    }

    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(component: &C, cx: &mut Context) -> Option<Markup> {
        let mut render_component: Option<Markup> = None;
        dispatch_actions(
            &ActionKey::new(
                UniqueId::of::<Self>(),
                Some(cx.layout().type_id()),
                Some(UniqueId::of::<C>()),
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
