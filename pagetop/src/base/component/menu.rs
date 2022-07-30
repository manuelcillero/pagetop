use crate::prelude::*;

pub_const_handler!(COMPONENT_MENUITEM);

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
    weight    : isize,
    renderable: Renderable,
    item_type : MenuItemType,
}

impl ComponentTrait for MenuItem {
    fn new() -> Self {
        MenuItem {
            weight    : 0,
            renderable: render_always,
            item_type : MenuItemType::Void,
        }
    }

    fn handler(&self) -> Handler {
        COMPONENT_MENUITEM
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, context: &PageContext) -> bool {
        (self.renderable)(context)
    }

    fn default_render(&self, context: &mut PageContext) -> Markup {
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
                        (menu.items().render(context))
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
            weight    : 0,
            renderable: render_always,
            item_type : MenuItemType::Label(label.to_owned()),
        }
    }

    pub fn link(label: &str, path: &str) -> Self {
        MenuItem {
            weight    : 0,
            renderable: render_always,
            item_type : MenuItemType::Link(label.to_owned(), path.to_owned()),
        }
    }

    pub fn link_blank(label: &str, path: &str) -> Self {
        MenuItem {
            weight    : 0,
            renderable: render_always,
            item_type : MenuItemType::LinkBlank(label.to_owned(), path.to_owned()),
        }
    }

    pub fn html(html: Markup) -> Self {
        MenuItem {
            weight    : 0,
            renderable: render_always,
            item_type : MenuItemType::Html(html),
        }
    }

    pub fn separator() -> Self {
        MenuItem {
            weight    : 0,
            renderable: render_always,
            item_type : MenuItemType::Separator,
        }
    }

    pub fn submenu(label: &str, menu: Menu) -> Self {
        MenuItem {
            weight    : 0,
            renderable: render_always,
            item_type : MenuItemType::Submenu(label.to_owned(), menu),
        }
    }

    // MenuItem BUILDER.

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> Self {
        self.alter_renderable(renderable);
        self
    }

    // MenuItem ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, renderable: Renderable) -> &mut Self {
        self.renderable = renderable;
        self
    }

    // MenuItem GETTERS.

    pub fn item_type(&self) -> &MenuItemType {
        &self.item_type
    }
}

// Menu.

pub_const_handler!(COMPONENT_MENU);

hook_before_render_component!(HOOK_BEFORE_RENDER_MENU, Menu);

pub struct Menu {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    items     : ComponentsBundle,
    template  : String,
}

impl ComponentTrait for Menu {
    fn new() -> Self {
        Menu {
            weight    : 0,
            renderable: render_always,
            items     : ComponentsBundle::new(),
            id        : IdentifierValue::new(),
            classes   : Classes::new_with_default("sm sm-clean"),
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> Handler {
        COMPONENT_MENU
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, context: &PageContext) -> bool {
        (self.renderable)(context)
    }

    fn before_render(&mut self, context: &mut PageContext) {
        before_render_inline(self, context);
    }

    fn default_render(&self, context: &mut PageContext) -> Markup {
        context
            .alter(PageOp::AddStyleSheet(
                StyleSheet::located("/theme/menu/css/menu.css").with_version("1.1.1"),
            ))
            .alter(PageOp::AddStyleSheet(
                StyleSheet::located("/theme/menu/css/menu-clean.css").with_version("1.1.1"),
            ))
            .alter(PageOp::AddJavaScript(
                JavaScript::located("/theme/menu/js/menu.min.js").with_version("1.1.1"),
            ))
            .alter(PageOp::AddJQuery);

        let id = context.required_id::<Menu>(self.id());

        html! {
            ul id=(id) class=[self.classes().get()] {
                (self.items().render(context))
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
    // Menu BUILDER.

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.alter_id(id);
        self
    }

    pub fn with_classes(mut self, op: ClassesOp, classes: &str) -> Self {
        self.alter_classes(op, classes);
        self
    }

    pub fn with_item(mut self, item: MenuItem) -> Self {
        self.alter_item(item);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Menu ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, renderable: Renderable) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.with_value(id);
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter(op, classes);
        self
    }

    pub fn alter_item(&mut self, item: MenuItem) -> &mut Self {
        self.items.add(item);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Menu GETTERS.

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn items(&self) -> &ComponentsBundle {
        &self.items
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
