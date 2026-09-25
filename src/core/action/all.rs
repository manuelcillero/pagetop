use crate::UniqueId;
use crate::core::action::{ActionBox, ActionDispatcher, ActionReferer, ActionsList};

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::OnceLock;

// **< ACCIONES >***********************************************************************************

// Registro de acciones. Se construye una sola vez durante el arranque, al registrar las extensiones
// y antes de inicializarlas (`publish_actions()`). A partir de ahí es inmutable, por lo que
// despachar una acción no toma ningún bloqueo ni escribe en memoria compartida, evitando cualquier
// contención por más hilos que atiendan peticiones.
static ACTIONS: OnceLock<Registry> = OnceLock::new();

// Una clave del registro usa `TypeId`, que ya es un *hash* bien distribuido. No requiere SipHash.
type FastMap<K, V> = HashMap<K, V, BuildHasherDefault<FastHasher>>;

struct Registry(FastMap<Slot, ActionEntry>);

// Tipo de acción y, si lo hay, tipo del referente (p. ej. el componente). Es `Copy`, así que
// buscarlo no reserva memoria; el identificador del referente se busca aparte, en `ActionEntry`.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Slot {
    action_type_id: UniqueId,
    referer_type_id: Option<UniqueId>,
}

// Acciones de un tipo para un tipo de referente: las generales y, aparte, las filtradas por
// identificador. Es de sólo lectura y se obtiene ya construida del registro, sin bloqueos.
#[derive(Default)]
pub(crate) struct ActionEntry {
    general: ActionsList,
    by_id: HashMap<String, ActionsList>,
}

impl ActionEntry {
    // Acciones que se aplican a cualquier referente del tipo.
    pub(crate) fn general(&self) -> &ActionsList {
        &self.general
    }

    // Indica si hay alguna acción filtrada por identificador. Permite no calcular el identificador
    // del referente (que puede reservar memoria) cuando no hay ninguna.
    pub(crate) fn has_ids(&self) -> bool {
        !self.by_id.is_empty()
    }

    // Acciones que sólo se aplican al referente con ese identificador.
    pub(crate) fn with_id(&self, id: &str) -> Option<&ActionsList> {
        self.by_id.get(id)
    }
}

// Busca la entrada de un tipo de acción y de referente.
fn action_entry(
    action_type_id: UniqueId,
    referer_type_id: Option<UniqueId>,
) -> Option<&'static ActionEntry> {
    ACTIONS.get()?.0.get(&Slot {
        action_type_id,
        referer_type_id,
    })
}

// **< REGISTRAR ACCIONES >*************************************************************************

// Registra todas las acciones de las extensiones y congela el registro.
//
// Las extensiones instalan sus acciones durante el arranque, antes de atender ninguna petición, y
// sólo aquí. `collect` se ejecuta una única vez: si el registro ya está construido (p. ej. por otra
// llamada a `Application::prepare()` en el mismo proceso, como hacen las pruebas) no se vuelve a
// construir ni se duplican las acciones.
pub(crate) fn publish_actions(collect: impl FnOnce() -> Vec<ActionBox>) {
    ACTIONS.get_or_init(|| {
        let mut slots: FastMap<Slot, ActionEntry> = FastMap::default();
        for action in collect() {
            let referer = action.referer();
            let slot = Slot {
                action_type_id: action.type_id(),
                referer_type_id: referer.map(ActionReferer::referer_type_id),
            };
            let entry = slots.entry(slot).or_default();
            match referer.and_then(|r| r.id().map(str::to_owned)) {
                None => entry.general.add(action),
                Some(id) => entry.by_id.entry(id).or_default().add(action),
            }
        }
        Registry(slots)
    });
}

// Función de *hash* rápida para claves formadas por `TypeId` (multiplicación y rotación, como
// FxHash). No es apta para claves controladas por un atacante; aquí las claves son tipos.
#[derive(Default)]
struct FastHasher(u64);

impl FastHasher {
    const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

    #[inline]
    fn add(&mut self, word: u64) {
        self.0 = (self.0.rotate_left(5) ^ word).wrapping_mul(Self::SEED);
    }
}

impl Hasher for FastHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut chunks = bytes.chunks_exact(8);
        for chunk in &mut chunks {
            self.add(u64::from_le_bytes(chunk.try_into().unwrap()));
        }
        let rest = chunks.remainder();
        if !rest.is_empty() {
            let mut last = [0u8; 8];
            last[..rest.len()].copy_from_slice(rest);
            self.add(u64::from_le_bytes(last));
        }
    }

    #[inline]
    fn write_u8(&mut self, n: u8) {
        self.add(n as u64);
    }

    #[inline]
    fn write_u64(&mut self, n: u64) {
        self.add(n);
    }

    #[inline]
    fn write_u128(&mut self, n: u128) {
        self.add(n as u64);
        self.add((n >> 64) as u64);
    }

    #[inline]
    fn write_isize(&mut self, n: isize) {
        self.add(n as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.rotate_left(26)
    }
}

