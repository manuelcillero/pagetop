use crate::prelude::*;

new_handle!(COMPONENT_BASE_ANCHOR);

#[derive(Default)]
pub enum AnchorType {
    #[default]
    Link,
    Button,
    Location,
}

#[rustfmt::skip]
impl ToString for AnchorType {
    fn to_string(&self) -> String {
        match self {
            AnchorType::Button => "btn btn-primary".to_string(),
            _ => "".to_string(),
        }
    }
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

type AnchorIcon = TypedComponent<Icon>;

#[rustfmt::skip]
#[derive(Default)]
pub struct Anchor {
    weight     : Weight,
    renderable : Renderable,
    id         : OptionId,
    classes    : OptionClasses,
    font_size  : FontSize,
    anchor_type: AnchorType,
    href       : OptionString,
    html       : OptionTranslated,
    left_icon  : AnchorIcon,
    right_icon : AnchorIcon,
    target     : AnchorTarget,
}

impl ComponentTrait for Anchor {
    fn new() -> Self {
        Anchor::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_BASE_ANCHOR
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

    #[rustfmt::skip]
    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let target = match &self.target() {
            AnchorTarget::Blank         => Some("_blank"),
            AnchorTarget::Parent        => Some("_parent"),
            AnchorTarget::Top           => Some("_top"),
            AnchorTarget::Context(name) => Some(name.as_str()),
            _ => None,
        };
        PrepareMarkup::With(html! {
            a
                id=[self.id()]
                class=[self.classes().get()]
                href=[self.href().get()]
                target=[target]
            {
                (self.left_icon().prepare(cx))
                " " span { (self.html().escaped(cx.langid())) } " "
                (self.right_icon().prepare(cx))
            }
        })
    }
}

impl Anchor {
    pub fn link(href: impl Into<String>, html: L10n) -> Self {
        Anchor::new().with_href(href).with_html(html)
    }

    pub fn button(href: impl Into<String>, html: L10n) -> Self {
        Anchor::new()
            .with_type(AnchorType::Button)
            .with_href(href)
            .with_html(html)
    }

    pub fn location(id: impl Into<String>) -> Self {
        Anchor::new().with_type(AnchorType::Location).with_id(id)
    }

    // Anchor BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_font_size(&mut self, font_size: FontSize) -> &mut Self {
        self.classes.alter_value(
            ClassesOp::Replace(self.font_size.to_string()),
            font_size.to_string(),
        );
        self.font_size = font_size;
        self
    }

    #[fn_builder]
    pub fn alter_type(&mut self, anchor_type: AnchorType) -> &mut Self {
        self.classes.alter_value(
            ClassesOp::Replace(self.anchor_type.to_string()),
            anchor_type.to_string(),
        );
        self.anchor_type = anchor_type;
        self
    }

    #[fn_builder]
    pub fn alter_href(&mut self, href: impl Into<String>) -> &mut Self {
        self.href.alter_value(href);
        self
    }

    #[fn_builder]
    pub fn alter_html(&mut self, html: L10n) -> &mut Self {
        self.html.alter_value(html);
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

    // Anchor GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn font_size(&self) -> &FontSize {
        &self.font_size
    }

    pub fn anchor_type(&self) -> &AnchorType {
        &self.anchor_type
    }

    pub fn href(&self) -> &OptionString {
        &self.href
    }

    pub fn html(&self) -> &OptionTranslated {
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
}
