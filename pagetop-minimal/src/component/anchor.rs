use pagetop::prelude::*;

use crate::component::Icon;

define_handle!(COMPONENT_ANCHOR);

#[derive(Default)]
pub enum AnchorType {
    #[default]
    Link,
    Button,
    Location,
}

#[derive(Default)]
pub enum AnchorTarget {
    #[default]
    Default,
    Blank,
    Parent,
    Top,
    Context(String),
}

type AnchorIcon = OneComponent<Icon>;
type AnchorHtml = OneComponent<L10n>;

#[rustfmt::skip]
#[derive(Default)]
pub struct Anchor {
    weight     : isize,
    renderable : Renderable,
    id         : IdentifierValue,
    classes    : Classes,
    anchor_type: AnchorType,
    href       : AttributeValue,
    html       : AnchorHtml,
    left_icon  : AnchorIcon,
    right_icon : AnchorIcon,
    target     : AnchorTarget,
    template   : String,
}

impl ComponentTrait for Anchor {
    fn new() -> Self {
        Anchor::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_ANCHOR
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    #[rustfmt::skip]
    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        let target = match &self.target() {
            AnchorTarget::Blank         => Some("_blank"),
            AnchorTarget::Parent        => Some("_parent"),
            AnchorTarget::Top           => Some("_top"),
            AnchorTarget::Context(name) => Some(name.as_str()),
            _ => None,
        };
        html! {
            a
                id=[self.id()]
                class=[self.classes().get()]
                href=[self.href().get()]
                target=[target]
            {
                (self.left_icon().render(rcx))
                (" ") span { (self.html().render(rcx)) } (" ")
                (self.right_icon().render(rcx))
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

impl Anchor {
    pub fn link(href: &str, html: L10n) -> Self {
        Anchor::new().with_href(href).with_html(html)
    }

    pub fn button(href: &str, html: L10n) -> Self {
        Anchor::new()
            .with_type(AnchorType::Button)
            .with_href(href)
            .with_html(html)
    }

    pub fn location(id: &str) -> Self {
        Anchor::new().with_type(AnchorType::Location).with_id(id)
    }

    // Anchor BUILDER.

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
    pub fn alter_type(&mut self, anchor_type: AnchorType) -> &mut Self {
        self.anchor_type = anchor_type;
        self.classes.alter_value(
            ClassesOp::SetDefault,
            match self.anchor_type {
                AnchorType::Button => "btn btn-primary",
                _ => "",
            },
        );
        self
    }

    #[fn_builder]
    pub fn alter_href(&mut self, href: &str) -> &mut Self {
        self.href.alter_value(href);
        self
    }

    #[fn_builder]
    pub fn alter_html(&mut self, html: L10n) -> &mut Self {
        self.html.set(html);
        self
    }

    #[fn_builder]
    pub fn alter_left_icon(&mut self, icon: Icon) -> &mut Self {
        self.left_icon.set(icon);
        self
    }

    #[fn_builder]
    pub fn alter_right_icon(&mut self, icon: Icon) -> &mut Self {
        self.right_icon.set(icon);
        self
    }

    #[fn_builder]
    pub fn alter_target(&mut self, target: AnchorTarget) -> &mut Self {
        self.target = target;
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Anchor GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn anchor_type(&self) -> &AnchorType {
        &self.anchor_type
    }

    pub fn href(&self) -> &AttributeValue {
        &self.href
    }

    pub fn html(&self) -> &AnchorHtml {
        &self.html
    }

    pub fn left_icon(&self) -> &AnchorIcon {
        &self.left_icon
    }

    pub fn right_icon(&self) -> &AnchorIcon {
        &self.right_icon
    }

    pub fn target(&self) -> &AnchorTarget {
        &self.target
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
