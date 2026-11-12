//! Listado de usuarios: cabeceras ordenables, filas y paginación, en un `ListView`.

use pagetop::prelude::*;
use pagetop_htmx::hx_pager;
use pagetop_htmx::hx_table::sort_link;

use crate::ADMIN_USERS_PATH;
use crate::LOCALES_USER;
use crate::account::{Account, UserStatus};
use crate::provider;
use crate::service::user_admin::{UserListItem, UserSortField};
use crate::user_path;

#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct UserTable {
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

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let pager = Pager::new()
            .with_base_path(ADMIN_USERS_PATH.as_str())
            .with_extra_query("q", self.query().cloned().unwrap_or_default())
            .with_extra_query("sort", self.sort().as_str())
            .with_extra_query("dir", *self.dir())
            .with_current_page(self.page())
            .with_items_per_page(self.per_page())
            .with_total_items(self.total());

        let mut table = Table::new()
            .with_prop(PropsOp::add_classes("user-admin-table"))
            .with_column(self.sort_column(cx, UserSortField::Username, "col-username"))
            .with_column(self.sort_column(cx, UserSortField::Email, "col-email"))
            .with_column(Lc::t("col-display-name", &LOCALES_USER))
            .with_column(Lc::t("col-roles", &LOCALES_USER))
            .with_column(Lc::t("col-status", &LOCALES_USER))
            .with_column(Lc::t("col-actions", &LOCALES_USER))
            .with_empty(Lc::t("empty-users-list", &LOCALES_USER));

        let viewer_is_admin = cx
            .request()
            .and_then(|r| r.extension::<Account>())
            .is_some_and(|a| a.is_admin());

        for user in self.items() {
            let status = user.status;

            table.alter_row(
                table::Row::new()
                    .with_cell(username_cell(user))
                    .with_cell(user.email.as_str())
                    .with_cell(user.display_name.as_deref().unwrap_or("-"))
                    .with_cell(roles_cell(user, cx).await)
                    .with_cell(Lc::t(status_key(status), &LOCALES_USER))
                    .with_cell(actions_cell(user, viewer_is_admin, cx).await),
            );
        }

        let new_href = cx.route(util::join!(ADMIN_USERS_PATH.as_str(), "/new"));

        let mut list = ListView::new().with_id("user-table-wrapper").with_action(
            Button::anchor(Lc::t("btn-create-user", &LOCALES_USER), new_href.clone())
                .with_style(button::Style::Solid(Intent::Primary)),
        );
        // Alta de una cuenta que entrará con un proveedor de autenticación (ver `new_get()`).
        for provider in provider::active_providers() {
            let name = provider.label().lookup(cx).unwrap_or_default();
            let href = new_href.clone().with_param("provider", provider.id());
            list = list.with_action(
                Button::anchor(
                    Lc::t("btn-create-external-user", &LOCALES_USER).with_arg("provider", name),
                    href,
                )
                .with_style(button::Style::Solid(Intent::Neutral)),
            );
        }
        let mut list = list
            .with_table(table)
            .with_pager(hx_pager::boost(pager, "#user-table-wrapper"));
        Ok(list.render(cx).await)
    }
}

#[builder_impl]
impl UserTable {
    // **< UserTable BUILDER >**********************************************************************

    pub(crate) fn with_items(mut self, items: Vec<UserListItem>) -> Self {
        self.items = items;
        self
    }

    pub(crate) fn with_sort(mut self, sort: UserSortField) -> Self {
        self.sort = sort;
        self
    }

    pub(crate) fn with_dir(mut self, dir: SortDir) -> Self {
        self.dir = dir;
        self
    }

    pub(crate) fn with_query(mut self, query: impl Into<Option<String>>) -> Self {
        self.query = query.into();
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

    // Construye la cabecera ordenable de una columna: el enlace ya funciona sin HTMX (navega con
    // una petición normal); `pagetop_htmx::hx_table::sort_link()` añade aparte los atributos `hx-*`
    // para que la tabla se actualice sin recargar la página cuando la extensión esté disponible.
    fn sort_column(
        &self,
        cx: &Context,
        field: UserSortField,
        label_key: &'static str,
    ) -> table::Column {
        let is_active = *self.sort() == field;
        let active = is_active.then_some(*self.dir());
        let next_dir = SortDir::next_for(active);

        let mut route = cx.route(ADMIN_USERS_PATH.as_str());
        if let Some(q) = self.query().filter(|q| !q.is_empty()) {
            route.alter_param("q", q);
        }
        route
            .alter_param("sort", field.as_str())
            .alter_param("dir", next_dir);

        table::Column::new(Lc::t(label_key, &LOCALES_USER)).with_sort(sort_link(
            route,
            "#user-table-wrapper",
            active,
        ))
    }
}

// Construye la celda de usuario: el nombre enlaza a la pantalla de sólo lectura con todos sus
// datos y roles. Devuelve un componente `Html` para que el marcado se genere cuando `Table`
// renderice la celda, no al construir la fila.
fn username_cell(user: &UserListItem) -> Html {
    let username = user.username.clone();
    let id = user.id;
    Html::with(move |cx| {
        let view_href = cx.route(user_path(id, "view"));
        html! {
            a href=(view_href) { (username.as_str()) }
        }
    })
}

// Construye la celda de acciones con el botón de edición, salvo en la cuenta de un administrador
// si quien mira no lo es (no puede gestionarla, ver `handlers::admin::users::require_manageable`).
// Devuelve un componente `Html` para que el marcado se genere cuando `Table` renderice la celda, no
// al construir la fila.
async fn actions_cell(user: &UserListItem, viewer_is_admin: bool, cx: &mut Context) -> Html {
    if user.is_admin && !viewer_is_admin {
        return Html::default();
    }
    let edit_href = cx.route(user_path(user.id, "edit"));

    // El botón se renderiza aquí, no dentro del `Html::with()` de abajo: necesita pasar por su
    // propio ciclo de renderizado (`.render().await`) para que el tema activo lo estilice igual
    // que el resto de acciones (ver `pagetop-bootsier::theme::bs::button`).
    let edit_button = Button::anchor(Lc::t("btn-edit", &LOCALES_USER), edit_href)
        .with_style(button::Style::Solid(Intent::Primary))
        .with_size(button::Size::Small)
        .render(cx)
        .await;

    Html::with(move |_cx| edit_button.clone())
}

// Construye la celda de roles: sólo la insignia de administrador si lo es (tiene acceso sin
// restricciones, así que sus roles no añaden nada), o las insignias de cada rol asignado, o "-" si
// no tiene ninguno. Los badges se renderizan aquí mismo (con el `cx` del ciclo de renderizado de
// `Table`); el resto del marcado se difiere al `Html` que se devuelve, como en las demás celdas.
async fn roles_cell(user: &UserListItem, cx: &mut Context) -> Html {
    let mut badges = Vec::with_capacity(user.roles.len().max(1));
    if user.is_admin {
        badges.push(
            Badge::severe(Lc::t("badge-admin", &LOCALES_USER))
                .render(cx)
                .await,
        );
    } else {
        for role in &user.roles {
            badges.push(Badge::neutral(Lc::n(role.clone())).render(cx).await);
        }
    }

    Html::with(move |_cx| {
        html! {
            @if badges.is_empty() {
                "-"
            } @else {
                @for badge in &badges {
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
        UserStatus::AwaitingApproval => "status-awaiting-approval",
    }
}
