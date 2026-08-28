use pagetop_seaorm::db::*;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "menu_item_translations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub item_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub lang: String,
    pub title: String,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::menu_item::Entity",
        from = "Column::ItemId",
        to = "super::menu_item::Column::Id",
        on_delete = "Cascade"
    )]
    MenuItem,
}

impl Related<super::menu_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MenuItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
