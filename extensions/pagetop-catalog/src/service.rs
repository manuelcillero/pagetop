// Acceso a las tablas de los catálogos, construido con SeaQuery a partir de su definición, para que
// valga igual en cualquier motor.

use pagetop::prelude::*;
use pagetop_seaorm::db::query::{
    Alias, Expr, Func, LikeExpr, Order, Query, SelectStatement, SimpleExpr,
};
use pagetop_seaorm::db::{
    ConnectionTrait, DatabaseTransaction, DbErr, DbErrExt, QueryResult, TransactionTrait, dbconn,
};

use sea_orm::DatabaseBackend;
use serde_json::Value as Json;

use std::collections::{BTreeMap, HashMap};

use crate::catalog::{Catalog, FieldKind};
use crate::declare::{REGISTRY, catalog as find_catalog, guard_reasons};
use crate::record::{Change, ChangeAction, ChangeSet, FieldChange, FieldValue, Record};
use crate::schema::{
    CODE, CREATED_AT, ID, IS_ACTIVE, NAME, NAME_I18N, NAME_MAX_LEN, ROW_VERSION, UPDATED_AT,
    UPDATED_BY, WEIGHT,
};
use crate::{LOCALES_CATALOG, config};

// **< CatalogError >*******************************************************************************

#[derive(Debug, thiserror::Error)]
pub(crate) enum CatalogError {
    #[error("database error: {0}")]
    Db(#[from] DbErr),
    #[error("record not found")]
    NotFound,
    #[error("invalid input")]
    Invalid(Vec<Lc>),
    #[error("another record already has the same values")]
    Duplicate(Vec<Lc>),
    #[error("record changed by someone else since it was read")]
    Conflict,
    #[error("record in use")]
    InUse(Vec<Lc>),
}

impl CatalogError {
    // Mensajes para el usuario.
    pub(crate) fn messages(&self) -> Vec<Lc> {
        let t = |key| vec![Lc::t(key, &LOCALES_CATALOG)];
        match self {
            Self::Db(_) => t("error-internal"),
            Self::NotFound => t("error-not-found"),
            Self::Invalid(messages) | Self::InUse(messages) | Self::Duplicate(messages) => {
                messages.clone()
            }
            Self::Conflict => t("error-conflict"),
        }
    }

    // Una escritura rechazada por una restricción única (dos altas simultáneas con el mismo código,
    // o un cambio que repite valores únicos en otra tabla) es un registro repetido, no un fallo del
    // sistema. La comprobación previa cubre el caso habitual, con un mensaje más preciso.
    fn from_write(err: DbErr, catalog: &Catalog) -> Self {
        if err.is_unique_violation() {
            let message = if catalog.constraint_message().is_none() {
                Lc::t("error-duplicate-any", &LOCALES_CATALOG)
            } else {
                catalog.constraint_message().clone()
            };
            Self::Duplicate(vec![message])
        } else {
            Self::Db(err)
        }
    }

    // Un borrado rechazado por una clave foránea es un registro en uso, también si quien lo usa
    // es una extensión que el catálogo no conoce.
    fn from_delete(err: DbErr) -> Self {
        if err.is_foreign_key_violation() {
            Self::InUse(vec![Lc::t("error-in-use", &LOCALES_CATALOG)])
        } else {
            Self::Db(err)
        }
    }
}

// **< Listado >************************************************************************************

#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StatusFilter {
    #[default]
    Active,
    Inactive,
    All,
}

impl StatusFilter {
    pub(crate) fn from_query(value: Option<&str>) -> Self {
        match value {
            Some("inactive") => Self::Inactive,
            Some("all") => Self::All,
            _ => Self::Active,
        }
    }

    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::All => "all",
        }
    }
}

#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SortField {
    #[default]
    Weight,
    Code,
    Name,
}

impl SortField {
    pub(crate) fn from_query(value: Option<&str>) -> Self {
        match value {
            Some("code") => Self::Code,
            Some("name") => Self::Name,
            _ => Self::Weight,
        }
    }

    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Self::Weight => "weight",
            Self::Code => "code",
            Self::Name => "name",
        }
    }
}

