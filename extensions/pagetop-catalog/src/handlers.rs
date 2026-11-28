// Páginas de administración de los catálogos, iguales para todos: cada una recibe la definición del
// catálogo, que `Catalogs::configure_router()` fija al montar sus rutas.

use pagetop::prelude::*;
use pagetop_htmx::hx_table::sort_link;
use pagetop_htmx::prelude::*;
use pagetop_htmx::{hx, hx_confirm, hx_filter, hx_pager};

use serde::Deserialize;
use serde_json::Value as Json;

use std::collections::HashMap;

use crate::catalog::{Catalog, Field, FieldKind};
use crate::declare::catalog as find_catalog;
use crate::record::{FieldValue, Record};
use crate::schema::{CODE, NAME, NAME_MAX_LEN, WEIGHT};
use crate::service::{
    self, CatalogError, Input, ListParams, SortField, StatusFilter, column_label,
    translation_input, translation_languages,
};
use crate::{LOCALES_CATALOG, config};

// Identificador del listado, que actualizan el filtro, el orden, el paginador y las acciones de
// cada fila.
const LIST_ID: &str = "catalog-list";
const LIST_TARGET: &str = "#catalog-list";
const FILTER_ID: &str = "catalog-filter";

fn t(key: &'static str) -> Lc {
    Lc::t(key, &LOCALES_CATALOG)
}

// Ruta completa del catálogo, o de una de sus páginas (`"new"`, `"{id}/edit"`...).
fn path(catalog: &Catalog, rest: &str) -> String {
    if rest.is_empty() {
        admin_path(catalog.path())
    } else {
        admin_path(&util::join!(catalog.path(), "/", rest))
    }
}

fn record_path(catalog: &Catalog, id: i32, page: &str) -> String {
    path(catalog, &util::join!(id.to_string(), "/", page))
}

fn user_id(request: &HttpRequest) -> Option<i32> {
    request.extension::<CurrentUser>().and_then(|u| u.id())
}

fn internal_error(request: &HttpRequest, err: impl std::fmt::Display) -> ErrorPage {
    trace::error!("Catalog page failed: {err}");
    ErrorPage::InternalError(request.clone())
}

// **< Listado >************************************************************************************

#[derive(AutoDefault, Deserialize)]
pub(crate) struct ListQuery {
    #[serde(default)]
    q: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    sort: Option<String>,
    #[serde(default)]
    dir: Option<String>,
    // Como texto: con `flatten`, todos los valores llegan como texto.
    #[serde(default)]
    page: Option<String>,
    // Filtros por campo (`f_<columna>`).
    #[serde(flatten)]
    filters: HashMap<String, String>,
}

impl ListQuery {
    fn params(&self, catalog: &Catalog) -> ListParams {
        ListParams {
            query: self.q.clone().unwrap_or_default(),
            status: StatusFilter::from_query(self.status.as_deref()),
            filters: catalog
                .fields()
                .iter()
                .filter(|f| f.filter())
                .filter_map(|f| {
                    let value = self.filters.get(&filter_param(f.column()))?.trim();
                    (!value.is_empty()).then(|| (f.column(), value.to_owned()))
                })
                .collect(),
            sort: SortField::from_query(self.sort.as_deref()),
            dir: SortDir::from_query(self.dir.as_deref()),
            page: self
                .page
                .as_deref()
                .and_then(|p| p.parse().ok())
                .unwrap_or(1)
                .max(1),
        }
    }
}

// Parámetro del filtro por el campo `column`.
fn filter_param(column: &str) -> String {
    util::join!("f_", column)
}

// **< Nombres de los registros >*******************************************************************

// Nombre de cada registro del catálogo al que se refiere el calificador de `catalog`, por `id`.
async fn qualifier_names(
    catalog: &Catalog,
    cx: &Context,
) -> Result<HashMap<i32, String>, CatalogError> {
    let Some((column, _)) = catalog.qualifier() else {
        return Ok(HashMap::new());
    };
    let Some(FieldKind::Reference(target)) = catalog.field(column).map(Field::kind) else {
        return Ok(HashMap::new());
    };
    let Some(target) = find_catalog(target) else {
        return Ok(HashMap::new());
    };
    Ok(service::all(target)
        .await?
        .iter()
        .map(|r| (r.id(), r.localized_name(cx).to_owned()))
        .collect())
}

// Nombre con el que se identifica un registro: "código - nombre" o, sin código, el nombre,
// calificado si el catálogo tiene calificador.
fn record_label(
    catalog: &Catalog,
    record: &Record,
    qualifiers: &HashMap<i32, String>,
    cx: &Context,
) -> String {
    let name = record.localized_name(cx);
    if catalog.has_code() {
        return util::join!(record.code(), " - ", name);
    }
    if let Some((column, qualify)) = catalog.qualifier()
        && let Some(qualifier) = record
            .value(catalog, column)
            .and_then(FieldValue::as_int)
            .and_then(|id| qualifiers.get(&id))
    {
        return qualify(name, qualifier);
    }
    name.to_owned()
}

// Nombre de cada registro del catálogo, por `id`, con si está activo, en el orden de presentación.
async fn labels(catalog: &Catalog, cx: &Context) -> Result<Vec<(i32, String, bool)>, CatalogError> {
    let qualifiers = qualifier_names(catalog, cx).await?;
    let mut records = service::all(catalog).await?;
    records.sort_by(|a, b| {
        a.weight()
            .cmp(&b.weight())
            .then_with(|| a.name().cmp(b.name()))
    });
    Ok(records
        .iter()
        .map(|r| {
            (
                r.id(),
                record_label(catalog, r, &qualifiers, cx),
                r.is_active(),
            )
        })
        .collect())
}

// Ruta del listado con su estado (búsqueda, estado, orden y página), para volver a él.
fn list_route(catalog: &Catalog, cx: &Context, params: &ListParams, page: bool) -> RoutePath {
    with_state(cx.route(path(catalog, "")), params, page)
}

