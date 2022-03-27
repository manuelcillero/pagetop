use crate::prelude::*;

pub enum FormMethod {Get, Post}

pub struct Form {
    renderable: fn() -> bool,
    weight    : i8,
    id        : OptionId,
    action    : OptionAttr,
    method    : FormMethod,
    charset   : OptionAttr,
    elements  : PageContainer,
    template  : String,
}

impl PageComponent for Form {

    fn new() -> Self {
        Form {
            renderable: always,
            weight    : 0,
            id        : OptionId::none(),
            action    : OptionAttr::none(),
            method    : FormMethod::Post,
            charset   : OptionAttr::some("UTF-8"),
            elements  : PageContainer::new(),
            template  : "default".to_owned(),
        }
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, assets: &mut PageAssets) -> Markup {
        let method = match self.method {
            FormMethod::Get => None,
            FormMethod::Post => Some("post".to_owned())
        };
        html! {
            form
                id=[&self.id.option()]
                action=[&self.action.option()]
                method=[method]
                accept-charset=[&self.charset.option()]
            {
                div {
                    (self.elements.render(assets))
                }
            }
        }
    }
}

impl Form {

    // Form BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id.with_value(id);
        self
    }

    pub fn with_action(mut self, action: &str) -> Self {
        self.action.with_value(action);
        self
    }

    pub fn with_method(mut self, method: FormMethod) -> Self {
        self.method = method;
        self
    }

    pub fn with_charset(mut self, charset: &str) -> Self {
        self.charset.with_value(charset);
        self
    }

    pub fn add(mut self, element: impl PageComponent) -> Self {
        self.elements.add(element);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Form GETTERS.

    pub fn id(&self) -> &str {
        self.id.value()
    }

    pub fn action(&self) -> &str {
        self.action.value()
    }

    pub fn method(&self) -> &str {
        match &self.method {
            FormMethod::Get => "get",
            FormMethod::Post => "post"
        }
    }

    pub fn charset(&self) -> &str {
        self.charset.value()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
