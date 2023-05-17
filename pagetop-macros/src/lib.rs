mod maud;

use concat_string::concat_string;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, parse_str, ItemFn};

#[proc_macro]
#[proc_macro_error]
pub fn html(input: TokenStream) -> TokenStream {
    maud::expand(input.into()).into()
}

#[proc_macro_attribute]
pub fn fn_builder(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
        .map(|arg| arg.split_whitespace().next().unwrap().to_string())
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
        #fn_alter
    };
    expanded.into()
}
