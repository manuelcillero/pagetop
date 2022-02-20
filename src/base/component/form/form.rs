use crate::prelude::*;

pub enum FormMethod {Get, Post}

pub struct Form {
    renderable: fn() -> bool,
    weight    : i8,
    id        : Option<String>,
    action    : Option<String>,
    method    : FormMethod,
    charset   : Option<String>,
    elements  : PageContainer,
    template  : String,
}

impl PageComponent for Form {

    fn prepare() -> Self {
        Form {
            renderable: always,
            weight    : 0,
            id        : None,
            action    : None,
            method    : FormMethod::Post,
            charset   : Some("UTF-8".to_string()),
            elements  : PageContainer::new(),
            template  : "default".to_string(),
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
            FormMethod::Post => Some("post".to_string())
        };
        html! {
            form
                id=[&self.id]
                action=[&self.action]
                method=[method]
                accept-charset=[&self.charset]
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
        self.id = if id.is_empty() {
            None
        } else {
            Some(id.replace(" ", "_"))
        };
        self
    }

    pub fn with_action(mut self, action: &str) -> Self {
        self.action = if action.is_empty() {
            None
        } else {
            Some(action.to_string())
        };
        self
    }

    pub fn with_method(mut self, method: FormMethod) -> Self {
        self.method = method;
        self
    }

    pub fn with_charset(mut self, charset: &str) -> Self {
        self.charset = if charset.is_empty() {
            None
        } else {
            Some(charset.to_string())
        };
        self
    }

    pub fn add(mut self, element: impl PageComponent) -> Self {
        self.elements.add(element);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_string();
        self
    }

    // Form GETTERS.

    pub fn id(&self) -> &str {
        match &self.id {
            Some(id) => id.as_str(),
            _ => ""
        }
    }

    pub fn action(&self) -> &str {
        match &self.action {
            Some(action) => action.as_str(),
            _ => ""
        }
    }

    pub fn method(&self) -> &str {
        match &self.method {
            FormMethod::Get => "get",
            FormMethod::Post => "post"
        }
    }

    pub fn charset(&self) -> &str {
        match &self.charset {
            Some(charset) => charset.as_str(),
            _ => ""
        }
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
