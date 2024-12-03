//! Una colección de macros que impulsan el desarrollo con **PageTop**.

mod maud;
mod smart_default;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, parse_str, DeriveInput, ItemFn};

/// Macro (*attribute*) que asocia a un método `set_` su correspondiente método `with_` para aplicar
/// el patrón *builder*.
///
/// # Panics
///
/// Esta función provocará un *panic* si no encuentra identificadores en la lista de argumentos.
///
/// # Ejemplos
///
/// ```
/// #[fn_builder]
/// pub fn set_example(&mut self) -> &mut Self {
///     // implementación
/// }
/// ```
///
/// Añadirá al código el siguiente método:
///
/// ```
/// #[inline]
/// pub fn with_example(mut self) -> Self {
///     self.set_example();
///     self
/// }
/// ```
#[proc_macro_attribute]
pub fn fn_builder(_: TokenStream, item: TokenStream) -> TokenStream {
    let fn_set = parse_macro_input!(item as ItemFn);
    let fn_set_name = fn_set.sig.ident.to_string();

    if !fn_set_name.starts_with("set_") {
        let expanded = quote_spanned! {
            fn_set.sig.ident.span() =>
                compile_error!("expected a \"pub fn set_...() -> &mut Self\" method");
        };
        return expanded.into();
    }

    let fn_with_name = fn_set_name.replace("set_", "with_");
    let fn_with_generics = if fn_set.sig.generics.params.is_empty() {
        fn_with_name.clone()
    } else {
        let g = &fn_set.sig.generics;
        format!("{fn_with_name}{}", quote! { #g }.to_string())
    };

    let where_clause = fn_set
        .sig
        .generics
        .where_clause
        .as_ref()
        .map_or(String::new(), |where_clause| {
            format!("{} ", quote! { #where_clause }.to_string())
        });

    let args: Vec<String> = fn_set
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
            self.{fn_set_name}({});
            self
        }}
        "##, args.join(", "), params.join(", ")
    ).as_str()).unwrap();

    #[rustfmt::skip]
    let fn_set_doc = format!(r##"
        <p id="method.{fn_with_name}" style="margin-bottom: 12px;">Use
        <code class="code-header">pub fn <span class="fn" href="#method.{fn_with_name}">{fn_with_name}</span>(self, …) -> Self</code>
        for the <a href="#method.new">builder pattern</a>.
        </p>
    "##);

    let expanded = quote! {
        #[doc(hidden)]
        #fn_with
        #[inline]
        #[doc = #fn_set_doc]
        #fn_set
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
    let fn_set_doc = format!(r##"
        <p id="method.with_classes">Use
        <code class="code-header"><span class="fn" href="#method.with_classes">with_classes</span>(self, …) -> Self</code>
        to apply the <a href="#method.new">builder pattern</a>.
        </p>
    "##);

    let expanded = quote! {
        impl ComponentClasses for #name {
            #[inline]
            #[doc = #fn_set_doc]
            fn set_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
                self.classes.set_value(op, classes);
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
/// ```
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
/// ```
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