#[derive(AutoDefault, Clone, Debug)]
pub(crate) struct ListParams {
    pub query: String,
    pub status: StatusFilter,
    // Filtros por campo (`with_filter()`): columna y valor.
    pub filters: Vec<(&'static str, String)>,
    pub sort: SortField,
    pub dir: SortDir,
    pub page: u64,
}

pub(crate) struct ListPage {
    pub items: Vec<Record>,
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
}

// Patrón `LIKE` de una búsqueda, en minúsculas: `*` es el comodín; sin ninguno, busca el texto en
// cualquier posición. `%`, `_` y `\` del texto se escapan con `\`.
pub(crate) fn search_pattern(query: &str) -> Option<String> {
    let query = query.trim();
    if query.is_empty() {
        return None;
    }
    let mut pattern = String::with_capacity(query.len() + 2);
    let wildcard = query.contains('*');
    if !wildcard {
        pattern.push('%');
    }
    for c in query.chars().flat_map(char::to_lowercase) {
        match c {
            // Ningún texto guardado los tiene, y algunos motores no los admiten en una consulta.
            c if c.is_control() => {}
            '*' => pattern.push('%'),
            '%' | '_' | '\\' => {
                pattern.push('\\');
                pattern.push(c);
            }
            c => pattern.push(c),
        }
    }
    if !wildcard {
        pattern.push('%');
    }
    Some(pattern)
}

fn col(name: &str) -> Alias {
    Alias::new(name)
}

// Filtro del listado: estado y búsqueda por código o nombre, sin distinguir mayúsculas.
fn filtered(catalog: &Catalog, params: &ListParams, stmt: &mut SelectStatement) {
    stmt.from(col(catalog.table()));
    match params.status {
        StatusFilter::Active => {
            stmt.and_where(Expr::col(col(IS_ACTIVE)).eq(true));
        }
        StatusFilter::Inactive => {
            stmt.and_where(Expr::col(col(IS_ACTIVE)).eq(false));
        }
        StatusFilter::All => {}
    }
    if let Some(pattern) = search_pattern(&params.query) {
        let like = |c: &str| {
            Expr::expr(Func::lower(Expr::col(col(c))))
                .like(LikeExpr::new(pattern.clone()).escape('\\'))
        };
        if catalog.has_code() {
            stmt.cond_where(like(CODE).or(like(NAME)));
        } else {
            stmt.cond_where(like(NAME));
        }
    }
    for (column, value) in &params.filters {
        let Some(field) = catalog.field(column) else {
            continue;
        };
        let condition = match field.kind() {
            FieldKind::Reference(_) => value
                .parse::<i32>()
                .ok()
                .map(|v| Expr::col(col(column)).eq(v)),
            FieldKind::Choice(_) => Some(Expr::col(col(column)).eq(value.clone())),
            FieldKind::Boolean => parse_bool(value).map(|v| Expr::col(col(column)).eq(v)),
            _ => None,
        };
        // Un valor no válido no filtra nada, para no mostrar un listado vacío sin motivo aparente.
        if let Some(condition) = condition {
            stmt.and_where(condition);
        }
    }
}

pub(crate) async fn list(catalog: &Catalog, params: &ListParams) -> Result<ListPage, DbErr> {
    let db = dbconn();
    let backend = db.get_database_backend();
    let per_page = config::SETTINGS.list_page_size.max(1);

    let mut count = Query::select();
    count.expr_as(Expr::col(col(ID)).count(), col("total"));
    filtered(catalog, params, &mut count);
    let total = match db.query_one(backend.build(&count)).await? {
        Some(row) => row.try_get::<i64>("", "total")?.max(0) as u64,
        None => 0,
    };
    let last_page = total.div_ceil(per_page).max(1);
    let page = params.page.clamp(1, last_page);

    let mut select = select_record(catalog);
    filtered(catalog, params, &mut select);
    ordered(catalog, params, &mut select);
    select.limit(per_page).offset((page - 1) * per_page);
    let rows = db.query_all(backend.build(&select)).await?;
    let items = rows
        .iter()
        .map(|row| read_record(catalog, row))
        .collect::<Result<_, _>>()?;
    Ok(ListPage {
        items,
        page,
        per_page,
        total,
    })
}

// Orden del listado elegido. El `id` desempata los registros con el mismo peso y nombre (posibles
// sin código), para que la paginación no repita ni salte ninguno.
fn ordered(catalog: &Catalog, params: &ListParams, stmt: &mut SelectStatement) {
    let order = match params.dir {
        SortDir::Asc => Order::Asc,
        SortDir::Desc => Order::Desc,
    };
    match params.sort {
        SortField::Weight => {
            stmt.order_by(col(WEIGHT), order.clone())
                .order_by(col(NAME), Order::Asc)
                .order_by(col(ID), Order::Asc);
        }
        SortField::Code if catalog.has_code() => {
            stmt.order_by(col(CODE), order);
        }
        SortField::Code | SortField::Name => {
            stmt.order_by(col(NAME), order)
                .order_by(col(ID), Order::Asc);
        }
    }
}

// Todos los registros del catálogo, activos o no, para mostrar los nombres a los que se refieren
// otros catálogos.
pub(crate) async fn all(catalog: &Catalog) -> Result<Vec<Record>, DbErr> {
    let db = dbconn();
    let mut select = select_record(catalog);
    select.from(col(catalog.table()));
    let rows = db
        .query_all(db.get_database_backend().build(&select))
        .await?;
    rows.iter().map(|row| read_record(catalog, row)).collect()
}

// **< Lectura >************************************************************************************

fn select_record(catalog: &Catalog) -> SelectStatement {
    let mut stmt = Query::select();
    stmt.columns([ID, NAME, WEIGHT, IS_ACTIVE, ROW_VERSION].map(col));
    if catalog.has_code() {
        stmt.column(col(CODE));
    }
    if catalog.translations() {
        stmt.column(col(NAME_I18N));
    }
    for field in catalog.fields() {
        stmt.column(col(field.column()));
    }
    stmt
}

fn read_record(catalog: &Catalog, row: &QueryResult) -> Result<Record, DbErr> {
    let translations = if catalog.translations() {
        match row.try_get::<Option<Json>>("", NAME_I18N)? {
            Some(Json::Object(map)) => map
                .into_iter()
                .filter_map(|(k, v)| v.as_str().map(|v| (k, v.to_owned())))
                .collect(),
            _ => BTreeMap::new(),
        }
    } else {
        BTreeMap::new()
    };
    let mut values = Vec::with_capacity(catalog.fields().len());
    for field in catalog.fields() {
        let column = field.column();
        values.push(match field.kind() {
            FieldKind::Text | FieldKind::TextArea | FieldKind::Choice(_) => {
                FieldValue::Text(row.try_get::<Option<String>>("", column)?)
            }
            FieldKind::Boolean => {
                FieldValue::Bool(row.try_get::<Option<bool>>("", column)?.unwrap_or(false))
            }
            FieldKind::Integer { .. } | FieldKind::Reference(_) => {
                FieldValue::Int(row.try_get::<Option<i32>>("", column)?)
            }
            FieldKind::Decimal { .. } => {
                FieldValue::Decimal(row.try_get::<Option<f64>>("", column)?)
            }
        });
    }
    Ok(Record::new(
        row.try_get("", ID)?,
        if catalog.has_code() {
            row.try_get("", CODE)?
        } else {
            String::new()
        },
        row.try_get("", NAME)?,
        translations,
        row.try_get("", WEIGHT)?,
        row.try_get("", IS_ACTIVE)?,
        row.try_get("", ROW_VERSION)?,
        values,
    ))
}

pub(crate) async fn find(catalog: &Catalog, id: i32) -> Result<Record, CatalogError> {
    find_in(dbconn(), catalog, id).await
}

async fn find_in(
    db: &impl ConnectionTrait,
    catalog: &Catalog,
    id: i32,
) -> Result<Record, CatalogError> {
    let mut select = select_record(catalog);
    select
        .from(col(catalog.table()))
        .and_where(Expr::col(col(ID)).eq(id));
    match db
        .query_one(db.get_database_backend().build(&select))
        .await?
    {
        Some(row) => Ok(read_record(catalog, &row)?),
        None => Err(CatalogError::NotFound),
    }
}

// **< Entrada del formulario >*********************************************************************

// Nombre del campo del formulario con la traducción del nombre al idioma `langid`.
pub(crate) fn translation_input(langid: &str) -> String {
    util::join!("i18n-", langid)
}

// Idiomas a los que se puede traducir el nombre: los de la aplicación salvo el predeterminado, en
// el que se escribe el propio nombre.
pub(crate) fn translation_languages() -> Vec<(String, Lc)> {
    let default = Locale::default().langid().to_string();
    Locale::supported_languages()
        .into_iter()
        .map(|(langid, label)| (langid.to_string(), label))
        .filter(|(langid, _)| *langid != default)
        .collect()
}

// Valores de un registro, tal como llegan del formulario y ya comprobados uno a uno.
#[derive(AutoDefault, Clone, Debug)]
pub(crate) struct Input {
    pub code: String,
    pub name: String,
    pub weight: i32,
    pub translations: BTreeMap<String, String>,
    pub values: Vec<FieldValue>,
}

impl Input {
    // Los valores de un registro, para rellenar el formulario de edición.
    pub(crate) fn from_record(record: &Record) -> Self {
        Input {
            code: record.code().to_owned(),
            name: record.name().to_owned(),
            weight: record.weight(),
            translations: record.translations().clone(),
            values: record.values().to_vec(),
        }
    }