// Añade a `route` el estado del listado.
fn with_state(mut route: RoutePath, params: &ListParams, page: bool) -> RoutePath {
    if !params.query.is_empty() {
        route.alter_param("q", &params.query);
    }
    if params.status != StatusFilter::default() {
        route.alter_param("status", params.status.as_str());
    }
    if params.sort != SortField::default() {
        route.alter_param("sort", params.sort.as_str());
    }
    if params.dir != SortDir::default() {
        route.alter_param("dir", params.dir.as_str());
    }
    for (column, value) in &params.filters {
        route.alter_param(filter_param(column), value);
    }
    if page && params.page > 1 {
        route.alter_param("page", params.page.to_string());
    }
    route
}

async fn filter(
    catalog: &Catalog,
    params: &ListParams,
    cx: &Context,
) -> Result<ListFilter, CatalogError> {
    let status = |value: StatusFilter, key| {
        form::select::Item::new(value.as_str(), t(key)).with_selected(params.status == value)
    };
    let mut filter = ListFilter::new()
        .with_id(FILTER_ID)
        .with_filter_action(path(catalog, ""))
        .with_field(
            form::input::Field::search()
                .with_name("q")
                .with_value(&params.query)
                .with_label(t("filter-search"))
                // Sin ayuda bajo el campo, que descuadraría la fila del filtro: la pista va dentro.
                .with_placeholder(if catalog.has_code() {
                    t("placeholder-search")
                } else {
                    t("placeholder-search-name")
                }),
        );
    for field in catalog.fields().iter().filter(|f| f.filter()) {
        let current = params
            .filters
            .iter()
            .find(|(column, _)| *column == field.column())
            .map_or("", |(_, value)| value.as_str());
        let options: Vec<(String, Lc)> = match field.kind() {
            FieldKind::Reference(target) => match find_catalog(target) {
                Some(target) => labels(target, cx)
                    .await?
                    .into_iter()
                    .map(|(id, label, _)| (id.to_string(), Lc::n(label)))
                    .collect(),
                None => Vec::new(),
            },
            FieldKind::Choice(choices) => choices
                .iter()
                .map(|(value, label)| ((*value).to_owned(), label.clone()))
                .collect(),
            FieldKind::Boolean => vec![
                ("true".to_owned(), t("value-yes")),
                ("false".to_owned(), t("value-no")),
            ],
            _ => Vec::new(),
        };
        let mut select = form::select::Field::new()
            .with_name(filter_param(field.column()))
            .with_label(field.label().clone())
            .with_item(
                form::select::Item::new("", t("filter-all")).with_selected(current.is_empty()),
            );
        for (value, label) in options {
            let selected = value == current;
            select =
                select.with_item(form::select::Item::new(value, label).with_selected(selected));
        }
        filter = filter.with_field(select);
    }
    Ok(hx_filter::boost(
        filter.with_field(
            form::select::Field::new()
                .with_name("status")
                .with_label(t("filter-status"))
                .with_item(status(StatusFilter::Active, "status-active"))
                .with_item(status(StatusFilter::Inactive, "status-inactive"))
                .with_item(status(StatusFilter::All, "status-all")),
        ),
        LIST_TARGET,
    ))
}

fn sort_column(
    catalog: &Catalog,
    cx: &Context,
    params: &ListParams,
    field: SortField,
    label: Lc,
) -> table::Column {
    let active = (params.sort == field).then_some(params.dir);
    let mut sorted = params.clone();
    sorted.sort = field;
    sorted.dir = SortDir::next_for(active);
    table::Column::new(label).with_sort(sort_link(
        list_route(catalog, cx, &sorted, false),
        LIST_TARGET,
        active,
    ))
}

// Texto de un valor en el listado, el historial y los formularios de sólo lectura.
fn display_value(field: &Field, value: &FieldValue, names: &HashMap<i32, String>) -> Lc {
    match (field.kind(), value) {
        (_, value) if value.is_empty() => Lc::n("-"),
        (FieldKind::Boolean, value) => t(if value.as_bool() {
            "value-yes"
        } else {
            "value-no"
        }),
        (FieldKind::Choice(options), FieldValue::Text(Some(text))) => options
            .iter()
            .find(|(v, _)| v == text)
            .map_or_else(|| Lc::n(text.clone()), |(_, label)| label.clone()),
        (FieldKind::Reference(_), FieldValue::Int(Some(id))) => {
            Lc::n(names.get(id).cloned().unwrap_or_else(|| id.to_string()))
        }
        (_, value) => Lc::n(value.to_form()),
    }
}

// Nombres de los registros de los catálogos a los que se refieren los campos `fields`, por
// catálogo y `id`.
async fn reference_names(
    fields: impl Iterator<Item = &Field>,
    cx: &Context,
) -> Result<HashMap<&'static str, HashMap<i32, String>>, CatalogError> {
    let mut names = HashMap::new();
    for field in fields {
        if let FieldKind::Reference(target) = field.kind()
            && !names.contains_key(target)
            && let Some(catalog) = find_catalog(target)
        {
            let labels = labels(catalog, cx).await?;
            names.insert(
                *target,
                labels
                    .into_iter()
                    .map(|(id, label, _)| (id, label))
                    .collect(),
            );
        }
    }
    Ok(names)
}

