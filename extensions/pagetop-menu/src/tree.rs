//! Tipos en memoria del árbol de menús y función de construcción.

use std::collections::HashMap;

use pagetop::locale::{Locale, RequestLocale};
use pagetop::prelude::*;

use crate::cache::FlatMenu;
use crate::entity::menu_item_translation;
use crate::{action, cache, repo};

// **< MenuKey >************************************************************************************

/// Selector para localizar un menú por `id` o por `machine_name`.
pub enum MenuKey {
    Id(i32),
    Name(String),
}

impl From<i32> for MenuKey {
    fn from(id: i32) -> Self {
        MenuKey::Id(id)
    }
}

impl From<&str> for MenuKey {
    fn from(name: &str) -> Self {
        MenuKey::Name(name.to_owned())
    }
}

impl From<String> for MenuKey {
    fn from(name: String) -> Self {
        MenuKey::Name(name)
    }
}

// **< URL de un ítem de menú >*********************************************************************

/// Construye la URL de un ítem de menú a partir del texto crudo guardado en BD.
///
/// Devuelve `None` para `<nolink>` o una cadena vacía (título de sección, sin enlace). El
/// `RoutePath` devuelto está sin resolver todavía: [`try_resolve_menu_url()`] es quien decide, en
/// el momento del renderizado, si debe pasar por [`Context::route()`] o dejarse tal cual.
pub fn menu_item_url(url: &str) -> Option<RoutePath> {
    (!url.is_empty() && url != "<nolink>").then(|| RoutePath::new(url.to_owned()))
}

/// Resuelve la URL de un ítem de menú para renderizado.
///
/// Las internas pasan por [`Context::route()`] para preservar `lang` cuando corresponda; las
/// externas ([`RoutePath::is_external()`]) se devuelven tal cual, sin tocar el idioma, porque no
/// pertenecen al espacio de rutas de la aplicación. `None` (sin enlace) se propaga tal cual.
pub fn try_resolve_menu_url(url: Option<&RoutePath>, cx: &Context) -> Option<RoutePath> {
    url.map(|path| {
        if path.is_external() {
            path.clone()
        } else {
            cx.route(path.path().to_owned())
        }
    })
}

// **< TreeOptions >********************************************************************************

/// Opciones de construcción del árbol de menú.
#[derive(Clone, Debug, Default)]
pub struct TreeOptions {
    /// Profundidad máxima de nodos a incluir (`None` = sin límite).
    pub max_depth: Option<u8>,
    /// Si `true`, incluye también los ítems con `enabled = false`.
    pub include_disabled: bool,
}

// **< MenuNode >***********************************************************************************

/// Nodo del árbol de menú. Sus campos son `pub` para que las acciones puedan modificarlos.
#[derive(Clone, Debug)]
pub struct MenuNode {
    pub item_id: i32,
    pub title: String,
    /// `None` si el ítem no tiene enlace (`<nolink>` o URL vacía): título de sección.
    pub url: Option<RoutePath>,
    pub weight: i32,
    pub depth: u8,
    pub enabled: bool,
    pub expanded: bool,
    pub provider: String,
    pub external_key: Option<String>,
    /// Atributos HTML adicionales inyectados por `DecorateMenuItem` en tiempo de render.
    pub attrs: HashMap<String, String>,
    pub children: Vec<MenuNode>,
    /// `true` si este nodo o algún descendiente coincide con la ruta actual.
    pub in_active_trail: bool,
    /// `true` si este nodo coincide exactamente con la ruta actual.
    pub is_active: bool,
}

// **< MenuTree >***********************************************************************************

/// Árbol completo de un menú, listo para renderizar.
#[derive(Clone, Debug)]
pub struct MenuTree {
    pub menu_id: i32,
    pub machine_name: String,
    pub title: String,
    pub roots: Vec<MenuNode>,
}

impl MenuTree {
    /// Aplica `f` recursivamente a todos los nodos del árbol (post-orden).
    pub fn walk_mut<F: FnMut(&mut MenuNode)>(&mut self, f: &mut F) {
        walk_nodes_mut(&mut self.roots, f);
    }
}

fn walk_nodes_mut<F: FnMut(&mut MenuNode)>(nodes: &mut [MenuNode], f: &mut F) {
    for node in nodes.iter_mut() {
        walk_nodes_mut(&mut node.children, f);
        f(node);
    }
}

// **< build_tree >*********************************************************************************

