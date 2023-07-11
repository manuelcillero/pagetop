use pagetop::prelude::*;
use pagetop_jquery::JQuery;

use_handle!(COMPONENT_MEGAMENUITEM);

type Label = OneComponent<L10n>;
type Content = OneComponent<L10n>;

#[derive(Default)]
pub enum MegaMenuItemType {
    #[default]
    Void,
    Label(Label),
    Link(Label, String),
    LinkBlank(Label, String),
    Html(Content),
    Submenu(Label, MegaMenu),
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

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.item_type() {
            MegaMenuItemType::Void => PrepareMarkup::None,

            MegaMenuItemType::Label(label) => PrepareMarkup::With(html! {
                li class="label" { a href="#" { (label.prepare(cx)) } }
            }),
            MegaMenuItemType::Link(label, path) => PrepareMarkup::With(html! {
                li class="link" { a href=(path) { (label.prepare(cx)) } }
            }),
            MegaMenuItemType::LinkBlank(label, path) => PrepareMarkup::With(html! {
                li class="link_blank" {
                    a href=(path) target="_blank" { (label.prepare(cx)) }
                }
            }),
            MegaMenuItemType::Html(content) => PrepareMarkup::With(html! {
                li class="html" { (content.prepare(cx)) }
            }),
            MegaMenuItemType::Submenu(label, menu) => PrepareMarkup::With(html! {
                li class="submenu" {
                    a href="#" { (label.prepare(cx)) }
                    ul {
                        (menu.items().prepare(cx))
                    }
                }
            }),
            MegaMenuItemType::Separator => PrepareMarkup::With(html! {
                li class="separator" { }
            }),
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
            item_type: MegaMenuItemType::Label(OneComponent::new_with(label)),
            ..Default::default()
        }
    }

    pub fn link(label: L10n, path: &str) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Link(OneComponent::new_with(label), path.to_owned()),
            ..Default::default()
        }
    }

    pub fn link_blank(label: L10n, path: &str) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::LinkBlank(OneComponent::new_with(label), path.to_owned()),
            ..Default::default()
        }
    }

    pub fn html(content: L10n) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Html(OneComponent::new_with(content)),
            ..Default::default()
        }
    }

    pub fn submenu(label: L10n, menu: MegaMenu) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Submenu(OneComponent::new_with(label), menu),
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

use_handle!(COMPONENT_MEGAMENU);

actions_for_component!(MegaMenu);

#[rustfmt::skip]
#[derive(Default)]
pub struct MegaMenu {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    items     : PackComponents,
    template  : String,
}

impl ComponentTrait for MegaMenu {
    fn new() -> Self {
        MegaMenu::default().with_classes(ClassesOp::SetDefault, "sm sm-clean")
    }

    fn handle(&self) -> Handle {
        COMPONENT_MEGAMENU
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn before_prepare_component(&mut self, cx: &mut Context) {
        run_actions_before_prepare_megamenu(self, cx);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        cx.alter(ContextOp::AddStyleSheet(
            StyleSheet::at("/megamenu/css/menu.css").with_version("1.1.1"),
        ))
        .alter(ContextOp::AddStyleSheet(
            StyleSheet::at("/megamenu/css/menu-clean.css").with_version("1.1.1"),
        ))
        .alter(ContextOp::AddJavaScript(
            JavaScript::at("/megamenu/js/menu.min.js").with_version("1.1.1"),
        ));
        JQuery.enable_jquery(cx);

        let id = cx.required_id::<MegaMenu>(self.id());

        PrepareMarkup::With(html! {
            ul id=(id) class=[self.classes().get()] {
                (self.items().prepare(cx))
            }
            script type="text/javascript" defer {
                "jQuery(function(){jQuery('#" (id) "').smartmenus({"
                "hideTimeout: 0,"
                "showTimeout: 80,"
                "});});"
            }
        })
    }

    fn after_prepare_component(&mut self, cx: &mut Context) {
        run_actions_after_prepare_megamenu(self, cx);
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

    pub fn with_item(mut self, item: MegaMenuItem) -> Self {
        self.items.alter_pack(PackOp::Add, item);
        self
    }

    pub fn alter_items(&mut self, op: PackOp, item: MegaMenuItem) -> &mut Self {
        self.items.alter_pack(op, item);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // MegaMenu GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn items(&self) -> &PackComponents {
        &self.items
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
