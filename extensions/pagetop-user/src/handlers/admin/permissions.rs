//! Handler de listado de permisos.
//!
//! Sólo lectura: los permisos se declaran en código mediante la acción `DeclarePermissions`, no se
//! crean, editan ni eliminan desde la UI. La única forma de "gestionarlos" es asignarlos a un rol
//! (ver `handlers::admin::roles::permissions_get/post`).

use pagetop::prelude::*;

use crate::LOCALES_USER;

/// GET /admin/user/permissions - Catálogo de permisos agrupado.
pub(crate) async fn list_get(request: HttpRequest) -> Result<Response, ErrorPage> {
    let page = Page::admin(request);

    let registry = PermissionRegistry::global();
    let groups = registry.groups_sorted(page.context());
    let mut page = page.with_title(Lc::t("title-admin-permissions", &LOCALES_USER));

    for (group, group_label) in groups {
        let mut table = Table::new().with_prop(PropsOp::add_classes("user-admin-table"));
        for permission in registry.by_group(group) {
            table = table.with_row(
                table::Row::new()
                    .with_cell(permission.label())
                    .with_cell(permission.key().as_ref()),
            );
        }
        page = page.with_child(
            Block::new()
                .with_title(group_label.clone())
                .with_child(table),
        );
    }

    Ok(page.render().await.into_response())
}