async fn list_view(
    catalog: &'static Catalog,
    cx: &mut Context,
    params: &ListParams,
) -> Result<ListView, CatalogError> {
    let page = service::list(catalog, params).await?;
    let can_manage = cx
        .request()
        .is_some_and(|r| has_permission(r, catalog.manage_permission()));
    let listed: Vec<&Field> = catalog.fields().iter().filter(|f| f.in_list()).collect();
    let names = reference_names(listed.iter().copied(), cx).await?;
    let qualifiers = qualifier_names(catalog, cx).await?;

    let mut table = Table::new().with_prop(PropsOp::add_classes("catalog-table"));
    if catalog.has_code() {
        table = table.with_column(sort_column(
            catalog,
            cx,
            params,
            SortField::Code,
            t("field-code"),
        ));
    }
    table = table.with_column(sort_column(
        catalog,
        cx,
        params,
        SortField::Name,
        t("field-name"),
    ));
    for field in &listed {
        table = table.with_column(field.label().clone());
    }
    table = table
        .with_column(sort_column(
            catalog,
            cx,
            params,
            SortField::Weight,
            t("field-weight"),
        ))
        .with_column(t("col-actions"))
        .with_empty(t("empty-list"));

    let mut state = params.clone();
    state.page = page.page;
    for record in &page.items {
        let mut row = table::Row::new();
        if catalog.has_code() {
            row = row.with_cell(record.code().to_owned());
        }
        let name = record.localized_name(cx).to_owned();
        row = row.with_cell(if record.is_active() {
            Html::with(move |_| html! { (name) })
        } else {
            let badge = Badge::neutral(t("status-inactive")).render(cx).await;
            Html::with(move |_| html! { (name) " " (badge) })
        });
        for field in &listed {
            let value = record
                .value(catalog, field.column())
                .cloned()
                .unwrap_or_else(|| FieldValue::empty(field.kind()));
            let empty = HashMap::new();
            let names = match field.kind() {
                FieldKind::Reference(target) => names.get(target).unwrap_or(&empty),
                _ => &empty,
            };
            row = row.with_cell(display_value(field, &value, names));
        }
        row = row.with_cell(record.weight().to_string());
        let label = record_label(catalog, record, &qualifiers, cx);
        let actions = row_actions(catalog, cx, record, label, &state, can_manage).await;
        table = table.with_row(row.with_cell(table::Cell::new(actions)));
    }

    let mut pager = Pager::new()
        .with_base_path(path(catalog, ""))
        .with_extra_query("q", params.query.clone())
        .with_extra_query("status", params.status.as_str())
        .with_extra_query("sort", params.sort.as_str())
        .with_extra_query("dir", params.dir.as_str());
    for (column, value) in &params.filters {
        pager = pager.with_extra_query(filter_param(column), value.clone());
    }
    let pager = pager
        .with_current_page(page.page)
        .with_items_per_page(page.per_page)
        .with_total_items(page.total);

    let mut list = ListView::new().with_id(LIST_ID);
    if can_manage {
        list = list
            .with_action(
                Button::anchor(t("btn-new"), path(catalog, "new"))
                    .with_style(button::Style::Solid(Intent::Primary)),
            )
            .with_action(
                Button::anchor(t("btn-import"), path(catalog, "import"))
                    .with_style(button::Style::Outline(Intent::Neutral)),
            );
    }
    // La exportación lleva los registros que cumplen el filtro vigente.
    let mut export = cx.route(path(catalog, "export.csv"));
    export = with_state(export, params, false);
    list = list.with_action(
        Button::anchor(t("btn-export"), export.to_string())
            .with_style(button::Style::Outline(Intent::Neutral)),
    );
    Ok(list
        .with_table(table)
        .with_pager(hx_pager::boost(pager, LIST_TARGET))
        .with_filter_id(FILTER_ID)
        .with_filter_param("sort", params.sort.as_str())
        .with_filter_param("dir", params.dir.as_str()))
}

// Botones de una fila: editar (o ver, sin permiso para gestionar), desactivar o reactivar y
// borrar. Las dos últimas se envían con HTMX y devuelven el listado actualizado, con su estado.
async fn row_actions(
    catalog: &Catalog,
    cx: &mut Context,
    record: &Record,
    name: String,
    state: &ListParams,
    can_manage: bool,
) -> Flex {
    let hidden = Lc::n(name.clone());
    let edit = cx.route(record_path(catalog, record.id(), "edit"));
    let mut buttons = Flex::buttons();
    if !can_manage {
        return buttons.with_child(
            Button::anchor(t("btn-view"), edit)
                .with_size(button::Size::Small)
                .with_hidden_label(hidden),
        );
    }
    buttons = buttons.with_child(
        Button::edit(edit)
            .with_size(button::Size::Small)
            .with_hidden_label(hidden.clone()),
    );
    let action_route = |action: &str| {
        with_state(
            cx.route(record_path(catalog, record.id(), action)),
            state,
            true,
        )
        .to_string()
    };
    let (status_action, status_label) = if record.is_active() {
        ("deactivate", "btn-deactivate")
    } else {
        ("reactivate", "btn-reactivate")
    };
    buttons = buttons.with_child(
        Button::plain(t(status_label))
            .with_style(button::Style::Outline(Intent::Neutral))
            .with_size(button::Size::Small)
            .with_hidden_label(hidden.clone())
            .with_prop(PropsOp::set(hx::POST, action_route(status_action)))
            .with_prop(PropsOp::set(hx::TARGET, LIST_TARGET))
            .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML)),
    );
    buttons.with_child(hx_confirm::confirm_delete(
        Button::delete()
            .with_size(button::Size::Small)
            .with_hidden_label(hidden)
            .with_prop(PropsOp::set(hx::POST, action_route("delete")))
            .with_prop(PropsOp::set(hx::TARGET, LIST_TARGET))
            .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML_SCROLL_TOP)),
        t("confirm-delete").with_arg("name", name),
    ))
}

// Respuesta del listado: sólo el listado si la petición llega de HTMX (filtro, orden, paginador o
// acciones de una fila), o la página completa, con el filtro y el diálogo de borrado.
async fn list_response(
    catalog: &'static Catalog,
    request: HttpRequest,
    params: &ListParams,
    messages: Vec<(MessageLevel, Lc)>,
) -> Result<Response, ErrorPage> {
    if request.is_htmx() {
        let mut cx = Context::admin(request.clone());
        for (level, text) in messages {
            cx.push_message(level, text);
        }
        let mut list = list_view(catalog, &mut cx, params)
            .await
            .map_err(|e| internal_error(&request, e))?;
        return Ok(HtmxResponse::new(list.render(&mut cx).await).into_response());
    }
    let mut page = Page::admin(request.clone()).with_title(catalog.title().clone());
    for (level, text) in messages {
        page.push_message(level, text);
    }
    let list = list_view(catalog, page.context_mut(), params)
        .await
        .map_err(|e| internal_error(&request, e))?;
    Ok(page
        .with_child(
            filter(catalog, params, &Context::admin(request.clone()))
                .await
                .map_err(|e| internal_error(&request, e))?,
        )
        .with_child(list)
        .with_child(hx_confirm::dialog_delete())
        .render()
        .await?
        .into_response())
}

