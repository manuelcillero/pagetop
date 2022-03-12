use crate::prelude::*;

pub fn migration() -> String {
    let mut m = db::Migration::new();

    m.create_table("user", |t| {
        t.add_column("id", db::types::primary());
        t.add_column("title", db::types::varchar(255));
        t.add_column("is_completed", db::types::boolean().default(false));
    });

    m.make::<db::Database>()
}
