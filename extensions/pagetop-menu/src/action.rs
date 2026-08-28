//! Tipos de acción que `pagetop-menu` expone para que otras extensiones extiendan el sistema.
//!
//! El flujo de `build_tree()` despacha estas acciones en este orden:
//!
//! 1. `AlterMenuTree` - filtros, reordenamientos, inyección de nodos en caliente.
//! 2. Cálculo del active trail por URL.
//! 3. `ResolveActiveTrail` - matching personalizado para rutas paramétricas.
//! 4. `DecorateMenuItem` - añadir atributos HTML a nodos individuales.
//!
//! El sembrado inicial usa `DeclareDefaultMenus` y `DeclareDefaultMenuItems`.

use pagetop::prelude::*;

use crate::repo::NewMenuItem;
use crate::tree::{MenuNode, MenuTree};

// **< Tipo de callbacks >**************************************************************************

pub type FnMenuDefs = fn(&mut MenuDefs);
pub type FnItemBag = fn(&mut ItemBag);
pub type FnAlterTree = fn(&mut MenuTree, &Context);
pub type FnDecorate = fn(&mut MenuNode, &Context);

// **< MenuDefs >***********************************************************************************

/// Bolsa de declaraciones de menú para `DeclareDefaultMenus`.
pub struct MenuDefs {
    pub(crate) entries: Vec<(String, String)>,
}

impl MenuDefs {
    /// Declara que el menú `machine_name` debe existir con el título dado.
    /// Si ya existe en la lista, no se añade de nuevo.
    pub fn ensure(&mut self, machine_name: impl Into<String>, title: impl Into<String>) {
        let name = machine_name.into();
        if !self.entries.iter().any(|(n, _)| n == &name) {
            self.entries.push((name, title.into()));
        }
    }
}

// **< ItemBag >************************************************************************************

/// Bolsa de declaraciones de ítems para `DeclareDefaultMenuItems`.
pub struct ItemBag {
    pub(crate) items: Vec<NewMenuItem>,
}

impl ItemBag {
    /// Añade un ítem declarado por código a la bolsa.
    pub fn add(&mut self, item: NewMenuItem) {
        self.items.push(item);
    }
}

// **< DeclareDefaultMenus >************************************************************************

/// Acción para declarar los menús que una extensión necesita.
///
/// Se despacha durante el sembrado inicial (`seed::run()`). El callback recibe un
/// [`MenuDefs`] y puede llamar a `ensure()` para declarar los menús necesarios.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop_menu::action::{DeclareDefaultMenus, MenuDefs};
///
/// fn my_menus(defs: &mut MenuDefs) {
///     defs.ensure("main", "Main navigation");
///     defs.ensure("footer", "Footer links");
/// }
/// // En Extension::actions():
/// // DeclareDefaultMenus::new(my_menus)
/// ```
pub struct DeclareDefaultMenus {
    f: FnMenuDefs,
    weight: Weight,
}

impl ActionDispatcher for DeclareDefaultMenus {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareDefaultMenus {
    pub fn new(f: FnMenuDefs) -> Self {
        DeclareDefaultMenus { f, weight: 0 }
    }

    pub fn with_weight(mut self, w: Weight) -> Self {
        self.weight = w;
        self
    }

    pub(crate) fn dispatch(defs: &mut MenuDefs) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, None),
            |action: &Self| (action.f)(defs),
        );
    }
}

// **< DeclareDefaultMenuItems >********************************************************************

/// Acción para declarar los ítems por defecto de un menú concreto.
///
/// El `referer_id` es el `machine_name` del menú al que pertenecen los ítems.
/// Se despacha durante el sembrado inicial (`seed::run()`) para cada menú conocido.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop_menu::action::{DeclareDefaultMenuItems, ItemBag};
/// use pagetop_menu::NewMenuItem;
///
/// fn blog_items(bag: &mut ItemBag) {
///     bag.add(NewMenuItem::new()
///         .with_provider("my-blog")
///         .with_external_key("blog.index")
///         .with_title("Blog")
///         .with_url("/blog")
///         .with_weight(10));
/// }
/// // En Extension::actions():
/// // DeclareDefaultMenuItems::new("main", blog_items)
/// ```
pub struct DeclareDefaultMenuItems {
    menu_name: String,
    f: FnItemBag,
    weight: Weight,
}

