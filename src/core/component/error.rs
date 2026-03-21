use crate::html::{html, Markup};
use crate::{AutoDefault, Getters};

/// Error producido durante el renderizado de un componente.
///
/// Se usa en [`Component::prepare_component()`](super::Component::prepare_component) para devolver
/// un [`Err`]. Puede incluir un marcado HTML alternativo para renderizar el componente de manera
/// diferente en caso de error.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # struct MyComponent;
/// # impl Component for MyComponent {
/// #     fn new() -> Self { MyComponent }
/// fn prepare_component(&self, _cx: &mut Context) -> Result<Markup, ComponentError> {
///     Err(ComponentError::new("Database connection failed")
///         .with_fallback(html! { p class="error" { "Content temporarily unavailable." } }))
/// }
/// # }
/// ```
#[derive(AutoDefault, Debug, Getters)]
pub struct ComponentError {
    /// Mensaje descriptivo del error.
    message: String,
    /// Marcado HTML alternativo para mostrar si el componente falla.
    fallback: Markup,
}

impl ComponentError {
    /// Crea un nuevo error para un componente con un marcado alternativo vacío.
    pub fn new(message: impl Into<String>) -> Self {
        ComponentError {
            message: message.into(),
            fallback: html! {},
        }
    }

    // **< ComponentError BUILDER >*****************************************************************

    /// Asigna el marcado HTML alternativo (*fallback*) que se mostrará si el componente falla.
    ///
    /// Si no se proporciona, no se renderizará nada del componente.
    pub fn with_fallback(mut self, fallback: Markup) -> Self {
        self.fallback = fallback;
        self
    }

    // **< ComponentError GETTERS >*****************************************************************

    /// Consume el error y devuelve su marcado alternativo.
    ///
    /// Se invoca internamente en [`ComponentRender`](crate::core::component::ComponentRender).
    pub(crate) fn into_fallback(self) -> Markup {
        self.fallback
    }
}

impl std::fmt::Display for ComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ComponentError {}
