use pagetop_seaorm::db::*;

use chrono::NaiveDateTime;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "menu_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub menu_id: i32,
    pub parent_id: Option<i32>,
    pub url: String,
    pub weight: i32,
    pub enabled: bool,
    pub expanded: bool,
    pub provider: String,
    pub external_key: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::menu::Entity",
        from = "Column::MenuId",
        to = "super::menu::Column::Id",
        on_delete = "Cascade"
    )]
    Menu,
    #[sea_orm(has_many = "super::menu_item_translation::Entity")]
    MenuItemTranslations,
}

impl Related<super::menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Menu.def()
    }
}

impl Related<super::menu_item_translation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MenuItemTranslations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