    // Valores vacíos, para el formulario de alta.
    pub(crate) fn empty(catalog: &Catalog) -> Self {
        Input {
            values: catalog
                .fields()
                .iter()
                .map(|f| FieldValue::empty(f.kind()))
                .collect(),
            ..Default::default()
        }
    }

    // Lee y comprueba los valores enviados. Devuelve siempre los valores leídos (para volver a
    // mostrar el formulario) y los mensajes de los que no son válidos. Al editar (`creating` a
    // `false`) el código no se comprueba, porque no cambia: se conserva el guardado.
    pub(crate) fn parse(
        catalog: &Catalog,
        form: &HashMap<String, String>,
        creating: bool,
    ) -> (Self, Vec<Lc>) {
        let get = |key: &str| form.get(key).map(|v| v.trim()).unwrap_or("");
        let mut errors = Vec::new();
        // Caracteres de control (salvo los saltos de línea y los tabuladores de un texto largo):
        // no los escribe un usuario y PostgreSQL rechaza el carácter nulo.
        let bad = |value: &str, multiline: bool| {
            value
                .chars()
                .any(|c| c.is_control() && !(multiline && matches!(c, '\n' | '\r' | '\t')))
        };
        let invalid = |key: &'static str, label: &Lc| {
            Lc::t(key, &LOCALES_CATALOG).with_lc("field", label.clone())
        };

        let code = get(CODE).to_owned();
        let code_label = Lc::t("field-code", &LOCALES_CATALOG);
        if creating && catalog.has_code() {
            if code.is_empty() {
                errors.push(invalid("error-required", &code_label));
            } else if code.chars().count() > catalog.code_max_len() {
                errors.push(
                    invalid("error-too-long", &code_label)
                        .with_number("max", catalog.code_max_len() as u32),
                );
            } else if code.chars().any(char::is_whitespace) {
                errors.push(Lc::t("error-code-spaces", &LOCALES_CATALOG));
            }
        }

        if catalog.has_code() && bad(&code, false) {
            errors.push(invalid("error-invalid-characters", &code_label));
        }

        let name = get(NAME).to_owned();
        let name_label = Lc::t("field-name", &LOCALES_CATALOG);
        if bad(&name, false) {
            errors.push(invalid("error-invalid-characters", &name_label));
        }
        if name.is_empty() {
            errors.push(invalid("error-required", &name_label));
        } else if name.chars().count() > NAME_MAX_LEN {
            errors.push(
                invalid("error-too-long", &name_label).with_number("max", NAME_MAX_LEN as u32),
            );
        }

        let weight = match get(WEIGHT) {
            "" => 0,
            value => value.parse().unwrap_or_else(|_| {
                errors.push(invalid(
                    "error-not-a-number",
                    &Lc::t("field-weight", &LOCALES_CATALOG),
                ));
                0
            }),
        };

        let mut translations = BTreeMap::new();
        if catalog.translations() {
            for (langid, _) in translation_languages() {
                let text = get(&translation_input(&langid));
                if bad(text, false) {
                    errors.push(invalid("error-invalid-characters", &name_label));
                } else if text.chars().count() > NAME_MAX_LEN {
                    errors.push(
                        invalid("error-too-long", &name_label)
                            .with_number("max", NAME_MAX_LEN as u32),
                    );
                } else if !text.is_empty() {
                    translations.insert(langid, text.to_owned());
                }
            }
        }

        let mut values = Vec::with_capacity(catalog.fields().len());
        for field in catalog.fields() {
            let raw = get(field.column());
            let label = field.label();
            if bad(raw, matches!(field.kind(), FieldKind::TextArea)) {
                errors.push(invalid("error-invalid-characters", label));
                values.push(FieldValue::empty(field.kind()));
                continue;
            }
            let value = match field.kind() {
                FieldKind::Text | FieldKind::TextArea => {
                    if let Some(max) = field.max_len()
                        && raw.chars().count() > max
                    {
                        errors
                            .push(invalid("error-too-long", label).with_number("max", max as u32));
                    }
                    FieldValue::Text((!raw.is_empty()).then(|| raw.to_owned()))
                }
                FieldKind::Boolean => FieldValue::Bool(matches!(raw, "true" | "on" | "1")),
                FieldKind::Integer { min, max } => match raw {
                    "" => FieldValue::Int(None),
                    raw => match raw.parse::<i32>() {
                        Ok(n)
                            if min.is_some_and(|min| n < min) || max.is_some_and(|max| n > max) =>
                        {
                            errors.push(invalid("error-out-of-range", label));
                            FieldValue::Int(Some(n))
                        }
                        Ok(n) => FieldValue::Int(Some(n)),
                        Err(_) => {
                            errors.push(invalid("error-not-a-number", label));
                            FieldValue::Int(None)
                        }
                    },
                },
                FieldKind::Decimal { min, max } => match raw {
                    "" => FieldValue::Decimal(None),
                    raw => match raw.replace(',', ".").parse::<f64>() {
                        Ok(n) if !n.is_finite() => {
                            errors.push(invalid("error-not-a-decimal", label));
                            FieldValue::Decimal(None)
                        }
                        Ok(n)
                            if min.is_some_and(|min| n < min) || max.is_some_and(|max| n > max) =>
                        {
                            errors.push(invalid("error-out-of-range", label));
                            FieldValue::Decimal(Some(n))
                        }
                        Ok(n) => FieldValue::Decimal(Some(n)),
                        Err(_) => {
                            errors.push(invalid("error-not-a-decimal", label));
                            FieldValue::Decimal(None)
                        }
                    },
                },
                FieldKind::Choice(options) => {
                    if !raw.is_empty() && !options.iter().any(|(value, _)| *value == raw) {
                        errors.push(invalid("error-invalid-option", label));
                    }
                    FieldValue::Text((!raw.is_empty()).then(|| raw.to_owned()))
                }
                FieldKind::Reference(_) => match raw {
                    "" => FieldValue::Int(None),
                    raw => match raw.parse::<i32>() {
                        Ok(n) => FieldValue::Int(Some(n)),
                        Err(_) => {
                            errors.push(invalid("error-invalid-option", label));
                            FieldValue::Int(None)
                        }
                    },
                },
            };
            if field.required() && value.is_empty() {
                errors.push(invalid("error-required", label));
            }
            values.push(value);
        }

