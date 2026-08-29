//! Definiciones para crear menús ([`Nav`]).
//!
//! Cada [`nav::Item`] representa un elemento individual del menú
//! [`Nav`], con distintos comportamientos según su finalidad, como enlaces de navegación o menús
//! desplegables [`Dropdown`].
//!
//! Los ítems pueden estar activos, deshabilitados o abrirse en nueva ventana según su contexto y
//! configuración, y permiten incluir etiquetas localizables usando [`Lc`].

use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;

mod props;
pub use props::Kind;

pub use pagetop::base::component::Nav;
pub use pagetop::base::component::nav::{Item, ItemKind};

const EXTRA_KIND: &str = "bootsier.nav.kind";

// Marca interna (nunca expuesta en `NavBootsier`) que `theme::bs::navbar::item` fija sobre el clon
// de un `Nav` embebido en una `Navbar`, para que use `navbar-nav` en vez de `nav` como clase base.
pub(crate) const EXTRA_IN_NAVBAR: &str = "bootsier.nav.in_navbar";

/// Extensión de Bootsier para [`Nav`].
///
/// Permite establecer el estilo visual usando el método [`with_kind()`](Self::with_kind).
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let nav = bs::Nav::new()
///     .with_kind(bs::nav::Kind::Pills)
///     .with_layout(nav::Layout::End)
///     .with_item(bs::nav::Item::link(Lc::n("Home"), "/"))
///     .with_item(bs::nav::Item::link_blank(Lc::n("External"), "https://docs.rs"))
///     .with_item(bs::nav::Item::dropdown(
///         bs::Dropdown::new()
///             .with_title(Lc::n("Options"))
///             .with_item(TypedOp::AddMany(vec![
///                 bs::dropdown::Item::link(Lc::n("Action"), "/action"),
///                 bs::dropdown::Item::link(Lc::n("Another"), "/another"),
///             ])),
///     ))
///     .with_item(bs::nav::Item::link_disabled(Lc::n("Disabled"), "#"));
/// ```
#[builder_impl]
pub trait NavBootsier {
    /// Cambia el estilo del menú (*Tabs*, *Pills*, *Underline* o *Default*).
    fn with_kind(self, kind: Kind) -> Self;
}

#[builder_impl]
impl NavBootsier for Nav {
    fn with_kind(mut self, kind: Kind) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_KIND, kind));
        self
    }
}

// **< Nav SETUP >**********************************************************************************

pub(crate) fn setup(nav: &mut Nav) {
    let kind = nav.props().extra_or(EXTRA_KIND, Kind::default());
    let in_navbar = nav.props().extra_or(EXTRA_IN_NAVBAR, false);
    let mut classes = if in_navbar { "navbar-nav" } else { "nav" }.to_string();
    kind.push_to(&mut classes);
    layout_class(*nav.nav_layout(), &mut classes);
    nav.alter_prop(PropsOp::prepend_classes(classes));
}

// Traduce el `nav::Layout` semántico de base al vocabulario de utilidades de Bootstrap.
fn layout_class(layout: nav::Layout, classes: &mut String) {
    let class = match layout {
        nav::Layout::Default => "",
        nav::Layout::Start => "justify-content-start",
        nav::Layout::Center => "justify-content-center",
        nav::Layout::End => "justify-content-end",
        nav::Layout::Vertical => "flex-column",
        nav::Layout::Fill => "nav-fill",
        nav::Layout::Justified => "nav-justified",
    };
    if class.is_empty() {
        return;
    }
    if !classes.is_empty() {
        classes.push(' ');
    }
    classes.push_str(class);
}

// **< Item RENDER >********************************************************************************

// Idéntico a `nav::Item::prepare()` salvo el disparador del desplegable, que necesita
// `data-bs-toggle="dropdown"` para que el JS de Bootstrap lo reconozca (la clase `dropdown-toggle`
// por sí sola sólo aporta el estilo, no la inicialización).
pub(crate) async fn item_render(item: &Item, cx: &mut Context) -> Result<Markup, ComponentError> {
    Ok(match item.item_kind() {
        ItemKind::Void => html! {},

        ItemKind::Label(label) => html! {
            li (item.props()) {
                span class="nav-link disabled" aria-disabled="true" {
                    (label.using(cx))
                }
            }
        },

        ItemKind::Link {
            label,
            route,
            blank,
            disabled,
        } => {
            let route_link = route.resolve(cx);
            let current_path = cx.request().map(|request| request.path());
            let is_current = item
                .active_override()
                .copied()
                .unwrap_or(!*disabled && (current_path == Some(route_link.path())));

            let mut classes = "nav-link".to_string();
            if is_current {
                classes.push_str(" active");
            }
            if *disabled {
                classes.push_str(" disabled");
            }

            let href = (!*disabled).then_some(route_link);
            let target = (!*disabled && *blank).then_some("_blank");
            let rel = (!*disabled && *blank).then_some("noopener noreferrer");

            let aria_current = (href.is_some() && is_current).then_some("page");
            let aria_disabled = (*disabled).then_some("true");

            html! {
                li (item.props()) {
                    a
                        class=(classes)
                        href=[href]
                        target=[target]
                        rel=[rel]
                        aria-current=[aria_current]
                        aria-disabled=[aria_disabled]
                    {
                        (label.using(cx))
                    }
                }
            }
        }

        ItemKind::Html(html) => html! {
            li (item.props()) {
                (html.render(cx).await)
            }
        },

        ItemKind::Dropdown(menu) => {
            if let Some(dd) = menu.get() {
                let items = dd.items().render(cx).await;
                if items.is_empty() {
                    return Ok(html! {});
                }
                let title = dd.title().lookup(cx).unwrap_or_else(|| {
                    Lc::t("dropdown", &LOCALES_BOOTSIER)
                        .lookup(cx)
                        .unwrap_or_else(|| "Dropdown".to_string())
                });
                html! {
                    li (item.props()) {
                        a
                            class="nav-link dropdown-toggle"
                            data-bs-toggle="dropdown"
                            href="#"
                            role="button"
                            aria-haspopup="true"
                            aria-expanded="false"
                        {
                            (title)
                        }
                        ul class="dropdown-menu" {
                            (items)
                        }
                    }
                }
            } else {
                html! {}
            }
        }
    })
}
