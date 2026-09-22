use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;

/// Crea un [`nav::Item`] con el selector de modo de color (claro / oscuro / automático), pensado
/// para añadirse a cualquier [`Nav`](super::Nav), suelto o dentro de una
/// [`Navbar`](super::Navbar).
///
/// El estado se recuerda en `localStorage` y se aplica con `/bootsier/js/bootsier.theme.min.js`,
/// cargado siempre por Bootsier (ver [`crate::Bootsier::before_render_page_body()`]).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let navbar = bs::Navbar::simple().with_item(bs::navbar::Item::nav(
///     bs::Nav::new()
///         .with_item(bs::nav::Item::link(Lc::n("Home"), "/"))
///         .with_item(bs::theme_toggle()),
/// ));
/// ```
pub fn theme_toggle() -> nav::Item {
    nav::Item::html(Html::with(|cx| {
        html! {
            a class="nav-link" href="#" id="bd-theme"
              data-bs-toggle="dropdown" aria-expanded="false"
              aria-label=[Lc::t("shell_theme_toggle", &LOCALES_BOOTSIER).lookup(cx)]
            {
                i class="bi bi-sun-fill" data-bs-theme-icon="light" {}
                i class="bi bi-moon-fill d-none" data-bs-theme-icon="dark" {}
                i class="bi bi-circle-half d-none" data-bs-theme-icon="auto" {}
            }
            ul class="dropdown-menu dropdown-menu-end" aria-labelledby="bd-theme"
               style="--bs-dropdown-min-width: 8rem"
            {
                li {
                    button type="button"
                           class="dropdown-item d-flex align-items-center"
                           data-bs-theme-value="light"
                           aria-pressed="false"
                    {
                        i class="bi bi-sun-fill me-2" {}
                        (Lc::t("shell_theme_light", &LOCALES_BOOTSIER).using(cx))
                        i class="bi bi-check-lg ms-auto d-none" {}
                    }
                }
                li {
                    button type="button"
                           class="dropdown-item d-flex align-items-center"
                           data-bs-theme-value="dark"
                           aria-pressed="false"
                    {
                        i class="bi bi-moon-fill me-2" {}
                        (Lc::t("shell_theme_dark", &LOCALES_BOOTSIER).using(cx))
                        i class="bi bi-check-lg ms-auto d-none" {}
                    }
                }
                li {
                    button type="button"
                           class="dropdown-item d-flex align-items-center"
                           data-bs-theme-value="auto"
                           aria-pressed="false"
                    {
                        i class="bi bi-circle-half me-2" {}
                        (Lc::t("shell_theme_auto", &LOCALES_BOOTSIER).using(cx))
                        i class="bi bi-check-lg ms-auto d-none" {}
                    }
                }
            }
        }
    }))
    .with_prop(PropsOp::add_classes("dropdown"))
}
