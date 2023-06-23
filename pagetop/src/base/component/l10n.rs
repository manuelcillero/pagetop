use crate::prelude::*;

use std::collections::HashMap;

use_handle!(COMPONENT_L10N);

#[derive(Default)]
pub enum L10nOp {
    #[default]
    None,
    Text(&'static str),
    Translated(&'static str, &'static Locales),
    Escaped(&'static str, &'static Locales),
}

#[derive(Default)]
pub struct L10n {
    op: L10nOp,
    args: HashMap<&'static str, String>,
}

impl ComponentTrait for L10n {
    fn new() -> Self {
        L10n::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_L10N
    }

    fn prepare_component(&self, rcx: &mut RenderContext) -> Markup {
        match self.op() {
            L10nOp::None => html! {},
            L10nOp::Text(text) => html! { (text) },
            L10nOp::Translated(key, locales) => html! {
                (locales
                    .lookup_with_args(
                        rcx.langid(),
                        key,
                        &self.args().iter().fold(HashMap::new(), |mut args, (key, value)| {
                            args.insert(key.to_string(), value.to_owned().into());
                            args
                        })
                    )
                    .unwrap_or(key.to_string())
                )
            },
            L10nOp::Escaped(key, locales) => html! {
                (PreEscaped(locales
                    .lookup_with_args(
                        rcx.langid(),
                        key,
                        &self.args().iter().fold(HashMap::new(), |mut args, (key, value)| {
                            args.insert(key.to_string(), value.to_owned().into());
                            args
                        })
                    )
                    .unwrap_or(key.to_string())
                ))
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

impl L10n {
    pub fn n(text: &'static str) -> Self {
        L10n {
            op: L10nOp::Text(text),
            ..Default::default()
        }
    }

    pub fn t(key: &'static str, locales: &'static Locales) -> Self {
        L10n {
            op: L10nOp::Translated(key, locales),
            ..Default::default()
        }
    }

    pub fn e(key: &'static str, locales: &'static Locales) -> Self {
        L10n {
            op: L10nOp::Escaped(key, locales),
            ..Default::default()
        }
    }

    // L10n BUILDER.

    #[fn_builder]
    pub fn alter_op(&mut self, op: L10nOp) -> &mut Self {
        self.op = op;
        self
    }

    #[fn_builder]
    pub fn alter_arg(&mut self, arg: &'static str, value: String) -> &mut Self {
        self.args.insert(arg, value);
        self
    }

    pub fn clear_args(&mut self) -> &mut Self {
        self.args.drain();
        self
    }

    // L10n GETTERS.

    pub fn op(&self) -> &L10nOp {
        &self.op
    }

    pub fn args(&self) -> &HashMap<&str, String> {
        &self.args
    }
}