/// GET {catálogo} - Listado del catálogo.
pub(crate) async fn list_get(
    catalog: &'static Catalog,
    request: HttpRequest,
    query: ListQuery,
) -> Result<Response, ErrorPage> {
    list_response(catalog, request, &query.params(catalog), Vec::new()).await
}

// Resultado de una acción sobre una fila: con HTMX, el listado con su estado y el mensaje; sin
// HTMX, vuelta al listado con el mensaje.
async fn row_action_response(
    catalog: &'static Catalog,
    request: HttpRequest,
    query: ListQuery,
    result: Result<(), CatalogError>,
    done: &'static str,
) -> Result<Response, ErrorPage> {
    let messages = match &result {
        Ok(()) => vec![(MessageLevel::Info, t(done))],
        Err(CatalogError::Db(err)) => return Err(internal_error(&request, err)),
        Err(CatalogError::NotFound) if !request.is_htmx() => {
            return Err(ErrorPage::NotFound(request));
        }
        Err(err) => err
            .messages()
            .into_iter()
            .map(|m| (MessageLevel::Error, m))
            .collect(),
    };
    let params = query.params(catalog);
    if request.is_htmx() {
        return list_response(catalog, request, &params, messages).await;
    }
    let cx = Context::admin(request);
    let mut response = Redirect::see_other(list_route(catalog, &cx, &params, true)).into_response();
    for (level, text) in messages {
        response = response.with_message(&cx, level, text);
    }
    Ok(response)
}

/// POST {catálogo}/{id}/deactivate y {catálogo}/{id}/reactivate.
pub(crate) async fn status_post(
    catalog: &'static Catalog,
    request: HttpRequest,
    id: i32,
    query: ListQuery,
    active: bool,
) -> Result<Response, ErrorPage> {
    require_permission(&request, catalog.manage_permission())?;
    let result = service::set_active(catalog, id, active, user_id(&request)).await;
    let done = if active {
        "done-reactivated"
    } else {
        "done-deactivated"
    };
    row_action_response(catalog, request, query, result, done).await
}

/// POST {catálogo}/{id}/delete - Borra el registro si nadie lo usa.
pub(crate) async fn delete_post(
    catalog: &'static Catalog,
    request: HttpRequest,
    id: i32,
    query: ListQuery,
) -> Result<Response, ErrorPage> {
    require_permission(&request, catalog.manage_permission())?;
    let result = service::delete(catalog, id, user_id(&request)).await;
    row_action_response(catalog, request, query, result, "done-deleted").await
}

// **< Formularios >********************************************************************************

// Campo del formulario para una columna propia.
fn field_control(
    field: &Field,
    value: &FieldValue,
    options: &HashMap<&'static str, Vec<(String, String)>>,
) -> ChildOp {
    let name = field.column();
    let label = field.label().clone();
    let help = field.help().clone();
    let required = field.required();
    let max = |len: Option<usize>| len.and_then(|l| u16::try_from(l).ok());
    match field.kind() {
        FieldKind::Text => form::input::Field::text()
            .with_name(name)
            .with_value(value.to_form())
            .with_label(label)
            .with_help(help)
            .with_maxlength(max(field.max_len()))
            .with_required(required)
            .into(),
        FieldKind::TextArea => form::Textarea::new()
            .with_name(name)
            .with_value(value.to_form())
            .with_label(label)
            .with_help(help)
            .with_maxlength(max(field.max_len()))
            .with_required(required)
            .into(),
        FieldKind::Boolean => form::Checkbox::check()
            .with_name(name)
            .with_label(label)
            .with_help(help)
            .with_checked(value.as_bool())
            .into(),
        FieldKind::Decimal { .. } => form::input::Field::text()
            .with_name(name)
            .with_value(value.to_form())
            .with_label(label)
            .with_help(help)
            .with_inputmode(form::input::Mode::Decimal)
            .with_required(required)
            .into(),
        FieldKind::Integer { .. } => form::input::Field::text()
            .with_name(name)
            .with_value(value.to_form())
            .with_label(label)
            .with_help(help)
            .with_inputmode(form::input::Mode::Numeric)
            .with_required(required)
            .into(),
        FieldKind::Choice(choices) => {
            let current = value.to_form();
            let mut select = form::select::Field::new()
                .with_name(name)
                .with_label(label)
                .with_help(help)
                .with_required(required)
                .with_item(
                    form::select::Item::new("", Lc::n("-")).with_selected(current.is_empty()),
                );
            for (value, label) in choices {
                select = select.with_item(
                    form::select::Item::new(*value, label.clone()).with_selected(*value == current),
                );
            }
            select.into()
        }
        FieldKind::Reference(target) => {
            let current = value.to_form();
            let mut select = form::select::Field::new()
                .with_name(name)
                .with_label(label)
                .with_help(help)
                .with_required(required)
                .with_item(
                    form::select::Item::new("", Lc::n("-")).with_selected(current.is_empty()),
                );
            for (id, text) in options.get(target).into_iter().flatten() {
                select = select.with_item(
                    form::select::Item::new(id.clone(), Lc::n(text.clone()))
                        .with_selected(*id == current),
                );
            }
            select.into()
        }
    }
}

