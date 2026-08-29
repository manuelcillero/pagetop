//! Almacén de configuración persistente basado en `settings`.
//!
//! Proporciona una API async para leer y escribir valores JSON en la tabla `settings`.

use pagetop::datetime::Utc;
use pagetop::{Getters, builder_impl};
use pagetop_seaorm::db::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, dbconn,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::entity::setting::{ActiveModel, Column, Entity};
use crate::error::AdminError;

// **< API pública >*********************************************************************************

/// Lee un valor persistido, devolviendo el `Default` del tipo si no existe.
pub async fn get<T: DeserializeOwned + Default>(key: &str) -> T {
    get_async(key).await.unwrap_or_default()
}

/// Lee un valor persistido, devolviendo `default` si no existe o hay error.
pub async fn get_or<T: DeserializeOwned>(key: &str, default: T) -> T {
    get_async(key).await.unwrap_or(default)
}

/// Escribe un valor en la tabla `settings`.
///
/// Si la clave no existe se inserta; si existe, se actualiza.
pub async fn set<T: Serialize>(key: &str, value: &T, scope: &str, user_id: Option<i32>) {
    set_async(key, value, scope, user_id).await.ok();
}

/// Elimina una entrada de `settings` por clave.
pub async fn delete(key: &str) {
    delete_async(key).await.ok();
}

/// Devuelve todos los pares `(key, value_json)` de un `scope` dado.
pub async fn list_scope(scope: &str) -> Vec<(String, String)> {
    list_scope_async(scope).await.unwrap_or_default()
}

// **< Implementación asíncrona >********************************************************************

async fn get_async<T: DeserializeOwned>(key: &str) -> Result<T, AdminError> {
    let model = Entity::find_by_id(key)
        .one(dbconn())
        .await?
        .ok_or_else(|| AdminError::NotFound(key.to_owned()))?;
    Ok(serde_json::from_str(&model.value)?)
}

async fn set_async<T: Serialize>(
    key: &str,
    value: &T,
    scope: &str,
    user_id: Option<i32>,
) -> Result<(), AdminError> {
    let value_json = serde_json::to_string(value)?;
    let now = Utc::now().naive_utc();

    let existing = Entity::find_by_id(key).one(dbconn()).await?;
    if existing.is_some() {
        let model = ActiveModel {
            key: ActiveValue::Unchanged(key.to_owned()),
            scope: ActiveValue::Set(scope.to_owned()),
            value: ActiveValue::Set(value_json),
            updated_at: ActiveValue::Set(now),
            updated_by: ActiveValue::Set(user_id),
        };
        model.update(dbconn()).await?;
    } else {
        let model = ActiveModel {
            key: ActiveValue::Set(key.to_owned()),
            scope: ActiveValue::Set(scope.to_owned()),
            value: ActiveValue::Set(value_json),
            updated_at: ActiveValue::Set(now),
            updated_by: ActiveValue::Set(user_id),
        };
        model.insert(dbconn()).await?;
    }
    Ok(())
}

async fn delete_async(key: &str) -> Result<(), AdminError> {
    Entity::delete_by_id(key).exec(dbconn()).await?;
    Ok(())
}

async fn list_scope_async(scope: &str) -> Result<Vec<(String, String)>, AdminError> {
    let rows = Entity::find()
        .filter(Column::Scope.eq(scope))
        .all(dbconn())
        .await?;
    Ok(rows.into_iter().map(|m| (m.key, m.value)).collect())
}

// **< Tipos de esquema >****************************************************************************

/// Tipo de campo de configuración para un [`SettingsSchema`].
#[derive(Clone, Debug)]
pub enum SettingFieldType {
    /// Texto libre con longitud máxima opcional.
    Text { max_length: Option<usize> },
    /// Número (entero o decimal).
    Number { min: Option<f64>, max: Option<f64> },
    /// Casilla de verificación (booleano).
    Boolean,
    /// Lista de opciones `(valor, etiqueta)`.
    Select { options: Vec<(String, String)> },
}

/// Definición de un campo dentro de un [`SettingsSchema`].
#[derive(Clone, Debug, Getters)]
pub struct SettingField {
    /// Devuelve el nombre del campo (clave dentro del scope, p. ej. `"site_name"`).
    name: String,
    /// Devuelve la etiqueta visible en el formulario.
    label: String,
    /// Devuelve el tipo de campo.
    field_type: SettingFieldType,
    /// Devuelve si el campo es obligatorio.
    required: bool,
    /// Devuelve el texto de ayuda opcional bajo el campo.
    help_text: Option<String>,
    /// Devuelve el valor por defecto como cadena JSON.
    default_value: Option<String>,
}

#[builder_impl]
impl SettingField {
    /// Crea un campo de texto con nombre y etiqueta.
    pub fn text(name: impl Into<String>, label: impl Into<String>) -> Self {
        SettingField {
            name: name.into(),
            label: label.into(),
            field_type: SettingFieldType::Text { max_length: None },
            required: false,
            help_text: None,
            default_value: None,
        }
    }

    /// Crea un campo numérico con nombre y etiqueta.
    pub fn number(name: impl Into<String>, label: impl Into<String>) -> Self {
        SettingField {
            name: name.into(),
            label: label.into(),
            field_type: SettingFieldType::Number {
                min: None,
                max: None,
            },
            required: false,
            help_text: None,
            default_value: None,
        }
    }

    /// Crea un campo booleano con nombre y etiqueta.
    pub fn boolean(name: impl Into<String>, label: impl Into<String>) -> Self {
        SettingField {
            name: name.into(),
            label: label.into(),
            field_type: SettingFieldType::Boolean,
            required: false,
            help_text: None,
            default_value: None,
        }
    }

    /// Crea un campo de selección con nombre, etiqueta y opciones.
    pub fn select(
        name: impl Into<String>,
        label: impl Into<String>,
        options: Vec<(String, String)>,
    ) -> Self {
        SettingField {
            name: name.into(),
            label: label.into(),
            field_type: SettingFieldType::Select { options },
            required: false,
            help_text: None,
            default_value: None,
        }
    }

    /// Establece si el campo es obligatorio.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Añade texto de ayuda bajo el campo.
    pub fn with_help(mut self, text: impl Into<String>) -> Self {
        self.help_text = Some(text.into());
        self
    }

    /// Establece el valor por defecto (como valor JSON serializado).
    pub fn with_default<T: Serialize>(mut self, value: &T) -> Self {
        self.default_value = serde_json::to_string(value).ok();
        self
    }
}

/// Esquema de formulario de configuración: describe un grupo de ajustes en un scope.
#[derive(Clone, Debug, Getters)]
pub struct SettingsSchema {
    /// Devuelve el identificador del grupo (prefijo de las claves en `settings`).
    scope: String,
    /// Devuelve los campos del formulario en orden de presentación.
    fields: Vec<SettingField>,
}

#[builder_impl]
impl SettingsSchema {
    /// Crea un nuevo esquema vacío para el `scope` dado.
    pub fn new(scope: impl Into<String>) -> Self {
        SettingsSchema {
            scope: scope.into(),
            fields: Vec::new(),
        }
    }

    /// Añade un campo al esquema.
    pub fn with_field(mut self, field: SettingField) -> Self {
        self.fields.push(field);
        self
    }

    /// Devuelve la clave completa de un campo: `"{scope}.{field_name}"`.
    pub fn key_for(&self, field_name: &str) -> String {
        format!("{}.{}", self.scope, field_name)
    }
}
