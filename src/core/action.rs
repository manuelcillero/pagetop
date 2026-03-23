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

// **< actions! >***********************************************************************************

/// Facilita la implementación del método [`actions()`](crate::core::extension::Extension::actions).
///
/// Evita escribir repetidamente `Box::new(...)` para cada acción de la lista, manteniendo el código
/// más limpio.
///
/// # Ejemplo
///
/// Extensión que ajusta un botón antes de renderizarlo y transforma su HTML final:
///
/// ```rust,ignore
/// impl Extension for MyExtension {
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![
///             action::component::BeforeRender::<Button>::new(before_render_button),
///             action::component::TransformMarkup::<Button>::new(transform_button_markup),
///         ]
///     }
/// }
///
/// fn before_render_button(c: &mut Button, cx: &mut Context) {
///     todo!()
/// }
///
/// fn transform_button_markup(c: &Button, cx: &mut Context, markup: Markup) -> Markup {
///     todo!()
/// }
/// ```
#[macro_export]
macro_rules! actions {
    () => {
        Vec::<ActionBox>::new()
    };
    ( $($action:expr),+ $(,)? ) => {{
        vec![$(Box::new($action),)+]
    }};
}
