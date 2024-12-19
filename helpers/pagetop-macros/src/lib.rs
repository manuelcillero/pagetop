//! <div align="center">
//!
//! <h1>PageTop Macros</h1>
//!
//! <p>Una colección de macros que mejoran la experiencia de desarrollo con <strong>PageTop</strong>.</p>
//!
//! [![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
//! [![Doc API](https://img.shields.io/docsrs/pagetop-macros?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-macros)
//! [![Crates.io](https://img.shields.io/crates/v/pagetop-macros.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-macros)
//! [![Descargas](https://img.shields.io/crates/d/pagetop-macros.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-macros)
//!
//! </div>
//!
//! # 📌 Sobre PageTop
//!
//! [`PageTop`](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la
//! web clásica combinando SSR (*renderizado en el servidor*), HTML, CSS y JS, para crear soluciones
//! web modulares, extensibles y configurables.

mod maud;
mod smart_default;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, parse_str, DeriveInput, ItemFn};

/// Macro (*attribute*) que asocia a un método `alter_` su correspondiente método `with_` para
/// aplicar el patrón *builder*.
///
/// # Panics
///
/// Esta función provocará un *panic* si no encuentra identificadores en la lista de argumentos.
///
/// # Ejemplos
///
/// ```rust#ignore
/// #[fn_builder]
/// pub fn alter_example(&mut self) -> &mut Self {
///     // implementación
/// }
/// ```
///
/// Añadirá al código el siguiente método:
///
/// ```rust#ignore
/// #[inline]
/// pub fn with_example(mut self) -> Self {
///     self.alter_example();
///     self
/// }
/// ```
#[proc_macro_attribute]
pub fn fn_builder(_: TokenStream, item: TokenStream) -> TokenStream {
    let fn_alter = parse_macro_input!(item as ItemFn);
    let fn_alter_name = fn_alter.sig.ident.to_string();

    if !fn_alter_name.starts_with("alter_") {
        let expanded = quote_spanned! {
            fn_alter.sig.ident.span() =>
                compile_error!("expected a \"pub fn alter_...() -> &mut Self\" method");
        };
        return expanded.into();
    }

    let fn_with_name = fn_alter_name.replace("alter_", "with_");
    let fn_with_generics = if fn_alter.sig.generics.params.is_empty() {
        fn_with_name.clone()
    } else {
        let g = &fn_alter.sig.generics;
        format!("{fn_with_name}{}", quote! { #g }.to_string())
    };

    let where_clause = fn_alter
        .sig
        .generics
        .where_clause
        .as_ref()
        .map_or(String::new(), |where_clause| {
            format!("{} ", quote! { #where_clause }.to_string())
        });

    let args: Vec<String> = fn_alter
        .sig
        .inputs
        .iter()
        .skip(1)
        .map(|arg| arg.to_token_stream().to_string())
        .collect();

    let params: Vec<String> = args
        .iter()
        .map(|arg| {
            arg.split_whitespace()
                .next()
                .unwrap()
                .trim_end_matches(':')
                .to_string()
        })
        .collect();

    #[rustfmt::skip]
    let fn_with = parse_str::<ItemFn>(format!(r##"
        pub fn {fn_with_generics}(mut self, {}) -> Self {where_clause} {{
            self.{fn_alter_name}({});
            self
        }}
        "##, args.join(", "), params.join(", ")
    ).as_str()).unwrap();

    #[rustfmt::skip]
    let fn_alter_doc = format!(r##"
        <p id="method.{fn_with_name}" style="margin-bottom: 12px;">Use
        <code class="code-header">pub fn <span class="fn" href="#method.{fn_with_name}">{fn_with_name}</span>(self, …) -> Self</code>
        for the <a href="#method.new">builder pattern</a>.
        </p>
    "##);

    let expanded = quote! {
        #[doc(hidden)]
        #fn_with
        #[inline]
        #[doc = #fn_alter_doc]
        #fn_alter
    };
    expanded.into()
}

#[proc_macro]
#[proc_macro_error]
pub fn html(input: TokenStream) -> TokenStream {
    maud::expand(input.into()).into()
}

#[proc_macro_derive(AutoDefault, attributes(default))]
pub fn derive_auto_default(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match smart_default::body_impl::impl_my_derive(&input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro_derive(ComponentClasses)]
pub fn derive_component_classes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    #[rustfmt::skip]
    let fn_alter_doc = format!(r##"
        <p id="method.with_classes">Use
        <code class="code-header"><span class="fn" href="#method.with_classes">with_classes</span>(self, …) -> Self</code>
        to apply the <a href="#method.new">builder pattern</a>.
        </p>
    "##);

    let expanded = quote! {
        impl ComponentClasses for #name {
            #[inline]
            #[doc = #fn_alter_doc]
            fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
                self.classes.alter_value(op, classes);
                self
            }

            fn classes(&self) -> &OptionClasses {
                &self.classes
            }
        }
    };

    TokenStream::from(expanded)
}

/// Define una función `main` asíncrona como punto de entrada de `PageTop`.
///
/// # Ejemplos
///
/// ```rust#ignore
/// #[pagetop::main]
/// async fn main() {
///     async { println!("Hello world!"); }.await
/// }
/// ```
#[proc_macro_attribute]
pub fn main(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut output: TokenStream = (quote! {
        #[::pagetop::service::rt::main(system = "::pagetop::service::rt::System")]
    })
    .into();

    output.extend(item);
    output
}

/// Define funciones de prueba asíncronas para usar con `PageTop`.
///
/// # Ejemplos
///
/// ```rust#ignore
/// #[pagetop::test]
/// async fn test() {
///     assert_eq!(async { "Hello world" }.await, "Hello world");
/// }
/// ```
#[proc_macro_attribute]
pub fn test(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut output: TokenStream = (quote! {
        #[::pagetop::service::rt::test(system = "::pagetop::service::rt::System")]
    })
    .into();

    output.extend(item);
    output
}
