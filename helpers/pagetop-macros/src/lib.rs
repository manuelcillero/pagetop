mod maud;
mod smart_default;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, parse_str, DeriveInput, ItemFn};

/// Macro attribute to generate builder methods from `set_` methods.
///
/// This macro takes a method with the `set_` prefix and generates a corresponding method with the
/// `with_` prefix to use in the builder pattern.
///
/// # Panics
///
/// This function will panic if a parameter identifier is not found in the argument list.
///
/// # Examples
///
/// ```
/// #[fn_builder]
/// pub fn set_example(&mut self) -> &mut Self {
///     // implementation
/// }
/// ```
///
/// Will generate:
///
/// ```
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
    let fn_with = parse_str::<ItemFn>(format!(r#"
        pub fn {fn_with_generics}(mut self, {}) -> Self {where_clause} {{
            self.{fn_set_name}({});
            self
        }}
        "#, args.join(", "), params.join(", ")
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

/// Marks async main function as the `PageTop` entry-point.
///
/// # Examples
/// ```
/// #[pagetop::main]
/// async fn main() {
///     async { println!("Hello world"); }.await
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

/// Marks async test functions to use the `PageTop` entry-point.
///
/// # Examples
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
