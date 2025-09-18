/*!
<div align="center">

<h1>PageTop Macros</h1>

<p>Una colección de macros que mejoran la experiencia de desarrollo con <strong>PageTop</strong>.</p>

[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-licencia)
[![Doc API](https://img.shields.io/docsrs/pagetop-macros?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-macros)
[![Crates.io](https://img.shields.io/crates/v/pagetop-macros.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-macros)
[![Descargas](https://img.shields.io/crates/d/pagetop-macros.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-macros)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Créditos

Esta librería incluye entre sus macros una adaptación de
[maud-macros](https://crates.io/crates/maud_macros)
([0.27.0](https://github.com/lambda-fairy/maud/tree/v0.27.0/maud_macros)) de
[Chris Wong](https://crates.io/users/lambda-fairy) y una versión renombrada de
[SmartDefault](https://crates.io/crates/smart_default) (0.7.1) de
[Jane Doe](https://crates.io/users/jane-doe), llamada `AutoDefault`. Estas macros eliminan la
necesidad de referenciar `maud` o `smart_default` en las dependencias del archivo `Cargo.toml` de
cada proyecto PageTop.
*/

#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/favicon.ico"
)]

mod maud;
mod smart_default;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

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

/// Macro (*attribute*) que asocia un método *builder* `with_` con un método `alter_`.
///
/// La macro añade automáticamente un método `alter_` para modificar la instancia actual usando
/// `&mut self`, y redefine el método *builder* `with_`, que consume la instancia (`mut self`), para
/// delegar la lógica de la modificación al nuevo método `alter_`, reutilizando así la misma
/// implementación.
///
/// Esta macro emitirá un error en tiempo de compilación si la función anotada no cumple con la
/// firma esperada para el método *builder*: `pub fn with_...(mut self, ...) -> Self`.
///
/// # Ejemplos
///
/// Si defines un método `with_` como este:
///
/// ```rust,ignore
/// #[builder_fn]
/// pub fn with_example(mut self, value: impl Into<String>) -> Self {
///     self.value = Some(value.into());
///     self
/// }
/// ```
///
/// la macro generará automáticamente el siguiente método `alter_`:
///
/// ```rust,ignore
/// pub fn alter_example(&mut self, value: impl Into<String>) -> &mut Self {
///     self.value = Some(value.into());
///     self
/// }
/// ```
///
/// y reescribirá el método `with_` para delegar la modificación al método `alter_`:
///
/// ```rust,ignore
/// pub fn with_example(mut self, value: impl Into<String>) -> Self {
///     self.alter_example(value);
///     self
/// }
/// ```
///
/// Así, cada método *builder* `with_...()` generará automáticamente su correspondiente método
/// `alter_...()`, que permitirá más adelante modificar instancias existentes.
#[proc_macro_attribute]
pub fn builder_fn(_: TokenStream, item: TokenStream) -> TokenStream {
    use syn::{parse2, FnArg, Ident, ImplItemFn, Pat, ReturnType, TraitItemFn, Type};

    let ts: proc_macro2::TokenStream = item.clone().into();

    enum Kind {
        Impl(ImplItemFn),
        Trait(TraitItemFn),
    }

    // Detecta si estamos en `impl` o `trait`.
    let kind = if let Ok(it) = parse2::<ImplItemFn>(ts.clone()) {
        Kind::Impl(it)
    } else if let Ok(tt) = parse2::<TraitItemFn>(ts.clone()) {
        Kind::Trait(tt)
    } else {
        return quote! {
            compile_error!("#[builder_fn] only supports methods in `impl` blocks or `trait` items");
        }
        .into();
    };

    // Extrae piezas comunes (sig, attrs, vis, bloque?, es_trait?).
    let (sig, attrs, vis, body_opt, is_trait) = match &kind {
        Kind::Impl(m) => (&m.sig, &m.attrs, Some(&m.vis), Some(&m.block), false),
        Kind::Trait(t) => (&t.sig, &t.attrs, None, t.default.as_ref(), true),
    };

    let with_name = sig.ident.clone();
    let with_name_str = sig.ident.to_string();

    // Valida el nombre del método.
    if !with_name_str.starts_with("with_") {
        return quote_spanned! {
            sig.ident.span() => compile_error!("expected a named `with_...()` method");
        }
        .into();
    }

    // Sólo se exige `pub` en `impl` (en `trait` no aplica).
    let vis_pub = match (is_trait, vis) {
        (false, Some(v)) => quote! { #v },
        _ => quote! {},
    };

    // Validaciones comunes.
    if sig.asyncness.is_some() {
        return quote_spanned! {
            sig.asyncness.span() => compile_error!("`with_...()` cannot be `async`");
        }
        .into();
    }
    if sig.constness.is_some() {
        return quote_spanned! {
            sig.constness.span() => compile_error!("`with_...()` cannot be `const`");
        }
        .into();
    }
    if sig.abi.is_some() {
        return quote_spanned! {
            sig.abi.span() => compile_error!("`with_...()` cannot be `extern`");
        }
        .into();
    }
    if sig.unsafety.is_some() {
        return quote_spanned! {
            sig.unsafety.span() => compile_error!("`with_...()` cannot be `unsafe`");
        }
        .into();
    }

    // En `impl` se exige exactamente `mut self`; y en `trait` se exige `self` (sin &).
    let receiver_ok = match sig.inputs.first() {
        Some(FnArg::Receiver(r)) => {
            // Rechaza `self: SomeType`.
            if r.colon_token.is_some() {
                false
            } else if is_trait {
                // Exactamente `self` (sin &, sin mut).
                r.reference.is_none() && r.mutability.is_none()
            } else {
                // Exactamente `mut self`.
                r.reference.is_none() && r.mutability.is_some()
            }
        }
        _ => false,
    };
    if !receiver_ok {
        let msg = if is_trait {
            "expected `self` (not `mut self`, `&self` or `&mut self`) in trait method"
        } else {
            "expected first argument to be exactly `mut self`"
        };
        let err = sig
            .inputs
            .first()
            .map(|a| a.span())
            .unwrap_or(sig.ident.span());
        return quote_spanned! {
            err => compile_error!(#msg);
        }
        .into();
    }

    // Valida que el método devuelve exactamente `Self`.
    match &sig.output {
        ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Path(p) if p.qself.is_none() && p.path.is_ident("Self") => {}
            _ => {
                return quote_spanned! {
                    ty.span() => compile_error!("expected return type to be exactly `Self`");
                }
                .into();
            }
        },
        _ => {
            return quote_spanned! {
                sig.output.span() => compile_error!("expected return type to be exactly `Self`");
            }
            .into();
        }
    }

    // Genera el nombre del método alter_...().
    let stem = with_name_str.strip_prefix("with_").expect("validated");
    let alter_ident = Ident::new(&format!("alter_{stem}"), with_name.span());

    // Extrae genéricos y cláusulas where.
    let generics = &sig.generics;
    let where_clause = &sig.generics.where_clause;

    // Extrae identificadores de los argumentos para la llamada (sin `mut` ni patrones complejos).
    let args: Vec<_> = sig.inputs.iter().skip(1).collect();
    let call_idents: Vec<Ident> = {
        let mut v = Vec::new();
        for arg in sig.inputs.iter().skip(1) {
            match arg {
                FnArg::Typed(pat) => {
                    if let Pat::Ident(pat_ident) = pat.pat.as_ref() {
                        v.push(pat_ident.ident.clone());
                    } else {
                        return quote_spanned! {
                            pat.pat.span() => compile_error!(
                                "each parameter must be a simple identifier, e.g. `value: T`"
                            );
                        }
                        .into();
                    }
                }
                _ => {
                    return quote_spanned! {
                        arg.span() => compile_error!("unexpected receiver in parameter list");
                    }
                    .into();
                }
            }
        }
        v
    };

    // Extrae atributos descartando la documentación para incluir en `alter_...()`.
    let non_doc_attrs: Vec<_> = attrs
        .iter()
        .filter(|&a| !a.path().is_ident("doc"))
        .cloned()
        .collect();

    // Documentación del método alter_...().
    let alter_doc =
        format!("Equivalente a [`Self::{with_name_str}()`], pero fuera del patrón *builder*.");

    // Genera el código final.
    let expanded = match body_opt {
        None => {
            quote! {
                #(#attrs)*
                fn #with_name #generics (self, #(#args),*) -> Self #where_clause;

                #(#non_doc_attrs)*
                #[doc = #alter_doc]
                fn #alter_ident #generics (&mut self, #(#args),*) -> &mut Self #where_clause;
            }
        }
        Some(body) => {
            let with_fn = if is_trait {
                quote! {
                    #vis_pub fn #with_name #generics (self, #(#args),*) -> Self #where_clause {
                        let mut s = self;
                        s.#alter_ident(#(#call_idents),*);
                        s
                    }
                }
            } else {
                quote! {
                    #vis_pub fn #with_name #generics (mut self, #(#args),*) -> Self #where_clause {
                        self.#alter_ident(#(#call_idents),*);
                        self
                    }
                }
            };
            quote! {
                #(#attrs)*
                #with_fn

                #(#non_doc_attrs)*
                #[doc = #alter_doc]
                #vis_pub fn #alter_ident #generics (&mut self, #(#args),*) -> &mut Self #where_clause {
                    #body
                }
            }
        }
    };
    expanded.into()
}

/// Define una función `main` asíncrona como punto de entrada de PageTop.
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

/// Define funciones de prueba asíncronas para usar con PageTop.
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
