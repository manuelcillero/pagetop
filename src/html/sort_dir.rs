use crate::AutoDefault;

/// Representa una dirección de ordenación (ascendente o descendente).
///
/// Es un tipo definido exclusivamente para trabajar con datos ordenables. No depende de ningún
/// componente. Sirve tanto para representar el estado de un listado ordenable en la capa de
/// servicio (interpretando el valor de la *query string* con [`from_query()`](Self::from_query) y
/// serializándolo con [`as_str()`](Self::as_str)), como para calcular la dirección que debe llevar
/// un enlace de ordenación de la interfaz si se vuelve a pulsar ([`toggled()`](Self::toggled),
/// [`next_for()`](Self::next_for)).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum SortDir {
    /// Orden ascendente. Es la dirección por defecto.
    #[default]
    Asc,
    /// Orden descendente.
    Desc,
}

impl SortDir {
    /// Interpreta el valor del parámetro de ordenación procedente de una *query string*
    /// (`"asc"`/`"desc"`).
    ///
    /// Cualquier valor distinto de `"desc"` (incluido `None`, una cadena vacía o un valor no
    /// reconocido) se interpreta como [`Asc`](Self::Asc).
    ///
    /// ```rust
    /// use pagetop::html::SortDir;
    ///
    /// assert_eq!(SortDir::from_query(Some("desc")), SortDir::Desc);
    /// assert_eq!(SortDir::from_query(Some("asc")), SortDir::Asc);
    /// assert_eq!(SortDir::from_query(Some("")), SortDir::Asc);
    /// assert_eq!(SortDir::from_query(None), SortDir::Asc);
    /// ```
    #[inline]
    pub fn from_query(value: Option<&str>) -> Self {
        match value {
            Some("desc") => SortDir::Desc,
            _ => SortDir::Asc,
        }
    }

    /// Devuelve el valor de esta dirección tal como se representa en una *query string*
    /// (`"asc"`/`"desc"`).
    ///
    /// ```rust
    /// use pagetop::html::SortDir;
    ///
    /// assert_eq!(SortDir::Asc.as_str(), "asc");
    /// assert_eq!(SortDir::Desc.as_str(), "desc");
    /// ```
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            SortDir::Asc => "asc",
            SortDir::Desc => "desc",
        }
    }

    /// Devuelve la dirección contraria a esta.
    ///
    /// ```rust
    /// use pagetop::html::SortDir;
    ///
    /// assert_eq!(SortDir::Asc.toggled(), SortDir::Desc);
    /// assert_eq!(SortDir::Desc.toggled(), SortDir::Asc);
    /// ```
    #[inline]
    pub const fn toggled(self) -> Self {
        match self {
            SortDir::Asc => SortDir::Desc,
            SortDir::Desc => SortDir::Asc,
        }
    }

    /// Calcula la dirección que debe llevar un enlace de ordenación si se vuelve a pulsar, dado su
    /// estado actual (`current`) que puede ser `Some(dir)` si el listado ya está ordenado por este
    /// enlace, o `None` si el orden vigente lo determina otro campo.
    ///
    /// Si el enlace ya ordena, [alterna la dirección](Self::toggled); si no, siempre empieza en
    /// [`Asc`](Self::Asc), sea cual sea la dirección vigente del listado en conjunto.
    ///
    /// ```rust
    /// use pagetop::html::SortDir;
    ///
    /// // Otro campo determina el orden vigente: el próximo clic aquí empieza en ascendente.
    /// assert_eq!(SortDir::next_for(None), SortDir::Asc);
    ///
    /// // Este mismo campo ya ordena en ascendente: el próximo clic debería invertirlo.
    /// assert_eq!(SortDir::next_for(Some(SortDir::Asc)), SortDir::Desc);
    /// assert_eq!(SortDir::next_for(Some(SortDir::Desc)), SortDir::Asc);
    /// ```
    #[inline]
    pub const fn next_for(current: Option<SortDir>) -> SortDir {
        match current {
            Some(dir) => dir.toggled(),
            None => SortDir::Asc,
        }
    }
}

/// Permite pasar un [`SortDir`] allí donde se espere `impl AsRef<str>`, por ejemplo, en el
/// parámetro `value` de [`RoutePath::with_param()`](crate::html::RoutePath::with_param) o su
/// equivalente `alter_param()`, sin tener que escribir `.as_str()` a mano.
///
/// ```rust
/// use pagetop::html::SortDir;
///
/// assert_eq!(SortDir::Asc.as_ref(), "asc");
/// assert_eq!(SortDir::Desc.as_ref(), "desc");
/// ```
impl AsRef<str> for SortDir {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Permite pasar un [`SortDir`] allí donde se espere `impl Into<String>`, por ejemplo, en
/// [`Pager::with_extra_query()`](crate::base::component::Pager::with_extra_query), sin tener que
/// escribir `as_str().to_owned()` a mano.
///
/// ```rust
/// use pagetop::html::SortDir;
///
/// assert_eq!(String::from(SortDir::Asc), "asc");
/// assert_eq!(String::from(SortDir::Desc), "desc");
/// ```
impl From<SortDir> for String {
    fn from(dir: SortDir) -> Self {
        dir.as_str().to_owned()
    }
}
