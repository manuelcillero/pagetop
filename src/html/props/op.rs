use crate::CowStr;
use crate::core::TypeInfo;
use crate::html::flex::FlexItem;
use crate::html::grid::GridItem;
use crate::html::props::extra::PropsExtra;
use crate::html::spacing::{Margin, Padding};
use crate::locale::{LangId, Lc};

use std::any::Any;
use std::borrow::Cow;
use std::sync::Arc;

// **< PropsOp >************************************************************************************

/// Operaciones sobre el identificador, clases CSS, atributos HTML y valores extra en
/// [`Props`](crate::html::props::Props).
///
/// Cada variante lleva los datos necesarios para ejecutarse. El método recomendado para usarlas es
/// recurrir a los constructores asociados como [`set_id()`](Self::set_id),
/// [`add_classes()`](Self::add_classes), [`set()`](Self::set), etc.
///
/// Las variantes `*Id` operan sobre el atributo `id` del componente, las variantes `*Classes` sobre
/// la lista de clases CSS y las variantes `*Style`/`*Styles` sobre las declaraciones del atributo
/// `style`.
///
/// Las variantes [`Set`](Self::Set), [`Rename`](Self::Rename) y [`Remove`](Self::Remove) son
/// operaciones de propósito general sobre el resto de atributos. `Set` añade o reemplaza cualquier
/// atributo HTML por nombre y valor, que puede ser texto fijo o un texto localizado
/// ([`set()`](Self::set)); `Rename` le cambia el nombre conservando su valor, y está pensada para
/// que un tema traduzca a su propio vocabulario los atributos que un componente ya expone, sin que
/// éste tenga que conocer ningún tema en concreto; y `Remove` lo elimina. Ninguna de las tres
/// admite `"id"`, `"class"` ni `"style"` (en mayúsculas o minúsculas), ya que tienen sus propias
/// variantes; con esos nombres, la operación se ignora (y falla en desarrollo y en pruebas).
///
/// Las variantes `*Extra` permiten añadir valores tipados usando una clave. Están pensadas para
/// ampliar el comportamiento de componentes ya existentes. Como no es posible añadir campos a la
/// estructura de un componente ya definido, temas y extensiones pueden definir un trait con nuevos
/// métodos que leen y escriben valores extra en [`Props`](crate::html::props::Props). Esos valores
/// se interpretan como si
/// fueran valores internos del componente para tomar decisiones durante el renderizado.
///
/// [`FlexItem`](Self::FlexItem) aplica un posicionamiento Flexbox a nivel de ítem,
/// [`GridItem`](Self::GridItem) aplica un posicionamiento CSS Grid a nivel de ítem, y
/// [`Margin`](Self::Margin) y [`Padding`](Self::Padding) aplican márgenes y relleno interno; los
/// cuatro sobre cualquier componente.
#[derive(Clone, Debug)]
pub enum PropsOp {
    /// Establece el identificador del componente normalizando el valor: recorta espacios, convierte
    /// a minúsculas y sustituye los espacios intermedios por `_`. Si el resultado es vacío, elimina
    /// el identificador.
    SetId(CowStr),
    /// Establece el identificador del componente **sólo si aún no hay ninguno definido**. Aplica la
    /// misma normalización que [`SetId`](Self::SetId); si el resultado es vacío, la operación no
    /// tiene efecto.
    EnsureId(CowStr),
    /// Añade la clase o clases que no existan al final de la lista. La operación se ignora si el
    /// valor contiene caracteres no ASCII.
    AddClasses(CowStr),
    /// Añade la clase o clases que no existan al principio de la lista. La operación se ignora si
    /// el valor contiene caracteres no ASCII.
    PrependClasses(CowStr),
    /// Sustituye **una o más** clases del primer valor por las clases indicadas en el segundo
    /// valor, insertando las nuevas en la posición de la primera clase a sustituir encontrada, con
    /// independencia del orden en que aparecen en el primer valor. Las que no existan se ignoran.
    /// Si **ninguna** de las clases a sustituir existe, la operación no tiene efecto y no se
    /// inserta nada. Se ignora si alguno de los dos valores contiene caracteres no ASCII.
    ReplaceClasses(CowStr, CowStr),
    /// A diferencia de [`ReplaceClasses`](Self::ReplaceClasses), exige que **todas** las clases del
    /// primer valor estén presentes, independientemente de su orden; si falta una sola, la
    /// operación no tiene efecto: ninguna clase se elimina ni se inserta. Si todas están presentes,
    /// las sustituye por las clases indicadas en el segundo valor, insertando las nuevas en la
    /// posición de la primera clase a sustituir encontrada. Se ignora si alguno de los dos valores
    /// contiene caracteres no ASCII.
    ReplaceAllClasses(CowStr, CowStr),
    /// Elimina la clase o clases indicadas de la lista. La operación se ignora si el valor contiene
    /// caracteres no ASCII.
    RemoveClasses(CowStr),
    /// Elimina todas las clases.
    ClearClasses,
    /// Añade una declaración de estilo (propiedad, valor) o sustituye su valor si la propiedad ya
    /// existe, conservando su posición; si no, se añade al final. A diferencia de las clases, el
    /// valor admite caracteres no ASCII (p. ej. `content`, `font-family`) y distingue mayúsculas y
    /// minúsculas. El nombre de la propiedad se normaliza a minúsculas. Si la propiedad o el valor
    /// quedan vacíos tras recortar espacios, la operación se ignora.
    AddStyle(CowStr, CowStr),
    /// Elimina la propiedad de estilo indicada, si existe.
    RemoveStyle(CowStr),
    /// Elimina todas las declaraciones de estilo.
    ClearStyles,
    /// Añade un atributo o sustituye su valor si ya existe.
    ///
    /// El valor es un [`PropValue`] para recoger un texto fijo o un texto localizado ([`Lc`]), que
    /// se traduce al renderizar con el idioma del [`Context`] y se omite si no tiene traducción. Un
    /// valor sustituye al anterior del mismo nombre, sea del tipo que sea.
    ///
    /// No admite `"id"`, `"class"` ni `"style"`, que ya tienen [`SetId`](Self::SetId),
    /// [`AddClasses`](Self::AddClasses) y [`AddStyle`](Self::AddStyle). Si se usan estos nombres,
    /// la operación se ignora (y falla en desarrollo y en pruebas).
    ///
    /// [`Context`]: crate::core::component::Context
    Set(CowStr, PropValue),
    /// Si el primer atributo (origen) existe, lo renombra al segundo (destino), conservando su
    /// valor; si el destino ya tiene su propio valor, se respeta sin sobrescribir y sólo se elimina
    /// el origen. Si el origen no existe, la operación no tiene efecto.
    ///
    /// No admite `"id"`, `"class"` ni `"style"` como origen ni como destino: la operación se ignora
    /// (y falla en desarrollo y en pruebas).
    Rename(CowStr, CowStr),
    /// Elimina el atributo indicado, si existe.
    ///
    /// No admite `"id"`, `"class"` ni `"style"`: se usan [`SetId`](Self::SetId) con un valor vacío,
    /// [`ClearClasses`](Self::ClearClasses) y [`ClearStyles`](Self::ClearStyles). Con esos nombres,
    /// la operación se ignora (y falla en desarrollo y en pruebas).
    Remove(CowStr),
    /// Almacena un valor extra tipado asociado a la clave indicada. Si ya existe uno con esa clave,
    /// lo reemplaza.
    SetExtra(&'static str, PropsExtra),
    /// Elimina el valor extra asociado a la clave indicada, si existe.
    RemoveExtra(&'static str),
    /// Aplica un posicionamiento [`FlexItem`] a un componente particular en un contenedor [`Flex`].
    /// Añade directamente sus estilos Flexbox al propio componente, generando clases CSS dinámicas.
    ///
    /// Existe como variante de `PropsOp`, y no como método builder de un componente, porque las
    /// propiedades de un ítem Flexbox tienen sentido sobre **cualquier** componente que pueda
    /// añadirse como hijo de un contenedor Flex (por ejemplo `Button`, `Nav`, un componente de
    /// terceros, incluso otro componente que sea, a su vez, un contenedor Flex o [`Grid`] para sus
    /// propios hijos).
    ///
    /// No existe una variante equivalente `PropsOp::Flex` para el contenedor, por el mismo motivo
    /// que no existe `PropsOp::Grid`. El único componente contenedor [`Flex`] ofrece sus propios
    /// constructores y builders tipados, con introspección (p. ej. [`Flex::direction()`]). Esto no
    /// afecta a [`GridItem`]; un componente puede llevar las dos variantes a la vez, con
    /// independencia de qué tipo de contenedor sea él mismo para sus propios hijos.
    ///
    /// [`Flex`]: crate::base::component::Flex
    /// [`Flex::direction()`]: crate::base::component::Flex::direction
    /// [`Grid`]: crate::base::component::Grid
    /// [`GridItem`]: Self::GridItem
    FlexItem(FlexItem),
    /// Aplica un posicionamiento [`GridItem`] a un componente particular en un contenedor
    /// [`Grid`]. Añade directamente sus estilos CSS Grid al propio componente, generando clases CSS
    /// dinámicas.
    ///
    /// Misma razón de ser que [`FlexItem`]; las propiedades de un ítem Grid tienen sentido sobre
    /// **cualquier** componente que pueda añadirse como hijo de un contenedor Grid (por ejemplo
    /// `Button`, `Nav`, un componente de terceros, incluso otro componente que sea, a su vez, un
    /// contenedor Grid o [`Flex`] para sus propios hijos).
    ///
    /// No existe una variante equivalente `PropsOp::Grid` para el contenedor, por el mismo motivo
    /// que no existe `PropsOp::Flex`. El único componente contenedor [`Grid`] ofrece sus propios
    /// constructores y builders tipados, con introspección (p. ej. [`Grid::columns()`]). Esto no
    /// afecta a [`FlexItem`]; un componente puede llevar las dos variantes a la vez, con
    /// independencia de qué tipo de contenedor sea él mismo para sus propios hijos.
    ///
    /// [`Grid`]: crate::base::component::Grid
    /// [`Grid::columns()`]: crate::base::component::Grid::columns
    /// [`Flex`]: crate::base::component::Flex
    /// [`FlexItem`]: Self::FlexItem
    GridItem(GridItem),
    /// Aplica un margen [`Margin`] a un componente particular. Añade directamente sus estilos al
    /// propio componente, generando clases CSS dinámicas.
    ///
    /// Como [`FlexItem`](Self::FlexItem) y [`GridItem`](Self::GridItem), existe como variante de
    /// `PropsOp` y no como método builder de un componente, porque el margen tiene sentido sobre
    /// **cualquier** componente.
    Margin(Margin),
    /// Aplica un relleno interno [`Padding`] a un componente particular, siguiendo los mismos
    /// criterios que [`Margin`](Self::Margin).
    Padding(Padding),
}

impl PropsOp {
    /// Crea la variante [`SetId`](Self::SetId) con el identificador indicado.
    pub fn set_id(id: impl Into<CowStr>) -> Self {
        Self::SetId(id.into())
    }

    /// Crea la variante [`EnsureId`](Self::EnsureId) con el identificador indicado.
    pub fn ensure_id(id: impl Into<CowStr>) -> Self {
        Self::EnsureId(id.into())
    }

    /// Crea la variante [`AddClasses`](Self::AddClasses) con la clase o clases indicadas.
    pub fn add_classes(classes: impl Into<CowStr>) -> Self {
        Self::AddClasses(classes.into())
    }

    /// Crea la variante [`PrependClasses`](Self::PrependClasses) con la clase o clases indicadas.
    pub fn prepend_classes(classes: impl Into<CowStr>) -> Self {
        Self::PrependClasses(classes.into())
    }

    /// Crea la variante [`ReplaceClasses`](Self::ReplaceClasses) con las clases a sustituir (`old`)
    /// y las nuevas clases (`new`).
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let props = Props::classes("button primary")
    ///     .with_prop(PropsOp::replace_classes("button", "btn"));
    /// assert_eq!(props.get_classes(), Some("btn primary".to_string()));
    ///
    /// // Basta con que exista alguna clase de `old` para aplicar el reemplazo.
    /// let props = Props::classes("btn primary")
    ///     .with_prop(PropsOp::replace_classes("primary secondary", "danger"));
    /// assert_eq!(props.get_classes(), Some("btn danger".to_string()));
    /// ```
    pub fn replace_classes(old: impl Into<CowStr>, new: impl Into<CowStr>) -> Self {
        Self::ReplaceClasses(old.into(), new.into())
    }

    /// Crea la variante [`ReplaceAllClasses`](Self::ReplaceAllClasses) con las clases a sustituir
    /// (`old`) y las nuevas clases (`new`).
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let props = Props::classes("btn primary")
    ///     .with_prop(PropsOp::replace_all_classes("btn primary", "btn danger"));
    /// assert_eq!(props.get_classes(), Some("btn danger".to_string()));
    ///
    /// // Si falta una sola clase de `old`, no hay reemplazo.
    /// let props = Props::classes("btn primary")
    ///     .with_prop(PropsOp::replace_all_classes("primary secondary", "danger"));
    /// assert_eq!(props.get_classes(), Some("btn primary".to_string()));
    /// ```
    pub fn replace_all_classes(old: impl Into<CowStr>, new: impl Into<CowStr>) -> Self {
        Self::ReplaceAllClasses(old.into(), new.into())
    }

    /// Crea la variante [`RemoveClasses`](Self::RemoveClasses) con la clase o clases indicadas.
    pub fn remove_classes(classes: impl Into<CowStr>) -> Self {
        Self::RemoveClasses(classes.into())
    }

    /// Crea la variante [`ClearClasses`](Self::ClearClasses).
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let props = Props::classes("btn primary")
    ///     .with_prop(PropsOp::clear_classes())
    ///     .with_prop(PropsOp::add_classes("card wide"));
    /// assert_eq!(props.get_classes(), Some("card wide".to_string()));
    /// ```
    pub fn clear_classes() -> Self {
        Self::ClearClasses
    }

    /// Crea la variante [`AddStyle`](Self::AddStyle) con la propiedad y el valor de estilo
    /// indicados.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let props = Props::default()
    ///     .with_prop(PropsOp::add_style("color", "red"))
    ///     .with_prop(PropsOp::add_style("font-weight", "bold"))
    ///     .with_prop(PropsOp::add_style("color", "blue"));
    /// assert_eq!(props.get_styles(), Some("color: blue; font-weight: bold".to_string()));
    /// ```
    pub fn add_style(property: impl Into<CowStr>, value: impl Into<CowStr>) -> Self {
        Self::AddStyle(property.into(), value.into())
    }

    /// Crea la variante [`RemoveStyle`](Self::RemoveStyle) para la propiedad de estilo indicada.
    pub fn remove_style(property: impl Into<CowStr>) -> Self {
        Self::RemoveStyle(property.into())
    }

    /// Crea la variante [`ClearStyles`](Self::ClearStyles).
    pub fn clear_styles() -> Self {
        Self::ClearStyles
    }

    /// Crea la variante [`Set`](Self::Set) con nombre y valor del atributo.
    ///
    /// El valor puede ser texto (`&'static str`, `String` o [`CowStr`]) o un texto localizado
    /// ([`Lc`]). Un [`Lc`] sirve para dar a cualquier componente un atributo traducido que no tenga
    /// como campo propio (`aria-label`, `placeholder`, `title`...), sin traducirlo antes: se
    /// traduce al renderizar, con el idioma del [`Context`] que recibe [`Props::unpack()`], y se
    /// escapa como cualquier otro valor. Si no tiene traducción, el atributo se omite.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let mut cx = Context::default().with_langid(&Locale::resolve("es-ES"));
    /// let props = Props::default()
    ///     .with_prop(PropsOp::set("hx-post", "/items/7/delete"))
    ///     .with_prop(PropsOp::set("title", Lc::l("button_delete")));
    ///
    /// let markup = html! { button (props.unpack(&mut cx)) { "X" } };
    /// assert_eq!(
    ///     markup.into_string(),
    ///     r#"<button hx-post="/items/7/delete" title="Eliminar">X</button>"#
    /// );
    /// ```
    ///
    /// [`Context`]: crate::core::component::Context
    /// [`Props::unpack()`]: crate::html::props::Props::unpack
    pub fn set(name: impl Into<CowStr>, value: impl Into<PropValue>) -> Self {
        Self::Set(name.into(), value.into())
    }

    /// Crea la variante [`Rename`](Self::Rename) para renombrar `from` a `to`.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let props = Props::new("data-dialog-toggle", "modal")
    ///     .with_prop(PropsOp::rename("data-dialog-toggle", "data-bs-toggle"));
    /// let value = props.get_prop("data-bs-toggle").and_then(|v| v.lookup(&Locale::default()));
    /// assert_eq!(value.as_deref(), Some("modal"));
    /// assert!(props.get_prop("data-dialog-toggle").is_none());
    ///
    /// // Si el destino tiene su propio valor no se sobrescribe, sólo se elimina el origen.
    /// let props = Props::new("data-bs-toggle", "collapse")
    ///     .with_prop(PropsOp::set("data-dialog-toggle", "modal"))
    ///     .with_prop(PropsOp::rename("data-dialog-toggle", "data-bs-toggle"));
    /// let value = props.get_prop("data-bs-toggle").and_then(|v| v.lookup(&Locale::default()));
    /// assert_eq!(value.as_deref(), Some("collapse"));
    /// assert!(props.get_prop("data-dialog-toggle").is_none());
    /// ```
    pub fn rename(from: impl Into<CowStr>, to: impl Into<CowStr>) -> Self {
        Self::Rename(from.into(), to.into())
    }

    /// Crea la variante [`Remove`](Self::Remove) para el atributo indicado.
    pub fn remove(name: impl Into<CowStr>) -> Self {
        Self::Remove(name.into())
    }

    /// Crea la variante [`SetExtra`](Self::SetExtra) con la clave y el valor indicados.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// const EXT_SIZE: &str = "myext.size";
    /// let props = Props::default().with_prop(PropsOp::set_extra(EXT_SIZE, 42_u32));
    /// assert_eq!(props.extra_or(EXT_SIZE, 0_u32), 42);
    /// ```
    pub fn set_extra<T: Any + Send + Sync + 'static>(key: &'static str, value: T) -> Self {
        Self::SetExtra(
            key,
            PropsExtra {
                value: Arc::new(value),
                type_name: TypeInfo::FullName.of::<T>(),
            },
        )
    }

    /// Crea la variante [`RemoveExtra`](Self::RemoveExtra) para la clave indicada.
    pub fn remove_extra(key: &'static str) -> Self {
        Self::RemoveExtra(key)
    }

    /// Crea la variante [`FlexItem`](Self::FlexItem) con el posicionamiento indicado.
    pub fn flex_item(placement: FlexItem) -> Self {
        Self::FlexItem(placement)
    }

    /// Crea la variante [`GridItem`](Self::GridItem) con el posicionamiento indicado.
    pub fn grid_item(placement: GridItem) -> Self {
        Self::GridItem(placement)
    }

    /// Crea la variante [`Margin`](Self::Margin) con el margen indicado.
    pub fn margin(margin: Margin) -> Self {
        Self::Margin(margin)
    }

    /// Crea la variante [`Padding`](Self::Padding) con el relleno interno indicado.
    pub fn padding(padding: Padding) -> Self {
        Self::Padding(padding)
    }
}

// **< PropValue >**********************************************************************************

/// Valor de un atributo HTML en [`PropsOp::Set`].
///
/// Puede ser un texto fijo o un texto localizado. Normalmente no se construye a mano:
/// [`PropsOp::set()`] lo crea a partir del valor que recibe, [`Text`](Self::Text) para
/// `&'static str`, `String` o [`CowStr`], y [`Lc`](Self::Lc) para un [`Lc`](crate::locale::Lc).
#[derive(Clone, Debug)]
pub enum PropValue {
    /// Texto que se escribe tal cual, escapado.
    Text(CowStr),
    /// Texto localizado, que se traduce al renderizar con el idioma del [`Context`]; si no tiene
    /// traducción, el atributo se omite.
    ///
    /// [`Context`]: crate::core::component::Context
    // En un `Arc` para que el enum ocupe lo mismo que `CowStr` y para que, al clonar el componente
    // para renderizarlo, no se copie la traducción con sus argumentos.
    Lc(Arc<Lc>),
}

impl PropValue {
    /// Devuelve el valor en el idioma indicado: el texto tal cual o la traducción del texto
    /// localizado, sin escapar.
    ///
    /// Devuelve `None` si el texto localizado no tiene traducción, el mismo caso en el que
    /// [`Props::unpack()`](crate::html::props::Props::unpack) omite el atributo. Como
    /// [`Lc::lookup()`](crate::locale::Lc::lookup), acepta cualquier fuente de idioma, también el
    /// [`Context`](crate::core::component::Context) del renderizado.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let es = Locale::resolve("es-ES");
    /// let title = PropValue::from(Lc::l("button_delete"));
    /// assert_eq!(title.lookup(&es).as_deref(), Some("Eliminar"));
    /// assert_eq!(PropValue::from("/items").lookup(&es).as_deref(), Some("/items"));
    /// ```
    pub fn lookup(&self, language: &impl LangId) -> Option<Cow<'_, str>> {
        match self {
            Self::Text(text) => Some(Cow::Borrowed(text.as_ref())),
            Self::Lc(lc) => lc.lookup(language).map(Cow::Owned),
        }
    }
}

impl From<&'static str> for PropValue {
    fn from(value: &'static str) -> Self {
        Self::Text(CowStr::Borrowed(value))
    }
}

impl From<String> for PropValue {
    fn from(value: String) -> Self {
        Self::Text(CowStr::Owned(value))
    }
}

impl From<CowStr> for PropValue {
    fn from(value: CowStr) -> Self {
        Self::Text(value)
    }
}

impl From<Lc> for PropValue {
    fn from(value: Lc) -> Self {
        Self::Lc(Arc::new(value))
    }
}