impl ActionDispatcher for DeclareDefaultMenuItems {
    fn referer_id(&self) -> Option<String> {
        Some(self.menu_name.clone())
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareDefaultMenuItems {
    pub fn new(menu_name: impl Into<String>, f: FnItemBag) -> Self {
        DeclareDefaultMenuItems {
            menu_name: menu_name.into(),
            f,
            weight: 0,
        }
    }

    pub fn with_weight(mut self, w: Weight) -> Self {
        self.weight = w;
        self
    }

    pub(crate) fn dispatch(menu_name: &str, bag: &mut ItemBag) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, Some(menu_name.to_owned())),
            |action: &Self| (action.f)(bag),
        );
    }
}

// **< AlterMenuTree >******************************************************************************

/// Acción para modificar el árbol de un menú antes de calcularse el active trail.
///
/// El `referer_id` es el `machine_name` del menú. Las acciones con `referer_id` `None`
/// no se registran aquí; usa un `machine_name` específico por menú.
///
/// Usos habituales: filtrar nodos por permisos, añadir nodos dinámicos, reordenar.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop_menu::action::AlterMenuTree;
/// use pagetop_menu::tree::MenuTree;
/// use pagetop::prelude::*;
///
/// fn hide_disabled(tree: &mut MenuTree, _cx: &Context) {
///     tree.roots.retain(|n| n.enabled);
/// }
/// // En Extension::actions():
/// // AlterMenuTree::new("main", hide_disabled)
/// ```
pub struct AlterMenuTree {
    menu_name: String,
    f: FnAlterTree,
    weight: Weight,
}

impl ActionDispatcher for AlterMenuTree {
    fn referer_id(&self) -> Option<String> {
        Some(self.menu_name.clone())
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl AlterMenuTree {
    pub fn new(menu_name: impl Into<String>, f: FnAlterTree) -> Self {
        AlterMenuTree {
            menu_name: menu_name.into(),
            f,
            weight: 0,
        }
    }

    pub fn with_weight(mut self, w: Weight) -> Self {
        self.weight = w;
        self
    }

    pub(crate) fn dispatch(menu_name: &str, tree: &mut MenuTree, cx: &Context) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, Some(menu_name.to_owned())),
            |action: &Self| (action.f)(tree, cx),
        );
    }
}

// **< ResolveActiveTrail >*************************************************************************

/// Acción para resolver el active trail en casos que la coincidencia por URL no cubre.
///
/// Se despacha después del cálculo automático de active trail. Útil para rutas
/// paramétricas (p. ej., `/blog/{slug}` que debe marcar activo al ítem `/blog`).
pub struct ResolveActiveTrail {
    menu_name: String,
    f: FnAlterTree,
    weight: Weight,
}

impl ActionDispatcher for ResolveActiveTrail {
    fn referer_id(&self) -> Option<String> {
        Some(self.menu_name.clone())
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl ResolveActiveTrail {
    pub fn new(menu_name: impl Into<String>, f: FnAlterTree) -> Self {
        ResolveActiveTrail {
            menu_name: menu_name.into(),
            f,
            weight: 0,
        }
    }

    pub fn with_weight(mut self, w: Weight) -> Self {
        self.weight = w;
        self
    }

    pub(crate) fn dispatch(menu_name: &str, tree: &mut MenuTree, cx: &Context) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, Some(menu_name.to_owned())),
            |action: &Self| (action.f)(tree, cx),
        );
    }
}

// **< DecorateMenuItem >***************************************************************************

/// Acción para decorar nodos individuales antes del render (atributos HTML, iconos, badges).
///
/// Se despacha para cada nodo del árbol después del cálculo del active trail,
/// por lo que el callback puede leer `node.is_active` y `node.in_active_trail`.
pub struct DecorateMenuItem {
    menu_name: String,
    f: FnDecorate,
    weight: Weight,
}

impl ActionDispatcher for DecorateMenuItem {
    fn referer_id(&self) -> Option<String> {
        Some(self.menu_name.clone())
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DecorateMenuItem {
    pub fn new(menu_name: impl Into<String>, f: FnDecorate) -> Self {
        DecorateMenuItem {
            menu_name: menu_name.into(),
            f,
            weight: 0,
        }
    }

    pub fn with_weight(mut self, w: Weight) -> Self {
        self.weight = w;
        self
    }

    pub(crate) fn dispatch(menu_name: &str, node: &mut MenuNode, cx: &Context) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, Some(menu_name.to_owned())),
            |action: &Self| (action.f)(node, cx),
        );
    }
}
