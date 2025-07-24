use crate::core::extension::ExtensionTrait;

/// Representa una referencia a un tema.
///
/// Los temas son también extensiones. Por tanto se deben definir igual, es decir, como instancias
/// estáticas globales que implementan [`ThemeTrait`], pero también [`ExtensionTrait`].
pub type ThemeRef = &'static dyn ThemeTrait;

/// Interfaz común que debe implementar cualquier tema de `PageTop`.
///
/// Un tema implementará [`ThemeTrait`] y los métodos que sean necesarios de [`ExtensionTrait`],
/// aunque el único obligatorio es [`theme()`](ExtensionTrait::theme).
///
/// ```rust
/// use pagetop::prelude::*;
///
/// pub struct MyTheme;
///
/// impl ExtensionTrait for MyTheme {
///     fn name(&self) -> L10n { L10n::n("My theme") }
///     fn description(&self) -> L10n { L10n::n("Un tema personal") }
///
///     fn theme(&self) -> Option<ThemeRef> {
///         Some(&Self)
///     }
/// }
///
/// impl ThemeTrait for MyTheme {}
/// ```
pub trait ThemeTrait: ExtensionTrait + Send + Sync {}
