use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;

/// Regiones específicas de la shell de Bootsier.
pub enum BootsierRegions {
    /// Barra lateral de navegación (`app-sidebar` de AdminLTE).
    ///
    /// Los componentes registrados aquí se renderizan directamente dentro del
    /// `<ul class="sidebar-menu">`, sin el `<div>` envolvente que añade
    /// [`Region`](pagetop::base::component::layout::Region) por defecto --
    /// [`Bootsier`](crate::Bootsier) intercepta este componente en `handle_component()` para
    /// renderizarlo así. Los elementos esperados son
    /// [`bs::sidebar::Item`](crate::theme::bs::sidebar::Item) y
    /// [`bs::sidebar::Section`](crate::theme::bs::sidebar::Section).
    ///
    /// Sólo se renderiza en la plantilla de administración (`CoreTemplates::Admin`), que se
    /// activa creando la página con [`Page::admin()`](pagetop::response::Page::admin). Registrar
    /// elementos aquí no tiene efecto en páginas creadas con `Page::new()`.
    ///
    /// # Registro global
    ///
    /// Para que los ítems aparezcan en todas las páginas con shell, regístralos durante
    /// el arranque de la aplicación con [`InRegion::Global`]:
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    /// use pagetop_bootsier::theme::bs::{BootsierRegions, sidebar};
    ///
    /// InRegion::Global(&BootsierRegions::Sidebar)
    ///     .add(sidebar::Section::titled(Lc::n("Administración")))
    ///     .add(sidebar::Item::link(Lc::n("Usuarios"), "/users", "people"))
    ///     .add(sidebar::Item::link(Lc::n("Roles"), "/roles", "shield-check"));
    /// ```
    ///
    /// # Registro por página
    ///
    /// Para añadir ítems sólo en una página concreta, usa [`Contextual::with_child_in`]:
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    /// use pagetop_bootsier::theme::bs::{BootsierRegions, sidebar};
    ///
    /// async fn dashboard(request: HttpRequest) -> Result<Markup, ErrorPage> {
    ///     Page::admin(request)
    ///         .with_child_in(
    ///             &BootsierRegions::Sidebar,
    ///             sidebar::Item::link(Lc::n("Panel"), "/dashboard", "grid"),
    ///         )
    ///         .render().await
    /// }
    /// ```
    Sidebar,

    /// Elementos adicionales en la barra de navegación superior (`app-header`).
    ///
    /// Los componentes registrados aquí se renderizan en el lado derecho de la barra superior,
    /// a continuación de los controles fijos (pantalla completa y selector de tema). Los elementos
    /// esperados son típicamente ítems de navegación (`<li class="nav-item">`).
    ///
    /// Esta región es opcional: si no tiene contenido, no añade ningún marcado al navbar.
    ///
    /// # Registro global
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    /// use pagetop_bootsier::theme::bs::BootsierRegions;
    ///
    /// InRegion::Global(&BootsierRegions::Navbar)
    ///     .add(Html::with(|_| html! {
    ///         li class="nav-item" {
    ///             a class="nav-link" href="/logout" { "Cerrar sesión" }
    ///         }
    ///     }));
    /// ```
    Navbar,
}

impl RegionName for BootsierRegions {
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::Sidebar => "bootsier-sidebar",
            Self::Navbar => "bootsier-navbar",
        }
    }

    #[inline]
    fn label(&self) -> Lc {
        match self {
            Self::Sidebar => Lc::t("region_sidebar", &LOCALES_BOOTSIER),
            Self::Navbar => Lc::t("region_navbar", &LOCALES_BOOTSIER),
        }
    }
}

// **< Region RENDER >******************************************************************************

// Regiones de Bootsier: se renderizan sin el `<div role="region">` envolvente que aplica
// `layout::Region::prepare()` por defecto -- sus elementos van directamente dentro del contenedor
// que los gestiona (sidebar-menu o navbar-nav). Devuelve `None` si `component` no envuelve una
// `BootsierRegions`, dejando que el resto de la cadena de temas (o el propio componente) resuelva
// el renderizado por defecto.
pub(crate) async fn render(
    component: &layout::Region,
    cx: &mut Context,
) -> Option<Result<Markup, ComponentError>> {
    match component.region().downcast_ref::<BootsierRegions>()? {
        BootsierRegions::Sidebar | BootsierRegions::Navbar => {
            Some(Ok(cx.render_region(component.region()).await))
        }
    }
}
