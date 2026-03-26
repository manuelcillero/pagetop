//! Acciones que operan sobre componentes.

use pagetop::prelude::*;

/// Tipo de función para manipular componentes y su contexto de renderizado.
///
/// Se usa en [`action::component::BeforeRender`] y [`action::component::AfterRender`] para alterar
/// el comportamiento predefinido de los componentes antes y después de renderizarlos.
///
/// Recibe referencias mutables del componente `component` y del contexto `cx`.
pub type FnActionWithComponent<C> = fn(component: &mut C, cx: &mut Context);

/// Tipo de función para alterar el [`Markup`] generado por un componente.
///
/// Se usa en [`action::component::TransformMarkup`] para permitir a las extensiones alterar el HTML
/// final producido por el renderizado de un componente. La edición trabaja a nivel de texto: el
/// [`Markup`] recibido expone su contenido como [`String`], lo que permite aplicar búsquedas,
/// sustituciones, concatenaciones y cualquier otra primitiva de trabajo con cadenas.
///
/// La función recibe una referencia inmutable al componente `component` (el renderizado ya ha
/// concluido, solo se necesita leer su estado), y al contexto `cx` (solo para consulta), y toma
/// posesión del `markup` producido hasta ese momento.
///
/// Devuelve el nuevo [`Markup`] transformado, que se encadena como entrada para la siguiente acción
/// registrada, si la hay.
pub type FnActionTransformMarkup<C> = fn(component: &C, cx: &Context, markup: Markup) -> Markup;

mod before_render_component;
pub use before_render_component::*;

mod after_render_component;
pub use after_render_component::*;

mod transform_markup_component;
pub use transform_markup_component::*;
