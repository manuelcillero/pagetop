//! Tabla de usuarios: cabeceras ordenables, filas y paginación embebida.

use pagetop::prelude::*;

use pagetop::base::component::table::{Column, Row};
use pagetop_htmx::hx;
use pagetop_htmx::hx_table::sort_link;

use crate::ADMIN_USERS_PATH;
use crate::LOCALES_USER;
use crate::account::UserStatus;
use crate::permission::UserPermission;
use crate::service::user_admin::{UserListItem, UserSortField};

#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct UserTable {
    props: Props,
    items: Vec<UserListItem>,
    sort: UserSortField,
    dir: SortDir,
    query: Option<String>,
    page: u64,
    per_page: u64,
    total: u64,
}

#[async_trait]
impl Component for UserTable {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::set_id("user-table-wrapper"));
        self.alter_prop(PropsOp::prepend_classes("user-admin-table-wrapper"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let pager = Pager::new()
            .with_base_path(ADMIN_USERS_PATH)
            .with_extra_query("q", self.query().cloned().unwrap_or_default())
            .with_extra_query("sort", self.sort().as_str())
            .with_extra_query("dir", *self.dir())
            .with_current_page(self.page())
            .with_items_per_page(self.per_page())
            .with_total_items(self.total())
            .with_prop(PropsOp::set(hx::BOOST, "true"))
            .with_prop(PropsOp::set(hx::TARGET, "#user-table-wrapper"))
            .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML_SCROLL_TOP))
            .with_prop(PropsOp::set(hx::PUSH_URL, "true"))
            .render(cx)
            .await;

        let mut table = Table::new()
            .with_prop(PropsOp::add_classes("user-admin-table"))
            .with_column(self.sort_column(cx, UserSortField::Username, "col-username"))
            .with_column(self.sort_column(cx, UserSortField::Email, "col-email"))
            .with_column(Lc::t("col-display-name", &LOCALES_USER))
            .with_column(Lc::t("col-roles", &LOCALES_USER))
            .with_column(Lc::t("col-status", &LOCALES_USER))
            .with_column(Lc::t("col-actions", &LOCALES_USER))
            .with_empty(Lc::t("empty-users-list", &LOCALES_USER));

        let waypoint = Waypoint::from(self.list_href(cx));
        let can_assign_roles = cx
            .request()
            .is_some_and(|r| has_permission(r, &UserPermission::AssignRoles));

        for user in self.items() {
            let status = user.status;

            table.alter_row(
                Row::new()
                    .with_cell(username_cell(user, &waypoint))
                    .with_cell(user.email.as_str())
                    .with_cell(user.display_name.as_deref().unwrap_or("-"))
                    .with_cell(roles_cell(user, cx).await)
                    .with_cell(Lc::t(status_key(status), &LOCALES_USER))
                    .with_cell(actions_cell(user, &waypoint, can_assign_roles, cx).await),
            );
        }

        let new_href = waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/new")));

        Ok(html! {
            div (self.props()) {
                div.user-admin-actions {
                    a href=(new_href) {
                        (Lc::t("btn-create-user", &LOCALES_USER).using(cx))
                    }
                }
                (table.render(cx).await)
                (pager)
            }
        })
    }
}

impl UserTable {
    // **< UserTable BUILDER >**********************************************************************

    #[builder_fn]
    pub(crate) fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    #[builder_fn]
    pub(crate) fn with_items(mut self, items: Vec<UserListItem>) -> Self {
        self.items = items;
        self
    }

    #[builder_fn]
    pub(crate) fn with_sort(mut self, sort: UserSortField) -> Self {
        self.sort = sort;
        self
    }

    #[builder_fn]
    pub(crate) fn with_dir(mut self, dir: SortDir) -> Self {
        self.dir = dir;
        self
    }

    #[builder_fn]
    pub(crate) fn with_query(mut self, query: impl Into<Option<String>>) -> Self {
        self.query = query.into();
        self
    }

