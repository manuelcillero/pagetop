mod maud;
mod smart_default;

use concat_string::concat_string;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, parse_str, DeriveInput, ItemFn};

#[proc_macro]
#[proc_macro_error]
pub fn html(input: TokenStream) -> TokenStream {
    maud::expand(input.into()).into()
}

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
        fn_with_name.to_owned()
    } else {
        let g = &fn_alter.sig.generics;
        concat_string!(fn_with_name, quote! { #g }.to_string())
    };

    let where_clause = fn_alter
        .sig
        .generics
        .where_clause
        .as_ref()
        .map_or(String::new(), |where_clause| {
            concat_string!(quote! { #where_clause }.to_string(), " ")
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
    let fn_with = parse_str::<ItemFn>(concat_string!("
        pub fn ", fn_with_generics, "(mut self, ", args.join(", "), ") -> Self ", where_clause, "{
            self.", fn_alter_name, "(", params.join(", "), ");
            self
        }
    ").as_str()).unwrap();

    #[rustfmt::skip]
    let fn_alter_doc = concat_string!(
        "<p id=\"method.", fn_with_name, "\">",
        "Use <code class=\"code-header\"> <span class=\"fn\" href=\"#method.", fn_with_name, "\">",
        fn_with_name,
        "</span>(self, …) -> Self </code> to apply the <a href=\"#method.new\">builder pattern</a>.",
        "</p>"
    );

    let expanded = quote! {
        #[doc(hidden)]
        #fn_with
        #[inline]
        #[doc = #fn_alter_doc]
        #fn_alter
    };
    expanded.into()
}

/// Marks async main function as the PageTop entry-point.
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

/// Marks async test functions to use the PageTop entry-point.
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

    let fn_alter_doc = concat_string!(
        "<p id=\"method.with_classes\">",
        "Use <code class=\"code-header\">",
        " <span class=\"fn\" href=\"#method.with_classes\">with_classes</span>(self, …) -> Self ",
        "</code> to apply the <a href=\"#method.new\">builder pattern</a>.",
        "</p>"
    );

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
