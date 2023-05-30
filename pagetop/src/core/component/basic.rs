use crate::core::component::{AnyComponent, ComponentTrait, RenderContext};
use crate::html::{html, Markup, PreEscaped};
use crate::locale::{translate, Locale, Locales};
use crate::{define_handle, fn_builder, paste, Handle};

use std::collections::HashMap;

macro_rules! basic_components {
    ( $($COMPONENT_HANDLE:ident: $Component:ty => $TypeValue:ty),* ) => { $( paste! {

        define_handle!($COMPONENT_HANDLE);

        pub enum [< $Component Op >] {
            None,
            Value($TypeValue),
            Translated(&'static str, &'static Locales),
            Escaped(&'static str, &'static Locales),
        }

        pub struct $Component {
            op: [< $Component Op >],
            args: HashMap<&'static str, String>,
        }

        impl Default for $Component {
            fn default() -> Self {
                $Component {
                    op: [< $Component Op >]::None,
                    args: HashMap::new(),
                }
            }
        }

        impl ComponentTrait for $Component {
            fn new() -> Self {
                $Component::default()
            }

            fn handle(&self) -> Handle {
                $COMPONENT_HANDLE
            }

            fn default_render(&self, rcx: &mut RenderContext) -> Markup {
                match self.op() {
                    [< $Component Op >]::None => html! {},
                    [< $Component Op >]::Value(value) => html! { (value) },
                    [< $Component Op >]::Translated(key, locales) => html! {
                        (translate(
                            key,
                            Locale::Using(
                                rcx.langid(),
                                locales,
                                &self.args().iter().fold(HashMap::new(), |mut args, (key, value)| {
                                    args.insert(key.to_string(), value.to_owned().into());
                                    args
                                })
                            )
                        ))
                    },
                    [< $Component Op >]::Escaped(key, locales) => html! {
                        (PreEscaped(translate(
                            key,
                            Locale::Using(
                                rcx.langid(),
                                locales,
                                &self.args().iter().fold(HashMap::new(), |mut args, (key, value)| {
                                    args.insert(key.to_string(), value.to_owned().into());
                                    args
                                })
                            )
                        )))
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

        impl $Component {
            pub fn n(value: $TypeValue) -> Self {
                $Component {
                    op: [< $Component Op >]::Value(value),
                    ..Default::default()
                }
            }

            pub fn t(key: &'static str, locales: &'static Locales) -> Self {
                $Component {
                    op: [< $Component Op >]::Translated(key, locales),
                    ..Default::default()
                }
            }

            pub fn e(key: &'static str, locales: &'static Locales) -> Self {
                $Component {
                    op: [< $Component Op >]::Escaped(key, locales),
                    ..Default::default()
                }
            }

            // $Component BUILDER.

            #[fn_builder]
            pub fn alter_op(&mut self, op: [< $Component Op >]) -> &mut Self {
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

            // $Component GETTERS.

            pub fn op(&self) -> &[< $Component Op >] {
                &self.op
            }

            pub fn args(&self) -> &HashMap<&str, String> {
                &self.args
            }
        }

    } )* };
}

basic_components!(
    COMPONENT_HTML: Html => Markup,
    COMPONENT_TEXT: Text => &'static str
);
