//! Operaciones de base de datos para menús e ítems de menú.

use chrono::Utc;

use pagetop::locale::Locale;
use pagetop_seaorm::db::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set, dbconn,
};

use crate::entity::{menu, menu_item, menu_item_translation, menu_translation};
use crate::error::MenuError;
use crate::tree::MenuKey;

// **< Tipos de entrada >***************************************************************************

/// Datos necesarios para crear un nuevo menú.
pub struct NewMenu {
    pub machine_name: String,
    /// Títulos por idioma: `(lang, title)`. Al menos uno es obligatorio.
    pub titles: Vec<(String, String)>,
    pub locked: bool,
}

/// Datos de un ítem nuevo o para hacer upsert.
pub struct NewMenuItem {
    pub parent_key: Option<String>,
    /// Títulos por idioma: `(lang, title)`. Al menos uno es obligatorio.
    pub titles: Vec<(String, String)>,
    pub url: String,
    pub weight: i32,
    pub enabled: bool,
    pub expanded: bool,
    pub provider: String,
    pub external_key: Option<String>,
}

impl NewMenuItem {
    pub fn new() -> Self {
        NewMenuItem {
            parent_key: None,
            titles: Vec::new(),
            url: String::new(),
            weight: 0,
            enabled: true,
            expanded: false,
            provider: "user".into(),
            external_key: None,
        }
    }

    /// Añade el título en el idioma por defecto de la aplicación.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        let lang = Locale::default_langid().to_string();
        self.titles.push((lang, title.into()));
        self
    }

    /// Añade el título en el idioma indicado.
    pub fn with_title_for(mut self, lang: impl Into<String>, title: impl Into<String>) -> Self {
        self.titles.push((lang.into(), title.into()));
        self
    }

    pub fn with_url(mut self, u: impl Into<String>) -> Self {
        self.url = u.into();
        self
    }

    pub fn with_weight(mut self, w: i32) -> Self {
        self.weight = w;
        self
    }

    pub fn with_enabled(mut self, v: bool) -> Self {
        self.enabled = v;
        self
    }

    pub fn with_expanded(mut self, v: bool) -> Self {
        self.expanded = v;
        self
    }

    pub fn with_provider(mut self, p: impl Into<String>) -> Self {
        self.provider = p.into();
        self
    }

    pub fn with_external_key(mut self, k: impl Into<String>) -> Self {
        self.external_key = Some(k.into());
        self
    }

    pub fn with_parent_key(mut self, k: impl Into<String>) -> Self {
        self.parent_key = Some(k.into());
        self
    }
}

impl Default for NewMenuItem {
    fn default() -> Self {
        Self::new()
    }
}

// **< find_menu_model >****************************************************************************

/// Devuelve el modelo de BD del menú dado, o `None` si no existe.
pub async fn find_menu_model(key: &MenuKey) -> Option<menu::Model> {
    match key {
        MenuKey::Id(id) => menu::Entity::find_by_id(*id).one(dbconn()).await.ok()?,
        MenuKey::Name(n) => menu::Entity::find()
            .filter(menu::Column::MachineName.eq(n.as_str()))
            .one(dbconn())
            .await
            .ok()?,
    }
}

// **< create_menu >********************************************************************************

/// Crea un nuevo menú en la base de datos con sus traducciones iniciales.
pub async fn create_menu(input: NewMenu) -> Result<menu::Model, MenuError> {
    if input.machine_name.is_empty()
        || !input
            .machine_name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(MenuError::InvalidName(input.machine_name));
    }

    let now = Utc::now().naive_utc();
    let result = menu::Entity::insert(menu::ActiveModel {
        id: NotSet,
        machine_name: Set(input.machine_name),
        locked: Set(input.locked),
        created_at: Set(now),
        updated_at: Set(now),
    })
    .exec_with_returning(dbconn())
    .await?;

    for (lang, title) in input.titles {
        upsert_menu_translation(result.id, &lang, &title, None).await?;
    }

    Ok(result)
}

/// Crea un menú sólo si no existe ya uno con el mismo `machine_name`, y sincroniza
/// la traducción del título en el idioma por defecto.
pub async fn ensure_menu(machine_name: &str, title: &str) -> Result<(), MenuError> {
    let lang = Locale::default_langid().to_string();

    let existing = find_menu_model(&MenuKey::Name(machine_name.to_owned())).await;

    let menu_id = if let Some(m) = existing {
        m.id
    } else {
        create_menu(NewMenu {
            machine_name: machine_name.to_owned(),
            titles: vec![(lang.clone(), title.to_owned())],
            locked: false,
        })
        .await?
        .id
    };

    upsert_menu_translation(menu_id, &lang, title, None).await?;

    Ok(())
}