// **< DESPACHAR ACCIONES >*************************************************************************

/// Despacha y ejecuta las funciones de las acciones de tipo `A` que no tienen referente.
///
/// Recorre de forma segura y ordenada (por peso) la lista de funciones registradas para esa acción.
///
/// Sólo alcanza las acciones que no declaran un [`ActionReferer`](super::ActionReferer) en
/// [`ActionDispatcher::referer()`]. Las que actúan sobre un tipo de objeto (y, opcionalmente, una
/// instancia concreta) se despachan con [`dispatch_referer()`].
///
/// # Parámetros genéricos
///
/// - `A`: Tipo de acción que esperamos procesar. Debe implementar [`ActionDispatcher`].
/// - `F`: Función que se aplica para una acción dada.
///
/// # Ejemplo
///
/// ```rust,ignore
/// pub(crate) fn dispatch(page: &mut Page) {
///     dispatch_actions(|action: &Self| (action.f)(page));
/// }
/// ```
pub fn dispatch_actions<A, F>(f: F)
where
    A: ActionDispatcher,
    F: FnMut(&A),
{
    if let Some(entry) = action_entry(UniqueId::of::<A>(), None) {
        entry.general().for_each(f);
    }
}

/// Despacha las funciones de las acciones de tipo `A` que no tienen referente, con posible salida
/// anticipada.
///
/// Funciona igual que [`dispatch_actions`], pero el closure puede devolver
/// [`std::ops::ControlFlow::Continue`] para continuar ejecutando la siguiente acción; o
/// [`std::ops::ControlFlow::Break`] para detener la iteración inmediatamente.
///
/// # Ejemplo
///
/// ```rust,ignore
/// pub(crate) fn check(cx: &Context, key: &str) -> bool {
///     let mut granted = false;
///     try_dispatch_actions(|action: &Self| {
///         (action.f)(cx, key, &mut granted);
///         if granted {
///             std::ops::ControlFlow::Break(())
///         } else {
///             std::ops::ControlFlow::Continue(())
///         }
///     });
///     granted
/// }
/// ```
pub fn try_dispatch_actions<A, F>(f: F)
where
    A: ActionDispatcher,
    F: FnMut(&A) -> std::ops::ControlFlow<()>,
{
    if let Some(entry) = action_entry(UniqueId::of::<A>(), None) {
        entry.general().try_for_each(f);
    }
}

/// Despacha las funciones de las acciones de tipo `A` asociadas a un referente de tipo `R` (p. ej.
/// un componente).
///
/// Se aplican primero las acciones del tipo `R` y después las que sólo afectan al referente con el
/// identificador que devuelva `id`, cada lista ordenada por peso. Sólo se llama a `id` (que suele
/// reservar memoria) si hay alguna acción filtrada por identificador para `R`.
///
/// El referente se presta a `id` y a `f` por turnos, así que `f` puede modificarlo.
///
/// # Parámetros genéricos
///
/// - `A`: Tipo de acción que esperamos procesar. Debe implementar [`ActionDispatcher`].
/// - `R`: Tipo del referente, el mismo que declara [`ActionDispatcher::referer()`] en su
///   [`ActionReferer`].
///
/// # Ejemplo
///
/// ```rust,ignore
/// pub(crate) fn dispatch(component: &mut C, cx: &mut Context) {
///     dispatch_referer(
///         component,
///         |c| c.id(),
///         |action: &Self, c| (action.f)(c, cx),
///     );
/// }
/// ```
pub fn dispatch_referer<A, R>(
    referer: &mut R,
    id: impl FnOnce(&R) -> Option<String>,
    mut f: impl FnMut(&A, &mut R),
) where
    A: ActionDispatcher,
    R: 'static,
{
    // Sin ninguna acción registrada para este tipo de referente no hay nada que hacer.
    let Some(entry) = action_entry(UniqueId::of::<A>(), Some(UniqueId::of::<R>())) else {
        return;
    };
    entry.general().for_each(|action: &A| f(action, referer));
    if entry.has_ids()
        && let Some(id) = id(referer)
        && let Some(list) = entry.with_id(&id)
    {
        list.for_each(|action: &A| f(action, referer));
    }
}
