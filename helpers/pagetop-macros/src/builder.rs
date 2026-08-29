//! Núcleo compartido de `#[builder_fn]` y `#[builder_impl]`.

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    Attribute, Block, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, ItemTrait, Pat, ReturnType,
    Signature, TraitItem, TraitItemFn, Type, Visibility, WhereClause, parse_quote, parse2,
};

// Genera el código común a `#[builder_fn]` y `#[builder_impl]` para un único método `with_...()`.
//
// Recibe las piezas ya extraídas de un `ImplItemFn` o de un `TraitItemFn`: firma, atributos,
// visibilidad (ausente en métodos de trait), cuerpo (ausente en la declaración de un método trait
// sin implementación por defecto) y si el receptor es de trait (`self`) o de impl (`mut self`).
fn expand_builder(
    sig: &Signature,
    attrs: &[Attribute],
    vis: Option<&Visibility>,
    body_opt: Option<&Block>,
    is_trait: bool,
) -> TokenStream {
    let with_name = sig.ident.clone();
    let with_name_str = sig.ident.to_string();

    // Valida el nombre del método.
    if !with_name_str.starts_with("with_") {
        return quote_spanned! {
            sig.ident.span() => compile_error!("expected a named `with_...()` method");
        };
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
        };
    }
    if sig.constness.is_some() {
        return quote_spanned! {
            sig.constness.span() => compile_error!("`with_...()` cannot be `const`");
        };
    }
    if sig.abi.is_some() {
        return quote_spanned! {
            sig.abi.span() => compile_error!("`with_...()` cannot be `extern`");
        };
    }
    if sig.unsafety.is_some() {
        return quote_spanned! {
            sig.unsafety.span() => compile_error!("`with_...()` cannot be `unsafe`");
        };
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
        };
    }

    // Valida que el método devuelve exactamente `Self`.
    match &sig.output {
        ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Path(p) if p.qself.is_none() && p.path.is_ident("Self") => {}
            _ => {
                return quote_spanned! {
                    ty.span() => compile_error!("expected return type to be exactly `Self`");
                };
            }
        },
        _ => {
            return quote_spanned! {
                sig.output.span() => compile_error!("expected return type to be exactly `Self`");
            };
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
                        };
                    }
                }
                _ => {
                    return quote_spanned! {
                        arg.span() => compile_error!("unexpected receiver in parameter list");
                    };
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
        "Permite modificar la instancia (`&mut self`) con los mismos argumentos ",
        "pero sin consumirla."
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
    match body_opt {
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
                // Un cuerpo por defecto se compila junto a la propia definición del trait, donde
                // `Self` podría no ser `Sized`; a diferencia de una declaración sin cuerpo (rama
                // `None`), aquí sí hace falta acotarlo explícitamente para poder devolver `Self`
                // por valor. Se añade la cota sobre el `Punctuated` ya existente (en vez de
                // concatenar tokens a mano) para que la coma se coloque bien incluso si el `where`
                // original ya termina en una.
                let with_where: WhereClause = match where_clause {
                    Some(wc) => {
                        let mut wc = wc.clone();
                        wc.predicates.push(parse_quote!(Self: Sized));
                        wc
                    }
                    None => parse_quote!(where Self: Sized),
                };
                quote! {
                    #with_prefix
                    #force_inline
                    #vis_pub fn #with_name #generics (self, #(#args),*) -> Self #with_where {
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
    }
}

// Implementa `#[builder_fn]`: detecta si el ítem anotado es un método de `impl` o de `trait`,
// extrae sus piezas comunes y delega en `expand_builder`.
pub(crate) fn expand_fn(item: TokenStream) -> TokenStream {
    enum Kind {
        Impl(ImplItemFn),
        Trait(TraitItemFn),
    }

    // Detecta si estamos en `impl` o `trait`.
    let kind = if let Ok(it) = parse2::<ImplItemFn>(item.clone()) {
        Kind::Impl(it)
    } else if let Ok(tt) = parse2::<TraitItemFn>(item.clone()) {
        Kind::Trait(tt)
    } else {
        return quote! {
            compile_error!("#[builder_fn] only supports methods in `impl` blocks or `trait` items");
        };
    };

    // Extrae piezas comunes (sig, attrs, vis, bloque?, es_trait?).
    let (sig, attrs, vis, body_opt, is_trait) = match &kind {
        Kind::Impl(m) => (&m.sig, &m.attrs, Some(&m.vis), Some(&m.block), false),
        Kind::Trait(t) => (&t.sig, &t.attrs, None, t.default.as_ref(), true),
    };

    expand_builder(sig, attrs, vis, body_opt, is_trait)
}

// Comprueba si la lista de atributos contiene uno con el nombre dado.
fn has_attr(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|a| a.path().is_ident(name))
}

