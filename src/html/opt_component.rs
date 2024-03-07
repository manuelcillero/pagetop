use crate::core::component::{ComponentTrait, Context, OneComponent};
use crate::fn_builder;
use crate::html::{html, Markup};

pub struct OptionComponent<C: ComponentTrait>(Option<OneComponent<C>>);

impl<C: ComponentTrait> Default for OptionComponent<C> {
    fn default() -> Self {
        OptionComponent(None)
    }
}

impl<C: ComponentTrait> OptionComponent<C> {
    pub fn new(component: C) -> Self {
        OptionComponent::default().with_value(Some(component))
    }

    // OptionComponent BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, component: Option<C>) -> &mut Self {
        if let Some(component) = component {
            self.0 = Some(OneComponent::with(component));
        } else {
            self.0 = None;
        }
        self
    }

    // OptionComponent GETTERS.

    pub fn get(&self) -> Option<OneComponent<C>> {
        if let Some(value) = &self.0 {
            return Some(value.clone());
        }
        None
    }

    pub fn render(&self, cx: &mut Context) -> Markup {
        match &self.0 {
            Some(component) => component.render(cx),
            _ => html! {},
        }
    }
}