        (
            Input {
                code,
                name,
                weight,
                translations,
                values,
            },
            errors,
        )
    }

    fn to_record(&self, id: i32, is_active: bool, row_version: i32) -> Record {
        Record::new(
            id,
            self.code.clone(),
            self.name.clone(),
            self.translations.clone(),
            self.weight,
            is_active,
            row_version,
            self.values.clone(),
        )
    }

    fn translations_json(&self) -> Option<Json> {
        (!self.translations.is_empty()).then(|| {
            Json::Object(
                self.translations
                    .iter()
                    .map(|(k, v)| (k.clone(), Json::String(v.clone())))
                    .collect(),
            )
        })
    }
}

// Comprueba lo que depende de la base de datos (las referencias a otros catálogos, que deben
// existir y estar activas salvo que no cambien) y la validación propia del catálogo.
async fn validate(
    db: &impl ConnectionTrait,
    catalog: &Catalog,
    input: &Input,
    previous: Option<&Record>,
) -> Result<(), CatalogError> {
    let mut errors = Vec::new();
    for (index, field) in catalog.fields().iter().enumerate() {
        let FieldKind::Reference(target) = field.kind() else {
            continue;
        };
        let Some(id) = input.values[index].as_int() else {
            continue;
        };
        let unchanged = previous.is_some_and(|p| p.values()[index].as_int() == Some(id));
        let Some(target) = find_catalog(target) else {
            continue;
        };
        let valid = match find_in(db, target, id).await {
            Ok(record) => record.is_active() || unchanged,
            Err(CatalogError::NotFound) => false,
            Err(err) => return Err(err),
        };
        if !valid {
            errors.push(
                Lc::t("error-invalid-option", &LOCALES_CATALOG)
                    .with_lc("field", field.label().clone()),
            );
        }
    }
    if errors.is_empty() {
        check_unique(db, catalog, input, previous).await?;
    }
    if errors.is_empty()
        && let Some(validator) = catalog.validator()
    {
        let (id, is_active, version) =
            previous.map_or((0, true, 1), |p| (p.id(), p.is_active(), p.row_version()));
        if let Err(message) = validator(&input.to_record(id, is_active, version)) {
            errors.push(message);
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(CatalogError::Invalid(errors))
    }
}

// Comprueba que el registro no repite el código (al darlo de alta) ni ninguno de los conjuntos de
// columnas únicas del catálogo.
async fn check_unique(
    db: &impl ConnectionTrait,
    catalog: &Catalog,
    input: &Input,
    previous: Option<&Record>,
) -> Result<(), CatalogError> {
    // El código, al dar de alta (después no cambia), y los conjuntos de columnas únicas.
    let code: &[&[&str]] = if catalog.has_code() && previous.is_none() {
        &[&[CODE]]
    } else {
        &[]
    };
    for columns in code.iter().chain(catalog.unique().iter()) {
        let mut select = Query::select();
        select
            .expr_as(Expr::col(col(ID)).count(), col("total"))
            .from(col(catalog.table()));
        for column in columns.iter() {
            select.and_where(Expr::col(col(column)).eq(input_value(catalog, input, column)));
        }
        if let Some(previous) = previous {
            select.and_where(Expr::col(col(ID)).ne(previous.id()));
        }
        let taken = match db
            .query_one(db.get_database_backend().build(&select))
            .await?
        {
            Some(row) => row.try_get::<i64>("", "total")? > 0,
            None => false,
        };
        if taken {
            return Err(CatalogError::Duplicate(vec![duplicate_message(
                catalog, columns,
            )]));
        }
    }
    Ok(())
}

// Valor de una columna (común o propia) en la entrada, para compararlo en la base de datos.
fn input_value(catalog: &Catalog, input: &Input, column: &str) -> SimpleExpr {
    match column {
        CODE => input.code.clone().into(),
        NAME => input.name.clone().into(),
        WEIGHT => input.weight.into(),
        column => catalog
            .fields()
            .iter()
            .position(|f| f.column() == column)
            .map_or(
                SimpleExpr::Keyword(pagetop_seaorm::db::query::Keyword::Null),
                |i| sql_value(&input.values[i]),
            ),
    }
}

// Etiqueta de una columna (común o propia), para los mensajes.
pub(crate) fn column_label(catalog: &Catalog, column: &str) -> Lc {
    let t = |key| Lc::t(key, &LOCALES_CATALOG);
    match column {
        CODE => t("field-code"),
        NAME => t("field-name"),
        WEIGHT => t("field-weight"),
        IS_ACTIVE => t("field-active"),
        NAME_I18N => t("field-translations"),
        column => catalog
            .field(column)
            .map_or_else(|| Lc::n(column.to_owned()), |f| f.label().clone()),
    }
}

// "Ya hay otro registro con el mismo código", o "con la misma combinación de sede y nombre".
fn duplicate_message(catalog: &Catalog, columns: &[&str]) -> Lc {
    let mut message = Lc::t(
        if columns.len() == 1 {
            "error-duplicate"
        } else {
            "error-duplicate-combination"
        },
        &LOCALES_CATALOG,
    );
    for (i, column) in columns.iter().take(3).enumerate() {
        message = message.with_lc(
            util::join!("field", (i + 1).to_string()),
            column_label(catalog, column),
        );
    }
    message.with_number("count", columns.len().min(3) as u32)
}

// Lo que identifica un registro en el historial y en la auditoría: su código o, sin código, su
// nombre.
fn record_key(catalog: &Catalog, code: &str, name: &str) -> String {
    if catalog.has_code() {
        code.to_owned()
    } else {
        name.to_owned()
    }
}

// **< Escritura >**********************************************************************************

fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

// Valor de una columna propia para la sentencia.
fn sql_value(value: &FieldValue) -> SimpleExpr {
    match value {
        FieldValue::Text(text) => text.clone().into(),
        FieldValue::Bool(b) => (*b).into(),
        FieldValue::Int(n) => (*n).into(),
        FieldValue::Decimal(n) => (*n).into(),
    }
}

// Columnas que cambian entre dos versiones de un registro (de `None`, un alta).
fn diff(catalog: &Catalog, old: Option<&Input>, new: &Input) -> Vec<FieldChange> {
    let mut changes = Vec::new();
    let mut push = |column: &str, old: Json, new: Json| {
        if old != new {
            changes.push(FieldChange {
                column: column.to_owned(),
                old,
                new,
            });
        }
    };
    if old.is_none() && catalog.has_code() {
        push(CODE, Json::Null, Json::String(new.code.clone()));
    }
    push(
        NAME,
        old.map_or(Json::Null, |o| Json::String(o.name.clone())),
        Json::String(new.name.clone()),
    );
    push(
        WEIGHT,
        old.map_or(Json::Null, |o| Json::from(o.weight)),
        Json::from(new.weight),
    );
    if catalog.translations() {
        push(
            NAME_I18N,
            old.and_then(Input::translations_json).unwrap_or(Json::Null),
            new.translations_json().unwrap_or(Json::Null),
        );
    }
    for (index, field) in catalog.fields().iter().enumerate() {
        push(
            field.column(),
            old.map_or(Json::Null, |o| o.values[index].to_json()),
            new.values[index].to_json(),
        );
    }
    changes
}

// Ante una restricción única que salta al escribir (otra petición se adelantó entre la
// comprobación y la escritura), vuelve a comprobar, ya fuera de la transacción fallida, para dar el
// mismo mensaje que habría dado la comprobación previa. Si no es una de las del catálogo, devuelve
// el error original.
async fn precise_duplicate(
    catalog: &Catalog,
    input: &Input,
    previous: Option<&Record>,
    err: CatalogError,
) -> CatalogError {
    match check_unique(dbconn(), catalog, input, previous).await {
        Err(precise @ CatalogError::Duplicate(_)) => precise,
        _ => err,
    }
}

// Entrega los cambios de una operación a los registradores, en su transacción.
async fn record(
    txn: &DatabaseTransaction,
    catalog: &Catalog,
    user_id: Option<i32>,
    changes: Vec<Change>,
) -> Result<(), DbErr> {
    if changes.is_empty() {
        return Ok(());
    }
    let set = ChangeSet {
        catalog,
        user_id,
        changes,
    };
    for recorder in &REGISTRY.recorders {
        recorder.record(txn, &set).await?;
    }
    Ok(())
}

// Inserta y devuelve el `id` nuevo: con `RETURNING` en PostgreSQL y SQLite, con el último `id`
// insertado en MySQL.
async fn insert_id(
    txn: &DatabaseTransaction,
    mut stmt: pagetop_seaorm::db::query::InsertStatement,
) -> Result<i32, DbErr> {
    let backend = txn.get_database_backend();
    if backend == DatabaseBackend::MySql {
        let result = txn.execute(backend.build(&stmt)).await?;
        return Ok(result.last_insert_id() as i32);
    }
    stmt.returning_col(col(ID));
    match txn.query_one(backend.build(&stmt)).await? {
        Some(row) => row.try_get("", ID),
        None => Err(DbErr::RecordNotInserted),
    }
}

// Inserta un registro ya validado y devuelve su cambio.
async fn insert_in(
    txn: &DatabaseTransaction,
    catalog: &Catalog,
    input: &Input,
    active: bool,
    user_id: Option<i32>,
) -> Result<Change, CatalogError> {
    let now = now();
    let mut columns = vec![NAME, WEIGHT, IS_ACTIVE, CREATED_AT, UPDATED_AT, UPDATED_BY];
    let mut values: Vec<SimpleExpr> = vec![
        input.name.clone().into(),
        input.weight.into(),
        active.into(),
        now.into(),
        now.into(),
        user_id.into(),
    ];
    if catalog.has_code() {
        columns.push(CODE);
        values.push(input.code.clone().into());
    }
    if catalog.translations() {
        columns.push(NAME_I18N);
        values.push(input.translations_json().into());
    }
    for (field, value) in catalog.fields().iter().zip(&input.values) {
        columns.push(field.column());
        values.push(sql_value(value));
    }
    let mut stmt = Query::insert();
    stmt.into_table(col(catalog.table()))
        .columns(columns.into_iter().map(col))
        .values_panic(values);
    let id = insert_id(txn, stmt)
        .await
        .map_err(|e| CatalogError::from_write(e, catalog))?;
    let mut fields = diff(catalog, None, input);
    if !active {
        fields.push(FieldChange {
            column: IS_ACTIVE.to_owned(),
            old: Json::Null,
            new: Json::Bool(false),
        });
    }
    Ok(Change {
        record_id: id,
        code: record_key(catalog, &input.code, &input.name),
        action: ChangeAction::Create,
        fields,
    })
}

// Modifica un registro ya validado si sigue en la versión de `current`, y devuelve su cambio, o
// `None` si no cambia nada. Con `active`, cambia también si está activo.
async fn update_in(
    txn: &DatabaseTransaction,
    catalog: &Catalog,
    current: &Record,
    input: &Input,
    active: Option<bool>,
    user_id: Option<i32>,
) -> Result<Option<Change>, CatalogError> {
    let mut fields = diff(catalog, Some(&Input::from_record(current)), input);
    let active = active.filter(|a| *a != current.is_active());
    if let Some(active) = active {
        fields.push(FieldChange {
            column: IS_ACTIVE.to_owned(),
            old: Json::Bool(!active),
            new: Json::Bool(active),
        });
    }
    if fields.is_empty() {
        return Ok(None);
    }

    let mut stmt = Query::update();
    stmt.table(col(catalog.table()))
        .value(col(NAME), input.name.clone())
        .value(col(WEIGHT), input.weight)
        .value(col(UPDATED_AT), now())
        .value(col(UPDATED_BY), user_id)
        .value(col(ROW_VERSION), Expr::col(col(ROW_VERSION)).add(1));
    if let Some(active) = active {
        stmt.value(col(IS_ACTIVE), active);
    }
    if catalog.translations() {
        stmt.value(col(NAME_I18N), input.translations_json());
    }
    for (field, value) in catalog.fields().iter().zip(&input.values) {
        stmt.value(col(field.column()), sql_value(value));
    }
    stmt.and_where(Expr::col(col(ID)).eq(current.id()))
        .and_where(Expr::col(col(ROW_VERSION)).eq(current.row_version()));
    let backend = txn.get_database_backend();
    let result = txn
        .execute(backend.build(&stmt))
        .await
        .map_err(|e| CatalogError::from_write(e, catalog))?;
    if result.rows_affected() == 0 {
        return Err(CatalogError::Conflict);
    }
    let action = match (active, fields.len()) {
        (Some(true), 1) => ChangeAction::Reactivate,
        (Some(false), 1) => ChangeAction::Deactivate,
        _ => ChangeAction::Update,
    };
    Ok(Some(Change {
        record_id: current.id(),
        code: record_key(catalog, current.code(), current.name()),
        action,
        fields,
    }))
}

pub(crate) async fn create(
    catalog: &Catalog,
    input: &Input,
    user_id: Option<i32>,
) -> Result<i32, CatalogError> {
    let txn = dbconn().begin().await?;
    validate(&txn, catalog, input, None).await?;
    let change = match insert_in(&txn, catalog, input, true, user_id).await {
        Err(err @ CatalogError::Duplicate(_)) => {
            drop(txn);
            return Err(precise_duplicate(catalog, input, None, err).await);
        }
        result => result?,
    };
    let id = change.record_id;
    record(&txn, catalog, user_id, vec![change]).await?;
    txn.commit().await?;
    Ok(id)
}

// Modifica el registro `id` si sigue en la versión `row_version` con la que se leyó; si otro lo ha
// cambiado entretanto, no lo pisa y devuelve `CatalogError::Conflict`. El código no cambia.
pub(crate) async fn update(
    catalog: &Catalog,
    id: i32,
    row_version: i32,
    input: &Input,
    user_id: Option<i32>,
) -> Result<(), CatalogError> {
    let txn = dbconn().begin().await?;
    let current = find_in(&txn, catalog, id).await?;
    if current.row_version() != row_version {
        return Err(CatalogError::Conflict);
    }
    let mut input = input.clone();
    input.code = current.code().to_owned();
    validate(&txn, catalog, &input, Some(&current)).await?;
    let change = match update_in(&txn, catalog, &current, &input, None, user_id).await {
        Err(err @ CatalogError::Duplicate(_)) => {
            drop(txn);
            return Err(precise_duplicate(catalog, &input, Some(&current), err).await);
        }
        result => result?,
    };
    if let Some(change) = change {
        record(&txn, catalog, user_id, vec![change]).await?;
        txn.commit().await?;
    }
    Ok(())
}

// Desactiva o reactiva el registro `id`. Desactivarlo exige que ninguna salvaguarda lo impida.
pub(crate) async fn set_active(
    catalog: &Catalog,
    id: i32,
    active: bool,
    user_id: Option<i32>,
) -> Result<(), CatalogError> {
    if !active {
        let reasons = guard_reasons(catalog, id).await?;
        if !reasons.is_empty() {
            return Err(CatalogError::InUse(reasons));
        }
    }
    let txn = dbconn().begin().await?;
    let current = find_in(&txn, catalog, id).await?;
    if current.is_active() == active {
        return Ok(());
    }
    let mut stmt = Query::update();
    stmt.table(col(catalog.table()))
        .value(col(IS_ACTIVE), active)
        .value(col(UPDATED_AT), now())
        .value(col(UPDATED_BY), user_id)
        .value(col(ROW_VERSION), Expr::col(col(ROW_VERSION)).add(1))
        .and_where(Expr::col(col(ID)).eq(id))
        .and_where(Expr::col(col(IS_ACTIVE)).eq(!active));
    let backend = txn.get_database_backend();
    // Si otra petición lo cambió entretanto, ya está como se pedía: no hay nada que registrar.
    if txn.execute(backend.build(&stmt)).await?.rows_affected() == 0 {
        return Ok(());
    }

    let change = Change {
        record_id: id,
        code: record_key(catalog, current.code(), current.name()),
        action: if active {
            ChangeAction::Reactivate
        } else {
            ChangeAction::Deactivate
        },
        fields: vec![FieldChange {
            column: IS_ACTIVE.to_owned(),
            old: Json::Bool(!active),
            new: Json::Bool(active),
        }],
    };
    record(&txn, catalog, user_id, vec![change]).await?;
    txn.commit().await?;
    Ok(())
}

// Borra el registro `id` si ninguna salvaguarda lo impide y nadie lo referencia (lo garantizan las
// claves foráneas).
pub(crate) async fn delete(
    catalog: &Catalog,
    id: i32,
    user_id: Option<i32>,
) -> Result<(), CatalogError> {
    let reasons = guard_reasons(catalog, id).await?;
    if !reasons.is_empty() {
        return Err(CatalogError::InUse(reasons));
    }
    let txn = dbconn().begin().await?;
    let current = find_in(&txn, catalog, id).await?;
    let mut stmt = Query::delete();
    stmt.from_table(col(catalog.table()))
        .and_where(Expr::col(col(ID)).eq(id));
    let backend = txn.get_database_backend();
    let result = txn
        .execute(backend.build(&stmt))
        .await
        .map_err(CatalogError::from_delete)?;
    // Otra petición lo borró entretanto.
    if result.rows_affected() == 0 {
        return Err(CatalogError::NotFound);
    }

    let change = Change {
        record_id: id,
        code: record_key(catalog, current.code(), current.name()),
        action: ChangeAction::Delete,
        fields: Vec::new(),
    };
    record(&txn, catalog, user_id, vec![change]).await?;
    txn.commit().await?;
    Ok(())
}

// Historial del registro, del primer registrador que lo ofrezca.
pub(crate) async fn history(
    catalog: &Catalog,
    id: i32,
) -> Result<Option<Vec<crate::record::HistoryEntry>>, DbErr> {
    for recorder in &REGISTRY.recorders {
        if let Some(entries) = recorder.history(catalog, id).await? {
            return Ok(Some(entries));
        }
    }
    Ok(None)
}

// Si hay algún registrador declarado: sin ninguno, no hay historial que mostrar.
pub(crate) fn has_recorders() -> bool {
    !REGISTRY.recorders.is_empty()
}

// **< CSV >****************************************************************************************

// Prefijo de las columnas del CSV con las traducciones del nombre (`name_i18n:es-ES`).
const I18N_PREFIX: &str = "name_i18n:";

// Columnas del CSV: las comunes, las propias y una por idioma de traducción.
pub(crate) fn csv_header(catalog: &Catalog) -> Vec<String> {
    let mut header: Vec<String> = Vec::new();
    if catalog.has_code() {
        header.push(CODE.to_owned());
    }
    header.extend([NAME, WEIGHT, IS_ACTIVE].iter().map(|c| (*c).to_owned()));
    header.extend(catalog.fields().iter().map(|f| f.column().to_owned()));
    if catalog.translations() {
        for (langid, _) in translation_languages() {
            header.push(util::join!(I18N_PREFIX, &langid));
        }
    }
    header
}

// Lo que identifica en el CSV a cada registro de los catálogos a los que se refieren los campos (su
// código o, sin código, su nombre), por catálogo e `id`.
async fn reference_codes(
    catalog: &Catalog,
) -> Result<HashMap<&'static str, HashMap<i32, String>>, DbErr> {
    let mut codes = HashMap::new();
    for field in catalog.fields() {
        if let FieldKind::Reference(target) = field.kind()
            && !codes.contains_key(target)
            && let Some(target_catalog) = find_catalog(target)
        {
            let records = all(target_catalog).await?;
            codes.insert(
                *target,
                records
                    .into_iter()
                    .map(|r| {
                        let key = if target_catalog.has_code() {
                            r.code()
                        } else {
                            r.name()
                        };
                        (r.id(), key.to_owned())
                    })
                    .collect(),
            );
        }
    }
    Ok(codes)
}