/// Construye el árbol del menú indicado aplicando caché, acciones y active trail.
///
/// Devuelve `None` si el menú no existe en la base de datos.
pub async fn build_tree(key: MenuKey, cx: &Context, opts: &TreeOptions) -> Option<MenuTree> {
    let menu = repo::find_menu_model(&key).await?;

    let flat = cache::get_or_load(menu.id, &menu.machine_name).await;

    let lang = RequestLocale::from_request(cx.request())
        .langid()
        .to_string();

    let menu_title = resolve_menu_title(menu.id, &lang).await;

    let roots = build_nodes(&flat, None, 1, opts, &lang);

    let mut tree = MenuTree {
        menu_id: menu.id,
        machine_name: menu.machine_name.clone(),
        title: menu_title,
        roots,
    };

    // Acciones de alteración del árbol (filtros, reordenamientos, etc.).
    action::AlterMenuTree::dispatch(&menu.machine_name, &mut tree, cx);

    // Cálculo del active trail por coincidencia de URL.
    let current_path = cx.request().map(|r| r.path()).unwrap_or("/");
    compute_active_trail(&mut tree.roots, current_path);

    // Acciones de resolución de active trail para rutas paramétricas o especiales.
    action::ResolveActiveTrail::dispatch(&menu.machine_name, &mut tree, cx);

    // Decoración de nodos (atributos HTML, iconos, badges...).
    let name = menu.machine_name.clone();
    tree.walk_mut(&mut |node| {
        action::DecorateMenuItem::dispatch(&name, node, cx);
    });

    Some(tree)
}

// **< Funciones internas >*************************************************************************

pub(crate) fn build_nodes(
    flat: &FlatMenu,
    parent_id: Option<i32>,
    depth: u8,
    opts: &TreeOptions,
    lang: &str,
) -> Vec<MenuNode> {
    if opts.max_depth.map(|d| depth > d).unwrap_or(false) {
        return vec![];
    }

    let mut nodes: Vec<MenuNode> = flat
        .items
        .iter()
        .filter(|m| m.parent_id == parent_id && (opts.include_disabled || m.enabled))
        .map(|m| {
            let translations = flat
                .translations
                .get(&m.id)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            MenuNode {
                item_id: m.id,
                title: resolve_item_title(translations, lang),
                url: menu_item_url(&m.url),
                weight: m.weight,
                depth,
                enabled: m.enabled,
                expanded: m.expanded,
                provider: m.provider.clone(),
                external_key: m.external_key.clone(),
                attrs: HashMap::new(),
                children: build_nodes(flat, Some(m.id), depth + 1, opts, lang),
                in_active_trail: false,
                is_active: false,
            }
        })
        .collect();

    nodes.sort_by_key(|n| n.weight);
    nodes
}

pub(crate) fn compute_active_trail(nodes: &mut [MenuNode], current_path: &str) -> bool {
    let mut any_active = false;
    for node in nodes.iter_mut() {
        let self_active = node
            .url
            .as_ref()
            .is_some_and(|p| !p.is_external() && p.path() == current_path);
        let child_active = compute_active_trail(&mut node.children, current_path);
        node.is_active = self_active;
        node.in_active_trail = self_active || child_active;
        if node.in_active_trail {
            any_active = true;
        }
    }
    any_active
}

// Resuelve el título del menú desde la BD aplicando la cadena de fallback de idioma.
async fn resolve_menu_title(menu_id: i32, lang: &str) -> String {
    use crate::entity::menu_translation;
    use pagetop_seaorm::db::{ColumnTrait, EntityTrait, QueryFilter, dbconn};

    let rows = menu_translation::Entity::find()
        .filter(menu_translation::Column::MenuId.eq(menu_id))
        .all(dbconn())
        .await
        .unwrap_or_default();

    resolve_title_from(
        rows.iter().map(|r| (r.lang.as_str(), r.title.as_str())),
        lang,
    )
}

// Resuelve el título de un ítem aplicando la cadena de fallback de idioma.
fn resolve_item_title(translations: &[menu_item_translation::Model], lang: &str) -> String {
    resolve_title_from(
        translations
            .iter()
            .map(|t| (t.lang.as_str(), t.title.as_str())),
        lang,
    )
}

// Cadena de fallback: exacto -> base del lang -> idioma por defecto -> base del defecto ->
// cualquiera.
fn resolve_title_from<'a>(
    translations: impl Iterator<Item = (&'a str, &'a str)> + Clone,
    lang: &str,
) -> String {
    let base_lang = lang.split('-').next().unwrap_or(lang);
    let default_lang = Locale::default_langid().to_string();
    let base_default = default_lang.split('-').next().unwrap_or("").to_owned();

    let candidates = [
        lang,
        base_lang,
        default_lang.as_str(),
        base_default.as_str(),
    ];

    for candidate in candidates {
        if candidate.is_empty() {
            continue;
        }
        if let Some((_, title)) = translations.clone().find(|(l, _)| *l == candidate) {
            return title.to_owned();
        }
    }

    // Cualquier traducción disponible como último recurso.
    translations
        .clone()
        .next()
        .map(|(_, t)| t.to_owned())
        .unwrap_or_default()
}