// Decide qué hacer con un único método durante el barrido de `#[builder_impl]`, sea de un `impl`
// o de un `trait`. Devuelve `None` si el método no es un `with_...()` a barrer (ni siquiera marcado
// con `#[builder_skip]`), en cuyo caso el llamador lo reemite intacto.
fn sweep_with_fn(
    sig: &Signature,
    attrs: &[Attribute],
    vis: Option<&Visibility>,
    body_opt: Option<&Block>,
    is_trait: bool,
) -> Option<TokenStream> {
    if !sig.ident.to_string().starts_with("with_") {
        return None;
    }
    let skip = has_attr(attrs, "builder_skip");
    // Se rechaza `with_...()` marcado a la vez con `#[builder_skip]` y `#[builder_fn]`.
    if skip && has_attr(attrs, "builder_fn") {
        return Some(quote_spanned! {
            sig.ident.span() => compile_error!(
                "`#[builder_skip]` and `#[builder_fn]` cannot be combined on the same method"
            );
        });
    }
    if skip {
        return None;
    }
    // Descarta atributos auxiliares para no reprocesar ni dejar atributos desconocidos.
    let clean: Vec<Attribute> = attrs
        .iter()
        .filter(|a| !a.path().is_ident("builder_fn") && !a.path().is_ident("builder_skip"))
        .cloned()
        .collect();
    Some(expand_builder(sig, &clean, vis, body_opt, is_trait))
}

// Implementa `#[builder_impl]`: aplica `expand_builder` a todos los métodos `with_...()` de un
// bloque `impl` o de una definición de `trait`, dejando el resto de ítems intactos.
pub(crate) fn expand_impl(item: TokenStream) -> TokenStream {
    if let Ok(item_impl) = parse2::<ItemImpl>(item.clone()) {
        return expand_item_impl(item_impl);
    }
    if let Ok(item_trait) = parse2::<ItemTrait>(item) {
        return expand_item_trait(item_trait);
    }
    quote! {
        compile_error!("#[builder_impl] only supports `impl` blocks or `trait` definitions");
    }
}

fn expand_item_impl(item: ItemImpl) -> TokenStream {
    let ItemImpl {
        attrs,
        defaultness,
        unsafety,
        impl_token,
        generics,
        trait_,
        self_ty,
        items,
        ..
    } = item;

    let mut out = Vec::new();

    for it in items {
        match it {
            ImplItem::Fn(f) => {
                match sweep_with_fn(&f.sig, &f.attrs, Some(&f.vis), Some(&f.block), false) {
                    Some(ts) => out.push(ts),
                    None => {
                        // Método no-builder (o `with_...()` marcado con `#[builder_skip]`): se
                        // reemite intacto, retirando siempre `#[builder_skip]` (atributo inerte).
                        let mut f = f;
                        f.attrs.retain(|a| !a.path().is_ident("builder_skip"));
                        out.push(quote! { #f });
                    }
                }
            }
            other => out.push(quote! { #other }),
        }
    }

    let (impl_generics, _type_generics, where_clause) = generics.split_for_impl();

    // Reconstruye la parte `Trait for` si el impl es de trait.
    let trait_ = trait_.map(|(bang, path, for_token)| quote! { #bang #path #for_token });

    quote! {
        #(#attrs)*
        #defaultness #unsafety #impl_token #impl_generics #trait_ #self_ty #where_clause {
            #(#out)*
        }
    }
}

fn expand_item_trait(item: ItemTrait) -> TokenStream {
    let ItemTrait {
        attrs,
        vis,
        unsafety,
        auto_token,
        trait_token,
        ident,
        generics,
        colon_token,
        supertraits,
        items,
        ..
    } = item;

    let mut out = Vec::new();

    for it in items {
        match it {
            TraitItem::Fn(f) => {
                match sweep_with_fn(&f.sig, &f.attrs, None, f.default.as_ref(), true) {
                    Some(ts) => out.push(ts),
                    None => {
                        // Método no-builder (o `with_...()` marcado con `#[builder_skip]`): se
                        // reemite intacto, retirando siempre `#[builder_skip]` (atributo inerte).
                        let mut f = f;
                        f.attrs.retain(|a| !a.path().is_ident("builder_skip"));
                        out.push(quote! { #f });
                    }
                }
            }
            other => out.push(quote! { #other }),
        }
    }

    // El propio nombre de la lista de genéricos (`<T: Bound>`) es el que lleva las cotas en una
    // definición de trait, a diferencia de su uso como tipo; por eso se usa `impl_generics` y no
    // `type_generics` para reconstruir `trait Nombre<...>`.
    let (impl_generics, _type_generics, where_clause) = generics.split_for_impl();

    quote! {
        #(#attrs)*
        #vis #unsafety #auto_token #trait_token #ident #impl_generics
        #colon_token #supertraits
        #where_clause
        {
            #(#out)*
        }
    }
}
