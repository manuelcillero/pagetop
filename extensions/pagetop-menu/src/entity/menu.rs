use pagetop_seaorm::db::*;

use chrono::NaiveDateTime;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "menus")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub machine_name: String,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::menu_translation::Entity")]
    MenuTranslations,
    #[sea_orm(has_many = "super::menu_item::Entity")]
    MenuItems,
}

impl Related<super::menu_translation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MenuTranslations.def()
    }
}

impl Related<super::menu_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MenuItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
