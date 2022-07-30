pub type Handler = u64;

// https://stackoverflow.com/a/71464396
pub const fn handler(
    module_path: &'static str,
    file: &'static str,
    line: u32,
    column: u32,
) -> Handler {
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

#[macro_export]
macro_rules! pub_const_handler {
    ( $HANDLER:ident ) => {
        pub const $HANDLER: $crate::util::Handler =
            $crate::util::handler(module_path!(), file!(), line!(), column!());
    };
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

#[macro_export]
macro_rules! theme_static_files {
    ( $cfg:ident, $dir:expr ) => {{
        let static_files = &$crate::config::SETTINGS.dev.static_files;
        if static_files.is_empty() {
            $cfg.service($crate::app::ResourceFiles::new($dir, generate()));
        } else {
            $cfg.service(
                $crate::app::ActixFiles::new($dir, $crate::concat_string!(static_files, $dir))
                    .show_files_listing(),
            );
        }
    }};
}