// Las filas del CSV con los registros que cumplen el filtro del listado (sin paginar), en su
// orden, cabecera incluida. Las referencias a otros catálogos van por su código o, sin código, por
// su nombre.
pub(crate) async fn export(
    catalog: &Catalog,
    params: &ListParams,
) -> Result<Vec<Vec<String>>, DbErr> {
    let db = dbconn();
    let mut select = select_record(catalog);
    filtered(catalog, params, &mut select);
    ordered(catalog, params, &mut select);
    let records: Vec<Record> = db
        .query_all(db.get_database_backend().build(&select))
        .await?
        .iter()
        .map(|row| read_record(catalog, row))
        .collect::<Result<_, _>>()?;
    let codes = reference_codes(catalog).await?;

    let mut rows = vec![csv_header(catalog)];
    for record in records {
        let mut row = Vec::with_capacity(rows[0].len());
        if catalog.has_code() {
            row.push(record.code().to_owned());
        }
        row.extend([
            record.name().to_owned(),
            record.weight().to_string(),
            record.is_active().to_string(),
        ]);
        for (field, value) in catalog.fields().iter().zip(record.values()) {
            row.push(match (field.kind(), value) {
                (FieldKind::Reference(target), FieldValue::Int(Some(id))) => codes
                    .get(target)
                    .and_then(|c| c.get(id))
                    .cloned()
                    .unwrap_or_else(|| id.to_string()),
                (_, value) => value.to_form(),
            });
        }
        if catalog.translations() {
            for (langid, _) in translation_languages() {
                row.push(
                    record
                        .translations()
                        .get(&langid)
                        .cloned()
                        .unwrap_or_default(),
                );
            }
        }
        rows.push(row);
    }
    Ok(rows)
}