// Opciones de los campos que se refieren a otros catálogos: sus registros activos y, si el valor
// actual es uno inactivo, también ése, para no perderlo al guardar.
async fn reference_options(
    catalog: &Catalog,
    input: &Input,
    cx: &Context,
) -> Result<HashMap<&'static str, Vec<(String, String)>>, CatalogError> {
    let mut options: HashMap<&'static str, Vec<(String, String)>> = HashMap::new();
    for (index, field) in catalog.fields().iter().enumerate() {
        let FieldKind::Reference(target) = field.kind() else {
            continue;
        };
        let Some(target_catalog) = find_catalog(target) else {
            continue;
        };
        let current = input.values[index].as_int();
        let entry = options.entry(*target).or_default();
        for (id, label, active) in labels(target_catalog, cx).await? {
            if (active || current == Some(id)) && !entry.iter().any(|(v, _)| *v == id.to_string()) {
                entry.push((id.to_string(), label));
            }
        }
    }
    Ok(options)
}

enum FormMode {
    Create,
    Edit { id: i32, row_version: i32 },
    View { id: i32 },
}

async fn form_page(
    catalog: &'static Catalog,
    request: HttpRequest,
    mode: FormMode,
    input: &Input,
    errors: Vec<Lc>,
) -> Result<Response, ErrorPage> {
    let mut page = Page::admin(request.clone());
    for error in errors {
        page.push_message(MessageLevel::Error, error);
    }
    let options = reference_options(catalog, input, page.context())
        .await
        .map_err(|e| internal_error(&request, e))?;

    let (title, action, readonly, editing) = match &mode {
        FormMode::Create => (t("title-new"), path(catalog, "new"), false, false),
        FormMode::Edit { id, .. } => (
            t("title-edit"),
            record_path(catalog, *id, "edit"),
            false,
            true,
        ),
        FormMode::View { id } => (
            t("title-view"),
            record_path(catalog, *id, "edit"),
            true,
            true,
        ),
    };
    let title = title.with_lc("catalog", catalog.title().clone());

    let mut fields = form::Fieldset::new()
        .with_help(catalog.form_help().clone())
        .with_disabled(readonly);
    if catalog.has_code() {
        fields = fields.with_child(
            form::input::Field::text()
                .with_name(CODE)
                .with_value(&input.code)
                .with_label(t("field-code"))
                .with_help(if editing {
                    t("help-code-fixed")
                } else {
                    t("help-code")
                })
                .with_maxlength(u16::try_from(catalog.code_max_len()).ok())
                .with_readonly(editing)
                .with_required(!editing),
        );
    }
    fields = fields.with_child(
        form::input::Field::text()
            .with_name(NAME)
            .with_value(&input.name)
            .with_label(t("field-name"))
            .with_help(if catalog.name_help().is_none() {
                t("help-name")
            } else {
                catalog.name_help().clone()
            })
            .with_maxlength(NAME_MAX_LEN as u16)
            .with_required(true),
    );
    for (field, value) in catalog.fields().iter().zip(&input.values) {
        fields = fields.with_child(field_control(field, value, &options));
    }
    fields = fields.with_child(
        form::input::Field::text()
            .with_name(WEIGHT)
            .with_value(input.weight.to_string())
            .with_label(t("field-weight"))
            .with_help(t("help-weight"))
            .with_inputmode(form::input::Mode::Numeric),
    );
    if catalog.translations() {
        let mut translations = form::Fieldset::new()
            .with_legend(t("field-translations"))
            .with_help(t("help-translations"));
        for (langid, label) in translation_languages() {
            translations = translations.with_child(
                form::input::Field::text()
                    .with_name(translation_input(&langid))
                    .with_value(input.translations.get(&langid).map_or("", String::as_str))
                    .with_label(label)
                    .with_maxlength(NAME_MAX_LEN as u16),
            );
        }
        fields = fields.with_child(translations);
    }

    let list = path(catalog, "");
    let mut form = Form::new().with_action(action).with_child(fields);
    if let FormMode::Edit { row_version, .. } = mode {
        form = form.with_child(form::Hidden::field("row_version", row_version.to_string()));
    }
    let mut buttons = Flex::buttons();
    if readonly {
        buttons = buttons.with_child(Button::back(list));
    } else {
        buttons = buttons.with_child(Button::save());
        if matches!(mode, FormMode::Create) {
            buttons = buttons.with_child(Button::save_and_new());
        }
        buttons = buttons.with_child(Button::cancel(list));
    }
    if let FormMode::Edit { id, .. } | FormMode::View { id } = mode
        && service::has_recorders()
    {
        buttons = buttons.with_child(
            Button::anchor(t("btn-history"), record_path(catalog, id, "history"))
                .with_style(button::Style::Outline(Intent::Neutral)),
        );
    }
    Ok(page
        .with_title(title)
        .with_child(form.with_child(buttons))
        .render()
        .await?
        .into_response())
}

// Respuesta tras guardar: vuelta al listado o, con "Guardar y nuevo", un formulario de alta vacío.
fn saved(
    catalog: &Catalog,
    request: HttpRequest,
    save_and_new: bool,
    done: &'static str,
) -> Response {
    let cx = Context::admin(request);
    let target = if save_and_new {
        path(catalog, "new")
    } else {
        path(catalog, "")
    };
    let message = if save_and_new {
        button::SaveAndNew::notice()
    } else {
        t(done)
    };
    Redirect::see_other(cx.route(target))
        .into_response()
        .with_message(&cx, MessageLevel::Info, message)
}

/// GET {catálogo}/new - Formulario de alta.
pub(crate) async fn new_get(
    catalog: &'static Catalog,
    request: HttpRequest,
) -> Result<Response, ErrorPage> {
    require_permission(&request, catalog.manage_permission())?;
    form_page(
        catalog,
        request,
        FormMode::Create,
        &Input::empty(catalog),
        Vec::new(),
    )
    .await
}

