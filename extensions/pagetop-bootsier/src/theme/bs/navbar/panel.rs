use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;
use crate::theme::*;

/// Panel lateral deslizante (*offcanvas*) para una barra de navegación [`Navbar`].
///
/// Es el panel en el que se muestran los elementos de la barra cuando está colapsada. Se abre con
/// su botón de despliegue y sólo recoge lo que tiene sentido en una barra: el título, el borde
/// desde el que se despliega, la capa de fondo y el desplazamiento de la página. El resto lo decide
/// la propia barra ya que el contenido son sus elementos, el punto de corte a partir del cual se
/// muestra (definido en [`Navbar::with_expand()`]) y el identificador que se deriva del suyo.
///
/// Para un panel lateral independiente, con contenido propio, se usa [`Offcanvas`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let panel = bs::navbar::Panel::new()
///     .with_title(Lc::n("Main menu"))
///     .with_placement(bs::offcanvas::Placement::End)
///     .with_backdrop(bs::offcanvas::Backdrop::Static);
///
/// let navbar = bs::Navbar::offcanvas(panel)
///     .with_item(bs::navbar::Item::nav(
///         bs::Nav::new().with_item(bs::nav::Item::link(Lc::n("Home"), "/"))
///     ));
/// ```
///
/// [`Navbar`]: crate::theme::bs::Navbar
/// [`Navbar::with_expand()`]: crate::theme::bs::Navbar::with_expand
/// [`Offcanvas`]: crate::theme::bs::Offcanvas
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Panel {
    /// Devuelve el título del panel.
    title: Lc,
    /// Devuelve el borde desde el que se despliega el panel.
    #[getters(copy)]
    placement: bs::offcanvas::Placement,
    /// Devuelve el comportamiento configurado para la capa de fondo.
    #[getters(copy)]
    backdrop: bs::offcanvas::Backdrop,
    /// Indica si la página principal puede desplazarse mientras el panel está abierto.
    #[getters(copy)]
    body_scroll: bs::offcanvas::BodyScroll,
}

#[builder_impl]
impl Panel {
    /// Crea un panel sin título, que se despliega desde el borde inicial, con una capa de fondo y
    /// sin permitir el desplazamiento de la página mientras está abierto.
    pub fn new() -> Self {
        Self::default()
    }

    /// Establece el título del encabezado.
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Indica desde qué borde de la ventana se despliega y se ancla el panel.
    pub fn with_placement(mut self, placement: bs::offcanvas::Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Ajusta la capa de fondo del panel para definir su comportamiento al hacer clic fuera del
    /// panel.
    pub fn with_backdrop(mut self, backdrop: bs::offcanvas::Backdrop) -> Self {
        self.backdrop = backdrop;
        self
    }

    /// Permite o bloquea el desplazamiento de la página principal mientras el panel está abierto.
    pub fn with_body_scroll(mut self, scrolling: bs::offcanvas::BodyScroll) -> Self {
        self.body_scroll = scrolling;
        self
    }

    // Renderiza el panel con `id` como identificador y `body` (los elementos de la barra ya
    // renderizados) como contenido. Con `navbar-expand-{bp}` Bootstrap lo muestra en línea a partir
    // del punto de corte de la barra, así que no lleva clase de punto de corte propia.
    pub(super) fn render_panel(&self, cx: &Context, id: &str, body: &Markup) -> Markup {
        let title = self.title().using(cx);
        let id_label = util::join!(id, "-label");
        let id_target = util::join!("#", id);
        let classes = util::join!("offcanvas ", self.placement().as_str());

        html! {
            div
                id=(id)
                class=(classes)
                tabindex="-1"
                data-bs-scroll=[self.body_scroll().opt_str()]
                data-bs-backdrop=[self.backdrop().opt_str()]
                aria-labelledby=[(!title.is_empty()).then_some(&id_label)]
            {
                div class="offcanvas-header" {
                    @if !title.is_empty() {
                        h5 id=(&id_label) class="offcanvas-title" { (title) }
                    }
                    button
                        type="button"
                        class="btn-close"
                        data-bs-dismiss="offcanvas"
                        data-bs-target=(id_target)
                        aria-label=[Lc::t("offcanvas_close", &LOCALES_BOOTSIER).lookup(cx)]
                    {}
                }
                div class="offcanvas-body" { (body) }
            }
        }
    }
}
