use crate::core::component::{Component, Context};
use crate::html::{Markup, html};
use crate::{AutoDefault, UniqueId, builder_fn};

use std::fmt;
use std::sync::Arc;
use std::vec::IntoIter;

// **< Child >**************************************************************************************

/// Representa un componente hijo encapsulado para su uso en una lista [`Children`].
///
/// Envuelve el componente en `Arc<dyn Component>`, compartido y de sólo lectura. Clonar un `Child`
/// sólo incrementa el contador de referencias. Para renderizar obtiene una copia propia con
/// [`ComponentClone::clone_box()`](crate::core::component::ComponentClone::clone_box), de modo que
/// el componente original nunca se modifica.
#[derive(AutoDefault, Clone)]
pub struct Child(Option<Arc<dyn Component>>);

impl fmt::Debug for Child {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            None => write!(f, "Child(None)"),
            Some(c) => write!(f, "Child({})", c.name()),
        }
    }
}

impl Child {
    /// Crea un nuevo `Child` a partir de un componente.
    pub fn with(component: impl Component) -> Self {
        Child(Some(Arc::new(component)))
    }

    // **< Child BUILDER >**************************************************************************

    /// Establece un componente nuevo, o lo vacía.
    ///
    /// Si se proporciona `Some(component)`, se encapsula como [`Child`]; y si es `None`, se limpia.
    #[builder_fn]
    pub fn with_component<C: Component>(mut self, component: Option<C>) -> Self {
        self.0 = component.map(|c| Arc::new(c) as Arc<dyn Component>);
        self
    }

    // **< Child GETTERS >**************************************************************************

    /// Devuelve el identificador del componente, si existe y está definido.
    #[inline]
    pub fn id(&self) -> Option<String> {
        self.0.as_ref().and_then(|c| c.id())
    }

    // **< Child RENDER >***************************************************************************

    /// Renderiza el componente con el contexto proporcionado.
    pub async fn render(&self, cx: &mut Context) -> Markup {
        match &self.0 {
            None => html! {},
            Some(m) => {
                let mut component = m.clone_box();
                component.render(cx).await
            }
        }
    }

    // **< Child HELPERS >**************************************************************************

    // Devuelve el [`UniqueId`] del tipo del componente, si el Child no está vacío.
    #[inline]
    fn type_id(&self) -> Option<UniqueId> {
        self.0.as_ref().map(|c| c.type_id())
    }
}

impl<C: Component + 'static> From<Embed<C>> for Child {
    /// Convierte un [`Embed<C>`] en un [`Child`], consumiendo el componente tipado.
    ///
    /// Útil cuando se tiene un [`Embed`] para añadir a una lista [`Children`]:
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let my_embed = Embed::with(Html::with(|_| html! { "Text" }));
    /// let children = Children::new().with_child(Child::from(my_embed));
    ///
    /// // De forma equivalente se puede usar la conversión implícita hacia `Child`:
    /// let my_embed = Embed::with(Html::with(|_| html! { "Text" }));
    /// let child: Child = my_embed.into();
    /// let children = children.with_child(child);
    /// ```
    fn from(embed: Embed<C>) -> Self {
        Child(embed.0.map(|arc| arc as Arc<dyn Component>))
    }
}

impl<T: Component + 'static> From<T> for Child {
    /// Convierte cualquier componente en un [`Child`], equivalente a [`Child::with()`].
    #[inline]
    fn from(component: T) -> Self {
        Child::with(component)
    }
}

impl<T: Component + 'static> From<T> for ChildOp {
    /// Convierte un componente en [`ChildOp::Add`], permitiendo pasar componentes directamente a
    /// métodos como [`Children::with_child`] sin envolverlos explícitamente.
    #[inline]
    fn from(component: T) -> Self {
        ChildOp::Add(Child::with(component))
    }
}

impl From<Child> for ChildOp {
    /// Convierte un [`Child`] en [`ChildOp::Add`].
    #[inline]
    fn from(child: Child) -> Self {
        ChildOp::Add(child)
    }
}