/// POST {catálogo}/new - Da de alta un registro.
pub(crate) async fn new_post(
    catalog: &'static Catalog,
    request: HttpRequest,
    form: HashMap<String, String>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, catalog.manage_permission())?;
    let (input, errors) = Input::parse(catalog, &form, true);
    let result = if errors.is_empty() {
        service::create(catalog, &input, user_id(&request))
            .await
            .map(|_| ())
    } else {
        Err(CatalogError::Invalid(errors))
    };
    match result {
        Ok(()) => {
            let save_and_new = form.contains_key("save_and_new");
            Ok(saved(catalog, request, save_and_new, "done-created"))
        }
        Err(CatalogError::Db(err)) => Err(internal_error(&request, err)),
        Err(err) => form_page(catalog, request, FormMode::Create, &input, err.messages()).await,
    }
}

/// GET {catálogo}/{id}/edit - Formulario de edición, o de consulta sin permiso para gestionar.
pub(crate) async fn edit_get(
    catalog: &'static Catalog,
    request: HttpRequest,
    id: i32,
) -> Result<Response, ErrorPage> {
    let record = match service::find(catalog, id).await {
        Ok(record) => record,
        Err(CatalogError::NotFound) => return Err(ErrorPage::NotFound(request)),
        Err(err) => return Err(internal_error(&request, err)),
    };
    let mode = if has_permission(&request, catalog.manage_permission()) {
        FormMode::Edit {
            id,
            row_version: record.row_version(),
        }
    } else {
        FormMode::View { id }
    };
    form_page(
        catalog,
        request,
        mode,
        &Input::from_record(&record),
        Vec::new(),
    )
    .await
}

/// POST {catálogo}/{id}/edit - Modifica un registro.
pub(crate) async fn edit_post(
    catalog: &'static Catalog,
    request: HttpRequest,
    id: i32,
    form: HashMap<String, String>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, catalog.manage_permission())?;
    let row_version = form
        .get("row_version")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let (input, errors) = Input::parse(catalog, &form, false);
    let result = if errors.is_empty() {
        service::update(catalog, id, row_version, &input, user_id(&request)).await
    } else {
        Err(CatalogError::Invalid(errors))
    };
    match result {
        Ok(()) => Ok(saved(catalog, request, false, "done-updated")),
        Err(CatalogError::NotFound) => Err(ErrorPage::NotFound(request)),
        Err(CatalogError::Db(err)) => Err(internal_error(&request, err)),
        // Con otra versión, se muestra la vigente: lo que se había escrito se perdería sin
        // remedio si se guardara encima.
        Err(CatalogError::Conflict) => {
            let record = match service::find(catalog, id).await {
                Ok(record) => record,
                Err(CatalogError::NotFound) => return Err(ErrorPage::NotFound(request)),
                Err(err) => return Err(internal_error(&request, err)),
            };
            let mode = FormMode::Edit {
                id,
                row_version: record.row_version(),
            };
            let messages = CatalogError::Conflict.messages();
            form_page(
                catalog,
                request,
                mode,
                &Input::from_record(&record),
                messages,
            )
            .await
        }
        Err(err) => {
            let mode = FormMode::Edit { id, row_version };
            form_page(catalog, request, mode, &input, err.messages()).await
        }
    }
}

// **< Historial >**********************************************************************************

// Valor de una columna en el historial, que se traduce al idioma de quien lo consulta.
fn history_value(catalog: &Catalog, column: &str, value: &Json) -> Lc {
    match value {
        Json::Null => Lc::n("-"),
        Json::Bool(b) => t(if *b { "value-yes" } else { "value-no" }),
        Json::String(s) => match catalog.field(column).map(Field::kind) {
            Some(FieldKind::Choice(options)) => options
                .iter()
                .find(|(v, _)| v == s)
                .map_or_else(|| Lc::n(s.clone()), |(_, label)| label.clone()),
            _ => Lc::n(s.clone()),
        },
        Json::Object(map) => Lc::n(
            map.iter()
                .map(|(k, v)| util::join!(k, ": ", v.as_str().unwrap_or_default()))
                .collect::<Vec<_>>()
                .join("; "),
        ),
        other => Lc::n(other.to_string()),
    }
}

/// GET {catálogo}/{id}/history - Historial de cambios del registro.
pub(crate) async fn history_get(
    catalog: &'static Catalog,
    request: HttpRequest,
    id: i32,
) -> Result<Response, ErrorPage> {
    let record = match service::find(catalog, id).await {
        Ok(record) => record,
        Err(CatalogError::NotFound) => return Err(ErrorPage::NotFound(request)),
        Err(err) => return Err(internal_error(&request, err)),
    };
    let Some(entries) = service::history(catalog, id)
        .await
        .map_err(|e| internal_error(&request, e))?
    else {
        return Err(ErrorPage::NotFound(request));
    };

    let page = Page::admin(request);
    let mut table = Table::new()
        .with_prop(PropsOp::add_classes("catalog-history"))
        .with_column(t("col-when"))
        .with_column(t("col-who"))
        .with_column(t("col-action"))
        .with_column(t("col-changes"))
        .with_empty(t("empty-history"));
    for entry in &entries {
        let when = page.format_datetime(entry.at, DateFormat::Medium, TimeFormat::Short);
        let changes: Vec<(Lc, Lc, Lc)> = entry
            .fields
            .iter()
            .map(|c| {
                (
                    column_label(catalog, &c.column),
                    history_value(catalog, &c.column, &c.old),
                    history_value(catalog, &c.column, &c.new),
                )
            })
            .collect();
        let comment = entry.comment.clone();
        let changes = Html::with(move |cx| {
            html! {
                @if !changes.is_empty() {
                    ul class="catalog-history-changes" {
                        @for (label, old, new) in &changes {
                            li { (label.using(cx)) ": " (old.using(cx)) " -> " (new.using(cx)) }
                        }
                    }
                }
                @if let Some(comment) = &comment {
                    p class="catalog-history-comment" { (comment) }
                }
            }
        });
        table = table.with_row(
            table::Row::new()
                .with_cell(when)
                .with_cell(entry.user.clone().unwrap_or_else(|| "-".to_owned()))
                .with_cell(entry.action.label())
                .with_cell(table::Cell::new(changes)),
        );
    }
    let back = record_path(catalog, id, "edit");
    let qualifiers = match qualifier_names(catalog, page.context()).await {
        Ok(qualifiers) => qualifiers,
        Err(err) => {
            trace::error!("Catalog page failed: {err}");
            HashMap::new()
        }
    };
    let title = t("title-history").with_arg(
        "record",
        record_label(catalog, &record, &qualifiers, page.context()),
    );
    Ok(page
        .with_title(title)
        .with_child(table)
        .with_child(Button::back(back))
        .render()
        .await?
        .into_response())
}

