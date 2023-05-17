// #![doc(html_root_url = "https://docs.rs/maud_macros/0.25.0")]
// TokenStream values are reference counted, and the mental overhead of tracking
// lifetimes outweighs the marginal gains from explicit borrowing
// #![allow(clippy::needless_pass_by_value)]

mod ast;
mod escape;
mod generate;
mod parse;

use proc_macro_crate::{crate_name, FoundCrate};
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::quote;

pub fn expand(input: TokenStream) -> TokenStream {
    let output_ident = TokenTree::Ident(Ident::new("__maud_output", Span::mixed_site()));
    // Heuristic: the size of the resulting markup tends to correlate with the
    // code size of the template itself
    let size_hint = input.to_string().len();
    let markups = parse::parse(input);
    let stmts = generate::generate(markups, output_ident.clone());

    let found_crate = crate_name("pagetop").expect("pagetop is present in `Cargo.toml`");
    let pre_escaped = match found_crate {
        FoundCrate::Itself => quote!(
            crate::html::PreEscaped(#output_ident)
        ),
        _ => quote!(
            pagetop::html::PreEscaped(#output_ident)
        )
    };

    quote!({
        extern crate alloc;
        let mut #output_ident = alloc::string::String::with_capacity(#size_hint);
        #stmts
        #pre_escaped
    })
}
