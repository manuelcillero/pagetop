use crate::core::component::{Component, Context};
use crate::html::{html, Markup};
use crate::{builder_fn, AutoDefault, UniqueId};

use parking_lot::Mutex;

pub use parking_lot::MutexGuard as ComponentGuard;

use std::fmt;
use std::vec::IntoIter;

/// Representa un componente hijo encapsulado para su uso en una lista [`Children`].
#[derive(AutoDefault)]
pub struct Child(Option<Mutex<Box<dyn Component>>>);

impl Clone for Child {
    fn clone(&self) -> Self {
        Child(self.0.as_ref().map(|m| Mutex::new(m.lock().clone_box())))
    }
}

impl fmt::Debug for Child {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            None => write!(f, "Child(None)"),
            Some(c) => write!(f, "Child({})", c.lock().name()),
        }
    }
}

impl Child {
    /// Crea un nuevo `Child` a partir de un componente.
    pub fn with(component: impl Component) -> Self {
        Child(Some(Mutex::new(Box::new(component))))
    }

    // **< Child BUILDER >**************************************************************************

    /// Establece un componente nuevo, o lo vacía.
    ///
    /// Si se proporciona `Some(component)`, se encapsula como [`Child`]; y si es `None`, se limpia.
    #[builder_fn]
    pub fn with_component<C: Component>(mut self, component: Option<C>) -> Self {
        self.0 = component.map(|c| Mutex::new(Box::new(c) as Box<dyn Component>));
        self
    }

    // **< Child GETTERS >**************************************************************************

    /// Devuelve el identificador del componente, si existe y está definido.
    #[inline]
    pub fn id(&self) -> Option<String> {
        self.0.as_ref().and_then(|c| c.lock().id())
    }

    // **< Child RENDER >***************************************************************************

    /// Renderiza el componente con el contexto proporcionado.
    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.as_ref().map_or(html! {}, |c| c.lock().render(cx))
    }

    // **< Child HELPERS >**************************************************************************

    /// Devuelve el [`UniqueId`] del tipo del componente, si existe.
    #[inline]
    fn type_id(&self) -> Option<UniqueId> {
        self.0.as_ref().map(|c| c.lock().type_id())
    }
}

impl<C: Component + 'static> From<Slot<C>> for Child {
    /// Convierte un [`Slot<C>`] en un [`Child`], consumiendo el componente tipado.
    ///
    /// Útil cuando se tiene un [`Slot`] y se necesita añadirlo a una lista [`Children`]:
    ///
    /// ```rust,ignore
    /// children.add(Child::from(my_slot));
    /// // o equivalentemente:
    /// children.add(my_slot.into());
    /// ```
    fn from(typed: Slot<C>) -> Self {
        if let Some(m) = typed.0 {
            Child(Some(Mutex::new(
                Box::new(m.into_inner()) as Box<dyn Component>
            )))
        } else {
            Child(None)
        }
    }
}

impl<T: Component + 'static> From<T> for Child {
    #[inline]
    fn from(component: T) -> Self {
        Child::with(component)
    }
}

// *************************************************************************************************

/// Variante tipada de [`Child`] para componentes con un tipo concreto conocido.
///
/// A diferencia de [`Child`], que encapsula cualquier componente como `dyn Component`, `Slot`
/// mantiene el tipo concreto `C` y permite acceder directamente a sus métodos específicos a través
/// de [`get()`](Slot::get).
///
/// Se utiliza habitualmente para incrustar un componente dentro de otro cuando no se necesita una
/// lista completa de hijos ([`Children`]), sino un único componente tipado en un campo concreto.
#[derive(AutoDefault)]
pub struct Slot<C: Component>(Option<Mutex<C>>);

impl<C: Component + Clone> Clone for Slot<C> {
    fn clone(&self) -> Self {
        Slot(self.0.as_ref().map(|m| Mutex::new(m.lock().clone())))
    }
}

impl<C: Component> fmt::Debug for Slot<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            None => write!(f, "Slot(None)"),
            Some(c) => write!(f, "Slot({})", c.lock().name()),
        }
    }
}

impl<C: Component> Slot<C> {
    /// Crea un nuevo `Slot` a partir de un componente.
    pub fn with(component: C) -> Self {
        Slot(Some(Mutex::new(component)))
    }

    // **< Slot BUILDER >*********************************************************************

    /// Establece un componente nuevo, o lo vacía.
    ///
    /// Si se proporciona `Some(component)`, se encapsula como [`Slot`]; y si es `None`, se
    /// limpia.
    #[builder_fn]
    pub fn with_component(mut self, component: Option<C>) -> Self {
        self.0 = component.map(Mutex::new);
        self
    }

    // **< Slot GETTERS >*********************************************************************

    /// Devuelve el identificador del componente, si existe y está definido.
    #[inline]
    pub fn id(&self) -> Option<String> {
        self.0.as_ref().and_then(|c| c.lock().id())
    }

