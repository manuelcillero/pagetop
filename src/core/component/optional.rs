use crate::core::component::{Component, Typed};
use crate::html::{html, Context, Markup};
use crate::{builder_fn, AutoDefault};

/// Contenedor **opcional** para un componente [`Typed`].
///
/// Un `TypedOpt` actúa como un contenedor para incluir o no un subcomponente tipado. Internamente
/// encapsula `Option<Typed<C>>`, pero ofrece una API más sencilla para construir estructuras
/// jerárquicas o contenidas de componentes.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// let icon = Icon::default();
/// let icon = TypedOpt::new(icon);
/// assert!(icon.get().is_some());
/// ```
#[derive(AutoDefault)]
pub struct TypedOpt<C: Component>(Option<Typed<C>>);

impl<C: Component> TypedOpt<C> {
    /// Crea un nuevo [`TypedOpt`].
    ///
    /// El componente se envuelve automáticamente en un [`Typed`] y se almacena.
    pub fn new(component: C) -> Self {
        TypedOpt(Some(Typed::with(component)))
    }

    // TypedOpt BUILDER ****************************************************************************

    /// Establece un componente nuevo, o lo vacía.
    ///
    /// Si se proporciona `Some(component)`, se guarda como [`Typed`]; y si es `None`, se limpia.
    #[builder_fn]
    pub fn with_component(mut self, component: Option<C>) -> Self {
        self.0 = component.map(Typed::with);
        self
    }

    // TypedOpt GETTERS ****************************************************************************

    /// Devuelve un clon (incrementa el contador `Arc`) de [`Typed<C>`], si existe.
    pub fn get(&self) -> Option<Typed<C>> {
        self.0.clone()
    }

    // TypedOpt RENDER *****************************************************************************

    /// Renderiza el componente, si existe.
    pub fn render(&self, cx: &mut Context) -> Markup {
        if let Some(component) = &self.0 {
            component.render(cx)
        } else {
            html! {}
        }
    }
}