// **< Embed >**************************************************************************************

/// Contenedor tipado para un *único* componente de un tipo concreto conocido.
///
/// A diferencia de [`Child`], que encapsula cualquier componente como `dyn Component`, `Embed`
/// mantiene el tipo concreto `C` y permite acceder directamente a sus métodos específicos a través
/// de [`get()`](Embed::get).
///
/// Se usa habitualmente para incrustar un componente dentro de otro cuando no se necesita una lista
/// completa de hijos ([`Children`]), sino un único componente tipado en un campo concreto.
#[derive(AutoDefault)]
pub struct Embed<C: Component>(Option<Arc<C>>);

// Arc<C>: Clone no requiere C: Clone, pero #[derive(Clone)] añadiría ese bound innecesariamente.
impl<C: Component> Clone for Embed<C> {
    fn clone(&self) -> Self {
        Embed(self.0.clone())
    }
}

impl<C: Component> fmt::Debug for Embed<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            None => write!(f, "Embed(None)"),
            Some(c) => write!(f, "Embed({})", c.name()),
        }
    }
}

impl<C: Component> Embed<C> {
    /// Crea un nuevo `Embed` a partir de un componente.
    pub fn with(component: C) -> Self {
        Embed(Some(Arc::new(component)))
    }

    // **< Embed BUILDER >**************************************************************************

    /// Establece un componente nuevo, o lo vacía.
    ///
    /// Si se proporciona `Some(component)`, se encapsula como [`Embed`]; y si es `None`, se limpia.
    #[builder_fn]
    pub fn with_component(mut self, component: Option<C>) -> Self {
        self.0 = component.map(Arc::new);
        self
    }

    // **< Embed GETTERS >**************************************************************************

    /// Devuelve el identificador del componente, si existe y está definido.
    #[inline]
    pub fn id(&self) -> Option<String> {
        self.0.as_deref().and_then(|c| c.id())
    }

    /// Devuelve una referencia inmutable al componente incrustado, si existe.
    ///
    /// Para acceso mutable, usa [`get_mut`](Embed::get_mut).
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let embed = Embed::with(Html::with(|_| html! { "Prueba" }));
    /// if let Some(component) = embed.get() {
    ///     assert_eq!(component.name(), "Html");
    /// }
    /// ```
    pub fn get(&self) -> Option<&C> {
        self.0.as_deref()
    }

    /// Devuelve una referencia mutable al componente incrustado, si existe.
    ///
    /// Si el [`Arc`] interno es compartido (por ejemplo, justo después de clonar el componente
    /// padre), aplica *copy-on-write*: clona `C` antes de devolver la referencia mutable. El
    /// prototipo almacenado queda intacto.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let mut embed = Embed::with(Block::new().with_title(L10n::n("Title")));
    /// if let Some(component) = embed.get_mut() {
    ///     component.alter_title(L10n::n("New Title"));
    /// }
    /// ```
    pub fn get_mut(&mut self) -> Option<&mut C>
    where
        C: Clone,
    {
        self.0.as_mut().map(Arc::make_mut)
    }

    // **< Embed RENDER >***************************************************************************

    /// Renderiza el componente con el contexto proporcionado.
    pub async fn render(&self, cx: &mut Context) -> Markup {
        match &self.0 {
            None => html! {},
            Some(m) => {
                let mut component = m.clone_box();
                component.render(cx).await
            }
        }
    }
}

// **< Children >***********************************************************************************

