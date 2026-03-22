//! Acciones predefinidas para alterar el funcionamiento interno de PageTop.

use crate::prelude::*;

/// Tipo de función para manipular componentes y su contexto de renderizado.
///
/// Se usa en acciones definidas en [`action::component`] y [`action::theme`] para alterar el
/// comportamiento de los componentes.
///
/// Recibe referencias mutables (`&mut`) del componente `component` y del contexto `cx`.
pub type FnActionWithComponent<C> = fn(component: &mut C, cx: &mut Context);

/// Tipo de función para modificar el [`Markup`] generado por un componente.
///
/// Se usa en [`action::component::AlterMarkup`] para permitir a las extensiones modificar el HTML
/// final producido por el renderizado de un componente. La edición trabaja a nivel de texto: el
/// [`Markup`] recibido expone su contenido como [`String`], lo que permite aplicar búsquedas,
/// sustituciones, concatenaciones y cualquier otra primitiva de trabajo con cadenas.
///
/// La función recibe referencias mutables del componente `component` y del contexto `cx`, y toma
/// posesión del `markup` producido hasta ese momento. Devuelve el nuevo [`Markup`] modificado, que
/// se encadena como entrada para la siguiente acción registrada, si la hay.
pub type FnActionAlterMarkup<C> = fn(component: &mut C, cx: &mut Context, markup: Markup) -> Markup;

// **< Acciones por tipo >**************************************************************************

pub mod component;

pub mod theme;

pub mod page;
