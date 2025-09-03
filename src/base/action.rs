//! Acciones predefinidas para alterar el funcionamiento interno de PageTop.

use crate::prelude::*;

/// Tipo de función para manipular componentes y su contexto de renderizado.
///
/// Se usa en acciones definidas en [`component`] y [`theme`] para alterar el comportamiento de los
/// componentes.
///
/// Recibe referencias mutables (`&mut`) del componente `component` y del contexto `cx`.
pub type FnActionWithComponent<C> = fn(component: &mut C, cx: &mut Context);

pub mod component;

pub mod theme;

pub mod page;
