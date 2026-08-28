//! Caché in-process de los ítems de menú por `machine_name`.
//!
//! Almacena la lista plana de modelos y sus traducciones para cada menú. El árbol se
//! reconstruye en memoria en cada petición a partir de esta estructura. Hoy no existe ningún
//! camino que escriba ítems tras el sembrado inicial (`seed::run()`, antes de la primera
//! petición), así que no hace falta invalidación: cuando exista una vía de escritura en caliente
//! (p. ej. una UI de administración), deberá invalidar la entrada correspondiente aquí.

use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::{Arc, RwLock};

use crate::entity::{menu_item, menu_item_translation};

/// Contenido plano de un menú listo para construir el árbol.
pub struct FlatMenu {
    pub items: Vec<menu_item::Model>,
    /// Traducciones indexadas por `item_id`.
    pub translations: HashMap<i32, Vec<menu_item_translation::Model>>,
}

static CACHE: LazyLock<RwLock<HashMap<String, Arc<FlatMenu>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Devuelve el contenido cacheado del menú dado, o lo carga desde BD si no está.
pub async fn get_or_load(menu_id: i32, machine_name: &str) -> Arc<FlatMenu> {
    {
        let guard = CACHE.read().expect("cache read lock poisoned");
        if let Some(flat) = guard.get(machine_name) {
            return Arc::clone(flat);
        }
    }

    let flat = load_from_db(menu_id).await;
    let arc = Arc::new(flat);
    CACHE
        .write()
        .expect("cache write lock poisoned")
        .insert(machine_name.to_owned(), Arc::clone(&arc));
    arc
}

async fn load_from_db(menu_id: i32) -> FlatMenu {
    use pagetop_seaorm::db::{ColumnTrait, EntityTrait, QueryFilter, dbconn};

    let items = menu_item::Entity::find()
        .filter(menu_item::Column::MenuId.eq(menu_id))
        .all(dbconn())
        .await
        .unwrap_or_default();

    let item_ids: Vec<i32> = items.iter().map(|i| i.id).collect();

    let all_translations = if item_ids.is_empty() {
        vec![]
    } else {
        menu_item_translation::Entity::find()
            .filter(menu_item_translation::Column::ItemId.is_in(item_ids))
            .all(dbconn())
            .await
            .unwrap_or_default()
    };

    let mut translations: HashMap<i32, Vec<menu_item_translation::Model>> = HashMap::new();
    for t in all_translations {
        translations.entry(t.item_id).or_default().push(t);
    }

    FlatMenu {
        items,
        translations,
    }
}
