//! Tabla de roles: cabeceras ordenables y filas con acciones.

use pagetop::prelude::*;

use pagetop::base::component::table::{Column, Row};
use pagetop_htmx::hx;
use pagetop_htmx::hx_table::sort_link;

use crate::ADMIN_ROLES_PATH;
use crate::LOCALES_USER;
use crate::service::role_admin::{RoleListItem, RoleSortField};

#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct RoleTable {
    props: Props,
    items: Vec<RoleListItem>,
    sort: RoleSortField,
    dir: SortDir,
    page: u64,
    per_page: u64,
    total: u64,
    /// Mensaje de error a mostrar (p. ej. al fallar un borrado). No persiste entre peticiones.
    message: Option<Lc>,
}

#[async_trait]
impl Component for RoleTable {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::set_id("role-table-wrapper"));
        self.alter_prop(PropsOp::prepend_classes("user-admin-table-wrapper"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let pager = Pager::new()
            .with_base_path(ADMIN_ROLES_PATH)
            .with_extra_query("sort", self.sort().as_str())
            .with_extra_query("dir", *self.dir())
            .with_current_page(self.page())
            .with_items_per_page(self.per_page())
            .with_total_items(self.total())
            .with_prop(PropsOp::set(hx::BOOST, "true"))
            .with_prop(PropsOp::set(hx::TARGET, "#role-table-wrapper"))
            .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML_SCROLL_TOP))
            .with_prop(PropsOp::set(hx::PUSH_URL, "true"))
            .render(cx)
            .await;

        let mut table = Table::new()
            .with_prop(PropsOp::add_classes("user-admin-table"))
            .with_column(self.sort_column(cx, RoleSortField::MachineName, "col-machine-name"))
            .with_column(self.sort_column(cx, RoleSortField::Label, "col-label"))
            .with_column(Lc::t("col-type", &LOCALES_USER))
            .with_column(Lc::t("col-users-count", &LOCALES_USER))
            .with_column(Lc::t("col-actions", &LOCALES_USER))
            .with_empty(Lc::t("empty-roles-list", &LOCALES_USER));

        let confirm_dialog = Dialog::new()
            .with_id("confirm-delete-role")
            .with_child(Html::with(|cx| {
                html! { p { (Lc::t("confirm-delete-role", &LOCALES_USER).using(cx)) } }
            }))
            .with_footer(
                Button::plain(Lc::t("btn-cancel", &LOCALES_USER))
                    .with_prop(PropsOp::set("data-dialog-dismiss", "modal")),
            )
            .with_footer(Html::with(
                |_cx| html! { span id="role-delete-confirm-action" {} },
            ))
            .render(cx)
            .await;

        let waypoint = Waypoint::from(self.list_href(cx));

        for role in self.items() {
            let system_badge = if role.locked {
                Some(
                    Badge::labeled(Lc::t("badge-system-role", &LOCALES_USER))
                        .with_prop(PropsOp::add_classes("user-admin-badge-system"))
                        .render(cx)
                        .await,
                )
            } else {
                None
            };

            table.alter_row(
                Row::new()
                    .with_cell(role.machine_name.as_str())
                    .with_cell(label_cell(role, &waypoint))
                    .with_cell(Html::with(move |_cx| {
                        html! {
                            @if let Some(badge) = &system_badge { (badge) }
                        }
                    }))
                    .with_cell(role.user_count.to_string())
                    .with_cell(
                        actions_cell(role, &waypoint, *self.sort(), *self.dir(), self.page(), cx)
                            .await,
                    ),
            );
        }

        let new_href = waypoint.append_to(cx.route(format!("{ADMIN_ROLES_PATH}/new")));

        Ok(html! {
            div (self.props()) {
                div.user-admin-actions {
                    a href=(new_href) {
                        (Lc::t("btn-create-role", &LOCALES_USER).using(cx))
                    }
                }
                @if let Some(message) = self.message() {
                    div.user-form-error role="alert" { (message.clone().using(cx)) }
                }
                (table.render(cx).await)
                (pager)
                (confirm_dialog)
            }
        })
    }
}

#[builder_impl]
impl RoleTable {
    // **< RoleTable BUILDER >**********************************************************************

    pub(crate) fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    pub(crate) fn with_items(mut self, items: Vec<RoleListItem>) -> Self {
        self.items = items;
        self
    }

    pub(crate) fn with_message(mut self, message: impl Into<Option<Lc>>) -> Self {
        self.message = message.into();
        self
    }

    pub(crate) fn with_sort(mut self, sort: RoleSortField) -> Self {
        self.sort = sort;
        self
    }

    pub(crate) fn with_dir(mut self, dir: SortDir) -> Self {
        self.dir = dir;
        self
    }

