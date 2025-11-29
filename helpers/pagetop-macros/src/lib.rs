/*!
<div align="center">

<h1>PageTop Macros</h1>

<p>Una colección de macros que mejoran la experiencia de desarrollo con <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-macros?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-macros)
[![Crates.io](https://img.shields.io/crates/v/pagetop-macros.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-macros)
[![Descargas](https://img.shields.io/crates/d/pagetop-macros.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-macros)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/helpers/pagetop-macros#licencia)

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
///
/// # Ejemplos
///
/// ```rust
/// # use pagetop_macros::AutoDefault;
/// # fn main() {
/// #[derive(AutoDefault)]
/// # #[derive(PartialEq)]
/// # #[allow(dead_code)]
/// enum Foo {
///     Bar,
///     #[default]
///     Baz {
///         #[default = 12]
///         a: i32,
///         b: i32,
///         #[default(Some(Default::default()))]
///         c: Option<i32>,
///         #[default(_code = "vec![1, 2, 3]")]
///         d: Vec<u32>,
///         #[default = "four"]
///         e: String,
///     },
///     Qux(i32),
/// }
///
/// assert!(Foo::default() == Foo::Baz {
///     a: 12,
///     b: 0,
///     c: Some(0),
///     d: vec![1, 2, 3],
///     e: "four".to_owned(),
/// });
/// # }
/// ```
///
/// * `Baz` tiene el atributo `#[default]`. Esto significa que el valor por defecto de `Foo` es
///   `Foo::Baz`. Solo una variante puede tener el atributo `#[default]`, y dicho atributo no debe
///   tener ningún valor asociado.
/// * `a` tiene el atributo `#[default = 12]`. Esto significa que su valor por defecto es `12`.
/// * `b` no tiene ningún atributo `#[default = ...]`. Su valor por defecto será, por tanto, el
///   valor por defecto de `i32`, es decir, `0`.
/// * `c` es un `Option<i32>`, y su valor por defecto es `Some(Default::default())`. Rust no puede
///   (actualmente) analizar `#[default = Some(Default::default())]`, pero podemos escribir
///   `#[default(Some(Default::default))]`.
/// * `d` contiene el token `!`, que (actualmente) no puede ser analizado ni siquiera usando
///   `#[default(...)]`, así que debemos codificarlo como una cadena y marcarlo con `_code =`.
/// * `e` es un `String`, por lo que el literal de cadena `"four"` se convierte automáticamente en
///   él. Esta conversión automática **solo** ocurre con literales de cadena (o de bytes), y solo si
///   no se usa `_code`.
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
/// La macro añade automáticamente un método `alter_` que permite modificar la instancia actual
/// usando `&mut self`; y redefine el método *builder* `with_`, que consume `mut self`, para delegar
/// la lógica al nuevo método `alter_`, reutilizando así la misma implementación.
///
/// Esta macro emitirá un error en tiempo de compilación si la función anotada no cumple con la
/// firma esperada para el método *builder*: `pub fn with_...(mut self, ...) -> Self`.
///
/// # Ejemplo
///
/// Si defines un método `with_` como este:
///
/// ```rust
/// # use pagetop_macros::builder_fn;
/// # struct Example {value: Option<String>};
/// # impl Example {
/// #[builder_fn]
/// pub fn with_example(mut self, value: impl Into<String>) -> Self {
///     self.value = Some(value.into());
///     self
/// }
/// # }
/// ```
///
/// la macro reescribirá el método `with_` y generará un nuevo método `alter_`:
///
/// ```rust
/// # struct Example {value: Option<String>};
/// # impl Example {
/// #[inline]
/// pub fn with_example(mut self, value: impl Into<String>) -> Self {
///     self.alter_example(value);
///     self
/// }
///
/// pub fn alter_example(&mut self, value: impl Into<String>) -> &mut Self {
///     self.value = Some(value.into());
///     self
/// }
/// # }
/// ```
///
/// De esta forma, cada método *builder* `with_...()` generará automáticamente su correspondiente
/// método `alter_...()` para modificar instancias existentes.
///
/// La documentación del método `with_...()` incluirá también la firma resumida del método
/// `alter_...()` y un alias de búsqueda con su nombre, de tal manera que buscando `alter_...` en la
/// documentación se mostrará la entrada del método `with_...()`.
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

    // Genera el nombre del método `alter_...()`.
    let stem = with_name_str.strip_prefix("with_").expect("validated");
    let alter_ident = Ident::new(&format!("alter_{stem}"), with_name.span());

    // Extrae genéricos y cláusulas `where`.
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

    // Separa atributos de documentación y resto.
    let mut doc_attrs = Vec::new();
    let mut other_attrs = Vec::new();
    let mut non_doc_or_inline_attrs = Vec::new();

    for a in attrs.iter() {
        let p = a.path();
        if p.is_ident("doc") {
            doc_attrs.push(a.clone());
        } else {
            other_attrs.push(a.clone());
            if !p.is_ident("inline") {
                non_doc_or_inline_attrs.push(a.clone());
            }
        }
    }

    // Firma resumida de la función `alter_...()` para mostrarla en la doc de `with_...()`.
    let alter_sig_tokens = if args.is_empty() {
        // Sin argumentos sólo se muestra `&mut self` (puede que no tenga mucho sentido).
        quote! { #vis_pub fn #alter_ident #generics (&mut self) -> &mut Self #where_clause }
    } else {
        // Con argumentos se muestra `&mut self, ...`.
        quote! { #vis_pub fn #alter_ident #generics (&mut self, ...) -> &mut Self #where_clause }
    };

    // Normaliza espacios raros tipo `& mut`.
    let alter_sig_str = alter_sig_tokens.to_string().replace("& mut", "&mut");

    // Nombre de la función `alter_...()` como alias de búsqueda.
    let alter_name_str = alter_ident.to_string();

    // Texto introductorio para la documentación adicional de `with_...()`.
    let with_alter_title = format!(
        "# {} el método `{}()` generado por [`#[builder_fn]`](pagetop_macros::builder_fn)",
        if doc_attrs.is_empty() {
            "Añade"
        } else {
            "También añade"
        },
        alter_name_str
    );
    let with_alter_doc = concat!(
        "Modifica la instancia actual (`&mut self`) con los mismos argumentos, ",
        "sin consumirla."
    );

    // Atributos completos que se aplican siempre a `with_...()`.
    let with_prefix = quote! {
        #(#other_attrs)*
        #(#doc_attrs)*
        #[doc(alias = #alter_name_str)]
        #[doc = ""]
        #[doc = #with_alter_title]
        #[doc = #with_alter_doc]
        #[doc = "```text"]
        #[doc = #alter_sig_str]
        #[doc = "```"]
    };

    // Genera el código final.
    let expanded = match body_opt {
        None => {
            quote! {
                #with_prefix
                fn #with_name #generics (self, #(#args),*) -> Self #where_clause;

                #(#non_doc_or_inline_attrs)*
                #[doc(hidden)]
                fn #alter_ident #generics (&mut self, #(#args),*) -> &mut Self #where_clause;
            }
        }
        Some(body) => {
            // Si no se indicó ninguna forma de `inline`, fuerza `#[inline]` para `with_...()`.
            let force_inline = if attrs.iter().any(|a| a.path().is_ident("inline")) {
                quote! {}
            } else {
                quote! { #[inline] }
            };

            let with_fn = if is_trait {
                quote! {
                    #with_prefix
                    #force_inline
                    #vis_pub fn #with_name #generics (self, #(#args),*) -> Self #where_clause {
                        let mut s = self;
                        s.#alter_ident(#(#call_idents),*);
                        s
                    }
                }
            } else {
                quote! {
                    #with_prefix
                    #force_inline
                    #vis_pub fn #with_name #generics (mut self, #(#args),*) -> Self #where_clause {
                        self.#alter_ident(#(#call_idents),*);
                        self
                    }
                }
            };

            quote! {
                #with_fn

                #(#non_doc_or_inline_attrs)*
                #[doc(hidden)]
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
