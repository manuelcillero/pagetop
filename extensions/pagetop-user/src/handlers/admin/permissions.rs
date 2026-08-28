//! Handler de listado de permisos.
//!
//! Sólo lectura: los permisos se declaran en código mediante la acción `DeclarePermissions`, no se
//! crean, editan ni eliminan desde la UI. La única forma de "gestionarlos" es asignarlos a un rol
//! (ver `handlers::admin::roles::permissions_get/post`).

use pagetop::prelude::*;

use crate::LOCALES_USER;
use crate::handlers::admin::frame;
use crate::permission::{self, UserPermission};

/// GET /admin/user/permissions - Catálogo de permisos agrupado.
pub(crate) async fn list_get(request: HttpRequest) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminPermissions)?;

    let registry = permission::registry();
    let title = Lc::t("title-admin-permissions", &LOCALES_USER);
    let mut content = frame(title.clone());

    for (group, group_label) in registry.groups() {
        let items: Vec<(CowStr, Lc)> = registry
            .by_group(group)
            .map(|permission| (permission.key(), permission.label()))
            .collect();
        content = content.with_child(Block::new().with_title(group_label.clone()).with_child(
            Html::with(move |cx| {
                html! {
                    table.user-admin-table {
                        tbody {
                            @for (key, label) in &items {
                                tr {
                                    td { (label.using(cx)) }
                                    td.user-admin-permission-key { (key) }
                                }
                            }
                        }
                    }
                }
            }),
        ));
    }

    let mut page = Page::admin(request).with_title(title).with_child(content);
    Ok(page.render().await.into_response())
}