    pub(crate) fn with_page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }

    pub(crate) fn with_per_page(mut self, per_page: u64) -> Self {
        self.per_page = per_page;
        self
    }

    pub(crate) fn with_total(mut self, total: u64) -> Self {
        self.total = total;
        self
    }

    // URL del listado con el estado actual (orden, página): es el valor que viaja como
    // `waypoint` en los enlaces de ver/editar/permisos, para poder volver exactamente a este
    // mismo estado. Se construye con `cx.route()` para que preserve el parámetro `lang` cuando
    // corresponda.
    fn list_href(&self, cx: &Context) -> String {
        cx.route(ADMIN_ROLES_PATH)
            .alter_param("sort", self.sort().as_str())
            .alter_param("dir", *self.dir())
            .alter_param("page", self.page().to_string())
            .to_string()
    }

    // Construye la cabecera ordenable de una columna: el enlace ya funciona sin HTMX (navega con
    // una petición normal); `pagetop_htmx::hx_table::sort_link()` añade aparte los atributos `hx-*`
    // para que la tabla se actualice sin recargar la página cuando la extensión esté disponible.
    fn sort_column(&self, cx: &Context, field: RoleSortField, label_key: &'static str) -> Column {
        let is_active = *self.sort() == field;
        let active = is_active.then_some(*self.dir());
        let next_dir = SortDir::next_for(active);
        let mut route = cx.route(ADMIN_ROLES_PATH);
        route
            .alter_param("sort", field.as_str())
            .alter_param("dir", next_dir);

        Column::new(Lc::t(label_key, &LOCALES_USER)).with_sort(sort_link(
            route,
            "#role-table-wrapper",
            active,
        ))
    }
}

// Construye la celda de etiqueta: enlaza a la pantalla de sólo lectura con todos los permisos del
// rol. Devuelve un componente `Html` para que el marcado se genere cuando `Table` renderice la
// celda, no al construir la fila.
fn label_cell(role: &RoleListItem, waypoint: &Waypoint) -> Html {
    let label = role.label.clone();
    let id = role.id;
    let waypoint = waypoint.clone();
    Html::with(move |cx| {
        let view_href = waypoint.append_to(cx.route(format!("{ADMIN_ROLES_PATH}/{id}/view")));
        html! {
            a href=(view_href) { (label.as_str()) }
        }
    })
}

// Construye la celda de acciones: gestionar permisos siempre, y editar/borrar sólo si el rol no
// está bloqueado por el sistema. Devuelve un componente `Html` para que el marcado se genere
// cuando `Table` renderice la celda, no al construir la fila.
async fn actions_cell(
    role: &RoleListItem,
    waypoint: &Waypoint,
    sort: RoleSortField,
    dir: SortDir,
    page: u64,
    cx: &mut Context,
) -> Html {
    let id = role.id;
    let locked = role.locked;
    let waypoint = waypoint.clone();

    let permissions_href =
        waypoint.append_to(cx.route(format!("{ADMIN_ROLES_PATH}/{id}/permissions")));

    // Los botones se renderizan aquí, no dentro del `Html::with()` de abajo: necesitan pasar por
    // su propio ciclo de renderizado (`.render().await`) para que el tema activo los estilice
    // igual (ver `pagetop-bootsier::theme::bs::button`), incluida la traducción de `data-dialog-*`
    // que usa el botón de borrado.
    let permissions_button = Button::anchor(
        Lc::t("btn-manage-permissions", &LOCALES_USER),
        permissions_href,
    )
    .with_style(button::Style::Solid(Intent::Neutral))
    .with_size(button::Size::Small)
    .render(cx)
    .await;

    let (edit_button, delete_button) = if locked {
        (None, None)
    } else {
        let edit_href = waypoint.append_to(cx.route(format!("{ADMIN_ROLES_PATH}/{id}/edit")));
        let edit_button = Button::anchor(Lc::t("btn-edit", &LOCALES_USER), edit_href)
            .with_style(button::Style::Solid(Intent::Primary))
            .with_size(button::Size::Small)
            .render(cx)
            .await;

        // Viaja como query string para que, tanto si el borrado falla como si tiene éxito, la
        // tabla vuelva a mostrarse en la misma página/orden en que estaba, en vez de reiniciarse.
        let confirm_href = cx
            .route(format!("{ADMIN_ROLES_PATH}/{id}/delete/confirm"))
            .alter_param("sort", sort.as_str())
            .alter_param("dir", dir)
            .alter_param("page", page.to_string())
            .to_string();

        let delete_button = Button::plain(Lc::t("btn-delete", &LOCALES_USER))
            .with_style(button::Style::Solid(Intent::Severe))
            .with_size(button::Size::Small)
            .with_prop(PropsOp::set(hx::GET, confirm_href))
            .with_prop(PropsOp::set(hx::TARGET, "#role-delete-confirm-action"))
            .with_prop(PropsOp::set("data-dialog-toggle", "modal"))
            .with_prop(PropsOp::set("data-dialog-target", "#confirm-delete-role"))
            .render(cx)
            .await;

        (Some(edit_button), Some(delete_button))
    };

    Html::with(move |_cx| {
        html! {
            (permissions_button)
            @if let Some(edit_button) = &edit_button {
                " "
                (edit_button)
            }
            @if let Some(delete_button) = &delete_button {
                " "
                (delete_button)
            }
        }
    })
}
