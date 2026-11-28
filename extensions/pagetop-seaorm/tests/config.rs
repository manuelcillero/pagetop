use pagetop::prelude::*;
use pagetop_seaorm::config::DatabaseSettings;

fn settings(database: &str) -> DatabaseSettings {
    config::load_from_str::<DatabaseSettings>(&format!("[database]\n{database}")).unwrap()
}

// Migrations use their own user only when there is one, it is not the application user and the
// engine has users (MySQL or PostgreSQL).
#[pagetop::test]
async fn migration_user_only_when_set_and_different() {
    let postgres = "db_type = \"postgres\"\ndb_user = \"app\"\n";

    assert!(settings(&format!("{postgres}migration_user = \"owner\"")).has_migration_user());

    // Absent, empty or blank (as an environment variable may leave it), or the application user.
    assert!(!settings(postgres).has_migration_user());
    assert!(!settings(&format!("{postgres}migration_user = \"\"")).has_migration_user());
    assert!(!settings(&format!("{postgres}migration_user = \"  \"")).has_migration_user());
    assert!(!settings(&format!("{postgres}migration_user = \"app\"")).has_migration_user());

    // SQLite has no users.
    let sqlite = "db_type = \"sqlite\"\nmigration_user = \"owner\"";
    assert!(!settings(sqlite).has_migration_user());
}

// The password is optional too: without it, the owner connects with an empty one.
#[pagetop::test]
async fn migration_pass_is_optional() {
    let settings = settings("db_type = \"postgres\"\nmigration_user = \"owner\"");
    assert!(settings.has_migration_user());
    assert!(settings.migration_pass.is_none());
}