// **< CSV >****************************************************************************************

/// GET {catálogo}/export.csv - Los registros que cumplen el filtro del listado, en su orden, en CSV.
pub(crate) async fn export_get(
    catalog: &'static Catalog,
    request: HttpRequest,
    query: ListQuery,
) -> Result<Response, ErrorPage> {
    let rows = service::export(catalog, &query.params(catalog))
        .await
        .map_err(|e| internal_error(&request, e))?;
    let separator = config::SETTINGS.csv_separator.as_char();
    // El nombre lleva la fecha y la hora de quien exporta (`colors-2026-11-28_13-05-42.csv`), para
    // distinguir dos exportaciones del mismo día y ordenarlas al listarlas.
    let now = Utc::now().with_timezone(&Context::admin(request.clone()).timezone());
    let disposition = format!(
        "attachment; filename=\"{}-{}.csv\"",
        catalog.name(),
        now.format("%Y-%m-%d_%H-%M-%S")
    );
    Ok((
        web::http::StatusCode::OK,
        [
            (
                web::http::header::CONTENT_TYPE,
                "text/csv; charset=utf-8".to_owned(),
            ),
            (web::http::header::CONTENT_DISPOSITION, disposition),
        ],
        crate::csv::encode(&rows, separator),
    )
        .into_response())
}

