use crate::{builder_fn, util, AutoDefault};

use std::borrow::Cow;
use std::collections::HashSet;

/// Operaciones disponibles sobre la lista de clases en [`Classes`].
///
/// Cada variante opera sobre **una o más clases** proporcionadas como una cadena separada por
/// espacios (p. ej. `"btn active"`), que se normalizan internamente a minúsculas en
/// [`Classes::with_classes()`].
#[derive(AutoDefault, Clone, Debug, PartialEq)]
pub enum ClassesOp {
    /// Añade las clases que no existan al final.
    #[default]
    Add,
    /// Añade las clases que no existan al principio.
    Prepend,
    /// Elimina las clases indicadas que existan.
    Remove,
    /// Sustituye una o varias clases existentes (indicadas en la variante) por las clases
    /// proporcionadas.
    Replace(Cow<'static, str>),
    /// Alterna presencia/ausencia de una o más clases.
    ///
    /// Si en una misma llamada se repite una clase (p. ej. `"a a"`) que ya existe, el resultado
    /// mantiene la pertenencia pero puede cambiar el orden (primero se elimina y luego se añade al
    /// final).
    Toggle,
    /// Sustituye la lista completa por las clases indicadas.
    Set,
}

/// Lista de clases CSS normalizadas para el atributo `class` de HTML.
///
/// Permite construir y modificar dinámicamente con [`ClassesOp`] una lista de clases CSS
/// normalizadas.
///
/// # Normalización
///
/// - Aunque el orden de las clases en el atributo `class` no afecta al resultado en CSS,
///   [`ClassesOp`] ofrece operaciones para controlar su orden de aparición por legibilidad.
/// - Solo se acepta una lista de clases con caracteres ASCII.
/// - Las clases se almacenan en minúsculas.
/// - No se permiten clases duplicadas tras la normalización (por ejemplo, `Btn` y `btn` se
///   consideran la misma clase).
/// - Las clases vacías se ignoran.
/// - Sin clases, [`get()`](Self::get) devuelve `None` (no `Some("")`).
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
        Self::default().with_classes(ClassesOp::default(), classes)
    }

    // **< Classes BUILDER >************************************************************************

    /// Modifica la lista de clases según la operación indicada.
    ///
    /// Realiza la operación indicada en `op` para las clases proporcionadas en `classes` sobre la
    /// lista de clases actual.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        let Some(normalized) =
            util::normalize_ascii_or_empty(classes.as_ref(), "Classes::with_classes")
        else {
            return self;
        };
        match op {
            ClassesOp::Add => {
                self.add(normalized.as_ref().split_ascii_whitespace(), self.0.len());
            }
            ClassesOp::Prepend => {
                self.add(normalized.as_ref().split_ascii_whitespace(), 0);
            }
            ClassesOp::Remove => {
                let mut classes_to_remove = normalized.as_ref().split_ascii_whitespace();

                // 0 clases: no se hace nada.
                let Some(first) = classes_to_remove.next() else {
                    return self;
                };

                // 1 clase: un único *retain*, sin reservas extra.
                let Some(second) = classes_to_remove.next() else {
                    self.0.retain(|c| c != first);
                    return self;
                };

                // 2+ clases: HashSet y un único *retain*.
                let mut to_remove: HashSet<&str> = HashSet::new();
                to_remove.insert(first);
                to_remove.insert(second);
                for class in classes_to_remove {
                    to_remove.insert(class);
                }
                self.0.retain(|c| !to_remove.contains(c.as_str()));
            }
            ClassesOp::Replace(classes_to_replace) => {
                let Some(classes_to_replace) = util::normalize_ascii_or_empty(
                    classes_to_replace.as_ref(),
                    "ClassesOp::Replace",
                ) else {
                    return self;
                };
                let mut pos = self.0.len();
                let mut replaced = false;
                for class in classes_to_replace.as_ref().split_ascii_whitespace() {
                    if let Some(replace_pos) = self.0.iter().position(|c| c == class) {
                        self.0.remove(replace_pos);
                        pos = pos.min(replace_pos);
                        replaced = true;
                    }
                }
                if replaced {
                    self.add(normalized.as_ref().split_ascii_whitespace(), pos);
                }
            }
            ClassesOp::Toggle => {
                for class in normalized.as_ref().split_ascii_whitespace() {
                    if let Some(pos) = self.0.iter().position(|c| c == class) {
                        self.0.remove(pos);
                    } else {
                        self.0.push(class.to_string());
                    }
                }
            }
            ClassesOp::Set => {
                self.0.clear();
                self.add(normalized.as_ref().split_ascii_whitespace(), 0);
            }
        }

        self
    }

    #[inline]
    fn add<'a, I>(&mut self, classes: I, mut pos: usize)
    where
        I: IntoIterator<Item = &'a str>,
    {
        for class in classes {
            // Inserción segura descartando duplicados.
            if !self.0.iter().any(|c| c == class) {
                let class = class.to_string();
                if pos >= self.0.len() {
                    self.0.push(class);
                } else {
                    self.0.insert(pos, class);
                }
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

    /// Devuelve `true` si la clase o **todas** las clases indicadas están presentes.
    pub fn contains(&self, classes: impl AsRef<str>) -> bool {
        let Ok(normalized) = util::normalize_ascii(classes.as_ref()) else {
            return false;
        };
        normalized
            .as_ref()
            .split_ascii_whitespace()
            .all(|class| self.0.iter().any(|c| c == class))
    }

    /// Devuelve `true` si la clase o **alguna** de las clases indicadas está presente.
    pub fn contains_any(&self, classes: impl AsRef<str>) -> bool {
        let Ok(normalized) = util::normalize_ascii(classes.as_ref()) else {
            return false;
        };
        normalized
            .as_ref()
            .split_ascii_whitespace()
            .any(|class| self.0.iter().any(|c| c == class))
    }
}
