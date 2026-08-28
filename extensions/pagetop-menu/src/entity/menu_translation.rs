use pagetop_seaorm::db::*;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "menu_translations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub menu_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub lang: String,
    pub title: String,
    pub description: Option<String>,
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
}

impl Related<super::menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Menu.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
