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
macro_rules! module_name {
    () => {{
        let name = module_path!();
        match name.rfind("::") {
            Some(position) => &name[(position + 2)..],
            None => name
        }
    }};
}

#[macro_export]
macro_rules! theme_static_files {
    ( $cfg:ident, $dir:expr ) => {{
        let static_files = &$crate::config::SETTINGS.dev.static_files;
        if static_files.is_empty() {
            $cfg.service(actix_web_static_files::ResourceFiles::new(
                $dir,
                generate()
            ));
        } else {
            $cfg.service(actix_files::Files::new(
                $dir,
                &[static_files, $dir].join("")
            ).show_files_listing());
        }
    }};
}