// **< upsert_item >********************************************************************************

/// Inserta o actualiza un ítem identificado por `(provider, external_key)`.
///
/// Si el ítem ya existe, actualiza `url`, `weight` y `parent_id`; los campos `enabled`
/// y `expanded` modificados por el administrador no se sobreescriben. Las traducciones
/// se sincronizan para los idiomas incluidos en `input.titles`.
pub async fn upsert_item(
    menu_id: i32,
    provider: &str,
    external_key: &str,
    input: NewMenuItem,
) -> Result<(), MenuError> {
    let existing = menu_item::Entity::find()
        .filter(menu_item::Column::Provider.eq(provider))
        .filter(menu_item::Column::ExternalKey.eq(external_key))
        .one(dbconn())
        .await?;

    let parent_id = resolve_parent_id(menu_id, &input.parent_key).await;
    let now = Utc::now().naive_utc();

    let item_id = if let Some(row) = existing {
        menu_item::ActiveModel {
            id: Set(row.id),
            url: Set(input.url),
            weight: Set(input.weight),
            parent_id: Set(parent_id),
            updated_at: Set(now),
            ..Default::default()
        }
        .update(dbconn())
        .await?;
        row.id
    } else {
        menu_item::Entity::insert(menu_item::ActiveModel {
            id: NotSet,
            menu_id: Set(menu_id),
            parent_id: Set(parent_id),
            url: Set(input.url),
            weight: Set(input.weight),
            enabled: Set(input.enabled),
            expanded: Set(input.expanded),
            provider: Set(provider.to_owned()),
            external_key: Set(Some(external_key.to_owned())),
            created_at: Set(now),
            updated_at: Set(now),
        })
        .exec_with_returning(dbconn())
        .await?
        .id
    };

    for (lang, title) in input.titles {
        upsert_item_translation(item_id, &lang, &title).await?;
    }

    Ok(())
}

// **< Funciones internas >*************************************************************************

async fn resolve_parent_id(menu_id: i32, parent_key: &Option<String>) -> Option<i32> {
    let key = parent_key.as_deref()?;
    let row = menu_item::Entity::find()
        .filter(menu_item::Column::MenuId.eq(menu_id))
        .filter(menu_item::Column::ExternalKey.eq(key))
        .one(dbconn())
        .await
        .ok()??;
    Some(row.id)
}

async fn upsert_menu_translation(
    menu_id: i32,
    lang: &str,
    title: &str,
    description: Option<&str>,
) -> Result<(), MenuError> {
    let existing = menu_translation::Entity::find()
        .filter(menu_translation::Column::MenuId.eq(menu_id))
        .filter(menu_translation::Column::Lang.eq(lang))
        .one(dbconn())
        .await?;

    if existing.is_some() {
        menu_translation::ActiveModel {
            menu_id: Set(menu_id),
            lang: Set(lang.to_owned()),
            title: Set(title.to_owned()),
            description: Set(description.map(str::to_owned)),
        }
        .update(dbconn())
        .await?;
    } else {
        menu_translation::Entity::insert(menu_translation::ActiveModel {
            menu_id: Set(menu_id),
            lang: Set(lang.to_owned()),
            title: Set(title.to_owned()),
            description: Set(description.map(str::to_owned)),
        })
        .exec(dbconn())
        .await?;
    }

    Ok(())
}

async fn upsert_item_translation(item_id: i32, lang: &str, title: &str) -> Result<(), MenuError> {
    let existing = menu_item_translation::Entity::find()
        .filter(menu_item_translation::Column::ItemId.eq(item_id))
        .filter(menu_item_translation::Column::Lang.eq(lang))
        .one(dbconn())
        .await?;

    if existing.is_some() {
        menu_item_translation::ActiveModel {
            item_id: Set(item_id),
            lang: Set(lang.to_owned()),
            title: Set(title.to_owned()),
        }
        .update(dbconn())
        .await?;
    } else {
        menu_item_translation::Entity::insert(menu_item_translation::ActiveModel {
            item_id: Set(item_id),
            lang: Set(lang.to_owned()),
            title: Set(title.to_owned()),
        })
        .exec(dbconn())
        .await?;
    }

    Ok(())
}
