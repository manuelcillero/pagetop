//! Functions and macro helpers.

use crate::Handle;

// *************************************************************************************************
// FUNCTIONS HELPERS.
// *************************************************************************************************

// https://stackoverflow.com/a/71464396
#[doc(hidden)]
pub const fn handle(
    module_path: &'static str,
    file: &'static str,
    line: u32,
    column: u32,
) -> Handle {
    let mut hash = 0xcbf29ce484222325;
    let prime = 0x00000100000001B3;

    let mut bytes = module_path.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(prime);
        i += 1;
    }

    bytes = file.as_bytes();
    i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(prime);
        i += 1;
    }

    hash ^= line as u64;
    hash = hash.wrapping_mul(prime);
    hash ^= column as u64;
    hash = hash.wrapping_mul(prime);
    hash
}

pub fn partial_type_name(type_name: &'static str, last: usize) -> &'static str {
    if last == 0 {
        return type_name;
    }
    let positions: Vec<_> = type_name.rmatch_indices("::").collect();
    if positions.len() < last {
        return type_name;
    }
    &type_name[(positions[last - 1].0 + 2)..]
}

pub fn single_type_name<T: ?Sized>() -> &'static str {
    partial_type_name(std::any::type_name::<T>(), 1)
}

// *************************************************************************************************
// MACRO HELPERS.
// *************************************************************************************************

#[macro_export]
/// Macro para construir grupos de pares clave-valor.
///
/// ```rust#ignore
/// let args = kv![
///     "userName" => "Roberto",
///     "photoCount" => 3,
///     "userGender" => "male",
/// ];
/// ```
macro_rules! kv {
    ( $($key:expr => $value:expr),* $(,)? ) => {{
        let mut a = std::collections::HashMap::new();
        $(
            a.insert($key.into(), $value.into());
        )*
        a
    }};
}

#[macro_export]
macro_rules! new_handle {
    ( $HANDLE:ident ) => {
        /// Public constant handle to represent a unique PageTop building element.
        pub const $HANDLE: $crate::Handle =
            $crate::util::handle(module_path!(), file!(), line!(), column!());
    };
    ( $HANDLE:ident for Crate ) => {
        /// Local constant handle to represent a unique PageTop building element.
        pub(crate) const $HANDLE: $crate::Handle =
            $crate::util::handle(module_path!(), file!(), line!(), column!());
    };
}
