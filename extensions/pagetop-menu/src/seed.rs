//! Sembrado inicial de menús e ítems declarados por extensiones.
//!
//! Se invoca desde `Extension::initialize()` después de aplicar las migraciones.
//! Es idempotente: puede ejecutarse en cada arranque sin duplicar datos.

use crate::action::{DeclareDefaultMenuItems, DeclareDefaultMenus, ItemBag, MenuDefs};
use crate::config::SETTINGS;
use crate::repo;

/// Ejecuta el sembrado completo:
///
/// 1. Recoge los menús declarados por extensiones vía `DeclareDefaultMenus`.
/// 2. Asegura que los menús configurados en `menu.default_menus` también existen.
/// 3. Para cada menú conocido, recoge y aplica los ítems declarados por extensiones.
pub(crate) async fn run() {
    // Paso 1: recoger declaraciones de menú.
    let mut defs = MenuDefs {
        entries: Vec::new(),
    };
    DeclareDefaultMenus::dispatch(&mut defs);

    // Paso 2: añadir los menús por defecto de configuración si no se declararon ya.
    for name in &SETTINGS.default_menus {
        if !defs.entries.iter().any(|(n, _)| n == name) {
            let title = capitalize(name);
            defs.entries.push((name.clone(), title));
        }
    }

    // Paso 3: crear cada menú que no exista y sembrar sus ítems.
    for (machine_name, title) in &defs.entries {
        repo::ensure_menu(machine_name, title).await.ok();

        let Some(menu_id) =
            repo::find_menu_model(&crate::tree::MenuKey::Name(machine_name.to_owned()))
                .await
                .map(|m| m.id)
        else {
            continue;
        };

        let mut bag = ItemBag { items: Vec::new() };
        DeclareDefaultMenuItems::dispatch(machine_name, &mut bag);

        for item in bag.items {
            if let Some(key) = item.external_key.clone() {
                let provider = item.provider.clone();
                repo::upsert_item(menu_id, &provider, &key, item).await.ok();
            }
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}