/// Operaciones para componentes hijo [`Child`] en una lista [`Children`].
pub enum ChildOp {
    /// Añade un hijo al final de la lista.
    Add(Child),
    /// Añade un hijo sólo si la lista está vacía.
    AddIfEmpty(Child),
    /// Añade varios hijos al final de la lista, en el orden recibido.
    AddMany(Vec<Child>),
    /// Inserta un hijo justo después del componente con el `id` dado, o al final si no existe.
    InsertAfterId(&'static str, Child),
    /// Inserta un hijo justo antes del componente con el `id` dado, o al principio si no existe.
    InsertBeforeId(&'static str, Child),
    /// Inserta un hijo al principio de la lista.
    Prepend(Child),
    /// Inserta varios hijos al principio de la lista, manteniendo el orden recibido.
    PrependMany(Vec<Child>),
    /// Elimina el primer hijo con el `id` dado.
    RemoveById(&'static str),
    /// Sustituye el primer hijo con el `id` dado por otro componente.
    ReplaceById(&'static str, Child),
    /// Vacía la lista eliminando todos los hijos.
    Reset,
}

/// Lista ordenada de componentes hijo ([`Child`]) mantenida por un componente padre.
///
/// Permite añadir, modificar, renderizar y consultar componentes hijo en orden de inserción, con
/// soporte para operaciones avanzadas como inserción relativa o reemplazo por identificador a
/// través de [`ChildOp`].
///
/// Los tipos que completan este sistema son:
///
/// - [`Child`]: representa un componente hijo encapsulado dentro de la lista. Almacena cualquier
///   componente sin necesidad de conocer su tipo concreto.
/// - [`Embed<C>`]: contenedor tipado para un *único* componente de tipo `C`. Preferible a
///   `Children` cuando el padre sólo necesita un componente y quiere acceso directo a los métodos
///   de `C`.
/// - [`ChildOp`]: operaciones disponibles sobre la lista. Cuando se necesita algo más que añadir al
///   final, se construye la variante adecuada y se pasa a [`with_child`](Self::with_child).
///
/// # Conversiones implícitas
///
/// Cualquier componente implementa `Into<ChildOp>` (equivalente a `ChildOp::Add`) e `Into<Child>`.
/// Gracias a esto, [`with_child`](Self::with_child) acepta un componente directamente o cualquier
/// variante de [`ChildOp`]:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// // Añadir al final de la lista (implícito):
/// let children = Children::new().with_child(Html::new());
///
/// // Operación explícita:
/// let children = children.with_child(ChildOp::Prepend(Html::new().into()));
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct Children(Vec<Child>);

impl Children {
    /// Crea una lista vacía.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea una lista con un componente hijo inicial.
    pub fn with(child: Child) -> Self {
        Self::default().with_child(child)
    }

    // **< Children BUILDER >***********************************************************************

    /// Añade un componente hijo o aplica una operación [`ChildOp`] sobre la lista.
    #[builder_fn]
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        match op.into() {
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

    // Añade un componente hijo al final de la lista. También lo usa `core::theme::regions` al
    // fusionar regiones, fuera de este módulo.
    #[inline]
    pub(crate) fn add(&mut self, child: Child) -> &mut Self {
        self.0.push(child);
        self
    }

    // Añade un componente hijo en la lista sólo si está vacía.
    #[inline]
    pub(crate) fn add_if_empty(&mut self, child: Child) -> &mut Self {
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
    pub async fn render(&self, cx: &mut Context) -> Markup {
        html! {
            @for c in &self.0 {
                (c.render(cx).await)
            }
        }
    }

    // **< Children HELPERS >***********************************************************************

    // Añade más de un componente hijo al final de la lista (en el orden recibido).
    #[inline]
    fn add_many<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = Child>,
    {
        self.0.extend(iter);
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

    // Inserta un hijo al principio de la lista.
    #[inline]
    fn prepend(&mut self, child: Child) -> &mut Self {
        self.0.insert(0, child);
        self
    }

    // Inserta más de un componente hijo al principio de la lista (manteniendo el orden recibido).
    #[inline]
    fn prepend_many<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = Child>,
    {
        let buf: Vec<Child> = iter.into_iter().collect();
        self.0.splice(0..0, buf);
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
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let children = Children::new().with_child(Html::new()).with_child(Html::new());
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
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let children = Children::new().with_child(Html::new()).with_child(Html::new());
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
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// async fn render_all(mut children: Children, context: &mut Context) {
    ///     for child in &mut children {
    ///         child.render(context).await;
    ///     }
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
