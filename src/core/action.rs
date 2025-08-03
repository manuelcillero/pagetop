//! API para definir acciones que inyectan código en el flujo de la aplicación.
//!
//! Permite crear acciones para que otros *crates* puedan inyectar código usando funciones *ad hoc*
//! que modifican el comportamiento predefinido en puntos concretos del flujo de ejecución de la
//! aplicación.

mod definition;
pub use definition::{ActionBox, ActionDispatcher, ActionKey};

mod list;
use list::ActionsList;

mod all;
pub(crate) use all::add_action;
pub use all::dispatch_actions;

/// Facilita la implementación del método
/// [`actions()`](crate::core::extension::ExtensionTrait::actions).
///
/// Evita escribir repetidamente `Box::new(...)` para cada acción de la lista, manteniendo el código
/// más limpio.
///
/// # Ejemplo
///
/// ```rust,ignore
/// use pagetop::prelude::*;
///
/// impl Extension for MyTheme {
///     fn actions(&self) -> Vec<ActionBox> {
///         actions_boxed![
///             action::theme::BeforeRender::<Button>::new(&Self, before_render_button),
///             action::theme::PrepareRender::<Error404>::new(&Self, render_error404),
///         ]
///     }
/// }
///
/// impl Theme for MyTheme {}
///
/// fn before_render_button(c: &mut Button, cx: &mut Context) { todo!() }
/// fn render_error404(c: &Error404, cx: &mut Context) -> PrepareMarkup { todo!() }
/// ```
#[macro_export]
macro_rules! actions_boxed {
    () => {
        Vec::<ActionBox>::new()
    };
    ( $($action:expr),+ $(,)? ) => {{
        vec![$(Box::new($action),)+]
    }};
}
