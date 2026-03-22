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

// **< actions_boxed! >*****************************************************************************

/// Facilita la implementación del método [`actions()`](crate::core::extension::Extension::actions).
///
/// Evita escribir repetidamente `Box::new(...)` para cada acción de la lista, manteniendo el código
/// más limpio.
///
/// # Ejemplos
///
/// Acciones de tema que ajustan un componente antes y después de renderizarlo:
///
/// ```rust,ignore
/// impl Extension for MyTheme {
///     fn actions(&self) -> Vec<ActionBox> {
///         actions_boxed![
///             action::theme::BeforeRender::<Button>::new(&Self, before_render_button),
///             action::theme::AfterRender::<Button>::new(&Self, after_render_button),
///         ]
///     }
/// }
///
/// impl Theme for MyTheme {}
///
/// fn before_render_button(c: &mut Button, cx: &mut Context) { todo!() }
/// fn after_render_button(c: &mut Button, cx: &mut Context) { todo!() }
/// ```
///
/// Acción de extensión que transforma el HTML final de un componente mediante edición de texto:
///
/// ```rust,ignore
/// impl Extension for MyExtension {
///     fn actions(&self) -> Vec<ActionBox> {
///         actions_boxed![
///             action::component::AlterMarkup::<Button>::new(alter_button_markup),
///         ]
///     }
/// }
///
/// fn alter_button_markup(c: &mut Button, cx: &mut Context, markup: Markup) -> Markup {
///     todo!()
/// }
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
