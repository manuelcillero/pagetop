use crate::prelude::*;

pub const MENU_COMPONENT: &str = "pagetop::component::menu";
pub const MENUITEM_COMPONENT: &str = "pagetop::component::menu_item";

pub enum MenuItemType {
    Label(String),
    Link(String, String),
    LinkBlank(String, String),
    Html(Markup),
    Separator,
    Submenu(String, Menu),
    Void,
}

// MenuItem.

pub struct MenuItem {
    renderable: fn() -> bool,
    weight    : isize,
    item_type : MenuItemType,
}

impl ComponentTrait for MenuItem {
    fn new() -> Self {
        MenuItem {
            renderable: render_always,
            weight    : 0,
            item_type : MenuItemType::Void,
        }
    }

    fn handler(&self) -> &'static str {
        MENUITEM_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, assets: &mut Assets) -> Markup {
        match self.item_type() {
            MenuItemType::Label(label) => html! {
                li class="label" { a href="#" { (label) } }
            },
            MenuItemType::Link(label, path) => html! {
                li class="link" { a href=(path) { (label) } }
            },
            MenuItemType::LinkBlank(label, path) => html! {
                li class="link_blank" {
                    a href=(path) target="_blank" { (label) }
                }
            },
            MenuItemType::Html(html) => html! {
                li class="html" { (*html) }
            },
            MenuItemType::Submenu(label, menu) => html! {
                li class="submenu" {
                    a href="#" { (label) }
                    ul {
                        (menu.items().render(assets))
                    }
                }
            },
            MenuItemType::Separator => html! {
                li class="separator" { }
            },
            MenuItemType::Void => html! {},
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl MenuItem {
    pub fn label(label: &str) -> Self {
        MenuItem {
            renderable: render_always,
            weight    : 0,
            item_type : MenuItemType::Label(label.to_owned()),
        }
    }

    pub fn link(label: &str, path: &str) -> Self {
        MenuItem {
            renderable: render_always,
            weight    : 0,
            item_type : MenuItemType::Link(
                label.to_owned(),
                path.to_owned(),
            ),
        }
    }

    pub fn link_blank(label: &str, path: &str) -> Self {
        MenuItem {
            renderable: render_always,
            weight    : 0,
            item_type : MenuItemType::LinkBlank(
                label.to_owned(),
                path.to_owned(),
            ),
        }
    }

    pub fn html(html: Markup) -> Self {
        MenuItem {
            renderable: render_always,
            weight    : 0,
            item_type : MenuItemType::Html(html),
        }
    }

    pub fn separator() -> Self {
        MenuItem {
            renderable: render_always,
            weight    : 0,
            item_type : MenuItemType::Separator,
        }
    }

    pub fn submenu(label: &str, menu: Menu) -> Self {
        MenuItem {
            renderable: render_always,
            weight    : 0,
            item_type : MenuItemType::Submenu(
                label.to_owned(),
                menu
            ),
        }
    }

    // MenuItem BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    // MenuItem ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    // MenuItem GETTERS.

    pub fn item_type(&self) -> &MenuItemType {
        &self.item_type
    }
}

// Menu.

pub struct Menu {
    renderable: fn() -> bool,
    weight    : isize,
    items     : ComponentsHolder,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl ComponentTrait for Menu {
    fn new() -> Self {
        Menu {
            renderable: render_always,
            weight    : 0,
            items     : ComponentsHolder::new(),
            id        : OptIden::new(),
            classes   : Classes::new_with_default("sm sm-clean"),
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        MENU_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, assets: &mut Assets) -> Markup {
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

        let id = assets.required_id::<Menu>(self.id());
        html! {
            ul id=(id) class=[self.classes()] {
                (self.items().render(assets))
            }
            script type="text/javascript" defer {
                "jQuery(function(){jQuery('#" (id) "').smartmenus({"
                "hideTimeout: 0,"
                "showTimeout: 80,"
                "});});"
            }
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Menu {

    // Menu CONTAINER.

    pub fn add(mut self, item: MenuItem) -> Self {
        self.items.add(item);
        self
    }

    pub fn items(&self) -> &ComponentsHolder {
        &self.items
    }

    // Menu BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.alter_id(id);
        self
    }

    pub fn with_classes(mut self, classes: &str, op: ClassesOp) -> Self {
        self.alter_classes(classes, op);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Menu ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.with_value(id);
        self
    }

    pub fn alter_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.classes.alter(classes, op);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Menu GETTERS.

    pub fn id(&self) -> &Option<String> {
        self.id.option()
    }

    pub fn classes(&self) -> &Option<String> {
        self.classes.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
