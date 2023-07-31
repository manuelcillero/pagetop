//! Funciones útiles.

use crate::Handle;

// *************************************************************************************************
// FUNCIONES ÚTILES.
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
// MACROS DECLARATIVAS.
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
/// Define un conjunto de ajustes de configuración usando tipos seguros y valores predefinidos.
///
/// Detiene la aplicación con un panic! si no pueden asignarse los ajustes de configuración.
///
/// Ver [`Cómo añadir ajustes de configuración`](config/index.html#cómo-añadir-ajustes-de-configuración).
macro_rules! default_settings {
    ( $($key:literal => $value:literal),* $(,)? ) => {
        #[doc = concat!(
            "Assigned or predefined values for configuration settings associated to the ",
            "[`Settings`] type."
        )]
        pub static SETTINGS: $crate::LazyStatic<Settings> = $crate::LazyStatic::new(|| {
            let mut settings = $crate::config::CONFIG.clone();
            $(
                settings.set_default($key, $value).unwrap();
            )*
            match settings.try_into() {
                Ok(s) => s,
                Err(e) => panic!("Error parsing settings: {}", e),
            }
        });
    };
}

#[macro_export]
macro_rules! new_handle {
    ( $HANDLE:ident ) => {
        /// Public constant handle to represent a unique PageTop building element.
        pub const $HANDLE: $crate::Handle =
            $crate::util::handle(module_path!(), file!(), line!(), column!());
    };
    ( $HANDLE:ident for Action ) => {
        /// Constant handle to represent a unique PageTop action.
        pub(crate) const $HANDLE: $crate::Handle =
            $crate::util::handle(module_path!(), file!(), line!(), column!());
    };
}

#[macro_export]
/// Define un conjunto de elementos de localización y funciones locales de traducción.
macro_rules! static_locales {
    ( $LOCALES:ident $(, $core_locales:literal)? ) => {
        use $crate::locale::*;

        fluent_templates::static_loader! {
            static $LOCALES = {
                locales: "src/locale",
                $( core_locales: $core_locales, )?
                fallback_language: "en-US",

                // Elimina las marcas Unicode que delimitan los argumentos.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }
    };
    ( $LOCALES:ident in $dir_locales:literal $(, $core_locales:literal)? ) => {
        use $crate::locale::*;

        fluent_templates::static_loader! {
            static $LOCALES = {
                locales: $dir_locales,
                $( core_locales: $core_locales, )?
                fallback_language: "en-US",

                // Elimina las marcas Unicode que delimitan los argumentos.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }
    };
}

#[macro_export]
macro_rules! static_files {
    ( $bundle:ident ) => {
        $crate::paste! {
            mod [<static_files_ $bundle>] {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
            }
        }
    };
    ( $bundle:ident => $STATIC:ident ) => {
        $crate::paste! {
            mod [<static_files_ $bundle>] {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
            }
            static $STATIC: LazyStatic<HashMapResources> = LazyStatic::new([
                <static_files_ $bundle>]::$bundle
            );
        }
    };
}

#[macro_export]
macro_rules! serve_static_files {
    ( $scfg:ident, $path:expr, $bundle:ident ) => {{
        $crate::paste! {
            let static_files = &$crate::config::SETTINGS.dev.static_files;
            if static_files.is_empty() {
                $scfg.service($crate::service::ResourceFiles::new(
                    $path,
                    [<static_files_ $bundle>]::$bundle(),
                ));
            } else {
                $scfg.service(
                    $crate::service::ActixFiles::new(
                        $path,
                        $crate::concat_string!(static_files, $path),
                    )
                    .show_files_listing(),
                );
            }
        }
    }};
}
