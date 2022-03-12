use crate::db::migration::*;

pub mod m20220312_000001_create_table_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220312_000001_create_table_user::Migration)]
    }
}