    #[builder_fn]
    pub(crate) fn with_page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }

    #[builder_fn]
    pub(crate) fn with_per_page(mut self, per_page: u64) -> Self {
        self.per_page = per_page;
        self
    }

    #[builder_fn]
    pub(crate) fn with_total(mut self, total: u64) -> Self {
        self.total = total;
        self
    }

    // URL del listado con el estado actual (búsqueda, orden, página): es el valor que viaja como
    // `waypoint` en los enlaces de ver/editar, para poder volver exactamente a este mismo estado.
    // Se construye con `cx.route()` para que preserve el parámetro `lang` cuando corresponda.
    fn list_href(&self, cx: &Context) -> String {
        let mut route = cx.route(ADMIN_USERS_PATH);
        if let Some(q) = self.query().filter(|q| !q.is_empty()) {
            route.alter_param("q", q);
        }
        route
            .alter_param("sort", self.sort().as_str())
            .alter_param("dir", *self.dir())
            .alter_param("page", self.page().to_string())
            .to_string()
    }

    // Construye la cabecera ordenable de una columna: el enlace ya funciona sin HTMX (navega con
    // una petición normal); `pagetop_htmx::hx_table::sort_link()` añade aparte los atributos `hx-*`
    // para que la tabla se actualice sin recargar la página cuando la extensión esté disponible.
    fn sort_column(&self, cx: &Context, field: UserSortField, label_key: &'static str) -> Column {
        let is_active = *self.sort() == field;
        let active = is_active.then_some(*self.dir());
        let next_dir = SortDir::next_for(active);

        let mut route = cx.route(ADMIN_USERS_PATH);
        if let Some(q) = self.query().filter(|q| !q.is_empty()) {
            route.alter_param("q", q);
        }
        route
            .alter_param("sort", field.as_str())
            .alter_param("dir", next_dir);

        Column::new(Lc::t(label_key, &LOCALES_USER)).with_sort(sort_link(
            route,
            "#user-table-wrapper",
            active,
        ))
    }
}

// Construye la celda de usuario: el nombre enlaza a la pantalla de sólo lectura con todos sus
// datos y roles. Devuelve un componente `Html` para que el marcado se genere cuando `Table`
// renderice la celda, no al construir la fila.
fn username_cell(user: &UserListItem, waypoint: &Waypoint) -> Html {
    let username = user.username.clone();
    let id = user.id;
    let waypoint = waypoint.clone();
    Html::with(move |cx| {
        let view_href = waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/{id}/view")));
        html! {
            a href=(view_href) { (username.as_str()) }
        }
    })
}

// Construye la celda de acciones: editar siempre, y gestionar roles sólo si el usuario autenticado
// tiene permiso para asignarlos. Devuelve un componente `Html` para que el marcado se genere
// cuando `Table` renderice la celda, no al construir la fila.
async fn actions_cell(
    user: &UserListItem,
    waypoint: &Waypoint,
    can_assign_roles: bool,
    cx: &mut Context,
) -> Html {
    let id = user.id;
    let edit_href = waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/{id}/edit")));

    // El botón se renderiza aquí, no dentro del `Html::with()` de abajo: necesita pasar por su
    // propio ciclo de renderizado (`.render().await`) para que el tema activo lo estilice igual
    // que el resto de acciones (ver `pagetop-bootsier::theme::bs::button`).
    let edit_button = Button::anchor(Lc::t("btn-edit", &LOCALES_USER), edit_href)
        .with_style(button::Style::Solid(Intent::Primary))
        .with_size(button::Size::Small)
        .render(cx)
        .await;

    let roles_button = if can_assign_roles {
        let roles_href = waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/{id}/roles")));
        Some(
            Button::anchor(Lc::t("btn-manage-roles", &LOCALES_USER), roles_href)
                .with_style(button::Style::Solid(Intent::Neutral))
                .with_size(button::Size::Small)
                .render(cx)
                .await,
        )
    } else {
        None
    };

    Html::with(move |_cx| {
        html! {
            (edit_button)
            @if let Some(roles_button) = &roles_button {
                " "
                (roles_button)
            }
        }
    })
}

// Construye la celda de roles: la insignia de administrador y las insignias de cada rol asignado,
// o "-" si el usuario no tiene ningún rol ni es administrador. Los badges se renderizan aquí mismo
// (con el `cx` del ciclo de renderizado de `Table`); el resto del marcado se difiere al `Html` que
// se devuelve, igual que el resto de celdas.
async fn roles_cell(user: &UserListItem, cx: &mut Context) -> Html {
    let admin_badge = if user.is_admin {
        Some(
            Badge::labeled(Lc::t("badge-admin", &LOCALES_USER))
                .with_prop(PropsOp::add_classes("user-admin-badge-admin"))
                .render(cx)
                .await,
        )
    } else {
        None
    };

    let mut role_badges = Vec::with_capacity(user.roles.len());
    for role in &user.roles {
        role_badges.push(
            Badge::labeled(Lc::n(role.clone()))
                .with_prop(PropsOp::add_classes("user-admin-badge"))
                .render(cx)
                .await,
        );
    }

    let is_admin = user.is_admin;
    Html::with(move |_cx| {
        html! {
            @if let Some(badge) = &admin_badge {
                (badge)
                " "
            }
            @if role_badges.is_empty() {
                @if !is_admin { "-" }
            } @else {
                @for badge in &role_badges {
                    (badge)
                    " "
                }
            }
        }
    })
}

pub(crate) fn status_key(status: UserStatus) -> &'static str {
    match status {
        UserStatus::Active => "status-active",
        UserStatus::Blocked => "status-blocked",
        UserStatus::Pending => "status-pending",
    }
}
