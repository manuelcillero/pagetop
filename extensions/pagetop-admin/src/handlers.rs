use std::collections::HashMap;

use pagetop::prelude::*;

use crate::LOCALES_ADMIN;
use crate::component::{AdminFrame, ConfigForm};
use crate::registry::{self, AdminPageKind, AdminPermission, AdminSection};
use crate::settings;

// **< Dashboard >**********************************************************************************

/// GET /admin - Dashboard de administración: lista todas las secciones disponibles.
pub async fn dashboard(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let reg = registry::global();
    let cx = Context::new(request.clone());
    let sections = reg.ordered_sections();

    let title = Lc::t("dashboard-title", &LOCALES_ADMIN);
    let content = render_sections(&cx, &sections);

    Page::admin(request)
        .with_title(title.clone())
        .with_child(
            AdminFrame::new()
                .with_title(title)
                .with_child(Html::with(move |_| content.clone())),
        )
        .render()
        .await
}

// **< Sección >************************************************************************************

/// GET /admin/{section} - Página de aterrizaje de una sección: lista sus páginas.
///
/// Sólo se monta (ver `configure_router()` en `lib.rs`) para las secciones cuyo `path` no coincida
/// con ninguna [`AdminPage`](crate::registry::AdminPage) registrada explícitamente por una
/// extensión; si una extensión reclama esa ruta con su propia página, esa página gana y este
/// handler nunca llega a montarse para ella.
pub async fn section_page(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let path = request.path().to_owned();
    let reg = registry::global();
    let section = reg
        .sections()
        .values()
        .find(|s| s.path == path)
        .ok_or_else(|| ErrorPage::NotFound(Some(request.clone())))?;

    require_permission(
        &request,
        section.permission.unwrap_or(&AdminPermission::Access),
    )?;

    let cx = Context::new(request.clone());
    let title = section.title.clone();
    let content = render_sections(&cx, &[section]);

    Page::admin(request)
        .with_title(title.clone())
        .with_child(
            AdminFrame::new()
                .with_title(title)
                .with_child(Html::with(move |_| content.clone())),
        )
        .render()
        .await
}

// **< Render compartido >**************************************************************************

// Rejilla de secciones con sus páginas (título + descripción). La usan tanto `dashboard()` (todas
// las secciones) como `section_page()` (una sola), filtrando siempre por permiso del usuario actual.
fn render_sections(cx: &Context, sections: &[&AdminSection]) -> Markup {
    let reg = registry::global();
    html! {
        div.admin-dashboard-sections {
            @for section in sections {
                @if section.is_visible(cx) {
                    @let pages: Vec<_> = reg
                        .pages_for_section(&section.key)
                        .into_iter()
                        .filter(|p| p.is_accessible(cx))
                        .collect();
                    @if !pages.is_empty() {
                        div.admin-dashboard-section {
                            h2.admin-dashboard-section-title {
                                a href=(cx.route(section.path.as_str())) { (section.title.using(cx)) }
                            }
                            ul.admin-dashboard-section-links {
                                @for page in pages {
                                    li.admin-dashboard-link {
                                        a href=(cx.route(page.path.as_str())) { (page.title.using(cx)) }
                                        @if let Some(description) = &page.description {
                                            span.admin-dashboard-link-desc {
                                                " - " (description.using(cx))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// **< Config form - GET >**************************************************************************

/// GET de una página [`AdminPageKind::ConfigForm`].
///
/// Lee el schema del registro y renderiza el formulario con los valores actuales.
pub async fn config_form_get(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let path = request.path().to_owned();
    let reg = registry::global();
    let page = reg
        .pages()
        .get(&path)
        .ok_or_else(|| ErrorPage::NotFound(Some(request.clone())))?;

    let AdminPageKind::ConfigForm(ref schema) = page.kind else {
        return Err(ErrorPage::NotFound(Some(request.clone())));
    };

    require_permission(&request, page.permission_key())?;

    let title = page.title.clone();
    let mut form = ConfigForm::with_schema(schema.clone());
    let mut cx = Context::new(request.clone());
    let content = form.render(&mut cx).await;

    Page::admin(request)
        .with_title(title.clone())
        .with_child(
            AdminFrame::new()
                .with_title(title)
                .with_child(Html::with(move |_| content.clone())),
        )
        .render()
        .await
}

// **< Config form - POST >*************************************************************************

/// POST de una página [`AdminPageKind::ConfigForm`].
///
/// Valida y persiste los valores en `settings`, luego redirige al GET con
/// estado `saved=true`.
pub async fn config_form_post(
    request: HttpRequest,
    web::Form(data): web::Form<HashMap<String, String>>,
) -> Result<Markup, ErrorPage> {
    let path = request.path().to_owned();
    let reg = registry::global();
    let page = reg
        .pages()
        .get(&path)
        .ok_or_else(|| ErrorPage::NotFound(Some(request.clone())))?;

    let AdminPageKind::ConfigForm(ref schema) = page.kind else {
        return Err(ErrorPage::NotFound(Some(request.clone())));
    };

    require_permission(&request, page.permission_key())?;

    let mut save_error = false;

    for field in schema.fields() {
        let key = schema.key_for(field.name());

        // Los checkbox HTML sólo envían el campo si están marcados.
        let raw = match field.field_type() {
            crate::settings::SettingFieldType::Boolean => data
                .get(field.name())
                .map(|s| s.as_str())
                .unwrap_or("false")
                .to_owned(),
            _ => {
                let Some(raw) = data.get(field.name()) else {
                    if *field.required() {
                        save_error = true;
                    }
                    continue;
                };
                raw.clone()
            }
        };

        settings::set::<String>(&key, &raw, schema.scope(), None).await;
    }

    let title = page.title.clone();
    let mut form = ConfigForm::with_schema(schema.clone()).with_saved(!save_error, save_error);
    let mut cx = Context::new(request.clone());
    let content = form.render(&mut cx).await;

    Page::admin(request)
        .with_title(title.clone())
        .with_child(
            AdminFrame::new()
                .with_title(title)
                .with_child(Html::with(move |_| content.clone())),
        )
        .render()
        .await
}
