use pagetop::prelude::*;

// **< Section >************************************************************************************

/// Encabezado de sección en la barra lateral de AdminLTE.
///
/// Renderiza un `<li class="nav-header">` con el texto localizable de la sección. Se usa para
/// agrupar visualmente los ítems de navegación ([`Item`](super::Item)) en la sidebar.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::bs::sidebar;
///
/// // Sección con texto fijo.
/// let section = sidebar::Section::titled(Lc::n("Administración"));
///
/// // Sección con texto localizable.
/// let section_i18n = sidebar::Section::titled(Lc::l("nav-admin"));
/// ```
#[derive(AutoDefault, Clone, Getters)]
pub struct Section {
    /// Devuelve el título localizable de la sección.
    title: Lc,
}

#[async_trait]
impl Component for Section {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {
            li class="nav-header" {
                (self.title().using(cx))
            }
        })
    }
}

impl Section {
    /// Crea un encabezado de sección con el título indicado.
    pub fn titled(title: Lc) -> Self {
        Self { title }
    }

    // **< Section BUILDER >************************************************************************

    /// Establece el título localizable de la sección.
    #[builder_fn]
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }
}
