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

Este *crate* incluye entre sus macros una adaptación de
[maud-macros](https://crates.io/crates/maud_macros)
([0.27.0](https://github.com/lambda-fairy/maud/tree/v0.27.0/maud_macros)) de
[Chris Wong](https://crates.io/users/lambda-fairy) y una versión renombrada de
[SmartDefault](https://crates.io/crates/smart_default) (0.7.1) de
[Jane Doe](https://crates.io/users/jane-doe), llamada `AutoDefault`. Estas macros eliminan la
necesidad de referenciar `maud` o `smart_default` en las dependencias del archivo `Cargo.toml` de
cada proyecto PageTop.
*/

#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/assets/favicon.ico"
)]

mod builder;
mod maud;
mod smart_default;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, ItemFn, parse_macro_input};

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
///     e: "four".to_string(),
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
/// ```rust,no_run
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
/// ```rust,no_run
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
///
/// Para aplicar la misma transformación a todos los métodos `with_...()` de un `impl` de una sola
/// vez, usa [`#[builder_impl]`](builder_impl).
#[proc_macro_attribute]
pub fn builder_fn(_: TokenStream, item: TokenStream) -> TokenStream {
    builder::expand_fn(item.into()).into()
}

/// Macro (*attribute*) que aplica [`#[builder_fn]`](builder_fn) a los métodos `with_` de un
/// `impl`/`trait`.
///
/// Cada método que empiece por `with_` se transforma igual que si llevara `#[builder_fn]`
/// individualmente: se genera su correspondiente método `alter_...()` y se añade la misma
/// documentación. El resto de ítems del bloque (métodos que no empiecen por `with_`, constantes
/// asociadas, tipos, etc.) no se modifican.
///
/// La política es estricta; si un método `with_...()` no cumple la firma esperada por
/// [`#[builder_fn]`](builder_fn) para su contexto, la macro emite el mismo error de compilación que
/// emitiría `#[builder_fn]` sobre ese método. Para excluir deliberadamente un método `with_...()`,
/// márcalo con `#[builder_skip]`; se mantendrá intacto, como cualquier otro método que no sea
/// *builder*.
///
/// Un `#[builder_fn]` explícito sobre un método dentro de un bloque `#[builder_impl]` es
/// redundante pero inofensivo, no se expande dos veces. Combinar `#[builder_skip]` y
/// `#[builder_fn]` sobre el mismo método sí es un error de compilación porque la intención de ambos
/// atributos sí es contradictoria.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop_macros::builder_impl;
/// # #[derive(Default)]
/// # struct Example { a: Option<String>, b: Option<u32> }
/// #[builder_impl]
/// impl Example {
///     pub fn with_a(mut self, value: impl Into<String>) -> Self {
///         self.a = Some(value.into());
///         self
///     }
///
///     pub fn with_b(mut self, value: u32) -> Self {
///         self.b = Some(value);
///         self
///     }
///
///     pub fn a(&self) -> Option<&str> {
///         self.a.as_deref()
///     }
/// }
///
/// let example = Example::default().with_a("hello").with_b(42);
/// ```
///
/// genera, para `with_a` y `with_b`, el mismo par `with_.../alter_...` que produciría anotar cada
/// uno individualmente con [`#[builder_fn]`](builder_fn); `a()` se reemite sin modificar.
///
/// Sobre una definición de `trait`, con receptor `self` (sin `mut`) en cada `with_...()`:
///
/// ```rust,no_run
/// # use pagetop_macros::builder_impl;
/// #[builder_impl]
/// pub trait Example {
///     /// Sin cuerpo por defecto: sólo genera la declaración.
///     fn with_a(self, value: impl Into<String>) -> Self;
///
///     /// Con cuerpo por defecto: genera también la implementación, heredable sin redefinirla.
///     fn with_b(self, value: u32) -> Self {
///         self
///     }
/// }
/// ```
///
/// Un `with_...()` de trait con cuerpo por defecto añade `where Self: Sized` automáticamente. A
/// diferencia de una declaración sin cuerpo, éste se compila junto a la propia definición del
/// trait, donde `Self` podría no ser `Sized`, y Rust lo exige para poder devolverlo por valor.
#[proc_macro_attribute]
pub fn builder_impl(_: TokenStream, item: TokenStream) -> TokenStream {
    builder::expand_impl(item.into()).into()
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
    let input = parse_macro_input!(item as ItemFn);
    expand_entry(input, false)
}

/// Define funciones de prueba asíncronas para usar con PageTop.
///
/// Usa el mismo *runtime* multi-hilo que [`#[pagetop::main]`](macro@main), para garantizar
/// compatibilidad con extensiones que ejecutan código asíncrono de forma síncrona.
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
    let input = parse_macro_input!(item as ItemFn);
    expand_entry(input, true)
}

// Genera la función síncrona que envuelve el cuerpo asíncrono original, común a `main` y `test`.
fn expand_entry(input: ItemFn, is_test: bool) -> TokenStream {
    if input.sig.asyncness.is_none() {
        return syn::Error::new_spanned(input.sig.fn_token, "the function must be `async`")
            .to_compile_error()
            .into();
    }

    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = input;
    sig.asyncness = None;

    // Ruta absoluta para evitar ambigüedad con `pagetop::test` bajo `use pagetop::prelude::*;`.
    let test_attr = is_test.then(|| quote! { #[::core::prelude::v1::test] });

    let expanded = quote! {
        #test_attr
        #(#attrs)*
        #vis #sig {
            #[allow(
                clippy::expect_used,
                clippy::diverging_sub_expression,
                clippy::needless_return,
                clippy::unwrap_in_result
            )]
            {
                return ::pagetop::util::build_runtime().block_on(async move #block);
            }
        }
    };
    expanded.into()
}
