use crate::AutoDefault;
use crate::core::AnyCast;
use crate::core::action::{ActionBox, ActionDispatcher};
use crate::trace;

// Lista de acciones, ordenada por peso.
//
// Se construye al registrar las acciones, durante el arranque. A partir de ahí es de sólo lectura y
// recorrerla no aplica ningún bloqueo (ver `ActionEntry`).
#[derive(AutoDefault)]
pub struct ActionsList(Vec<ActionBox>);

impl ActionsList {
    // Añade la acción y mantiene la lista ordenada por peso. La ordenación es estable: a igual peso
    // se conserva el orden de registro.
    pub fn add(&mut self, action: ActionBox) {
        self.0.push(action);
        self.0.sort_by_key(|a| a.weight());
    }

    pub fn for_each<A, F>(&self, mut f: F)
    where
        A: ActionDispatcher,
        F: FnMut(&A),
    {
        for a in self.0.iter() {
            if let Some(action) = (**a).downcast_ref::<A>() {
                f(action);
            } else {
                trace::error!("Failed to downcast action of type {}", (**a).type_name());
            }
        }
    }

    pub fn try_for_each<A, F>(&self, mut f: F)
    where
        A: ActionDispatcher,
        F: FnMut(&A) -> std::ops::ControlFlow<()>,
    {
        for a in self.0.iter() {
            if let Some(action) = (**a).downcast_ref::<A>() {
                if f(action).is_break() {
                    break;
                }
            } else {
                trace::error!("Failed to downcast action of type {}", (**a).type_name());
            }
        }
    }
}
