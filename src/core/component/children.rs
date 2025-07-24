use crate::core::component::ComponentTrait;
use crate::html::{html, Context, Markup};
use crate::{builder_fn, UniqueId};

use parking_lot::RwLock;

use std::sync::Arc;
use std::vec::IntoIter;

/// Representa un componente encapsulado de forma segura y compartida.
///
/// Esta estructura permite manipular y renderizar cualquier tipo que implemente [`ComponentTrait`],
/// garantizando acceso concurrente a través de [`Arc<RwLock<_>>`].
#[derive(Clone)]
pub struct Child(Arc<RwLock<dyn ComponentTrait>>);

impl Child {
    /// Crea un nuevo [`Child`] a partir de un componente.
    pub fn with(component: impl ComponentTrait) -> Self {
        Child(Arc::new(RwLock::new(component)))
    }

    // Child GETTERS *******************************************************************************

    /// Devuelve el identificador del componente, si está definido.
    pub fn id(&self) -> Option<String> {
        self.0.read().id()
    }

    // Child RENDER ********************************************************************************

    /// Renderiza el componente con el contexto proporcionado.
    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().render(cx)
    }

    // Child HELPERS *******************************************************************************

    // Devuelve el [`UniqueId`] del tipo del componente.
    fn type_id(&self) -> UniqueId {
        self.0.read().type_id()
    }
}

// *************************************************************************************************

/// Variante tipada de [`Child`] para evitar conversiones durante el uso.
///
/// Facilita el acceso a componentes del mismo tipo sin necesidad de hacer `downcast`.
pub struct Typed<C: ComponentTrait>(Arc<RwLock<C>>);

impl<C: ComponentTrait> Clone for Typed<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C: ComponentTrait> Typed<C> {
    /// Crea un nuevo [`Typed`] a partir de un componente.
    pub fn with(component: C) -> Self {
        Typed(Arc::new(RwLock::new(component)))
    }

    // Typed GETTERS *******************************************************************************

    /// Devuelve el identificador del componente, si está definido.
    pub fn id(&self) -> Option<String> {
        self.0.read().id()
    }

    // Typed RENDER ********************************************************************************

    /// Renderiza el componente con el contexto proporcionado.
    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().render(cx)
    }

    // Typed HELPERS *******************************************************************************

    /// Convierte el componente tipado en un [`Child`].
    fn to_child(&self) -> Child {
        Child(self.0.clone())
    }
}

// *************************************************************************************************

/// Operaciones con un componente [`Child`] en una lista [`Children`].
pub enum ChildOp {
    Add(Child),
    InsertAfterId(&'static str, Child),
    InsertBeforeId(&'static str, Child),
    Prepend(Child),
    RemoveById(&'static str),
    ReplaceById(&'static str, Child),
    Reset,
}

/// Operaciones con un componente tipado [`Typed<C>`] en una lista [`Children`].
pub enum TypedOp<C: ComponentTrait> {
    Add(Typed<C>),
    InsertAfterId(&'static str, Typed<C>),
    InsertBeforeId(&'static str, Typed<C>),
    Prepend(Typed<C>),
    RemoveById(&'static str),
    ReplaceById(&'static str, Typed<C>),
    Reset,
}

/// Lista ordenada de los componentes hijo ([`Child`]) asociados a un componente padre.
///
/// Esta colección permite añadir, modificar, renderizar y consultar componentes hijos en orden de
/// inserción, soportando operaciones avanzadas como inserción relativa o reemplazo por
/// identificador.
#[derive(Clone, Default)]
pub struct Children(Vec<Child>);

impl Children {
    /// Crea una lista vacía.
    pub fn new() -> Self {
        Children::default()
    }

    /// Crea una lista con un único hijo inicial.
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

    // Children BUILDER ****************************************************************************

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
    pub fn with_typed<C: ComponentTrait + Default>(mut self, op: TypedOp<C>) -> Self {
        match op {
            TypedOp::Add(typed) => self.add(typed.to_child()),
            TypedOp::InsertAfterId(id, typed) => self.insert_after_id(id, typed.to_child()),
            TypedOp::InsertBeforeId(id, typed) => self.insert_before_id(id, typed.to_child()),
            TypedOp::Prepend(typed) => self.prepend(typed.to_child()),
            TypedOp::RemoveById(id) => self.remove_by_id(id),
            TypedOp::ReplaceById(id, typed) => self.replace_by_id(id, typed.to_child()),
            TypedOp::Reset => self.reset(),
        }
    }

    /// Añade un hijo al final de la lista.
    #[inline]
    pub fn add(&mut self, child: Child) -> &mut Self {
        self.0.push(child);
        self
    }

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

    // Children GETTERS ****************************************************************************

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

    /// Devuelve un iterador sobre los componentes hijo con el identificador tipo ([`UniqueId`])
    /// indicado.
    pub fn iter_by_type_id(&self, type_id: UniqueId) -> impl Iterator<Item = &Child> {
        self.0.iter().filter(move |&c| c.type_id() == type_id)
    }

    // Children RENDER *****************************************************************************

    /// Renderiza todos los componentes hijo, en orden.
    pub fn render(&self, cx: &mut Context) -> Markup {
        html! {
            @for c in &self.0 {
                (c.render(cx))
            }
        }
    }
}

impl IntoIterator for Children {
    type Item = Child;
    type IntoIter = IntoIter<Child>;

    /// Consume la estructura `Children`, devolviendo un iterador que consume los elementos.
    ///
    /// ### Ejemplo de uso:
    /// ```rust#ignore
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
    /// ### Ejemplo de uso:
    /// ```rust#ignore
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
    /// ### Ejemplo de uso:
    /// ```rust#ignore
    /// let mut children = Children::new().with(child1).with(child2);
    /// for child in &mut children {
    ///     child.render(&mut context);
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