// Resultado previsto de una fila del CSV.
#[derive(Clone, Debug)]
pub(crate) enum RowOutcome {
    Create,
    // Columnas que cambian.
    Update(Vec<String>),
    Unchanged,
    Invalid(Vec<Lc>),
}

#[derive(Clone, Debug)]
pub(crate) struct ImportRow {
    pub line: usize,
    pub code: String,
    pub outcome: RowOutcome,
}

// Lo que se haría con cada fila de un CSV: qué se da de alta, qué cambia y qué errores impiden
// importarlo. Se calcula sin escribir nada, para mostrarlo antes de confirmar.
#[derive(AutoDefault, Clone, Debug)]
pub(crate) struct ImportPlan {
    pub rows: Vec<ImportRow>,
    // Errores del archivo en su conjunto (cabecera, formato, tamaño).
    pub errors: Vec<Lc>,
    // Lo que hay que escribir, en el orden del archivo.
    steps: Vec<ImportStep>,
}

#[derive(Clone, Debug)]
enum ImportStep {
    Create(Input, bool),
    Update(Box<Record>, Input, Option<bool>),
}

impl ImportPlan {
    pub(crate) fn is_valid(&self) -> bool {
        self.errors.is_empty()
            && self
                .rows
                .iter()
                .all(|r| !matches!(r.outcome, RowOutcome::Invalid(_)))
    }

