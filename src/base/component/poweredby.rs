use crate::prelude::*;

// Enlace a la página oficial de PageTop.
const PAGETOP_LINK: &str = concat!(
    "<a ",
    "href=\"https://pagetop.cillero.es\" ",
    "target=\"_blank\" ",
    "rel=\"noopener noreferrer\">",
    "PageTop",
    "</a>",
);

/// Componente que muestra el típico mensaje *Powered by* (*Funciona con*) en el pie de página.
///
/// Por defecto, usando [`default()`](Self::default) sólo se muestra un reconocimiento a PageTop.
/// Sin embargo, se puede usar [`new()`](Self::new) para crear una instancia con un texto de
/// copyright predeterminado.
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct PoweredBy {
    /// Devuelve el texto de copyright actual, si existe.
    copyright: Option<String>,
}

#[async_trait]
impl Component for PoweredBy {
    /// Crea una nueva instancia de `PoweredBy`.
    ///
    /// El copyright se genera automáticamente con el año actual y el nombre de la aplicación
    /// configurada en [`global::SETTINGS`], en el formato `YYYY © Nombre de la aplicación`.
    fn new() -> Self {
        let year = Utc::now().format("%Y").to_string();
        let c = util::join!(year, " © ", global::SETTINGS.app.name);
        PoweredBy { copyright: Some(c) }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {
            div id=[self.id()] class="poweredby" {
                @if let Some(c) = self.copyright() {
                    span class="poweredby__copyright" { (c) "." } " "
                }
                span class="poweredby__pagetop" {
                    (Lc::l("poweredby_pagetop").with_arg("pagetop_link", PAGETOP_LINK).using(cx))
                }
            }
        })
    }
}

#[builder_impl]
impl PoweredBy {
    // **< PoweredBy BUILDER >**********************************************************************

    /// Establece el texto de copyright que mostrará el componente.
    ///
    /// Al pasar `Some(valor)` se sobrescribe el texto de copyright por defecto. Al pasar `None` se
    /// eliminará, pero en este caso es necesario especificar el tipo explícitamente:
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let p1 = PoweredBy::default().with_copyright(Some("2001 © Foo Inc."));
    /// let p2 = PoweredBy::new().with_copyright(None::<String>);
    /// ```
    pub fn with_copyright(mut self, copyright: Option<impl Into<String>>) -> Self {
        self.copyright = copyright.map(Into::into);
        self
    }
}
