use crate::{builder_fn, AutoDefault};

use std::borrow::Cow;

/// Operaciones disponibles sobre la lista de clases en [`Classes`].
pub enum ClassesOp {
    /// Añade al final (si no existe).
    Add,
    /// Añade al principio.
    Prepend,
    /// Elimina la(s) clase(s) indicada(s).
    Remove,
    /// Sustituye una o varias clases por otras nuevas (`Replace("old other".into())`).
    Replace(Cow<'static, str>),
    /// Alterna presencia/ausencia de una o más clases.
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
/// - El [orden de las clases no es relevante](https://stackoverflow.com/a/1321712) en CSS, pero
///   [`ClassesOp`] ofrece operaciones para controlar su orden de aparición.
/// - Las clases se convierten a minúsculas.
/// - No se permiten clases duplicadas.
/// - Las clases vacías se ignoran.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let classes = Classes::new("Btn btn-primary")
///     .with_classes(ClassesOp::Add, "Active")
///     .with_classes(ClassesOp::Remove, "btn-primary");
///
/// assert_eq!(classes.get(), Some("btn active".to_string()));
/// assert!(classes.contains("active"));
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct Classes(Vec<String>);

impl Classes {
    /// Crea una nueva lista de clases a partir de la clase o clases proporcionadas en `classes`.
    pub fn new(classes: impl AsRef<str>) -> Self {
        Self::default().with_classes(ClassesOp::Prepend, classes)
    }

    // **< Classes BUILDER >************************************************************************

    /// Modifica la lista de clases según la operación indicada.
    ///
    /// Realiza la operación indicada en `op` para las clases proporcionadas en `classes` sobre la
    /// lista de clases actual.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
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
                    self.0.retain(|c| c != class);
                }
            }
            ClassesOp::Replace(classes_to_replace) => {
                let mut pos = self.0.len();
                let replace = classes_to_replace.to_ascii_lowercase();
                let replace: Vec<&str> = replace.split_ascii_whitespace().collect();
                for class in replace {
                    if let Some(replace_pos) = self.0.iter().position(|c| c == class) {
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

    // **< Classes GETTERS >************************************************************************

    /// Devuelve la cadena de clases, si existe.
    pub fn get(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.join(" "))
        }
    }

    /// Devuelve `true` si la clase está presente.
    pub fn contains(&self, class: impl AsRef<str>) -> bool {
        let class = class.as_ref().to_ascii_lowercase();
        self.0.iter().any(|c| c == &class)
    }
}
