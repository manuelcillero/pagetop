//! Funciones y macros útiles.

// MACROS ÚTILES ***********************************************************************************

#[macro_export]
/// Macro para construir una colección de pares clave-valor.
///
/// ```rust
/// use pagetop::hm;
/// use std::collections::HashMap;
///
/// let args:HashMap<&str, String> = hm![
///     "userName"   => "Roberto",
///     "photoCount" => "3",
///     "userGender" => "male",
/// ];
/// ```
macro_rules! hm {
    ( $($key:expr => $value:expr),* $(,)? ) => {{
        let mut a = std::collections::HashMap::new();
        $(
            a.insert($key.into(), $value.into());
        )*
        a
    }};
}
