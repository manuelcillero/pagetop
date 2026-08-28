/*!
<div align="center">

<h1>PageTop Menu</h1>

<p>Gestión centralizada y persistente de menús para <strong>PageTop</strong>.</p>

</div>

## Guía rápida

Declara la dependencia en tu `Cargo.toml` y reenvía a `pagetop-seaorm` el motor de base de datos
que vayas a usar:

```toml
[features]
sqlite = ["pagetop-seaorm/sqlite"]

[dependencies]
pagetop-menu = { version = "..." }
```

Añade `&pagetop_menu::Menu` a las dependencias de tu extensión, declara los menús
que necesitas y añade los componentes a tus páginas:

```rust,no_run
use pagetop::prelude::*;
use pagetop_menu::prelude::*;

pub struct MyApp;

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_menu::Menu]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            DeclareDefaultMenuItems::new("main", home_items),
        ]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/", web::get(home))
    }
}

fn home_items(bag: &mut ItemBag) {
    bag.add(NewMenuItem::new()
        .with_provider("myapp")
        .with_external_key("home")
        .with_title("Home")
        .with_url("/")
        .with_weight(0));
}

async fn home(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(MenuBlock::with("main"))
        .render().await
}
```
*/

use pagetop::prelude::*;
use pagetop_seaorm::install_migrations;

include_locales!(LOCALES_MENU);

pub mod action;
pub mod component;
pub mod config;
pub mod error;
pub mod tree;

pub(crate) mod cache;
pub(crate) mod entity;
pub(crate) mod migration;
pub(crate) mod repo;
pub(crate) mod seed;

pub use action::{
    AlterMenuTree, DeclareDefaultMenuItems, DeclareDefaultMenus, DecorateMenuItem, ItemBag,
    MenuDefs, ResolveActiveTrail,
};
pub use repo::NewMenuItem;
pub use tree::{MenuKey, MenuNode, MenuTree, TreeOptions, build_tree};

/// Prelude de `pagetop-menu`.
pub mod prelude {
    pub use crate::action::{
        AlterMenuTree, DeclareDefaultMenuItems, DeclareDefaultMenus, DecorateMenuItem, ItemBag,
        MenuDefs, ResolveActiveTrail,
    };
    pub use crate::component::{MenuBlock, MenuBreadcrumb};
    pub use crate::error::MenuError;
    pub use crate::repo::NewMenuItem;
    pub use crate::tree::{MenuKey, MenuNode, MenuTree, TreeOptions, build_tree};
}

// **< Extension >**********************************************************************************

/// Implementa la extensión `pagetop-menu`.
pub struct Menu;

#[async_trait]
impl Extension for Menu {
    fn name(&self) -> Lc {
        Lc::t("extension_name", &LOCALES_MENU)
    }

    fn description(&self) -> Lc {
        Lc::t("extension_description", &LOCALES_MENU)
    }

    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_seaorm::SeaORM]
    }

    async fn initialize(&self) {
        install_migrations!(
            m20260629_000001_create_menus,
            m20260629_000002_create_menu_translations,
            m20260629_000003_create_menu_items,
            m20260629_000004_create_menu_item_translations,
        );
        seed::run().await;
    }
}
