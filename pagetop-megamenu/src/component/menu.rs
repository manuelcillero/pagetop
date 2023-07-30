use pagetop::prelude::*;
use pagetop_jquery::JQuery;

use crate::component::MegaMenuItem;
use crate::LOCALES_MEGAMENU;

new_handle!(COMPONENT_MEGAMENU);

actions_for_component!(MegaMenu);

// SmartMenus library version.
const VERSION_SMARTMENUS: &str = "1.2.1";

#[rustfmt::skip]
#[derive(Default)]
pub struct MegaMenu {
    weight    : Weight,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    items     : PackComponents,
}

impl ComponentTrait for MegaMenu {
    fn new() -> Self {
        MegaMenu::default().with_classes(ClassesOp::SetDefault, "megamenu-menu sm sm-clean")
    }

    fn handle(&self) -> Handle {
        COMPONENT_MEGAMENU
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn before_prepare_component(&mut self, cx: &mut Context) {
        cx.alter(ContextOp::AddStyleSheet(
            StyleSheet::at("/megamenu/css/smartmenus.css").with_version(VERSION_SMARTMENUS),
        ))
        .alter(ContextOp::AddStyleSheet(
            StyleSheet::at("/megamenu/css/sm-clean.css").with_version(VERSION_SMARTMENUS),
        ))
        .alter(ContextOp::AddJavaScript(
            JavaScript::at("/megamenu/js/smartmenus.min.js").with_version(VERSION_SMARTMENUS),
        ));
        JQuery.enable_jquery(cx);

        run_actions_before_prepare_megamenu(self, cx);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let id_nav = cx.required_id::<MegaMenu>(self.id());
        let id_menu = concat_string!(id_nav, "-menu");
        let id_state = concat_string!(id_nav, "-state");

        PrepareMarkup::With(html! {
            nav id=(id_nav) class="megamenu" role="navigation" {
                input id=(id_state) class="megamenu-state" type="checkbox" {}
                label class="megamenu-btn" for=(id_state) {
                    span class="megamenu-btn-icon" {}
                    (L10n::t("toggle_menu", &LOCALES_MEGAMENU).prepare(cx))
                }
                ul id=(id_menu) class=[self.classes().get()] {
                    (self.items().prepare(cx))
                }
                script type="text/javascript" defer {
                    r###"
$(function() {
    $('#"###r (id_menu) r###"').smartmenus({
        hideTimeout: 0,
        showTimeout: 80,
    });
});
$(function() {
    var $menuState = $('#"###r (id_state) r###"');
    if ($menuState.length) {
        // Animate mobile menu.
        $menuState.change(function(e) {
            var $menu = $('#"###r (id_menu) r###"');
            if (this.checked) {
                $menu.hide().slideDown(250, function() { $menu.css('display', ''); });
            } else {
                $menu.show().slideUp(250, function() { $menu.css('display', ''); });
            }
        });
        // Hide mobile menu beforeunload.
        $(window).on('beforeunload unload', function() {
            if ($menuState[0].checked) {
                $menuState[0].click();
            }
        });
    }
});
                    "###r
                }
            }
        })
    }

    fn after_prepare_component(&mut self, cx: &mut Context) {
        run_actions_after_prepare_megamenu(self, cx);
    }
}

impl MegaMenu {
    // MegaMenu BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
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
        self.items.alter(PackOp::Add, ComponentRef::to(item));
        self
    }

    pub fn alter_items(&mut self, op: PackOp, item: MegaMenuItem) -> &mut Self {
        self.items.alter(op, ComponentRef::to(item));
        self
    }

    // MegaMenu GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn items(&self) -> &PackComponents {
        &self.items
    }
}
