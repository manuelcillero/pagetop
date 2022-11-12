pub use static_files::Resource as StaticResource;

use std::path::Path;

pub type HashMapResources = std::collections::HashMap<&'static str, StaticResource>;

/// This function uses the [static_files](https://docs.rs/static-files/latest/static_files/) library
/// to embed at compile time a bundle of static files in your binary.
///
/// Just create folder with static resources in your project (for example `static`):
///
/// ```bash
/// cd project_dir
/// mkdir static
/// echo "Hello, world" > static/hello
/// ```
///
/// Add to `Cargo.toml` the required dependencies:
///
/// ```toml
/// [dependencies]
/// pagetop = { ... }
/// static-files = "0.2.3"
///
/// [build-dependencies]
/// pagetop = { ... }
/// ```
///
/// Add `build.rs` with call to bundle resources (*guides* will be the magic word in this example):
///
/// ```rust#ignore
/// use pagetop::util::bundle_resources;
///
/// fn main() -> std::io::Result<()> {
///     bundle_resources("./static", "guides", None)
/// }
/// ```
///
/// Optionally, you can pass a function to filter those files into the `./static` folder which
/// should be included in the resources file:
///
/// ```rust#ignore
/// use pagetop::util::bundle_resources;
///
/// fn main() -> std::io::Result<()> {
///     bundle_resources("./static", "guides", Some(except_css_dir))
/// }
///
/// fn except_css_dir(p: &Path) -> bool {
///     if let Some(parent) = p.parent() {
///         !matches!(parent.to_str(), Some("/css"))
///     }
///     true
/// }
/// ```
///
/// This will create a file called `guides.rs` in the standard directory
/// [OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html) where all
/// intermediate and output artifacts are placed during compilation.
///
/// You don't need to access this file, just include it in your project source code and a module called
/// `resources_guides` will be added. Then simply reference the `bundle_guides` function to embed
/// the generated HashMap resources collection:
///
/// ```rust#ignore
/// use pagetop::prelude::*;
///
/// include!(concat!(env!("OUT_DIR"), "/guides.rs"));
/// static RESOURCES: LazyStatic<HashMapResources> = LazyStatic::new(bundle_guides);
/// ```
///
/// You can build more than one resources file to compile with your project.
pub fn bundle_resources(
    from_dir: &str,
    with_name: &str,
    filtering: Option<fn(p: &Path) -> bool>,
) -> std::io::Result<()> {
    let mut bundle = static_files::resource_dir(from_dir);
    bundle.with_generated_filename(
        Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join(format!("{}.rs", with_name)),
    );
    bundle.with_module_name(format!("resources_{}", with_name));
    bundle.with_generated_fn(format!("bundle_{}", with_name));
    if let Some(filter_files) = filtering {
        bundle.with_filter(filter_files);
    }
    bundle.build()
}

pub type Handle = u64;

// https://stackoverflow.com/a/71464396
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

#[macro_export]
macro_rules! pub_handle {
    ( $HANDLE:ident ) => {
        pub const $HANDLE: $crate::util::Handle =
            $crate::util::handle(module_path!(), file!(), line!(), column!());
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
/// ```rust#ignore
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
macro_rules! configure_service_for_static_files {
    ( $cfg:ident, $dir:expr, $embed:ident ) => {{
        let static_files = &$crate::config::SETTINGS.dev.static_files;
        if static_files.is_empty() {
            $cfg.service($crate::server::ResourceFiles::new($dir, $embed()));
        } else {
            $cfg.service(
                $crate::server::ActixFiles::new($dir, $crate::concat_string!(static_files, $dir))
                    .show_files_listing(),
            );
        }
    }};
}
