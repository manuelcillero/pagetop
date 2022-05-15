use crate::prelude::*;

pub const ANCHOR_COMPONENT: &str = "pagetop::component::anchor";

pub enum AnchorType {
    Button,
    Link,
    Location,
}

pub enum AnchorTarget {
    Blank,
    Context(String),
    Default,
    Parent,
    Top,
}

pub struct Anchor {
    renderable : fn() -> bool,
    weight     : isize,
    anchor_type: AnchorType,
    href       : OptAttr,
    html       : Markup,
    target     : AnchorTarget,
    id         : OptIden,
    classes    : Classes,
    template   : String,
}

impl ComponentTrait for Anchor {
    fn new() -> Self {
        Anchor {
            renderable : render_always,
            weight     : 0,
            anchor_type: AnchorType::Link,
            href       : OptAttr::new(),
            html       : html! {},
            target     : AnchorTarget::Default,
            id         : OptIden::new(),
            classes    : Classes::new(),
            template   : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        ANCHOR_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, _: &mut InContext) -> Markup {
        let target = match &self.target() {
            AnchorTarget::Blank         => Some("_blank"),
            AnchorTarget::Context(name) => Some(name.as_str()),
            AnchorTarget::Parent        => Some("_parent"),
            AnchorTarget::Top           => Some("_top"),
            _ => None,
        };
        html! {
            a
                id=[self.id()]
                class=[self.classes()]
                href=[self.href()]
                target=[target]
            {
                (*self.html())
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
        Anchor::new().with_type(AnchorType::Button).with_href(href).with_html(html)
    }

    pub fn location(id: &str) -> Self {
        Anchor::new().with_type(AnchorType::Location).with_id(id)
    }

    // Anchor BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
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

    pub fn with_target(mut self, target: AnchorTarget) -> Self {
        self.alter_target(target);
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

    // Anchor ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_type(&mut self, anchor_type: AnchorType) -> &mut Self {
        self.anchor_type = anchor_type;
        self.classes.alter(match self.anchor_type {
            AnchorType::Button => "btn btn-primary",
            _ => "",
        }, ClassesOp::SetDefault);
        self
    }

    pub fn alter_href(&mut self, href: &str) -> &mut Self {
        self.href.with_value(href);
        self
    }

    pub fn alter_html(&mut self, html: Markup) -> &mut Self {
        self.html = html;
        self
    }

    pub fn alter_target(&mut self, target: AnchorTarget) -> &mut Self {
        self.target = target;
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

    // Anchor GETTERS.

    pub fn anchor_type(&self) -> &AnchorType {
        &self.anchor_type
    }

    pub fn href(&self) -> &Option<String> {
        self.href.option()
    }

    pub fn html(&self) -> &Markup {
        &self.html
    }

    pub fn target(&self) -> &AnchorTarget {
        &self.target
    }

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
