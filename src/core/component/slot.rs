use crate::builder_fn;
use crate::core::component::{Component, Typed};
use crate::html::{html, Context, Markup};

/// Contenedor para un componente [`Typed`] opcional.
///
/// Un `TypedSlot` actúa como un contenedor dentro de otro componente para incluir o no un
/// subcomponente. Internamente encapsula `Option<Typed<C>>`, pero proporciona una API más sencilla
/// para construir estructuras jerárquicas.
///
/// # Ejemplo
///
/// ```rust,ignore
/// use pagetop::prelude::*;
///
/// let comp = MyComponent::new();
/// let opt = TypedSlot::new(comp);
/// assert!(opt.get().is_some());
/// ```
pub struct TypedSlot<C: Component>(Option<Typed<C>>);

impl<C: Component> Default for TypedSlot<C> {
    fn default() -> Self {
        TypedSlot(None)
    }
}

impl<C: Component> TypedSlot<C> {
    /// Crea un nuevo [`TypedSlot`].
    ///
    /// El componente se envuelve automáticamente en un [`Typed`] y se almacena.
    pub fn new(component: C) -> Self {
        TypedSlot(Some(Typed::with(component)))
    }

    // TypedSlot BUILDER *********************************************************************

    /// Establece un componente nuevo, o lo vacía.
    ///
    /// Si se proporciona `Some(component)`, se guarda en [`Typed`]; y si es `None`, se limpia.
    #[builder_fn]
    pub fn with_value(mut self, component: Option<C>) -> Self {
        self.0 = component.map(Typed::with);
        self
    }

    // TypedSlot GETTERS *********************************************************************

    /// Devuelve un clon (incrementa el contador `Arc`) de [`Typed<C>`], si existe.
    pub fn get(&self) -> Option<Typed<C>> {
        self.0.clone()
    }

    // TypedSlot RENDER ************************************************************************

    /// Renderiza el componente, si existe.
    pub fn render(&self, cx: &mut Context) -> Markup {
        if let Some(component) = &self.0 {
            component.render(cx)
        } else {
            html! {}
        }
    }
}
