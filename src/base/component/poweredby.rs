use crate::prelude::*;

/// Muestra un texto con información de copyright, típica en un pie de página.
///
/// Por defecto, usando [`default()`](Self::default) sólo se muestra un
/// reconocimiento a PageTop. Sin embargo, se puede usar [`new()`](Self::new)
/// para crear una instancia con un texto de copyright predeterminado.
#[derive(AutoDefault)]
pub struct PoweredBy {
    copyright: Option<String>,
}

impl Component for PoweredBy {
    /// Crea una nueva instancia de `PoweredBy`.
    ///
    /// El copyright se genera automáticamente con el año actual y el nombre de
    /// la aplicación configurada en [`global::SETTINGS`].
    fn new() -> Self {
        let year = Utc::now().format("%Y").to_string();
        let c = join!(year, " © ", global::SETTINGS.app.name);
        PoweredBy { copyright: Some(c) }
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let poweredby_pagetop = L10n::l("poweredby_pagetop")
            .with_arg(
                "pagetop_link",
                "<a href=\"https://crates.io/crates/pagetop\">PageTop</a>",
            )
            .to_markup(cx);

        PrepareMarkup::With(html! {
            div id=[self.id()] class="poweredby" {
                @if let Some(c) = self.copyright() {
                    span class="poweredby__copyright" { (c) "." } " "
                }
                span class="poweredby__pagetop" { (poweredby_pagetop) }
            }
        })
    }
}

impl PoweredBy {
    // PoweredBy BUILDER ***************************************************************************

    /// Establece el texto de copyright que mostrará el componente.
    ///
    /// Al pasar `Some(valor)` se sobrescribe el texto de copyright por defecto. Al pasar `None` se
    /// eliminará, pero en este caso es necesario especificar el tipo explícitamente:
    ///
    /// ```rust
    /// use pagetop::prelude::*;
    ///
    /// let p1 = PoweredBy::default().with_copyright(Some("2001 © Foo Inc."));
    /// let p2 = PoweredBy::new().with_copyright(None::<String>);
    /// ```
    #[builder_fn]
    pub fn with_copyright(mut self, copyright: Option<impl Into<String>>) -> Self {
        self.copyright = copyright.map(Into::into);
        self
    }

    // PoweredBy GETTERS ***************************************************************************

    /// Devuelve el texto de copyright actual, si existe.
    pub fn copyright(&self) -> Option<&str> {
        self.copyright.as_deref()
    }
}
