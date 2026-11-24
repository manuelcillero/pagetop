use pagetop_seaorm::db::sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use pagetop_seaorm::db::*;

// A database with a parent table with a unique column, and two child tables that reference it:
// one with `ON DELETE RESTRICT` (SQLite reports it with another error code) and one without.
async fn database() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    for sql in [
        "PRAGMA foreign_keys = ON",
        "CREATE TABLE parent (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE)",
        "CREATE TABLE child (id INTEGER PRIMARY KEY, \
         parent_id INTEGER NOT NULL REFERENCES parent (id) ON DELETE RESTRICT ON UPDATE RESTRICT)",
        "CREATE TABLE other (id INTEGER PRIMARY KEY, \
         parent_id INTEGER NOT NULL REFERENCES parent (id))",
        "INSERT INTO parent (id, name) VALUES (1, 'a'), (2, 'b')",
        "INSERT INTO child (id, parent_id) VALUES (1, 1)",
        "INSERT INTO other (id, parent_id) VALUES (1, 2)",
    ] {
        db.execute_unprepared(sql).await.unwrap();
    }
    db
}

#[tokio::test]
async fn a_duplicate_value_is_a_unique_violation() {
    let db = database().await;
    for sql in [
        "INSERT INTO parent (id, name) VALUES (3, 'a')",
        "INSERT INTO parent (id, name) VALUES (1, 'c')",
    ] {
        let err = db.execute_unprepared(sql).await.unwrap_err();
        assert!(err.is_unique_violation(), "{sql}: {err}");
        assert!(!err.is_foreign_key_violation(), "{sql}: {err}");
    }
}

#[tokio::test]
async fn a_broken_reference_is_a_foreign_key_violation() {
    let db = database().await;
    for sql in [
        "DELETE FROM parent WHERE id = 1",
        "UPDATE parent SET id = 10 WHERE id = 1",
        "DELETE FROM parent WHERE id = 2",
        "INSERT INTO child (id, parent_id) VALUES (2, 99)",
    ] {
        let err = db.execute_unprepared(sql).await.unwrap_err();
        assert!(err.is_foreign_key_violation(), "{sql}: {err}");
        assert!(!err.is_unique_violation(), "{sql}: {err}");
    }
}

#[tokio::test]
async fn other_errors_are_no_violation() {
    let db = database().await;
    let syntax = db.execute_unprepared("NOT SQL").await.unwrap_err();
    let not_null = db
        .execute_unprepared("INSERT INTO parent (id) VALUES (3)")
        .await
        .unwrap_err();
    // Same SQLite error code as a `RESTRICT` foreign key, but raised by a trigger.
    db.execute_unprepared(
        "CREATE TRIGGER no_names BEFORE INSERT ON parent \
         BEGIN SELECT RAISE(ABORT, 'no names'); END",
    )
    .await
    .unwrap();
    let trigger = db
        .execute_unprepared("INSERT INTO parent (id, name) VALUES (4, 'd')")
        .await
        .unwrap_err();
    for err in [syntax, not_null, trigger, DbErr::RecordNotFound("x".into())] {
        assert!(!err.is_unique_violation(), "{err}");
        assert!(!err.is_foreign_key_violation(), "{err}");
    }
}
