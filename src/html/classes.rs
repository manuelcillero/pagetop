use crate::{builder_fn, AutoDefault};

use std::borrow::Cow;

/// Operaciones disponibles sobre la lista de clases en [`Classes`].
///
/// Cada variante opera sobre **una o más clases** proporcionadas como una cadena separada por
/// espacios (p. ej. `"btn active"`), que se normalizan internamente a minúsculas en
/// [`Classes::with_classes()`].
pub enum ClassesOp {
    /// Añade las clases que no existan al final.
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
        Self::default().with_classes(ClassesOp::Prepend, classes)
    }

    // **< Classes BUILDER >************************************************************************

    /// Modifica la lista de clases según la operación indicada.
    ///
    /// Realiza la operación indicada en `op` para las clases proporcionadas en `classes` sobre la
    /// lista de clases actual.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        let classes = classes.as_ref();
        match op {
            ClassesOp::Add => {
                self.add(classes, self.0.len());
            }
            ClassesOp::Prepend => {
                self.add(classes, 0);
            }
            ClassesOp::Remove => {
                let mut classes_to_remove = classes.split_ascii_whitespace();

                // 0 clases: no se hace nada.
                let Some(first) = classes_to_remove.next() else {
                    return self;
                };

                // 1 clase: un único *retain*, cero reservas extra.
                let first = first.to_ascii_lowercase();
                let Some(second) = classes_to_remove.next() else {
                    self.0.retain(|c| c != &first);
                    return self;
                };

                // 2+ clases: se construye lista para borrar y un único *retain*.
                let mut to_remove = Vec::new();
                to_remove.push(first);
                to_remove.push(second.to_ascii_lowercase());
                for class in classes_to_remove {
                    to_remove.push(class.to_ascii_lowercase());
                }
                self.0.retain(|c| !to_remove.iter().any(|r| r == c));
            }
            ClassesOp::Replace(classes_to_replace) => {
                let mut pos = self.0.len();
                for class in classes_to_replace.split_ascii_whitespace() {
                    let class = class.to_ascii_lowercase();
                    if let Some(replace_pos) = self.0.iter().position(|c| c == &class) {
                        self.0.remove(replace_pos);
                        pos = pos.min(replace_pos);
                    }
                }
                self.add(classes, pos);
            }
            ClassesOp::Toggle => {
                for class in classes.split_ascii_whitespace() {
                    let class = class.to_ascii_lowercase();
                    if let Some(pos) = self.0.iter().position(|c| c == &class) {
                        self.0.remove(pos);
                    } else {
                        self.0.push(class);
                    }
                }
            }
            ClassesOp::Set => {
                self.0.clear();
                self.add(classes, 0);
            }
        }

        self
    }

    #[inline]
    fn add(&mut self, classes: &str, mut pos: usize) {
        for class in classes.split_ascii_whitespace() {
            let class = class.to_ascii_lowercase();
            // Inserción segura descartando duplicados.
            if !self.0.iter().any(|c| c == &class) {
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

    /// Devuelve `true` si **una única clase** está presente.
    ///
    /// Si necesitas comprobar varias clases, usa [`contains_all()`](Self::contains_all) o
    /// [`contains_any()`](Self::contains_any).
    pub fn contains(&self, class: impl AsRef<str>) -> bool {
        self.contains_class(class.as_ref())
    }

    /// Devuelve `true` si **todas** las clases indicadas están presentes.
    pub fn contains_all(&self, classes: impl AsRef<str>) -> bool {
        classes
            .as_ref()
            .split_ascii_whitespace()
            .all(|class| self.contains_class(class))
    }

    /// Devuelve `true` si **alguna** de las clases indicadas está presente.
    pub fn contains_any(&self, classes: impl AsRef<str>) -> bool {
        classes
            .as_ref()
            .split_ascii_whitespace()
            .any(|class| self.contains_class(class))
    }

    #[inline]
    fn contains_class(&self, class: &str) -> bool {
        self.0.iter().any(|c| c.eq_ignore_ascii_case(class))
    }
}
