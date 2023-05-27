use pagetop::prelude::*;

define_handle!(COMPONENT_MEGAMENUITEM);

#[derive(Default)]
pub enum MegaMenuItemType {
    #[default]
    Void,
    Label(ComponentArc),
    Link(ComponentArc, String),
    LinkBlank(ComponentArc, String),
    Html(Markup),
    Submenu(ComponentArc, MegaMenu),
    Separator,
}

// MegaMenuItem.

#[rustfmt::skip]
#[derive(Default)]
pub struct MegaMenuItem {
    weight    : isize,
    renderable: Renderable,
    item_type : MegaMenuItemType,
}

impl ComponentTrait for MegaMenuItem {
    fn new() -> Self {
        MegaMenuItem::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_MEGAMENUITEM
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        match self.item_type() {
            MegaMenuItemType::Void => html! {},

            MegaMenuItemType::Label(label) => html! {
                li class="label" { a href="#" { (label.render(rcx)) } }
            },
            MegaMenuItemType::Link(label, path) => html! {
                li class="link" { a href=(path) { (label.render(rcx)) } }
            },
            MegaMenuItemType::LinkBlank(label, path) => html! {
                li class="link_blank" {
                    a href=(path) target="_blank" { (label.render(rcx)) }
                }
            },
            MegaMenuItemType::Html(html) => html! {
                li class="html" { (*html) }
            },
            MegaMenuItemType::Submenu(label, menu) => html! {
                li class="submenu" {
                    a href="#" { (label.render(rcx)) }
                    ul {
                        (menu.items().render(rcx))
                    }
                }
            },
            MegaMenuItemType::Separator => html! {
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

impl MegaMenuItem {
    pub fn label(label: L10n) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Label(ComponentArc::new_with(label)),
            ..Default::default()
        }
    }

    pub fn link(label: L10n, path: &str) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Link(ComponentArc::new_with(label), path.to_owned()),
            ..Default::default()
        }
    }

    pub fn link_blank(label: L10n, path: &str) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::LinkBlank(ComponentArc::new_with(label), path.to_owned()),
            ..Default::default()
        }
    }

    pub fn html(html: Markup) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Html(html),
            ..Default::default()
        }
    }

    pub fn submenu(label: L10n, menu: MegaMenu) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Submenu(ComponentArc::new_with(label), menu),
            ..Default::default()
        }
    }

    pub fn separator() -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Separator,
            ..Default::default()
        }
    }

    // MegaMenuItem BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    // MegaMenuItem GETTERS.

    pub fn item_type(&self) -> &MegaMenuItemType {
        &self.item_type
    }
}

// MegaMenu.

define_handle!(COMPONENT_MEGAMENU);

hook_before_render_component!(HOOK_BEFORE_RENDER_MENU, MegaMenu);

#[rustfmt::skip]
#[derive(Default)]
pub struct MegaMenu {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    items     : ComponentsBundle,
    template  : String,
}

impl ComponentTrait for MegaMenu {
    fn new() -> Self {
        MegaMenu::default().with_classes(ClassesOp::SetDefault, "sm sm-clean")
    }

    fn handle(&self) -> Handle {
        COMPONENT_MEGAMENU
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
            StyleSheet::located("/megamenu/css/menu.css").with_version("1.1.1"),
        ))
        .alter(ContextOp::AddStyleSheet(
            StyleSheet::located("/megamenu/css/menu-clean.css").with_version("1.1.1"),
        ))
        .alter(ContextOp::AddJavaScript(
            JavaScript::located("/megamenu/js/menu.min.js").with_version("1.1.1"),
        ));
        pagetop_jquery::JQuery::add_jquery(rcx);

        let id = rcx.required_id::<MegaMenu>(self.id());

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

impl MegaMenu {
    // MegaMenu BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_item(&mut self, item: MegaMenuItem) -> &mut Self {
        self.items.add(item);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // MegaMenu GETTERS.

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
