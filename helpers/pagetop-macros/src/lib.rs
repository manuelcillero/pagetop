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
//! ## Sobre PageTop
//!
//! [PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la
//! web clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles
//! y configurables, basadas en HTML, CSS y JavaScript.

mod maud;
mod smart_default;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, ItemFn};

/// Macro (*attribute*) que asocia un método *builder* `with_` con un método `alter_` equivalente
/// que modifica la instancia actual usando una única implementación.
///
/// La macro genera automáticamente un método `alter_`, que modifica la instancia actual usando
/// `&mut self`, y redefine el método `with_` para delegar la lógica en el nuevo método `alter_`.
///
/// # Panics
///
/// Esta macro provocará un *panic* en tiempo de compilación si la función anotada no cumple con la
/// declaración `pub fn with_...(mut self, ...) -> Self`.
///
/// # Ejemplos
///
/// Si defines un método `with_` como este:
///
/// ```rust#ignore
/// #[fn_builder]
/// pub fn with_example(mut self, value: impl Into<String>) -> Self {
///     self.value = Some(value.into());
///     self
/// }
/// ```
///
/// La macro generará automáticamente el siguiente método `alter_`:
///
/// ```rust#ignore
/// pub fn alter_example(&mut self, value: impl Into<String>) -> &mut Self {
///     self.value = Some(value.into());
///     self
/// }
/// ```
///
/// Y redefinirá el método `with_` para que delegue en el método `alter_`:
///
/// ```rust#ignore
/// pub fn with_example(mut self, value: impl Into<String>) -> Self {
///     self.alter_example(value);
///     self
/// }
/// ```
///
/// Así, cada método *builder* `with_...()` generará automáticamente su correspondiente método
/// asociado `alter_...()`, permitiendo aplicar modificaciones en instancias existentes.
#[proc_macro_attribute]
pub fn fn_builder(_: TokenStream, item: TokenStream) -> TokenStream {
    let fn_with = parse_macro_input!(item as ItemFn);
    let fn_with_name = fn_with.sig.ident.clone();
    let fn_with_name_str = fn_with.sig.ident.to_string();

    // Valida el nombre del método.
    if !fn_with_name_str.starts_with("with_") {
        let expanded = quote_spanned! {
            fn_with.sig.ident.span() =>
                compile_error!("expected a \"pub fn with_...(mut self, ...) -> Self\" method");
        };
        return expanded.into();
    }

    // Valida que el primer argumento sea `mut self`.
    if let Some(syn::FnArg::Receiver(receiver)) = fn_with.sig.inputs.first() {
        if !receiver.mutability.is_some() {
            return quote_spanned! {
                receiver.span() => compile_error!("expected `mut self` as the first argument");
            }
            .into();
        }
    } else {
        return quote_spanned! {
            fn_with.sig.ident.span() => compile_error!("expected `mut self` as the first argument");
        }
        .into();
    }

    // Genera el nombre del método alter_...().
    let fn_alter_name_str = fn_with_name_str.replace("with_", "alter_");
    let fn_alter_name = syn::Ident::new(&fn_alter_name_str, fn_with.sig.ident.span());

    // Extrae genéricos y cláusulas where.
    let fn_generics = &fn_with.sig.generics;
    let where_clause = &fn_with.sig.generics.where_clause;

    // Extrae argumentos y parámetros de llamada.
    let args: Vec<_> = fn_with.sig.inputs.iter().skip(1).collect();
    let params: Vec<_> = fn_with
        .sig
        .inputs
        .iter()
        .skip(1)
        .map(|arg| match arg {
            syn::FnArg::Typed(pat) => &pat.pat,
            _ => panic!("unexpected argument type"),
        })
        .collect();

    // Extrae bloque del método.
    let fn_with_block = &fn_with.block;

    // Extrae documentación y otros atributos del método.
    let fn_with_attrs = &fn_with.attrs;

    // Genera el método alter_...() con el código del método with_...().
    let fn_alter_doc = format!(
        "Modifica la instancia en los mismos términos que para el patrón <em>builder</em> hace el \
        método asociado `{}()`.",
        fn_with_name_str,
    );
    let fn_alter = quote! {
        #[doc = #fn_alter_doc]
        pub fn #fn_alter_name #fn_generics(&mut self, #(#args),*) -> &mut Self #where_clause {
            #fn_with_block
        }
    };

    // Redefine el método with_...() para que llame a alter_...().
    let fn_with = quote! {
        #(#fn_with_attrs)*
        pub fn #fn_with_name #fn_generics(mut self, #(#args),*) -> Self #where_clause {
            self.#fn_alter_name(#(#params),*);
            self
        }
    };

    // Genera el código final.
    let expanded = quote! {
        #fn_with
        #[inline]
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
