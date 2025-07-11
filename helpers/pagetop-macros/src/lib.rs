//! <div align="center">
//!
//! <h1>PageTop Macros</h1>
//!
//! <p>Una colección de macros que mejoran la experiencia de desarrollo con <strong>PageTop</strong>.</p>
//!
//! [![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
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
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Macro para escribir plantillas HTML (basada en [Maud](https://docs.rs/maud)).
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    maud::expand(input.into()).into()
}

/// Deriva [`Default`] con atributos personalizados (basada en
/// [SmartDefault](https://docs.rs/smart-default)).
///
/// Al derivar una estructura con *AutoDefault* se genera automáticamente la implementación de
/// [`Default`]. Aunque, a diferencia de un simple `#[derive(Default)]`, el atributo
/// `#[derive(AutoDefault)]` permite usar anotaciones en los campos como `#[default = "..."]`,
/// funcionando incluso en estructuras con campos que no implementan [`Default`] o en *enums*.
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
/// # Ejemplo
///
/// ```rust,ignore
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
/// # Ejemplo
///
/// ```rust,ignore
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
