use pagetop::prelude::*;
use pagetop_seaorm::db::{DatabaseTransaction, DbErr};

use serde_json::Value as Json;

use std::collections::BTreeMap;

use crate::catalog::{Catalog, FieldKind};

// **< FieldValue >*********************************************************************************

/// Valor de una columna propia de un registro (ver [`Field`](crate::Field)).
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    /// Texto, opción o texto de varias líneas; `None` si está vacío.
    Text(Option<String>),
    /// Sí o no.
    Bool(bool),
    /// Número entero o referencia a otro catálogo; `None` si está vacío.
    Int(Option<i32>),
    /// Número con decimales; `None` si está vacío.
    Decimal(Option<f64>),
}

impl FieldValue {
    /// El valor vacío de un campo del tipo `kind`.
    pub(crate) fn empty(kind: &FieldKind) -> Self {
        match kind {
            FieldKind::Text | FieldKind::TextArea | FieldKind::Choice(_) => Self::Text(None),
            FieldKind::Boolean => Self::Bool(false),
            FieldKind::Integer { .. } | FieldKind::Reference(_) => Self::Int(None),
            FieldKind::Decimal { .. } => Self::Decimal(None),
        }
    }

    /// Si está vacío.
    pub fn is_empty(&self) -> bool {
        matches!(
            self,
            Self::Text(None) | Self::Int(None) | Self::Decimal(None)
        )
    }

    /// El texto, si es un texto con valor.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => text.as_deref(),
            _ => None,
        }
    }

    /// El número, si es un número o una referencia con valor.
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Self::Int(n) => *n,
            _ => None,
        }
    }

    /// El valor booleano; `false` si no es un campo booleano.
    pub fn as_bool(&self) -> bool {
        matches!(self, Self::Bool(true))
    }

    /// El valor en JSON, como se describe en un [`FieldChange`].
    pub fn to_json(&self) -> Json {
        match self {
            Self::Text(text) => text.clone().map_or(Json::Null, Json::String),
            Self::Bool(b) => Json::Bool(*b),
            Self::Int(n) => n.map_or(Json::Null, |n| Json::Number(n.into())),
            Self::Decimal(n) => n
                .and_then(serde_json::Number::from_f64)
                .map_or(Json::Null, Json::Number),
        }
    }

    // Texto con el que se compara en el formulario y en el listado.
    pub(crate) fn to_form(&self) -> String {
        match self {
            Self::Text(text) => text.clone().unwrap_or_default(),
            Self::Bool(b) => b.to_string(),
            Self::Int(n) => n.map(|n| n.to_string()).unwrap_or_default(),
            Self::Decimal(n) => n.map(|n| n.to_string()).unwrap_or_default(),
        }
    }
}

// **< Record >*************************************************************************************

/// Un registro de un catálogo: sus columnas comunes y los valores de sus columnas propias, en el
/// orden de [`Catalog::fields()`].
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Record {
    /// Devuelve la clave técnica.
    #[getters(copy)]
    id: i32,
    /// Devuelve el código, único en el catálogo.
    code: String,
    /// Devuelve el nombre, en el idioma predeterminado de la aplicación.
    name: String,
    /// Devuelve las traducciones del nombre, por identificador de idioma (`"en-US"`...).
    translations: BTreeMap<String, String>,
    /// Devuelve el orden de presentación (menor = antes).
    #[getters(copy)]
    weight: i32,
    /// Devuelve si está activo: uno inactivo no se ofrece para registros nuevos.
    #[getters(copy)]
    is_active: bool,
    /// Devuelve la versión del registro, que aumenta con cada cambio.
    #[getters(copy)]
    row_version: i32,
    /// Devuelve los valores de las columnas propias.
    values: Vec<FieldValue>,
}

impl Record {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        id: i32,
        code: String,
        name: String,
        translations: BTreeMap<String, String>,
        weight: i32,
        is_active: bool,
        row_version: i32,
        values: Vec<FieldValue>,
    ) -> Self {
        Record {
            id,
            code,
            name,
            translations,
            weight,
            is_active,
            row_version,
            values,
        }
    }

    /// El valor de la columna propia `column` del catálogo `catalog`.
    pub fn value(&self, catalog: &Catalog, column: &str) -> Option<&FieldValue> {
        let index = catalog.fields().iter().position(|f| f.column() == column)?;
        self.values.get(index)
    }

    /// El nombre en el idioma de `language` si tiene traducción a él; si no, el nombre.
    pub fn localized_name(&self, language: &impl LangId) -> &str {
        localized(&self.name, &self.translations, language)
    }
}

