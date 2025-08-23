use crate::{builder_fn, AutoDefault};

/// Operaciones disponibles sobre la lista de clases en [`AttrClasses`].
pub enum ClassesOp {
    /// Añade al final (si no existe).
    Add,
    /// Añade al principio.
    Prepend,
    /// Elimina coincidencias.
    Remove,
    /// Sustituye una o varias por las nuevas (`Replace("old other")`).
    Replace(String),
    /// Alterna presencia/ausencia.
    Toggle,
    /// Sustituye toda la lista.
    Set,
}

/// Cadena de clases CSS normalizadas para el atributo `class` de HTML.
///
/// Permite construir y modificar dinámicamente con [`ClassesOp`] una lista de clases CSS
/// normalizadas.
///
/// # Normalización
///
/// - El [orden de las clases no es relevante](https://stackoverflow.com/a/1321712) en CSS.
/// - No se permiten clases duplicadas.
/// - Las clases se convierten a minúsculas.
/// - Las clases vacías se ignoran.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// let classes = AttrClasses::new("Btn btn-primary")
///     .with_value(ClassesOp::Add, "Active")
///     .with_value(ClassesOp::Remove, "btn-primary");
///
/// assert_eq!(classes.get(), Some(String::from("btn active")));
/// assert!(classes.contains("active"));
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct AttrClasses(Vec<String>);

impl AttrClasses {
    pub fn new(classes: impl AsRef<str>) -> Self {
        AttrClasses::default().with_value(ClassesOp::Prepend, classes)
    }

    // AttrClasses BUILDER *************************************************************************

    #[builder_fn]
    pub fn with_value(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        let classes = classes.as_ref().to_ascii_lowercase();
        let classes: Vec<&str> = classes.split_ascii_whitespace().collect();

        if classes.is_empty() {
            return self;
        }

        match op {
            ClassesOp::Add => {
                self.add(&classes, self.0.len());
            }
            ClassesOp::Prepend => {
                self.add(&classes, 0);
            }
            ClassesOp::Remove => {
                for class in classes {
                    self.0.retain(|c| c.ne(&class.to_string()));
                }
            }
            ClassesOp::Replace(classes_to_replace) => {
                let mut pos = self.0.len();
                let replace: Vec<&str> = classes_to_replace.split_ascii_whitespace().collect();
                for class in replace {
                    if let Some(replace_pos) = self.0.iter().position(|c| c.eq(class)) {
                        self.0.remove(replace_pos);
                        if pos > replace_pos {
                            pos = replace_pos;
                        }
                    }
                }
                self.add(&classes, pos);
            }
            ClassesOp::Toggle => {
                for class in classes {
                    if !class.is_empty() {
                        if let Some(pos) = self.0.iter().position(|c| c.eq(class)) {
                            self.0.remove(pos);
                        } else {
                            self.0.push(class.to_string());
                        }
                    }
                }
            }
            ClassesOp::Set => {
                self.0.clear();
                self.add(&classes, 0);
            }
        }

        self
    }

    #[inline]
    fn add(&mut self, classes: &[&str], mut pos: usize) {
        for &class in classes {
            if !class.is_empty() && !self.0.iter().any(|c| c == class) {
                self.0.insert(pos, class.to_string());
                pos += 1;
            }
        }
    }

    // AttrClasses GETTERS *************************************************************************

    /// Devuele la cadena de clases, si existe.
    pub fn get(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.join(" "))
        }
    }

    /// Devuelve `true` si la clase está presente.
    pub fn contains(&self, class: impl AsRef<str>) -> bool {
        let class = class.as_ref();
        self.0.iter().any(|c| c == class)
    }
}