// Formulario para subir el CSV, con las columnas que admite.
async fn import_page(
    catalog: &'static Catalog,
    request: HttpRequest,
    errors: Vec<Lc>,
) -> Result<Response, ErrorPage> {
    let mut page = Page::admin(request);
    for error in errors {
        page.push_message(MessageLevel::Error, error);
    }
    let columns = service::csv_header(catalog).join(", ");
    // Las columnas que identifican cada registro y las obligatorias, como las exige la importación.
    let keys = catalog.key_columns();
    let mut required = keys.to_vec();
    if !required.contains(&NAME) {
        required.push(NAME);
    }
    let labels = |columns: &[&str]| {
        columns
            .iter()
            .map(|c| {
                column_label(catalog, c)
                    .lookup(page.context())
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>()
            .join(", ")
    };
    let help = t("help-import-file")
        .with_arg("columns", columns)
        .with_arg("required", labels(&required))
        .with_arg("keys", labels(keys))
        .with_lc("limits", import_limits());
    let form = Form::new()
        .with_action(path(catalog, "import"))
        .with_enctype(form::Enctype::Multipart)
        .with_child(
            form::File::new()
                .with_name("file")
                .with_label(t("field-import-file"))
                .with_help(help)
                .with_accept(".csv,text/csv")
                .with_required(true),
        )
        .with_child(
            Flex::buttons()
                .with_child(
                    Button::submit(t("btn-preview"))
                        .with_style(button::Style::Solid(Intent::Primary)),
                )
                .with_child(Button::cancel(path(catalog, ""))),
        );
    Ok(page
        .with_title(t("title-import").with_lc("catalog", catalog.title().clone()))
        .with_child(form)
        .render()
        .await?
        .into_response())
}

/// GET {catálogo}/import - Formulario de importación.
pub(crate) async fn import_get(
    catalog: &'static Catalog,
    request: HttpRequest,
) -> Result<Response, ErrorPage> {
    require_permission(&request, catalog.manage_permission())?;
    import_page(catalog, request, Vec::new()).await
}

// Los límites de un CSV importado: sus filas y el tamaño que admite, el menor entre el del
// catálogo y el del servidor para los envíos `multipart` (*0* es sin límite en los dos).
fn import_limits() -> Lc {
    let settings = &config::SETTINGS;
    let size = match (
        settings.import_max_size,
        global::SETTINGS.upload.max_request_size,
    ) {
        (0, 0) => None,
        (0, max) | (max, 0) => Some(max),
        (import, request) => Some(import.min(request)),
    };
    let rows = u32::try_from(settings.import_max_rows).unwrap_or(u32::MAX);
    match size {
        Some(size) => t("help-import-limits")
            .with_number("rows", rows)
            .with_arg("size", util::format_size(size as u64)),
        None => t("help-import-limits-rows").with_number("rows", rows),
    }
}

// El CSV de un envío `multipart` (el archivo subido o el campo de la confirmación), como texto.
enum CsvField {
    Text(String),
    // Vacío o ausente.
    Empty,
    NotUtf8,
    // Mayor que el límite, en bytes.
    TooLarge(usize),
}

// Lee el campo `name` de un envío `multipart`. El CSV se lee entero en memoria, así que admite
// como mucho `[catalog] import_max_size` bytes; los dos pasos de la importación lo envían como
// `multipart` para que el campo de la confirmación no crezca al codificarlo y quepa igual que el
// archivo subido. `Err` si el envío no se puede leer.
async fn read_csv(mut multipart: web::Multipart, name: &str) -> Result<CsvField, ()> {
    let max = config::SETTINGS.import_max_size;
    let mut bytes = Vec::new();
    while let Some(mut field) = multipart.next_field().await.map_err(|_| ())? {
        if field.name() != Some(name) {
            continue;
        }
        while let Some(chunk) = field.chunk().await.map_err(|_| ())? {
            if max > 0 && bytes.len() + chunk.len() > max {
                return Ok(CsvField::TooLarge(max));
            }
            bytes.extend_from_slice(&chunk);
        }
    }
    Ok(match String::from_utf8(bytes) {
        Ok(text) if text.trim().is_empty() => CsvField::Empty,
        Ok(text) => CsvField::Text(text),
        Err(_) => CsvField::NotUtf8,
    })
}

// El texto del CSV o, si no se puede importar, el formulario de importación con el motivo.
async fn csv_or_page(
    catalog: &'static Catalog,
    request: &HttpRequest,
    field: Result<CsvField, ()>,
) -> Result<Result<String, Response>, ErrorPage> {
    let error = match field {
        Ok(CsvField::Text(text)) => return Ok(Ok(text)),
        Ok(CsvField::Empty) => t("import-error-empty"),
        Ok(CsvField::NotUtf8) => t("import-error-encoding"),
        Ok(CsvField::TooLarge(max)) => {
            t("import-error-too-large").with_arg("max", util::format_size(max as u64))
        }
        Err(()) => return Err(ErrorPage::BadRequest(request.clone())),
    };
    Ok(Err(
        import_page(catalog, request.clone(), vec![error]).await?
    ))
}

/// POST {catálogo}/import - Previsualiza lo que haría el CSV subido, sin escribir nada.
pub(crate) async fn import_post(
    catalog: &'static Catalog,
    request: HttpRequest,
    multipart: web::Multipart,
) -> Result<Response, ErrorPage> {
    require_permission(&request, catalog.manage_permission())?;
    let text = match csv_or_page(catalog, &request, read_csv(multipart, "file").await).await? {
        Ok(text) => text,
        Err(page) => return Ok(page),
    };
    let plan = service::plan_import(catalog, &text)
        .await
        .map_err(|e| internal_error(&request, e))?;
    if !plan.errors.is_empty() {
        return import_page(catalog, request, plan.errors).await;
    }
    preview_page(catalog, request, &plan, text).await
}

// Lo que haría cada fila del CSV. Si todas son válidas y alguna cambia algo, el formulario para
// confirmarlo, con el propio CSV en un campo oculto: la importación no guarda nada en el servidor
// entre la previsualización y la confirmación.
async fn preview_page(
    catalog: &'static Catalog,
    request: HttpRequest,
    plan: &service::ImportPlan,
    text: String,
) -> Result<Response, ErrorPage> {
    use service::RowOutcome;

    let mut page = Page::admin(request);
    let created = plan.count(|o| matches!(o, RowOutcome::Create));
    let updated = plan.count(|o| matches!(o, RowOutcome::Update(_)));
    let unchanged = plan.count(|o| matches!(o, RowOutcome::Unchanged));
    let invalid = plan.count(|o| matches!(o, RowOutcome::Invalid(_)));
    let summary = t("import-summary")
        .with_number("created", created as u32)
        .with_number("updated", updated as u32)
        .with_number("unchanged", unchanged as u32)
        .with_number("invalid", invalid as u32);
    page.push_message(
        if invalid > 0 {
            MessageLevel::Error
        } else {
            MessageLevel::Info
        },
        summary,
    );

    let mut table = Table::new()
        .with_prop(PropsOp::add_classes("catalog-import"))
        .with_column(t("col-line"))
        .with_column(t("col-record"))
        .with_column(t("col-result"));
    for row in &plan.rows {
        let result: Vec<Lc> = match &row.outcome {
            RowOutcome::Create => vec![t("import-create")],
            RowOutcome::Unchanged => vec![t("import-unchanged")],
            RowOutcome::Update(columns) => vec![
                t("import-update").with_arg(
                    "columns",
                    columns
                        .iter()
                        .map(|c| {
                            column_label(catalog, c)
                                .lookup(page.context())
                                .unwrap_or_else(|| c.clone())
                        })
                        .collect::<Vec<_>>()
                        .join(", "),
                ),
            ],
            RowOutcome::Invalid(errors) => errors.clone(),
        };
        let cell = Html::with(move |cx| {
            html! {
                @for line in &result {
                    div { (line.using(cx)) }
                }
            }
        });
        table = table.with_row(
            table::Row::new()
                .with_cell(row.line.to_string())
                .with_cell(row.code.clone())
                .with_cell(table::Cell::new(cell)),
        );
    }

    let importable = plan.is_valid() && created + updated > 0;
    let mut buttons = Flex::buttons();
    if importable {
        buttons = buttons.with_child(
            Button::submit(t("btn-import")).with_style(button::Style::Solid(Intent::Primary)),
        );
    }
    buttons = buttons
        .with_child(Button::anchor(t("btn-other-file"), path(catalog, "import")))
        .with_child(Button::cancel(path(catalog, "")));
    let page = page
        .with_title(t("title-import").with_lc("catalog", catalog.title().clone()))
        .with_child(table);
    let mut page = if importable {
        page.with_child(
            Form::new()
                .with_action(path(catalog, "import/confirm"))
                .with_enctype(form::Enctype::Multipart)
                .with_child(form::Hidden::field("csv", text))
                .with_child(buttons),
        )
    } else {
        page.with_child(buttons)
    };
    Ok(page.render().await?.into_response())
}

/// POST {catálogo}/import/confirm - Importa el CSV previsualizado, todo o nada.
pub(crate) async fn import_confirm_post(
    catalog: &'static Catalog,
    request: HttpRequest,
    multipart: web::Multipart,
) -> Result<Response, ErrorPage> {
    require_permission(&request, catalog.manage_permission())?;
    let text = match csv_or_page(catalog, &request, read_csv(multipart, "csv").await).await? {
        Ok(text) => text,
        Err(page) => return Ok(page),
    };
    match service::import(catalog, &text, user_id(&request)).await {
        Ok((created, updated)) => {
            let cx = Context::admin(request);
            Ok(Redirect::see_other(cx.route(path(catalog, "")))
                .into_response()
                .with_message(
                    &cx,
                    MessageLevel::Info,
                    t("done-imported")
                        .with_number("created", created as u32)
                        .with_number("updated", updated as u32),
                ))
        }
        Err(CatalogError::Db(err)) => Err(internal_error(&request, err)),
        Err(err) => import_page(catalog, request, err.messages()).await,
    }
}
