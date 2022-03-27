use crate::prelude::*;

enum MenuItemType {
    Label(String),
    Link(String, String),
    LinkBlank(String, String),
    Html(Markup),
    Separator,
    Submenu(String, Menu),
}

// -----------------------------------------------------------------------------
// MenuItem.
// -----------------------------------------------------------------------------

pub struct MenuItem {
    renderable: fn() -> bool,
    weight    : i8,
    item_type : Option<MenuItemType>,
}

impl PageComponent for MenuItem {

    fn new() -> Self {
        MenuItem {
            renderable: always,
            weight    : 0,
            item_type : None,
        }
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, assets: &mut PageAssets) -> Markup {
        match &self.item_type {
            Some(MenuItemType::Label(label)) => html! {
                li class="label" { a href="#" { (label) } }
            },
            Some(MenuItemType::Link(label, path)) => html! {
                li class="link" { a href=(path) { (label) } }
            },
            Some(MenuItemType::LinkBlank(label, path)) => html! {
                li class="link_blank" {
                    a href=(path) target="_blank" { (label) }
                }
            },
            Some(MenuItemType::Html(html)) => html! {
                li class="html" { (*html) }
            },
            Some(MenuItemType::Submenu(label, menu)) => html! {
                li class="submenu" {
                    a href="#" { (label) }
                    ul {
                        (menu.render_items(assets))
                    }
                }
            },
            Some(MenuItemType::Separator) => html! {
                li class="separator" { }
            },
            None => html! {}
        }
    }
}

impl MenuItem {

    pub fn label(label: &str) -> Self {
        MenuItem {
            renderable: always,
            weight    : 0,
            item_type : Some(MenuItemType::Label(label.to_owned())),
        }
    }

    pub fn link(label: &str, path: &str) -> Self {
        MenuItem {
            renderable: always,
            weight    : 0,
            item_type : Some(MenuItemType::Link(
                label.to_owned(),
                path.to_owned(),
            )),
        }
    }

    pub fn link_blank(label: &str, path: &str) -> Self {
        MenuItem {
            renderable: always,
            weight    : 0,
            item_type : Some(MenuItemType::LinkBlank(
                label.to_owned(),
                path.to_owned(),
            )),
        }
    }

    pub fn html(html: Markup) -> Self {
        MenuItem {
            renderable: always,
            weight    : 0,
            item_type : Some(MenuItemType::Html(html)),
        }
    }

    pub fn separator() -> Self {
        MenuItem {
            renderable: always,
            weight    : 0,
            item_type : Some(MenuItemType::Separator),
        }
    }

    pub fn submenu(label: &str, menu: Menu) -> Self {
        MenuItem {
            renderable: always,
            weight    : 0,
            item_type : Some(MenuItemType::Submenu(
                label.to_owned(),
                menu
            )),
        }
    }

    // MenuItem BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }
}

// -----------------------------------------------------------------------------
// Menu.
// -----------------------------------------------------------------------------

pub struct Menu {
    renderable: fn() -> bool,
    weight    : i8,
    id        : OptIden,
    items     : PageContainer,
    template  : String,
}

impl PageComponent for Menu {

    fn new() -> Self {
        Menu {
            renderable: always,
            weight    : 0,
            id        : OptIden::none(),
            items     : PageContainer::new(),
            template  : "default".to_owned(),
        }
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, assets: &mut PageAssets) -> Markup {
        assets
            .add_stylesheet(StyleSheet::source(
                "/theme/menu/css/menu.css?ver=1.1.1"
            ))
            .add_stylesheet(StyleSheet::source(
                "/theme/menu/css/menu-clean.css?ver=1.1.1"
            ))
            .add_javascript(JavaScript::source(
                "/theme/menu/js/menu.min.js?ver=1.1.1"
            ))
            .add_jquery();

        let id = assets.serial_id(self.name(), self.id.value());
        html! {
            ul id=(id) class="sm sm-clean" {
                (self.render_items(assets))
            }
            script type="text/javascript" defer {
                "jQuery(function(){jQuery('#" (id) "').smartmenus({"
                "hideTimeout: 0,"
                "showTimeout: 80,"
                "});});"
            }
        }
    }
}

impl Menu {

    // Menu BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id.with_value(id);
        self
    }

    pub fn add(mut self, item: MenuItem) -> Self {
        self.items.add(item);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Menu GETTERS.

    pub fn id(&self) -> &str {
        self.id.value()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }

    // Menu EXTRAS.

    pub fn render_items(&self, assets: &mut PageAssets) -> Markup {
        html! { (self.items.render(assets)) }
    }
}

fn always() -> bool {
    true
}
