use crate::prelude::*;

use super::{Item, ItemKind};

/// Lista de elementos [`Item`] de un menú desplegable [`Dropdown`](super::Dropdown).
///
/// Se renderiza sin los encabezados ni los separadores que no separan nada (ver
/// [`render()`](Self::render)), así que cualquier tema que pinte el menú con `items().render(cx)`
/// aprovecha la limpieza sin hacer nada más.
#[derive(AutoDefault, Clone, Debug)]
pub struct Items(Children);

impl Items {
    /// Devuelve el número de elementos de la lista, también los que no se mostrarán.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Indica si la lista está vacía.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Renderiza los elementos sin los encabezados ni los separadores que no separan nada.
    ///
    /// Un encabezado sólo se muestra si le sigue algún elemento visible antes del siguiente
    /// encabezado o separador, y un separador sólo entre dos elementos visibles, una sola vez.
    /// Así, si se ocultan todos los enlaces de un grupo (por ejemplo, porque el usuario no tiene
    /// acceso a sus rutas, ver [`Endpoint`]), su encabezado desaparece con ellos. Un elemento es
    /// visible si su renderizado no queda vacío. Cada uno se renderiza como mucho una vez: los
    /// encabezados y separadores, sólo si se muestran.
    pub async fn render(&self, cx: &mut Context) -> Markup {
        let mut shown: Vec<Markup> = Vec::with_capacity(self.0.len());
        // Encabezado y separador a la espera de un elemento visible que los justifique.
        let mut header = None;
        let mut divider = None;
        for item in self.0.iter() {
            match item.downcast_ref::<Item>().map(Item::item_kind) {
                Some(ItemKind::Header(_)) => header = Some(item),
                Some(ItemKind::Divider) => {
                    header = None;
                    if !shown.is_empty() {
                        divider = Some(item);
                    }
                }
                _ => {
                    let markup = item.render(cx).await;
                    if markup.is_empty() {
                        continue;
                    }
                    if let Some(divider) = divider.take() {
                        shown.push(divider.render(cx).await);
                    }
                    if let Some(header) = header.take() {
                        shown.push(header.render(cx).await);
                    }
                    shown.push(markup);
                }
            }
        }
        html! { @for markup in &shown { (markup) } }
    }

    // Añade un elemento o modifica la lista con una operación `TypedOp`.
    pub(crate) fn alter_item(&mut self, op: TypedOp<Item>) {
        self.0.alter_child(op);
    }
}
