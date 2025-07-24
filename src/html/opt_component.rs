use crate::builder_fn;
use crate::core::component::{ComponentTrait, Typed};
use crate::html::{html, Context, Markup};

/// Contenedor de componente para incluir en otros componentes.
///
/// Este tipo encapsula `Option<Typed<C>>` para incluir un componente de manera segura en otros
/// componentes, útil para representar estructuras complejas.
///
/// # Ejemplo
///
/// ```rust,ignore
/// use pagetop::prelude::*;
///
/// let comp = MyComponent::new();
/// let opt = OptionComponent::new(comp);
/// assert!(opt.get().is_some());
/// ```
pub struct OptionComponent<C: ComponentTrait>(Option<Typed<C>>);

impl<C: ComponentTrait> Default for OptionComponent<C> {
    fn default() -> Self {
        OptionComponent(None)
    }
}

impl<C: ComponentTrait> OptionComponent<C> {
    /// Crea un nuevo [`OptionComponent`].
    ///
    /// El componente se envuelve automáticamente en un [`Typed`] y se almacena.
    pub fn new(component: C) -> Self {
        OptionComponent::default().with_value(Some(component))
    }

    // OptionComponent BUILDER *********************************************************************

    /// Establece un componente nuevo, o lo vacía.
    ///
    /// Si se proporciona `Some(component)`, se guarda en [`Typed`]; y si es `None`, se limpia.
    #[builder_fn]
    pub fn with_value(mut self, component: Option<C>) -> Self {
        if let Some(component) = component {
            self.0 = Some(Typed::with(component));
        } else {
            self.0 = None;
        }
        self
    }

    // OptionComponent GETTERS *********************************************************************

    /// Devuelve el componente, si existe.
    pub fn get(&self) -> Option<Typed<C>> {
        if let Some(value) = &self.0 {
            return Some(value.clone());
        }
        None
    }

    /// Renderiza el componente, si existe.
    pub fn render(&self, cx: &mut Context) -> Markup {
        if let Some(component) = &self.0 {
            component.render(cx)
        } else {
            html! {}
        }
    }
}
