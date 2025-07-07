// #![doc(html_root_url = "https://docs.rs/maud_macros/0.27.0")]
// TokenStream values are reference counted, and the mental overhead of tracking
// lifetimes outweighs the marginal gains from explicit borrowing
// #![allow(clippy::needless_pass_by_value)]

mod ast;
mod escape;
mod generate;

use ast::DiagnosticParse;
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro2_diagnostics::Diagnostic;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::parse::{ParseStream, Parser};

pub fn expand(input: TokenStream) -> TokenStream {
    // Heuristic: the size of the resulting markup tends to correlate with the
    // code size of the template itself
    let size_hint = input.to_string().len();

    let mut diagnostics = Vec::new();
    let markups = match Parser::parse2(
        |input: ParseStream| ast::Markups::diagnostic_parse(input, &mut diagnostics),
        input,
    ) {
        Ok(data) => data,
        Err(err) => {
            let err = err.to_compile_error();
            let diag_tokens = diagnostics.into_iter().map(Diagnostic::emit_as_expr_tokens);

            return quote! {{
                #err
                #(#diag_tokens)*
            }};
        }
    };

    let diag_tokens = diagnostics.into_iter().map(Diagnostic::emit_as_expr_tokens);

    let output_ident = Ident::new("__maud_output", Span::mixed_site());
    let stmts = generate::generate(markups, output_ident.clone());

    let found_crate = crate_name("pagetop").expect("pagetop must be in Cargo.toml");
    let crate_ident = match found_crate {
        FoundCrate::Itself => Ident::new("pagetop", Span::call_site()),
        FoundCrate::Name(ref name) => Ident::new(name, Span::call_site()),
    };
    let pre_escaped = quote! {
        #crate_ident::html::PreEscaped(#output_ident)
    };

    quote! {{
        extern crate alloc;
        let mut #output_ident = alloc::string::String::with_capacity(#size_hint);
        #stmts
        #(#diag_tokens)*
        #pre_escaped
    }}
}
