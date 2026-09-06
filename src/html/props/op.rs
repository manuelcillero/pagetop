use crate::CowStr;
use crate::core::TypeInfo;
use crate::html::flex::FlexItem;
use crate::html::props::extra::PropsExtra;

use std::any::Any;
use std::sync::Arc;

/// Operaciones sobre el identificador, clases CSS, atributos HTML y valores extra en
/// [`Props`](crate::html::props::Props).
///
/// Cada variante lleva los datos necesarios para ejecutarse. El método recomendado para usarlas es
/// recurrir a los constructores asociados como [`set_id()`](Self::set_id),
/// [`add_classes()`](Self::add_classes), [`set()`](Self::set), etc.
///
/// Las variantes `*Id` operan sobre el atributo `id` del componente. Cuando se usa `"id"` como
/// nombre de atributo en `Set`, el valor se normaliza igual que [`SetId`](Self::SetId).
///
/// Las variantes `*Classes` gestionan la lista de clases CSS. Además, `Set("class", ...)`
/// reemplaza la lista completa de clases y `Remove("class")` la vacía.
///
/// Las variantes `*Style` gestionan las declaraciones de estilos para el atributo `style`, con una
/// propiedad cada vez. Además, `Set("style", ...)` reemplaza la lista completa de estilos y
/// `Remove("style")` la vacía.
///
/// Las variantes [`Set`](Self::Set) y [`Remove`](Self::Remove) son operaciones de propósito
/// general. `Set` añade o reemplaza cualquier atributo HTML por nombre y valor, y `Remove` lo
/// elimina. Los atributos `id`, `class` y `style` tienen semántica especial documentada en estas
/// variantes.
///
/// [`Rename`](Self::Rename) cambia el nombre de un atributo genérico conservando su valor; no
/// reconoce `"id"`, `"class"` ni `"style"` como origen ni destino. Está pensada para que un tema
/// traduzca a su propio vocabulario los atributos que un componente ya expone, sin que éste tenga
/// que conocer ningún tema en concreto.
///
/// Las variantes `*Extra` permiten añadir valores tipados usando una clave. Están pensadas para
/// ampliar el comportamiento de componentes ya existentes. Como no es posible añadir campos a la
/// estructura de un componente ya definido, temas y extensiones pueden definir un trait con nuevos
/// métodos que leen y escriben valores extra en [`Props`](crate::html::props::Props). Esos valores
/// se interpretan como si
/// fueran valores internos del componente para tomar decisiones durante el renderizado.
///
/// Finalmente, [`FlexItem`](Self::FlexItem) aplica un posicionamiento Flexbox a nivel de ítem sobre
/// cualquier componente.
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
    /// Añade una declaración de estilo (propiedad, valor) o sustituye su valor si la propiedad ya
    /// existe, conservando su posición; si no, se añade al final. A diferencia de las clases, el
    /// valor admite caracteres no ASCII (p. ej. `content`, `font-family`) y distingue mayúsculas y
    /// minúsculas. El nombre de la propiedad se normaliza a minúsculas. Si la propiedad o el valor
    /// quedan vacíos tras recortar espacios, la operación se ignora.
    AddStyle(CowStr, CowStr),
    /// Elimina la propiedad de estilo indicada, si existe.
    RemoveStyle(CowStr),
    /// Añade un atributo o sustituye su valor si ya existe.
    ///
    /// Usar `"id"` como nombre de atributo aplica al valor la misma normalización que
    /// [`SetId`](Self::SetId).
    ///
    /// Usar `"class"` como nombre de atributo reemplaza la lista completa de clases por las nuevas
    /// indicadas; la operación se ignora si el valor contiene caracteres no ASCII.
    ///
    /// Usar `"style"` como nombre de atributo reemplaza la lista completa de estilos por los nuevos
    /// indicados, interpretando el valor como declaraciones `"propiedad: valor"` separadas por `;`
    /// (igual que el propio atributo `style` HTML). El separador `;` respeta paréntesis y comillas,
    /// tal que valores como una *data URI* (`background: url(data:image/png;base64,...)`) o una
    /// cadena con `;` (`content: "a;b"`) se interpretan correctamente. En cualquier caso, se
    /// recomienda usar [`PropsOp::add_style()`](Self::add_style) para declarar estilos.
    Set(CowStr, CowStr),
    /// Si el primer atributo (origen) existe, lo renombra al segundo (destino), conservando su
    /// valor; si el destino ya tiene su propio valor, se respeta sin sobrescribir y sólo se elimina
    /// el origen. Si el origen no existe, la operación no tiene efecto.
    ///
    /// Sólo actúa sobre atributos genéricos, por lo que los nombres `"id"`, `"class"` y `"style"`
    /// no se reconocen como origen ni destino.
    Rename(CowStr, CowStr),
    /// Elimina el atributo indicado. Usar `"id"` elimina el identificador; usar `"class"` vacía la
    /// lista de clases; y usar `"style"` vacía la lista de estilos.
    Remove(CowStr),
    /// Almacena un valor extra tipado asociado a la clave indicada. Si ya existe uno con esa clave,
    /// lo reemplaza.
    SetExtra(&'static str, PropsExtra),
    /// Elimina el valor extra asociado a la clave indicada, si existe.
    RemoveExtra(&'static str),
    /// Aplica un posicionamiento [`FlexItem`] a un componente particular en un contenedor [`Flex`].
    /// Añade directamente sus estilos Flexbox al propio componente, sin usar clases CSS.
    ///
    /// Existe como variante de `PropsOp`, y no como método builder de un componente, porque las
    /// propiedades de un ítem Flexbox tienen sentido sobre **cualquier** componente que pueda
    /// añadirse como hijo de un contenedor Flex (por ejemplo `Button`, `Nav`, un componente de
    /// terceros, incluso otro componente que sea, a su vez, un contenedor Flex para sus propios
    /// hijos).
    ///
    /// No existe una variante equivalente `PropsOp::Flex` para el componente contenedor. No hace
    /// falta porque los componentes contenedores, como `Container` o `Navbar`, ofrecen su propio
    /// `with_flex()` tipado y con introspección (p. ej. [`Container::flex()`]).
    ///
    /// [`Flex`]: crate::html::flex::Flex
    /// [`Container::flex()`]: crate::base::component::Container::flex
    FlexItem(FlexItem),
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

    /// Crea la variante [`Set`](Self::Set) con nombre y valor del atributo.
    pub fn set(name: impl Into<CowStr>, value: impl Into<CowStr>) -> Self {
        Self::Set(name.into(), value.into())
    }

    /// Crea la variante [`Rename`](Self::Rename) para renombrar `from` a `to`.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let props = Props::new("data-dialog-toggle", "modal")
    ///     .with_prop(PropsOp::rename("data-dialog-toggle", "data-bs-toggle"));
    /// assert_eq!(props.get_prop("data-bs-toggle"), Some("modal".to_string()));
    /// assert_eq!(props.get_prop("data-dialog-toggle"), None);
    ///
    /// // Si el destino tiene su propio valor no se sobrescribe, sólo se elimina el origen.
    /// let props = Props::new("data-bs-toggle", "collapse")
    ///     .with_prop(PropsOp::set("data-dialog-toggle", "modal"))
    ///     .with_prop(PropsOp::rename("data-dialog-toggle", "data-bs-toggle"));
    /// assert_eq!(props.get_prop("data-bs-toggle"), Some("collapse".to_string()));
    /// assert_eq!(props.get_prop("data-dialog-toggle"), None);
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
}
