use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, PreEscaped, PrepareMarkup};
use crate::locale::{Loader, Locales};
use crate::{create_handle, fn_builder, Handle};

use std::collections::HashMap;

create_handle!(COMPONENT_L10N);

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

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.op() {
            L10nOp::None => PrepareMarkup::None,
            L10nOp::Text(text) => PrepareMarkup::Text(text),
            L10nOp::Translated(key, locales) => PrepareMarkup::With(html! {
                (locales
                    .lookup_with_args(
                        cx.langid(),
                        key,
                        &self.args().iter().fold(HashMap::new(), |mut args, (key, value)| {
                            args.insert(key.to_string(), value.to_owned().into());
                            args
                        })
                    )
                    .unwrap_or(key.to_string())
                )
            }),
            L10nOp::Escaped(key, locales) => PrepareMarkup::With(html! {
                (PreEscaped(locales
                    .lookup_with_args(
                        cx.langid(),
                        key,
                        &self.args().iter().fold(HashMap::new(), |mut args, (key, value)| {
                            args.insert(key.to_string(), value.to_owned().into());
                            args
                        })
                    )
                    .unwrap_or(key.to_string())
                ))
            }),
        }
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
    pub fn alter_arg(&mut self, arg: &'static str, value: impl Into<String>) -> &mut Self {
        self.args.insert(arg, value.into());
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

    pub fn into_string(&self, cx: &mut Context) -> Option<String> {
        self.prepare_component(cx).into_string()
    }
}
