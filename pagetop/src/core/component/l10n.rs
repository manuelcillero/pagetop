use crate::core::component::{AnyComponent, ComponentTrait, RenderContext};
use crate::html::{html, Markup, PreEscaped};
use crate::locale::{translate, Locale, Locales};
use crate::{define_handle, fn_builder, Handle};

use std::collections::HashMap;

define_handle!(COMPONENT_L10N);

#[rustfmt::skip]
#[derive(Default)]
pub struct L10n {
    key    : &'static str,
    locales: Option<&'static Locales>,
    args   : HashMap<&'static str, String>,
    escaped: bool,
}

impl ComponentTrait for L10n {
    fn new() -> Self {
        L10n::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_L10N
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        if let Some(locales) = self.locales() {
            html! {
                @if self.escaped() {
                    (PreEscaped(translate(
                        self.key(),
                        Locale::Using(
                            rcx.language(),
                            locales,
                            &self.args().iter().fold(HashMap::new(), |mut args, (key, value)| {
                                args.insert(key.to_string(), value.to_owned().into());
                                args
                            })
                        )
                    )))
                } @else {
                    (translate(
                        self.key(),
                        Locale::Using(
                            rcx.language(),
                            locales,
                            &self.args().iter().fold(HashMap::new(), |mut args, (key, value)| {
                                args.insert(key.to_string(), value.to_owned().into());
                                args
                            })
                        )
                    ))
                }
            }
        } else {
            html! { (self.key()) }
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
            key: text,
            ..Default::default()
        }
    }

    pub fn t(key: &'static str, locales: &'static Locales) -> Self {
        L10n {
            key,
            locales: Some(locales),
            ..Default::default()
        }
    }

    pub fn e(key: &'static str, locales: &'static Locales) -> Self {
        L10n {
            key,
            locales: Some(locales),
            escaped: true,
            ..Default::default()
        }
    }

    // HtmL10n BUILDER.

    #[fn_builder]
    pub fn alter_key(&mut self, key: &'static str) -> &mut Self {
        self.key = key;
        self
    }

    #[fn_builder]
    pub fn alter_locales(&mut self, locales: &'static Locales) -> &mut Self {
        self.locales = Some(locales);
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

    // HtmL10n GETTERS.

    pub fn key(&self) -> &str {
        self.key
    }

    pub fn locales(&self) -> Option<&Locales> {
        self.locales
    }

    pub fn args(&self) -> &HashMap<&str, String> {
        &self.args
    }

    pub fn escaped(&self) -> bool {
        self.escaped
    }
}
