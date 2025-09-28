use crate::core::component::Component;
use crate::html::{html, Context, Markup};
use crate::{builder_fn, AutoDefault, UniqueId};

use parking_lot::RwLock;

use std::sync::Arc;
use std::vec::IntoIter;

/// Representa un componente encapsulado de forma segura y compartida.
///
/// Esta estructura permite manipular y renderizar un componente que implemente [`Component`], y
/// habilita acceso concurrente mediante [`Arc<RwLock<_>>`].
#[derive(AutoDefault, Clone)]
pub struct Child(Option<Arc<RwLock<dyn Component>>>);

impl Child {
    /// Crea un nuevo `Child` a partir de un componente.
    pub fn with(component: impl Component) -> Self {
        Child(Some(Arc::new(RwLock::new(component))))
    }

    // **< Child BUILDER >**************************************************************************

    /// Establece un componente nuevo, o lo vacía.
    ///
    /// Si se proporciona `Some(component)`, se encapsula como [`Child`]; y si es `None`, se limpia.
    #[builder_fn]
    pub fn with_component<C: Component>(mut self, component: Option<C>) -> Self {
        if let Some(c) = component {
            self.0 = Some(Arc::new(RwLock::new(c)));
        } else {
            self.0 = None;
        }
        self
    }

    // **< Child GETTERS >**************************************************************************

    /// Devuelve el identificador del componente, si existe y está definido.
    #[inline]
    pub fn id(&self) -> Option<String> {
        self.0.as_ref().and_then(|c| c.read().id())
    }

    // **< Child RENDER >***************************************************************************

    /// Renderiza el componente con el contexto proporcionado.
    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.as_ref().map_or(html! {}, |c| c.write().render(cx))
    }

    // **< Child HELPERS >**************************************************************************

    // Devuelve el [`UniqueId`] del tipo del componente, si existe.
    #[inline]
    fn type_id(&self) -> Option<UniqueId> {
        self.0.as_ref().map(|c| c.read().type_id())
    }
}

// *************************************************************************************************

/// Variante tipada de [`Child`] para evitar conversiones de tipo durante el uso.
///
/// Esta estructura permite manipular y renderizar un componente concreto que implemente
/// [`Component`], y habilita acceso concurrente mediante [`Arc<RwLock<_>>`].
#[derive(AutoDefault, Clone)]
pub struct Typed<C: Component>(Option<Arc<RwLock<C>>>);

impl<C: Component> Typed<C> {
    /// Crea un nuevo `Typed` a partir de un componente.
    pub fn with(component: C) -> Self {
        Typed(Some(Arc::new(RwLock::new(component))))
    }

    // **< Typed BUILDER >**************************************************************************

    /// Establece un componente nuevo, o lo vacía.
    ///
    /// Si se proporciona `Some(component)`, se encapsula como [`Typed`]; y si es `None`, se limpia.
    #[builder_fn]
    pub fn with_component(mut self, component: Option<C>) -> Self {
        self.0 = component.map(|c| Arc::new(RwLock::new(c)));
        self
    }

    // **< Typed GETTERS >**************************************************************************

    /// Devuelve el identificador del componente, si existe y está definido.
    #[inline]
    pub fn id(&self) -> Option<String> {
        self.0.as_ref().and_then(|c| c.read().id())
    }

    // **< Typed RENDER >***************************************************************************

    /// Renderiza el componente con el contexto proporcionado.
    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.as_ref().map_or(html! {}, |c| c.write().render(cx))
    }

    // **< Typed HELPERS >**************************************************************************

    // Convierte el componente tipado en un [`Child`].
    #[inline]
    fn into_child(self) -> Child {
        if let Some(c) = &self.0 {
            Child(Some(c.clone()))
        } else {
            Child(None)
        }
    }
}

// *************************************************************************************************

