//! Tipos y funciones esenciales para crear extensiones.

use std::any::Any;

/// Selector para identificar segmentos de la ruta de un tipo.
#[derive(Clone, Copy, Debug)]
pub enum TypeInfo {
    /// Ruta completa tal y como la devuelve [`core::any::type_name`].
    FullName,
    /// Último segmento de la ruta – por ejemplo `Vec<i32>` en lugar de `alloc::vec::Vec<i32>`.
    ShortName,
    /// Conserva todo **desde** `start` inclusive hasta el final.
    NameFrom(isize),
    /// Conserva todo **hasta e incluyendo** `end`.
    NameTo(isize),
    /// Conserva la subruta comprendida entre `start` y `end` (ambos inclusive).
    PartialName(isize, isize),
}

impl TypeInfo {
    /// Devuelve el segmento solicitado de la ruta para el tipo `T`.
    pub fn of<T: ?Sized>(&self) -> &'static str {
        let type_name = std::any::type_name::<T>();
        match self {
            TypeInfo::FullName => type_name,
            TypeInfo::ShortName => Self::partial(type_name, -1, None),
            TypeInfo::NameFrom(start) => Self::partial(type_name, *start, None),
            TypeInfo::NameTo(end) => Self::partial(type_name, 0, Some(*end)),
            TypeInfo::PartialName(start, end) => Self::partial(type_name, *start, Some(*end)),
        }
    }

    // Extrae un rango de segmentos de `type_name` (tokens separados por `::`).
    //
    // Los argumentos `start` y `end` identifican los índices de los segmentos teniendo en cuenta:
    //
    // * Los índices positivos cuentan **desde la izquierda**, empezando en `0`.
    // * Los índices negativos cuentan **desde la derecha**, `-1` es el último.
    // * Si `end` es `None`, el corte llega hasta el último segmento.
    // * Si la selección resulta vacía por índices desordenados o segmento inexistente, se devuelve
    //   la cadena vacía.
    //
    // Ejemplos (con `type_name = "alloc::vec::Vec<i32>"`):
    //
    // | Llamada                      | Resultado                |
    // |------------------------------|--------------------------|
    // | `partial(...,  0, None)`     | `"alloc::vec::Vec<i32>"` |
    // | `partial(...,  1, None)`     | `"vec::Vec<i32>"`        |
    // | `partial(..., -1, None)`     | `"Vec<i32>"`             |
    // | `partial(...,  0, Some(-2))` | `"alloc::vec"`           |
    // | `partial(..., -5, None)`     | `"alloc::vec::Vec<i32>"` |
    //
    // La porción devuelta vive tanto como `'static` porque `type_name` es `'static` y sólo se
    // presta.
    fn partial(type_name: &'static str, start: isize, end: Option<isize>) -> &'static str {
        let maxlen = type_name.len();

        // Localiza los límites de cada segmento a nivel 0 de `<…>`.
        let mut segments = Vec::new();
        let mut segment_start = 0; // Posición inicial del segmento actual.
        let mut angle_brackets = 0; // Profundidad dentro de '<…>'.
        let mut previous_char = '\0'; // Se inicializa a carácter nulo, no hay aún carácter previo.

        for (idx, c) in type_name.char_indices() {
            match c {
                ':' if angle_brackets == 0 => {
                    if previous_char == ':' {
                        if segment_start < idx - 1 {
                            segments.push((segment_start, idx - 1)); // No incluye último '::'.
                        }
                        segment_start = idx + 1; // Nuevo segmento tras '::'.
                    }
                }
                '<' => angle_brackets += 1,
                '>' => angle_brackets -= 1,
                _ => {}
            }
            previous_char = c;
        }

        // Incluye el último segmento si lo hubiese.
        if segment_start < maxlen {
            segments.push((segment_start, maxlen));
        }

        // Calcula la posición inicial.
        let start_pos = segments
            .get(if start >= 0 {
                start as usize
            } else {
                segments.len().saturating_sub(start.unsigned_abs())
            })
            .map_or(0, |&(s, _)| s);

        // Calcula la posición final.
        let end_pos = segments
            .get(if let Some(end) = end {
                if end >= 0 {
                    end as usize
                } else {
                    segments.len().saturating_sub(end.unsigned_abs())
                }
            } else {
                segments.len() - 1
            })
            .map_or(maxlen, |&(_, e)| e);

        // Devuelve la cadena parcial basada en las posiciones calculadas.
        if start_pos >= end_pos {
            return "";
        }
        &type_name[start_pos..end_pos]
    }
}

/// Proporciona información de tipo en tiempo de ejecución y conversión dinámica de tipos.
///
/// Este *trait* se implementa automáticamente para **todos** los tipos que implementen [`Any`], de
/// modo que basta con traer [`AnyInfo`] al ámbito (`use crate::AnyInfo;`) para disponer de estos
/// métodos adicionales, o usar el [`prelude`](crate::prelude) de `PageTop`.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// let n = 3u32;
/// assert_eq!(n.type_name(), "u32");
/// ```
pub trait AnyInfo: Any {
    /// Devuelve el nombre totalmente cualificado del tipo.
    fn type_name(&self) -> &'static str;

    /// Devuelve el nombre corto del tipo (último segmento del nombre).
    fn short_name(&self) -> &'static str;

    /// Devuelve una referencia a `dyn Any` para la conversión dinámica de tipos.
    fn as_any_ref(&self) -> &dyn Any;

    /// Devuelve una referencia mutable a `dyn Any` para la conversión dinámica de tipos.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AnyInfo for T {
    #[inline]
    fn type_name(&self) -> &'static str {
        TypeInfo::FullName.of::<T>()
    }

    #[inline]
    fn short_name(&self) -> &'static str {
        TypeInfo::ShortName.of::<T>()
    }

    #[inline]
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Extiende [`AnyInfo`] con utilidades de *downcasting* para conversión de tipos.
///
/// Preferible a usar directamente `Any::downcast_ref` porque conserva el *trait bound* [`AnyInfo`],
/// lo que permite seguir llamando a `type_name`, etc.
pub trait AnyCast: AnyInfo {
    /// Comprueba si la instancia subyacente es de tipo `T`.
    #[inline]
    fn is<T>(&self) -> bool
    where
        T: AnyInfo,
    {
        self.as_any_ref().is::<T>()
    }

    /// Intenta hacer *downcast* de un objeto para obtener una referencia de tipo `T`.
    #[inline]
    #[must_use]
    fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: AnyInfo,
    {
        self.as_any_ref().downcast_ref()
    }

    /// Intenta hacer *downcast* de un objeto para obtener una referencia mutable de tipo `T`.
    #[inline]
    #[must_use]
    fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: AnyInfo,
    {
        self.as_any_mut().downcast_mut()
    }
}

/// Implementación automática para cualquier tipo que ya cumpla [`AnyInfo`].
impl<T: ?Sized + AnyInfo> AnyCast for T {}

// Infraestructura para ampliar funcionalidades mediante extensiones.
pub mod extension;
