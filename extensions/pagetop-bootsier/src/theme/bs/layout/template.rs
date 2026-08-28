use pagetop::prelude::*;

use crate::config;
use crate::theme::{ContainerBootsier, bs};
use crate::{ADMINLTE_VERSION, LOCALES_BOOTSIER};

// Regiones de Bootsier: se renderiza sin el `<div role="region">` envolvente que aplica
// `layout::Template::prepare()` por defecto -- delega en `render_standard()`/`render_admin()`
// según la variante de `CoreTemplates` que envuelva el componente. Devuelve `None` si
// `component` no envuelve una `CoreTemplates`, dejando que el resto de la cadena de temas (o
// el propio componente) resuelva el renderizado por defecto.
pub(crate) async fn render(
    component: &layout::Template,
    cx: &mut Context,
) -> Option<Result<Markup, ComponentError>> {
    match component.template().downcast_ref::<CoreTemplates>()? {
        CoreTemplates::Standard => Some(Ok(render_standard(cx).await)),
        CoreTemplates::Admin => Some(Ok(render_admin(cx).await)),
    }
}

// Layout estándar: `CoreRegions::Header`, `CoreRegions::Aside`, `CoreRegions::Content` y
// `CoreRegions::Footer` envueltos en un contenedor de ancho configurable.
async fn render_standard(cx: &mut Context) -> Markup {
    bs::Container::new()
        .with_prop(PropsOp::add_classes("container-wrapper"))
        .with_width(bs::container::Width::FluidMax(
            config::SETTINGS.bootsier.max_width,
        ))
        .with_child(layout::Region::header())
        .with_child(layout::Region::aside())
        .with_child(layout::Region::default())
        .with_child(layout::Region::footer())
        .render(cx)
        .await
}

// Layout de administración: shell de AdminLTE 4 (barra superior, barra lateral con el contenido
// de BootsierRegions::Sidebar, área de contenido y pie).
async fn render_admin(cx: &mut Context) -> Markup {
    cx.alter_body_props(PropsOp::add_classes(
        "layout-fixed sidebar-expand-lg bg-body-tertiary",
    ));
    cx.alter_assets(AssetsOp::AddJavaScript(
        JavaScript::defer("/bootsier/js/bootsier.shell.min.js")
            .with_version(ADMINLTE_VERSION)
            .with_weight(-88),
    ));
    // `CoreRegions::Aside` es una región neutra del core: la usa `pagetop-admin` para su menú de
    // secciones sin que este tema tenga que depender de él. `BootsierRegions::Sidebar` sigue
    // disponible para que cualquier extensión añada elementos propios a mano.
    let aside = layout::Region::of(&CoreRegions::Aside).render(cx).await;
    let sidebar = layout::Region::of(&bs::BootsierRegions::Sidebar)
        .render(cx)
        .await;
    render_shell(cx, html! { (aside) (sidebar) }).await
}

async fn render_shell(cx: &mut Context, sidebar: Markup) -> Markup {
    let navbar = layout::Region::of(&bs::BootsierRegions::Navbar)
        .render(cx)
        .await;
    let content = layout::Region::default().render(cx).await;
    let footer = layout::Region::footer().render(cx).await;
    html! {
        div class="app-wrapper" {
            // Barra de navegación superior (app-header)
            nav class="app-header navbar navbar-expand bg-body" {
                div class="container-fluid" {
                    ul class="navbar-nav" {
                        li class="nav-item" {
                            a class="nav-link" data-lte-toggle="sidebar" href="#" role="button" {
                                i class="bi bi-list" {}
                            }
                        }
                    }
                    ul class="navbar-nav ms-auto" {
                        // Botón de pantalla completa
                        li class="nav-item" {
                            a class="nav-link" href="#" data-lte-toggle="fullscreen"
                              aria-label=[Lc::t("shell_fullscreen", &LOCALES_BOOTSIER).lookup(cx)]
                            {
                                i data-lte-icon="maximize" class="bi bi-fullscreen" {}
                                i data-lte-icon="minimize" class="bi bi-fullscreen-exit d-none" {}
                            }
                        }
                        // Selector de modo de color (claro / oscuro / automático)
                        li class="nav-item dropdown" {
                            a class="nav-link" href="#" id="bd-theme"
                              data-bs-toggle="dropdown" aria-expanded="false"
                              aria-label=[Lc::t("shell_theme_toggle", &LOCALES_BOOTSIER).lookup(cx)]
                            {
                                i class="bi bi-sun-fill" data-lte-theme-icon="light" {}
                                i class="bi bi-moon-fill d-none" data-lte-theme-icon="dark" {}
                                i class="bi bi-circle-half d-none" data-lte-theme-icon="auto" {}
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
                        (navbar)
                    }
                }
            }
            // Barra lateral (app-sidebar)
            aside class="app-sidebar bg-body-secondary shadow" data-bs-theme="dark" {
                div class="sidebar-brand" {
                    a href="/" class="brand-link" {
                        span class="brand-text fw-light" { (global::SETTINGS.app.name) }
                    }
                }
                div class="sidebar-wrapper" {
                    nav class="mt-2" {
                        ul class="nav sidebar-menu flex-column"
                           data-lte-toggle="treeview" role="menu"
                        {
                            (sidebar)
                        }
                    }
                }
            }
            // Área de contenido principal (app-main)
            main class="app-main" {
                div class="app-content" {
                    div class="container-fluid" {
                        (content)
                    }
                }
            }
            // Pie de página (app-footer)
            footer class="app-footer" {
                (footer)
            }
        }
    }
}