    pub(crate) fn count(&self, f: impl Fn(&RowOutcome) -> bool) -> usize {
        self.rows.iter().filter(|r| f(&r.outcome)).count()
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_lowercase().as_str() {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    }
}

// Valores de las columnas clave de un registro, como los da el CSV una vez convertidas las
// referencias en `id`.
fn record_key_values(catalog: &Catalog, record: &Record) -> Vec<String> {
    catalog
        .key_columns()
        .iter()
        .map(|column| match *column {
            CODE => record.code().to_owned(),
            NAME => record.name().to_owned(),
            column => record
                .value(catalog, column)
                .map(FieldValue::to_form)
                .unwrap_or_default(),
        })
        .collect()
}

// Prepara la importación de `text`: comprueba la cabecera y cada fila como lo haría el formulario,
// más las referencias (por código), los códigos repetidos y las salvaguardas de las filas que
// desactivan un registro.
pub(crate) async fn plan_import(catalog: &Catalog, text: &str) -> Result<ImportPlan, DbErr> {
    let mut plan = ImportPlan::default();
    let t = |key| Lc::t(key, &LOCALES_CATALOG);
    let rows = match crate::csv::decode(text) {
        Ok(rows) => rows,
        Err(err) => {
            plan.errors
                .push(t("import-error-quotes").with_number("line", err.0 as u32));
            return Ok(plan);
        }
    };
    let Some((header, data)) = rows.split_first() else {
        plan.errors.push(t("import-error-empty"));
        return Ok(plan);
    };
    if data.len() > config::SETTINGS.import_max_rows {
        plan.errors.push(
            t("import-error-too-many").with_number("max", config::SETTINGS.import_max_rows as u32),
        );
        return Ok(plan);
    }
    let known = csv_header(catalog);
    let header: Vec<String> = header.iter().map(|h| h.trim().to_owned()).collect();
    for column in &header {
        if !known.contains(column) {
            plan.errors
                .push(t("import-error-unknown-column").with_arg("column", column.clone()));
        }
    }
    let keys = catalog.key_columns();
    let mut required: Vec<&str> = keys.to_vec();
    if !required.contains(&NAME) {
        required.push(NAME);
    }
    for required in required {
        if !header.iter().any(|h| h == required) {
            plan.errors
                .push(t("import-error-missing-column").with_arg("column", required));
        }
    }
    if !plan.errors.is_empty() {
        return Ok(plan);
    }

    // Registros existentes por sus columnas clave, y claves de los catálogos referenciados.
    let existing: HashMap<Vec<String>, Record> = all(catalog)
        .await?
        .into_iter()
        .map(|r| (record_key_values(catalog, &r), r))
        .collect();
    let mut references: HashMap<&'static str, HashMap<String, i32>> = HashMap::new();
    for (target, by_id) in reference_codes(catalog).await? {
        references.insert(
            target,
            by_id.into_iter().map(|(id, code)| (code, id)).collect(),
        );
    }

    let mut seen: HashMap<String, usize> = HashMap::new();
    let cell_code = |values: &[String]| {
        header
            .iter()
            .position(|h| h == CODE)
            .and_then(|i| values.get(i))
            .map_or(String::new(), |v| v.trim().to_owned())
    };
    for (index, values) in data.iter().enumerate() {
        let line = index + 2;
        let cell = |column: &str| {
            header
                .iter()
                .position(|h| h == column)
                .map(|i| values.get(i).map_or("", |v| v.trim()))
        };
        // La clave de la fila, con las referencias ya convertidas en `id`.
        let raw_key: Vec<String> = keys
            .iter()
            .map(|k| cell(k).unwrap_or_default().to_owned())
            .collect();
        let key: Vec<String> = keys
            .iter()
            .zip(&raw_key)
            .map(
                |(column, raw)| match catalog.field(column).map(|f| f.kind()) {
                    Some(FieldKind::Reference(target)) => references
                        .get(target)
                        .and_then(|r| r.get(raw))
                        .map_or_else(|| raw.clone(), |id| id.to_string()),
                    _ => raw.clone(),
                },
            )
            .collect();
        let code = raw_key.join(" / ");
        let current = existing.get(&key);
        let mut errors = Vec::new();
        if let Some(first) = seen.insert(key.join("\u{1f}"), line) {
            errors.push(t("import-error-repeated").with_number("line", first as u32));
        }

        // Los valores de partida son los actuales: una columna ausente no cambia.
        let mut form: HashMap<String, String> = HashMap::new();
        if let Some(record) = current {
            let input = Input::from_record(record);
            form.insert(NAME.to_owned(), input.name.clone());
            form.insert(WEIGHT.to_owned(), input.weight.to_string());
            for (field, value) in catalog.fields().iter().zip(&input.values) {
                form.insert(field.column().to_owned(), value.to_form());
            }
            for (langid, text) in &input.translations {
                form.insert(translation_input(langid), text.clone());
            }
        }
        if catalog.has_code() {
            form.insert(CODE.to_owned(), cell_code(values));
        }
        for (i, column) in header.iter().enumerate() {
            let value = values.get(i).map_or("", |v| v.trim()).to_owned();
            if let Some(langid) = column.strip_prefix(I18N_PREFIX) {
                form.insert(translation_input(langid), value);
            } else if column != IS_ACTIVE && column != CODE {
                form.insert(column.clone(), value);
            }
        }
        for field in catalog.fields() {
            let Some(value) = form.get(field.column()).cloned() else {
                continue;
            };
            let label = field.label().clone();
            match field.kind() {
                FieldKind::Boolean if cell(field.column()).is_some() => match parse_bool(&value) {
                    Some(b) => {
                        form.insert(field.column().to_owned(), b.to_string());
                    }
                    None if value.is_empty() => {}
                    None => errors.push(t("error-invalid-option").with_lc("field", label)),
                },
                // Las referencias del CSV van por código (o nombre); las de partida, por `id`.
                FieldKind::Reference(target)
                    if cell(field.column()).is_some() && !value.is_empty() =>
                {
                    match references.get(target).and_then(|codes| codes.get(&value)) {
                        Some(id) => {
                            form.insert(field.column().to_owned(), id.to_string());
                        }
                        None => errors.push(t("error-invalid-option").with_lc("field", label)),
                    }
                }
                _ => {}
            }
        }
        let active = match cell(IS_ACTIVE) {
            None | Some("") => None,
            Some(value) => match parse_bool(value) {
                Some(active) => Some(active),
                None => {
                    errors.push(t("error-invalid-option").with_lc("field", t("field-active")));
                    None
                }
            },
        };

        let (mut input, parse_errors) = Input::parse(catalog, &form, current.is_none());
        errors.extend(parse_errors);
        if errors.is_empty() {
            let db = dbconn();
            if let Some(record) = current {
                input.code = record.code().to_owned();
            }
            if let Err(err) = validate(db, catalog, &input, current).await {
                errors.extend(err.messages());
            }
        }
        if errors.is_empty()
            && active == Some(false)
            && let Some(record) = current.filter(|r| r.is_active())
        {
            errors.extend(guard_reasons(catalog, record.id()).await?);
        }

        let outcome = if !errors.is_empty() {
            RowOutcome::Invalid(errors)
        } else if let Some(record) = current {
            let mut columns: Vec<String> = diff(catalog, Some(&Input::from_record(record)), &input)
                .into_iter()
                .map(|c| c.column)
                .collect();
            if active.is_some_and(|a| a != record.is_active()) {
                columns.push(IS_ACTIVE.to_owned());
            }
            if columns.is_empty() {
                RowOutcome::Unchanged
            } else {
                plan.steps
                    .push(ImportStep::Update(Box::new(record.clone()), input, active));
                RowOutcome::Update(columns)
            }
        } else {
            plan.steps
                .push(ImportStep::Create(input, active.unwrap_or(true)));
            RowOutcome::Create
        };
        plan.rows.push(ImportRow {
            line,
            code,
            outcome,
        });
    }
    Ok(plan)
}

// Importa `text` en una sola transacción, todo o nada: vuelve a preparar la importación (los datos
// pueden haber cambiado desde que se previsualizó) y, si alguna fila no es válida, no escribe nada.
// Devuelve las altas y las modificaciones.
pub(crate) async fn import(
    catalog: &Catalog,
    text: &str,
    user_id: Option<i32>,
) -> Result<(usize, usize), CatalogError> {
    let plan = plan_import(catalog, text).await?;
    if !plan.is_valid() {
        let mut messages = plan.errors.clone();
        if messages.is_empty() {
            messages.push(Lc::t("import-error-changed", &LOCALES_CATALOG));
        }
        return Err(CatalogError::Invalid(messages));
    }
    let txn = dbconn().begin().await?;
    let mut changes = Vec::new();
    let (mut created, mut updated) = (0, 0);
    for step in &plan.steps {
        match step {
            ImportStep::Create(input, active) => {
                changes.push(insert_in(&txn, catalog, input, *active, user_id).await?);
                created += 1;
            }
            ImportStep::Update(current, input, active) => {
                if let Some(change) =
                    update_in(&txn, catalog, current, input, *active, user_id).await?
                {
                    changes.push(change);
                    updated += 1;
                }
            }
        }
    }
    for change in &mut changes {
        change.action = ChangeAction::Import;
    }
    record(&txn, catalog, user_id, changes).await?;
    txn.commit().await?;
    Ok((created, updated))
}