/// El texto `name` en el idioma de `language` si `translations` lo traduce a él (por su
/// identificador completo, `"en-US"`, o sólo por el idioma, `"en"`); si no, `name`.
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use std::collections::BTreeMap;
/// let translations = BTreeMap::from([("en-US".to_string(), "Other".to_string())]);
/// let name = pagetop_catalog::localized("Otro", &translations, &Locale::resolve("en-US"));
/// assert_eq!(name, "Other");
/// ```
pub fn localized<'a>(
    name: &'a str,
    translations: &'a BTreeMap<String, String>,
    language: &impl LangId,
) -> &'a str {
    if translations.is_empty() {
        return name;
    }
    let langid = language.langid();
    translations
        .get(&langid.to_string())
        .or_else(|| translations.get(langid.language.as_str()))
        .map_or(name, String::as_str)
}

// **< Change >*************************************************************************************

/// Qué le ha pasado a un registro. Ver [`Change`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeAction {
    /// Alta.
    Create,
    /// Modificación.
    Update,
    /// Desactivación.
    Deactivate,
    /// Reactivación.
    Reactivate,
    /// Borrado.
    Delete,
    /// Alta o modificación al importar un CSV.
    Import,
}

impl ChangeAction {
    /// Todas, en el orden en que se presentan.
    pub const ALL: [Self; 6] = [
        Self::Create,
        Self::Update,
        Self::Deactivate,
        Self::Reactivate,
        Self::Delete,
        Self::Import,
    ];

    /// Valor que se guarda: `"create"`, `"update"`, `"deactivate"`, `"reactivate"`, `"delete"` o
    /// `"import"`.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Update => "update",
            Self::Deactivate => "deactivate",
            Self::Reactivate => "reactivate",
            Self::Delete => "delete",
            Self::Import => "import",
        }
    }

    /// La acción guardada como `value`, si es una de ellas.
    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|a| a.as_str() == value)
    }

    // Etiqueta para el historial.
    pub(crate) fn label(&self) -> Lc {
        Lc::t(
            util::join!("action-", self.as_str()),
            &crate::LOCALES_CATALOG,
        )
    }
}

/// Una columna que cambia: su nombre y sus valores anterior y nuevo, en JSON (`null` si estaba o
/// queda vacía).
#[derive(Clone, Debug, PartialEq)]
pub struct FieldChange {
    /// Columna.
    pub column: String,
    /// Valor anterior.
    pub old: Json,
    /// Valor nuevo.
    pub new: Json,
}

/// Un cambio en un registro de un catálogo. Ver [`ChangeRecorder`].
#[derive(Clone, Debug)]
pub struct Change {
    /// Registro que cambia.
    pub record_id: i32,
    /// Código del registro en el momento del cambio.
    pub code: String,
    /// Qué le pasa.
    pub action: ChangeAction,
    /// Columnas que cambian, con su valor anterior y el nuevo. En un alta, todas las que tienen
    /// valor, con el anterior a `null`; en un borrado, ninguna.
    pub fields: Vec<FieldChange>,
}

/// Los cambios de una misma operación, hechos por el mismo usuario en el mismo catálogo y en la
/// misma transacción. Ver [`ChangeRecorder`].
#[derive(Debug)]
pub struct ChangeSet<'a> {
    /// Catálogo.
    pub catalog: &'a Catalog,
    /// Usuario que hace los cambios (`None` si no hay ninguno identificado).
    pub user_id: Option<i32>,
    /// Los cambios, en el orden en que se hacen.
    pub changes: Vec<Change>,
}

/// Una entrada del historial de un registro. Ver [`ChangeRecorder::history()`].
#[derive(Clone, Debug)]
pub struct HistoryEntry {
    /// Cuándo se hizo el cambio.
    pub at: DateTime<Utc>,
    /// Nombre de quien lo hizo, si se conoce.
    pub user: Option<String>,
    /// Qué le pasó al registro.
    pub action: ChangeAction,
    /// Columnas que cambiaron.
    pub fields: Vec<FieldChange>,
    /// Motivo o comentario del cambio, si lo hubo.
    pub comment: Option<String>,
}

/// Registra los cambios de los catálogos, normalmente en una pista de auditoría de la aplicación.
///
/// `pagetop-catalog` no guarda por sí mismo quién cambió qué: cada operación (un alta, una
/// modificación, una desactivación, un borrado) se entrega a los registradores declarados con
/// [`DeclareChangeRecorders`](crate::DeclareChangeRecorders), **dentro de la misma transacción**
/// que el cambio. Si un registrador falla, el cambio se deshace: no hay cambios sin registrar.
///
/// Con un registrador que también ofrezca el historial ([`history()`](Self::history)), cada
/// registro tiene su página de historial.
#[async_trait]
pub trait ChangeRecorder: Send + Sync {
    /// Registra los cambios de una operación en la transacción `txn`.
    async fn record(&self, txn: &DatabaseTransaction, changes: &ChangeSet<'_>)
    -> Result<(), DbErr>;

    /// El historial del registro `record_id` del catálogo `catalog`, del cambio más reciente al
    /// más antiguo, o `None` si este registrador no lo ofrece (por defecto).
    async fn history(
        &self,
        _catalog: &Catalog,
        _record_id: i32,
    ) -> Result<Option<Vec<HistoryEntry>>, DbErr> {
        Ok(None)
    }
}
