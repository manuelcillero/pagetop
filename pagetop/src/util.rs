#[macro_export]
/// Macro para construir grupos de pares clave-valor.
///
/// ```
/// let args = args![
///     "userName" => "Roberto",
///     "photoCount" => 3,
///     "userGender" => "male"
/// ];
/// ```
macro_rules! args {
    ( $($KEY:expr => $VALUE:expr),* ) => {{
        let mut a = std::collections::HashMap::new();
        $(
            a.insert(String::from($KEY), $VALUE.into());
        )*
        a
    }};
}

pub fn valid_id(id: &str) -> Option<String> {
    let id = id.trim().replace(" ", "_").to_lowercase();
    match id.is_empty() {
        true => None,
        false => Some(id),
    }
}

pub fn optional_str(s: &str) -> Option<String> {
    let s = s.to_owned();
    match s.is_empty() {
        true => None,
        false => Some(s),
    }
}

pub fn assigned_str(optional: &Option<String>) -> &str {
    match optional {
        Some(o) => o.as_str(),
        _ => "",
    }
}