    /// Devuelve un acceso al componente interno.
    ///
    /// - Devuelve `Some(ComponentGuard<C>)` si existe el componente, o `None` si está vacío.
    /// - El acceso es **exclusivo**: mientras el *guard* esté activo, no habrá otros accesos.
    /// - Se recomienda mantener el *guard* **el menor tiempo posible** para evitar bloqueos
    ///   innecesarios.
    /// - Para modificar el componente, declara el *guard* como `mut`:
    ///   `if let Some(mut c) = child.get() { c.alter_title(...); }`.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let child = Slot::with(Html::with(|_| html! { "Prueba" }));
    /// {
    ///     if let Some(component) = child.get() {
    ///         assert_eq!(component.name(), "Html");
    ///     }
    /// }; // El *guard* se libera aquí, antes del *drop* de `child`.
    ///
    /// let child = Slot::with(Block::new().with_title(L10n::n("Título")));
    /// {
    ///     if let Some(mut component) = child.get() {
    ///         component.alter_title(L10n::n("Nuevo título"));
    ///     }
    /// }; // El *guard* se libera aquí, antes del *drop* de `child`.
    /// ```
    pub fn get(&self) -> Option<ComponentGuard<'_, C>> {
        self.0.as_ref().map(|m| m.lock())
    }

    // **< Slot RENDER >**********************************************************************

    /// Renderiza el componente con el contexto proporcionado.
    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.as_ref().map_or(html! {}, |c| c.lock().render(cx))
    }
}

// *************************************************************************************************

/// Operaciones para componentes hijo [`Child`] en una lista [`Children`].
pub enum ChildOp {
    Add(Child),
    AddIfEmpty(Child),
    AddMany(Vec<Child>),
    InsertAfterId(&'static str, Child),
    InsertBeforeId(&'static str, Child),
    Prepend(Child),
    PrependMany(Vec<Child>),
    RemoveById(&'static str),
    ReplaceById(&'static str, Child),
    Reset,
}

/// Lista ordenada de componentes hijo ([`Child`]) mantenida por un componente padre.
///
/// Esta lista permite añadir, modificar, renderizar y consultar componentes hijo en orden de
/// inserción, soportando operaciones avanzadas como inserción relativa o reemplazo por
/// identificador.
#[derive(AutoDefault, Clone, Debug)]
pub struct Children(Vec<Child>);

impl Children {
    /// Crea una lista vacía.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea una lista con un componente hijo inicial.
    pub fn with(child: Child) -> Self {
        Self::default().with_child(ChildOp::Add(child))
    }

    // **< Children BUILDER >***********************************************************************

    /// Ejecuta una operación con [`ChildOp`] en la lista.
    #[builder_fn]
    pub fn with_child(mut self, op: ChildOp) -> Self {
        match op {
            ChildOp::Add(any) => self.add(any),
            ChildOp::AddIfEmpty(any) => self.add_if_empty(any),
            ChildOp::AddMany(many) => self.add_many(many),
            ChildOp::InsertAfterId(id, any) => self.insert_after_id(id, any),
            ChildOp::InsertBeforeId(id, any) => self.insert_before_id(id, any),
            ChildOp::Prepend(any) => self.prepend(any),
            ChildOp::PrependMany(many) => self.prepend_many(many),
            ChildOp::RemoveById(id) => self.remove_by_id(id),
            ChildOp::ReplaceById(id, any) => self.replace_by_id(id, any),
            ChildOp::Reset => self.reset(),
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

    /// Añade un componente hijo en la lista sólo si está vacía.
    #[inline]
    pub fn add_if_empty(&mut self, child: Child) -> &mut Self {
        if self.0.is_empty() {
            self.0.push(child);
        }
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
        self.0.iter().filter(move |c| c.type_id() == Some(type_id))
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

    /// Añade más de un componente hijo al final de la lista (en el orden recibido).
    #[inline]
    fn add_many<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = Child>,
    {
        self.0.extend(iter);
        self
    }

    /// Inserta un hijo después del componente con el `id` dado, o al final si no se encuentra.
    #[inline]
    fn insert_after_id(&mut self, id: impl AsRef<str>, child: Child) -> &mut Self {
        let id = Some(id.as_ref());
        match self.0.iter().position(|c| c.id().as_deref() == id) {
            Some(index) => self.0.insert(index + 1, child),
            _ => self.0.push(child),
        };
        self
    }

    /// Inserta un hijo antes del componente con el `id` dado, o al principio si no se encuentra.
    #[inline]
    fn insert_before_id(&mut self, id: impl AsRef<str>, child: Child) -> &mut Self {
        let id = Some(id.as_ref());
        match self.0.iter().position(|c| c.id().as_deref() == id) {
            Some(index) => self.0.insert(index, child),
            _ => self.0.insert(0, child),
        };
        self
    }

    /// Inserta un hijo al principio de la colección.
    #[inline]
    fn prepend(&mut self, child: Child) -> &mut Self {
        self.0.insert(0, child);
        self
    }

    /// Inserta más de un componente hijo al principio de la lista (manteniendo el orden recibido).
    #[inline]
    fn prepend_many<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = Child>,
    {
        let buf: Vec<Child> = iter.into_iter().collect();
        self.0.splice(0..0, buf);
        self
    }

    /// Elimina el primer hijo con el `id` dado.
    #[inline]
    fn remove_by_id(&mut self, id: impl AsRef<str>) -> &mut Self {
        let id = Some(id.as_ref());
        if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == id) {
            self.0.remove(index);
        }
        self
    }

    /// Sustituye el primer hijo con el `id` dado por otro componente.
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

    /// Elimina todos los componentes hijo de la lista.
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
