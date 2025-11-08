/// Añade a los *slices* de elementos [`AsRef<str>`] un método para unir clases CSS.
///
/// El método es [`join_classes()`](JoinClasses::join_classes), que une las cadenas **no vacías**
/// del *slice* usando un espacio como separador.
pub trait JoinClasses {
    /// Une las cadenas **no vacías** de un *slice* usando un espacio como separador.
    ///
    /// Son cadenas vacías únicamente los elementos del *slice* cuya longitud es `0` (p. ej., `""`);
    /// no se realiza recorte ni normalización, por lo que elementos como `" "` no se consideran
    /// vacíos.
    ///
    /// Si todas las cadenas están vacías, devuelve una cadena vacía. Acepta elementos que
    /// implementen [`AsRef<str>`] como `&str`, [`String`] o `Cow<'_, str>`.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let classes = ["btn", "", "btn-primary"];
    /// assert_eq!(classes.join_classes(), "btn btn-primary");
    ///
    /// let empty: [&str; 3] = ["", "", ""];
    /// assert_eq!(empty.join_classes(), "");
    ///
    /// let border = String::from("border");
    /// let border_top = String::from("border-top-0");
    /// let v = vec![&border, "", "", "", &border_top];
    /// assert_eq!(v.as_slice().join_classes(), "border border-top-0");
    ///
    /// // Elementos con espacios afectan al resultado.
    /// let spaced = ["btn", " ", "primary  "];
    /// assert_eq!(spaced.join_classes(), "btn   primary  ");
    /// ```
    fn join_classes(&self) -> String;
}

impl<T> JoinClasses for [T]
where
    T: AsRef<str>,
{
    #[inline]
    fn join_classes(&self) -> String {
        let mut count = 0usize;
        let mut total = 0usize;
        for s in self.iter().map(T::as_ref).filter(|s| !s.is_empty()) {
            count += 1;
            total += s.len();
        }
        if count == 0 {
            return String::new();
        }
        let separator = " ";
        let mut result = String::with_capacity(total + separator.len() * count.saturating_sub(1));
        for (i, s) in self
            .iter()
            .map(T::as_ref)
            .filter(|s| !s.is_empty())
            .enumerate()
        {
            if i > 0 {
                result.push_str(separator);
            }
            result.push_str(s);
        }
        result
    }
}
