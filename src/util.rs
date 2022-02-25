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
    ( $($key:expr => $value:expr),* ) => {{
        let mut a = std::collections::HashMap::new();
        $(
            a.insert(String::from($key), $value.into());
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

pub fn optional_value(value: &str) -> Option<String> {
    let value = value.to_string();
    match value.is_empty() {
        true => None,
        false => Some(value),
    }
}

pub fn assigned_value(value: &Option<String>) -> &str {
    match value {
        Some(value) => value.as_str(),
        _ => "",
    }
}
