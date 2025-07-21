use crate::core::action::{ActionBox, ActionKey, ActionTrait, ActionsList};

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

// ACCIONES ****************************************************************************************

static ACTIONS: LazyLock<RwLock<HashMap<ActionKey, ActionsList>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// AÑADIR ACCIONES *********************************************************************************

// Registra una nueva acción en el sistema.
//
// Si ya existen acciones con la misma `ActionKey`, la acción se añade a la lista existente. Si no,
// se crea una nueva lista.
//
// # Uso típico
//
// Las extensiones llamarán a esta función durante su inicialización para instalar acciones
// personalizadas que modifiquen el comportamiento del núcleo o de otros componentes.
//
// ```rust,ignore
// add_action(Box::new(MyCustomAction::new()));
// ```
pub fn add_action(action: ActionBox) {
    let key = action.key();
    let mut actions = ACTIONS.write().unwrap();
    if let Some(list) = actions.get_mut(&key) {
        list.add(action);
    } else {
        let mut list = ActionsList::new();
        list.add(action);
        actions.insert(key, list);
    }
}

// DESPLEGAR ACCIONES ******************************************************************************

/// Despacha las funciones asociadas a un [`ActionKey`] y las ejecuta.
///
/// Permite recorrer de forma segura y ordenada (por peso) la lista de funciones asociadas a una
/// acción específica.
///
/// # Parámetros genéricos
/// - `A`: Tipo de acción que esperamos procesar. Debe implementar [`ActionTrait`].
/// - `F`: Función o cierre que recibe cada acción y devuelve un valor de tipo `B`.
///
/// # Ejemplo de uso
/// ```rust,ignore
/// dispatch_actions::<MyCustomAction, _>(&some_key, |action| {
///     action.do_something();
/// });
/// ```
///
/// Esto permite a PageTop o a otros módulos aplicar lógica específica a las acciones de un contexto
/// determinado, manteniendo la flexibilidad del sistema.
pub fn dispatch_actions<A, B, F>(key: &ActionKey, f: F)
where
    A: ActionTrait,
    F: FnMut(&A) -> B,
{
    if let Some(list) = ACTIONS.read().unwrap().get(key) {
        list.iter_map(f);
    }
}
