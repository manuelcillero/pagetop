use crate::prelude::*;

pub_handle!(COMPONENT_MENUITEM);

#[derive(Default)]
pub enum MenuItemType {
    #[default]
    Void,
    Label(String),
    Link(String, String),
    LinkBlank(String, String),
    Html(Markup),
    Submenu(String, Menu),
    Separator,
}

// MenuItem.

#[rustfmt::skip]
#[derive(Default)]
pub struct MenuItem {
    weight    : isize,
    renderable: Renderable,
    item_type : MenuItemType,
}

impl ComponentTrait for MenuItem {
    fn new() -> Self {
        MenuItem::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_MENUITEM
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        match self.item_type() {
            MenuItemType::Void => html! {},

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
                        (menu.items().render(rcx))
                    }
                }
            },
            MenuItemType::Separator => html! {
                li class="separator" { }
            },
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
            item_type: MenuItemType::Label(label.to_owned()),
            ..Default::default()
        }
    }

    pub fn link(label: &str, path: &str) -> Self {
        MenuItem {
            item_type: MenuItemType::Link(label.to_owned(), path.to_owned()),
            ..Default::default()
        }
    }

    pub fn link_blank(label: &str, path: &str) -> Self {
        MenuItem {
            item_type: MenuItemType::LinkBlank(label.to_owned(), path.to_owned()),
            ..Default::default()
        }
    }

    pub fn html(html: Markup) -> Self {
        MenuItem {
            item_type: MenuItemType::Html(html),
            ..Default::default()
        }
    }

    pub fn submenu(label: &str, menu: Menu) -> Self {
        MenuItem {
            item_type: MenuItemType::Submenu(label.to_owned(), menu),
            ..Default::default()
        }
    }

    pub fn separator() -> Self {
        MenuItem {
            item_type: MenuItemType::Separator,
            ..Default::default()
        }
    }

    // MenuItem BUILDER.

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, check: IsRenderable) -> Self {
        self.alter_renderable(check);
        self
    }

    // MenuItem ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    // MenuItem GETTERS.

    pub fn item_type(&self) -> &MenuItemType {
        &self.item_type
    }
}

// Menu.

pub_handle!(COMPONENT_MENU);

hook_before_render_component!(HOOK_BEFORE_RENDER_MENU, Menu);

#[rustfmt::skip]
#[derive(Default)]
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
        Menu::default().with_classes(ClassesOp::SetDefault, "sm sm-clean")
    }

    fn handle(&self) -> Handle {
        COMPONENT_MENU
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn before_render(&mut self, rcx: &mut RenderContext) {
        before_render_inline(self, rcx);
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        rcx.alter(ContextOp::AddStyleSheet(
            StyleSheet::located("/theme/menu/css/menu.css").with_version("1.1.1"),
        ))
        .alter(ContextOp::AddStyleSheet(
            StyleSheet::located("/theme/menu/css/menu-clean.css").with_version("1.1.1"),
        ))
        .alter(ContextOp::AddJavaScript(
            JavaScript::located("/theme/menu/js/menu.min.js").with_version("1.1.1"),
        ))
        .alter(ContextOp::AddJQuery);

        let id = rcx.required_id::<Menu>(self.id());

        html! {
            ul id=(id) class=[self.classes().get()] {
                (self.items().render(rcx))
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

    pub fn with_renderable(mut self, check: IsRenderable) -> Self {
        self.alter_renderable(check);
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

    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
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
