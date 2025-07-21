//! API para definir acciones que inyectan código en el flujo de la aplicación.
//!
//! Permite crear acciones en las librerías para que otros *crates* puedan inyectar código usando
//! funciones *ad hoc* que modifican el comportamiento predefinido en puntos concretos del flujo de
//! ejecución de la aplicación.

mod definition;
pub use definition::{ActionBase, ActionBox, ActionKey, ActionTrait};

mod list;
use list::ActionsList;

mod all;
pub(crate) use all::add_action;
pub use all::dispatch_actions;

/// Crea una lista de acciones para facilitar la implementación del método
/// [`actions`](crate::core::extension::ExtensionTrait#method.actions).
///
/// Esta macro crea vectores de [`ActionBox`], el tipo dinámico que encapsula cualquier acción que
/// implemente [`ActionTrait`]. Evita escribir repetidamente `Box::new(...)` para cada acción
/// inyectada, manteniendo el código más limpio.
///
/// # Ejemplos
///
/// Puede llamarse sin argumentos para crear un vector vacío:
///
/// ```rust,ignore
/// let my_actions = inject_actions![];
/// ```
///
/// O con una lista de acciones concretas:
///
/// ```rust,ignore
/// let my_actions = inject_actions![
///     MyFirstAction::new(),
///     MySecondAction::new().with_weight(10),
/// ];
/// ```
///
/// Internamente, expande a un `vec![Box::new(...), ...]`.
///
/// # Ejemplo típico en una extensión
///
/// ```rust,ignore
/// impl ExtensionTrait for MyExtension {
///     fn actions(&self) -> Vec<ActionBox> {
///         inject_actions![
///             CustomizeLoginAction::new(),
///             ModifyHeaderAction::new().with_weight(-5),
///         ]
///     }
/// }
/// ```
///
/// Así, `PageTop` podrá registrar todas estas acciones durante la inicialización de la extensión y
/// posteriormente despacharlas según corresponda.
#[macro_export]
macro_rules! inject_actions {
    () => {
        Vec::<ActionBox>::new()
    };
    ( $($action:expr),+ $(,)? ) => {{
        vec![$(Box::new($action),)+]
    }};
}
