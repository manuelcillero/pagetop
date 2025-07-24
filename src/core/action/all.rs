use crate::core::action::{ActionBox, ActionDispatcher, ActionKey, ActionsList};

use parking_lot::RwLock;

use std::collections::HashMap;
use std::sync::LazyLock;

// ACCIONES ****************************************************************************************

static ACTIONS: LazyLock<RwLock<HashMap<ActionKey, ActionsList>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// AÑADIR ACCIONES *********************************************************************************

// Registra una nueva acción en el sistema.
//
// Si ya existen acciones con la misma `ActionKey`, la acción se añade a la misma lista. Si no, se
// crea una nueva lista.
//
// Las extensiones llamarán a esta función durante su inicialización para instalar acciones
// personalizadas que modifiquen el comportamiento del *core* o de otros componentes.
pub fn add_action(action: ActionBox) {
    let key = ActionKey::new(
        action.type_id(),
        action.theme_type_id(),
        action.referer_type_id(),
        action.referer_id(),
    );
    let mut actions = ACTIONS.write();
    if let Some(list) = actions.get_mut(&key) {
        list.add(action);
    } else {
        let mut list = ActionsList::new();
        list.add(action);
        actions.insert(key, list);
    }
}

// DESPLEGAR ACCIONES ******************************************************************************

/// Despacha y ejecuta las funciones asociadas a una [`ActionKey`].
///
/// Permite recorrer de forma segura y ordenada (por peso) la lista de funciones asociadas a una
/// acción específica.
///
/// # Parámetros genéricos
/// - `A`: Tipo de acción que esperamos procesar. Debe implementar [`ActionDispatcher`].
/// - `F`: Función asociada a cada acción, devuelve un valor de tipo `B`.
///
/// # Ejemplo de uso
/// ```rust,ignore
/// pub(crate) fn dispatch(component: &mut C, cx: &mut Context) {
///     dispatch_actions(
///         &ActionKey::new(
///             UniqueId::of::<Self>(),
///             Some(cx.theme().type_id()),
///             Some(UniqueId::of::<C>()),
///             None,
///         ),
///         |action: &Self| (action.f)(component, cx),
///     );
/// }
/// ```
pub fn dispatch_actions<A, B, F>(key: &ActionKey, f: F)
where
    A: ActionDispatcher,
    F: FnMut(&A) -> B,
{
    if let Some(list) = ACTIONS.read().get(key) {
        list.iter_map(f);
    }
}