/// Operaciones con un componente hijo [`Child`] en una lista [`Children`].
pub enum ChildOp {
    Add(Child),
    InsertAfterId(&'static str, Child),
    InsertBeforeId(&'static str, Child),
    Prepend(Child),
    RemoveById(&'static str),
    ReplaceById(&'static str, Child),
    Reset,
}

/// Operaciones con un componente hijo tipado [`Typed<C>`] en una lista [`Children`].
pub enum TypedOp<C: Component> {
    Add(Typed<C>),
    InsertAfterId(&'static str, Typed<C>),
    InsertBeforeId(&'static str, Typed<C>),
    Prepend(Typed<C>),
    RemoveById(&'static str),
    ReplaceById(&'static str, Typed<C>),
    Reset,
}

/// Lista ordenada de componentes hijo ([`Child`]) mantenida por un componente padre.
///
/// Esta lista permite añadir, modificar, renderizar y consultar componentes hijo en orden de
/// inserción, soportando operaciones avanzadas como inserción relativa o reemplazo por
/// identificador.
#[derive(Clone, Default)]
pub struct Children(Vec<Child>);

impl Children {
    /// Crea una lista vacía.
    pub fn new() -> Self {
        Children::default()
    }

    /// Crea una lista con un componente hijo inicial.
    pub fn with(child: Child) -> Self {
        Children::default().with_child(ChildOp::Add(child))
    }

    // Fusiona varias listas de `Children` en una sola.
    pub(crate) fn merge(mixes: &[Option<&Children>]) -> Self {
        let mut opt = Children::default();
        for m in mixes.iter().flatten() {
            opt.0.extend(m.0.iter().cloned());
        }
        opt
    }

    // **< Children BUILDER >***********************************************************************

    /// Ejecuta una operación con [`ChildOp`] en la lista.
    #[builder_fn]
    pub fn with_child(mut self, op: ChildOp) -> Self {
        match op {
            ChildOp::Add(any) => self.add(any),
            ChildOp::InsertAfterId(id, any) => self.insert_after_id(id, any),
            ChildOp::InsertBeforeId(id, any) => self.insert_before_id(id, any),
            ChildOp::Prepend(any) => self.prepend(any),
            ChildOp::RemoveById(id) => self.remove_by_id(id),
            ChildOp::ReplaceById(id, any) => self.replace_by_id(id, any),
            ChildOp::Reset => self.reset(),
        }
    }

    /// Ejecuta una operación con [`TypedOp`] en la lista.
    #[builder_fn]
    pub fn with_typed<C: Component + Default>(mut self, op: TypedOp<C>) -> Self {
        match op {
            TypedOp::Add(typed) => self.add(typed.into_child()),
            TypedOp::InsertAfterId(id, typed) => self.insert_after_id(id, typed.into_child()),
            TypedOp::InsertBeforeId(id, typed) => self.insert_before_id(id, typed.into_child()),
            TypedOp::Prepend(typed) => self.prepend(typed.into_child()),
            TypedOp::RemoveById(id) => self.remove_by_id(id),
            TypedOp::ReplaceById(id, typed) => self.replace_by_id(id, typed.into_child()),
            TypedOp::Reset => self.reset(),
        }
    }

    /// Añade un componente hijo al final de la lista.
    ///
    /// Es un atajo para `children.alter_child(ChildOp::Add(child))`.
    #[inline]
    pub fn add(&mut self, child: Child) -> &mut Self {
        self.0.push(child);
        self
    }

    // **< Children GETTERS >***********************************************************************

    /// Devuelve el número de componentes hijo de la lista.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Indica si la lista está vacía.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Devuelve el primer componente hijo con el identificador indicado, si existe.
    pub fn get_by_id(&self, id: impl AsRef<str>) -> Option<&Child> {
        let id = Some(id.as_ref());
        self.0.iter().find(|c| c.id().as_deref() == id)
    }

    /// Devuelve un iterador sobre los componentes hijo con el identificador indicado.
    pub fn iter_by_id<'a>(&'a self, id: &'a str) -> impl Iterator<Item = &'a Child> + 'a {
        self.0.iter().filter(move |c| c.id().as_deref() == Some(id))
    }

    /// Devuelve un iterador sobre los componentes hijo con el identificador de tipo ([`UniqueId`])
    /// indicado.
    pub fn iter_by_type_id(&self, type_id: UniqueId) -> impl Iterator<Item = &Child> {
        self.0.iter().filter(move |&c| c.type_id() == Some(type_id))
    }

    // **< Children RENDER >************************************************************************

    /// Renderiza todos los componentes hijo, en orden.
    pub fn render(&self, cx: &mut Context) -> Markup {
        html! {
            @for c in &self.0 {
                (c.render(cx))
            }
        }
    }

    // **< Children HELPERS >***********************************************************************

    // Inserta un hijo después del componente con el `id` dado, o al final si no se encuentra.
    #[inline]
    fn insert_after_id(&mut self, id: impl AsRef<str>, child: Child) -> &mut Self {
        let id = Some(id.as_ref());
        match self.0.iter().position(|c| c.id().as_deref() == id) {
            Some(index) => self.0.insert(index + 1, child),
            _ => self.0.push(child),
        };
        self
    }

    // Inserta un hijo antes del componente con el `id` dado, o al principio si no se encuentra.
    #[inline]
    fn insert_before_id(&mut self, id: impl AsRef<str>, child: Child) -> &mut Self {
        let id = Some(id.as_ref());
        match self.0.iter().position(|c| c.id().as_deref() == id) {
            Some(index) => self.0.insert(index, child),
            _ => self.0.insert(0, child),
        };
        self
    }

    // Inserta un hijo al principio de la colección.
    #[inline]
    fn prepend(&mut self, child: Child) -> &mut Self {
        self.0.insert(0, child);
        self
    }

    // Elimina el primer hijo con el `id` dado.
    #[inline]
    fn remove_by_id(&mut self, id: impl AsRef<str>) -> &mut Self {
        let id = Some(id.as_ref());
        if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == id) {
            self.0.remove(index);
        }
        self
    }

    // Sustituye el primer hijo con el `id` dado por otro componente.
    #[inline]
    fn replace_by_id(&mut self, id: impl AsRef<str>, child: Child) -> &mut Self {
        let id = Some(id.as_ref());
        for c in &mut self.0 {
            if c.id().as_deref() == id {
                *c = child;
                break;
            }
        }
        self
    }

    // Elimina todos los componentes hijo de la lista.
    #[inline]
    fn reset(&mut self) -> &mut Self {
        self.0.clear();
        self
    }
}

impl IntoIterator for Children {
    type Item = Child;
    type IntoIter = IntoIter<Child>;

    /// Consume la estructura `Children`, devolviendo un iterador que consume los elementos.
    ///
    /// # Ejemplo de uso:
    ///
    /// ```rust,ignore
    /// let children = Children::new().with(child1).with(child2);
    /// for child in children {
    ///     println!("{:?}", child.id());
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Children {
    type Item = &'a Child;
    type IntoIter = std::slice::Iter<'a, Child>;

    /// Itera sobre una referencia inmutable de `Children`, devolviendo un iterador de referencia.
    ///
    /// # Ejemplo de uso:
    ///
    /// ```rust,ignore
    /// let children = Children::new().with(child1).with(child2);
    /// for child in &children {
    ///     println!("{:?}", child.id());
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Children {
    type Item = &'a mut Child;
    type IntoIter = std::slice::IterMut<'a, Child>;

    /// Itera sobre una referencia mutable de `Children`, devolviendo un iterador mutable.
    ///
    /// # Ejemplo de uso:
    ///
    /// ```rust,ignore
    /// let mut children = Children::new().with(child1).with(child2);
    /// for child in &mut children {
    ///     child.render(&mut context);
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
