mod maud;

use concat_string::concat_string;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, parse_str, DeriveInput, ItemFn};

#[proc_macro]
#[proc_macro_error]
pub fn html(input: TokenStream) -> TokenStream {
    maud::expand(input.into()).into()
}

#[proc_macro_attribute]
pub fn fn_with(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(item as ItemFn);
    let fn_name = fn_item.sig.ident.to_string();

    if !fn_name.starts_with("alter_") {
        let expanded = quote_spanned! {
            fn_item.sig.ident.span() =>
                compile_error!("expected a \"pub fn alter_...() -> &mut Self\" method");
        };
        return expanded.into();
    }

    let args: Vec<String> = fn_item
        .sig
        .inputs
        .iter()
        .skip(1)
        .map(|arg| arg.to_token_stream().to_string())
        .collect();

    let param: Vec<String> = args
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
        pub fn ", fn_name.replace("alter_", "with_"), "(mut self, ", args.join(", "), ") -> Self {
            self.", fn_name, "(", param.join(", "), ");
            self
        }
    ").as_str()).unwrap();

    let fn_alter = fn_item.into_token_stream();

    let expanded = quote! {
        #fn_with
        #[inline]
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

#[proc_macro_derive(ComponentClasses)]
pub fn component_classes_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ImplementClasses for #name {
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

#[proc_macro_derive(BaseHandle, attributes(handle))]
pub fn base_handle_derive(input: TokenStream) -> TokenStream {
    impl_handle(input, quote! { crate })
}

#[proc_macro_derive(BindHandle, attributes(handle))]
pub fn bind_handle_derive(input: TokenStream) -> TokenStream {
    impl_handle(input, quote! { pagetop })
}

fn impl_handle(input: TokenStream, crate_name: TokenStream2) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let name = &input.ident;
    let handle_name = format_ident!("HANDLE_{}", name.to_string().to_uppercase());

    let expanded = quote! {
        const #handle_name: #crate_name::Handle =
            #crate_name::util::handle(module_path!(), file!(), line!(), column!());

        impl #impl_generics #crate_name::ImplementHandle for #name #ty_generics #where_clause {
            #[inline]
            fn static_handle() -> #crate_name::Handle {
                #handle_name
            }

            #[inline]
            fn matches_handle(is: #crate_name::Handle) -> bool {
                is == #handle_name
            }

            #[inline]
            fn handle(&self) -> #crate_name::Handle {
                #handle_name
            }
        }
    };

    TokenStream::from(expanded)
}
