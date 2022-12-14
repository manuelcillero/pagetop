use crate::prelude::*;

pub_handle!(COMPONENT_ANCHOR);

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

pub type AnchorIcon = ComponentsBundle;

#[rustfmt::skip]
#[derive(Default)]
pub struct Anchor {
    weight     : isize,
    renderable : Renderable,
    id         : IdentifierValue,
    classes    : Classes,
    anchor_type: AnchorType,
    href       : AttributeValue,
    html       : HtmlMarkup,
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

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rsx: &RenderResources) -> bool {
        (self.renderable.check)(rsx)
    }

    #[rustfmt::skip]
    fn default_render(&self, rsx: &mut RenderResources) -> Markup {
        let target = match &self.target() {
            AnchorTarget::Blank         => Some("_blank"),
            AnchorTarget::Parent        => Some("_parent"),
            AnchorTarget::Top           => Some("_top"),
            AnchorTarget::Context(name) => Some(name.as_str()),
            _ => None,
        };
        html! {
            a
                id=[self.id().get()]
                class=[self.classes().get()]
                href=[self.href().get()]
                target=[target]
            {
                (self.left_icon().render(rsx))
                span { (*self.html()) }
                (self.right_icon().render(rsx))
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
    pub fn link(href: &str, html: Markup) -> Self {
        Anchor::new().with_href(href).with_html(html)
    }

    pub fn button(href: &str, html: Markup) -> Self {
        Anchor::new()
            .with_type(AnchorType::Button)
            .with_href(href)
            .with_html(html)
    }

    pub fn location(id: &str) -> Self {
        Anchor::new().with_type(AnchorType::Location).with_id(id)
    }

    // Anchor BUILDER.

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

    pub fn with_type(mut self, anchor_type: AnchorType) -> Self {
        self.alter_type(anchor_type);
        self
    }

    pub fn with_href(mut self, href: &str) -> Self {
        self.alter_href(href);
        self
    }

    pub fn with_html(mut self, html: Markup) -> Self {
        self.alter_html(html);
        self
    }

    pub fn with_left_icon(mut self, icon: Icon) -> Self {
        self.alter_left_icon(icon);
        self
    }

    pub fn with_right_icon(mut self, icon: Icon) -> Self {
        self.alter_right_icon(icon);
        self
    }

    pub fn with_target(mut self, target: AnchorTarget) -> Self {
        self.alter_target(target);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Anchor ALTER.

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

    pub fn alter_href(&mut self, href: &str) -> &mut Self {
        self.href.alter_value(href);
        self
    }

    pub fn alter_html(&mut self, html: Markup) -> &mut Self {
        self.html.markup = html;
        self
    }

    pub fn alter_left_icon(&mut self, icon: Icon) -> &mut Self {
        self.left_icon.clear();
        self.left_icon.add(icon);
        self
    }

    pub fn alter_right_icon(&mut self, icon: Icon) -> &mut Self {
        self.right_icon.clear();
        self.right_icon.add(icon);
        self
    }

    pub fn alter_target(&mut self, target: AnchorTarget) -> &mut Self {
        self.target = target;
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Anchor GETTERS.

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn anchor_type(&self) -> &AnchorType {
        &self.anchor_type
    }

    pub fn href(&self) -> &AttributeValue {
        &self.href
    }

    pub fn html(&self) -> &Markup {
        &self.html.markup
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
