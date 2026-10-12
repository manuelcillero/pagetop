use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, Expr, Fields, ItemStruct, LitStr, Meta};

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    match try_expand(attr, item) {
        Ok(output) => output,
        Err(error) => error.to_compile_error(),
    }
}

fn try_expand(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let mut section: Option<LitStr> = None;
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("section") {
            let name: LitStr = meta.value()?.parse()?;
            if name.value().trim().is_empty() {
                return Err(syn::Error::new_spanned(
                    &name,
                    "the section name cannot be empty",
                ));
            }
            section = Some(name);
            Ok(())
        } else {
            Err(meta.error("expected `section = \"...\"`"))
        }
    });
    syn::parse::Parser::parse2(parser, attr)?;

    let mut item: ItemStruct = syn::parse2(item)?;
    if !item.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &item.generics,
            "`#[settings]` does not support generic structs",
        ));
    }
    let Fields::Named(fields) = &mut item.fields else {
        return Err(syn::Error::new_spanned(
            &item.ident,
            "`#[settings]` requires a struct with named fields",
        ));
    };

    for field in &mut fields.named {
        if let Some(shown) = default_value_doc(&field.attrs)? {
            let has_doc = field.attrs.iter().any(|a| a.path().is_ident("doc"));
            let line = format!(" Por defecto: `{shown}`.");
            if has_doc {
                field.attrs.push(syn::parse_quote!(#[doc = ""]));
            }
            field.attrs.push(syn::parse_quote!(#[doc = #line]));
        }
    }

    let ident = &item.ident;
    let section_impl = section.map(|section| {
        quote! {
            impl ::pagetop::config::SettingsSection for #ident {
                const SECTION: &'static str = #section;
            }
        }
    });

    Ok(quote! {
        #[derive(::pagetop::AutoDefault, ::core::fmt::Debug, ::pagetop::serde::Deserialize)]
        #[serde(crate = "::pagetop::serde", default, deny_unknown_fields)]
        #item
        #section_impl
    })
}

// Texto con el valor por defecto de un campo para su documentación, si tiene `#[default = ...]` o
// `#[default(...)]`. De una ruta (p. ej. `LogRolling::Stdout`) se muestra sólo el último segmento,
// que es el valor que se escribe en el archivo de configuración.
fn default_value_doc(attrs: &[Attribute]) -> syn::Result<Option<String>> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("default")) else {
        return Ok(None);
    };
    let expr: Expr = match &attr.meta {
        Meta::NameValue(name_value) => name_value.value.clone(),
        Meta::List(list) => match list.parse_args::<Expr>() {
            Ok(expr) => expr,
            // Formas que `AutoDefault` admite pero que no son una expresión (p. ej. `_code = ...`).
            Err(_) => return Ok(None),
        },
        Meta::Path(_) => return Ok(None),
    };
    let shown = match &expr {
        Expr::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or_default(),
        other => compact(&other.to_token_stream().to_string()),
    };
    Ok(Some(shown))
}

// Quita los espacios que `quote` deja entre los elementos de una expresión (p. ej.
// `UnitValue :: Px (1440)` -> `UnitValue::Px(1440)`, o `- 1` -> `-1`).
fn compact(tokens: &str) -> String {
    let compacted = tokens
        .replace(" :: ", "::")
        .replace(" (", "(")
        .replace("( ", "(")
        .replace(" )", ")")
        .replace(" ,", ",");
    match compacted.strip_prefix("- ") {
        Some(rest) => format!("-{rest}"),
        None => compacted,
    }
}
