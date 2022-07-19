use crate::config::SETTINGS;
use crate::db::*;
use crate::{run_now, trace, Lazy};

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend, Statement};
use tracing_unwrap::ResultExt;

pub static DBCONN: Lazy<DbConn> = Lazy::new(|| {
    trace::info!(
        "Connecting to database \"{}\" using a pool of {} connections",
        &SETTINGS.database.db_name,
        &SETTINGS.database.max_pool_size
    );

    let db_uri = match SETTINGS.database.db_type.as_str() {
        "mysql" | "postgres" => {
            let mut tmp_uri = DbUri::parse(
                format!(
                    "{}://{}/{}",
                    &SETTINGS.database.db_type,
                    &SETTINGS.database.db_host,
                    &SETTINGS.database.db_name
                )
                .as_str(),
            )
            .unwrap();
            tmp_uri
                .set_username(&SETTINGS.database.db_user.as_str())
                .unwrap();
            // https://github.com/launchbadge/sqlx/issues/1624
            tmp_uri
                .set_password(Some(&SETTINGS.database.db_pass.as_str()))
                .unwrap();
            if SETTINGS.database.db_port != 0 {
                tmp_uri.set_port(Some(SETTINGS.database.db_port)).unwrap();
            }
            tmp_uri
        }
        "sqlite" => DbUri::parse(
            format!(
                "{}://{}",
                &SETTINGS.database.db_type, &SETTINGS.database.db_name
            )
            .as_str(),
        )
        .unwrap(),
        _ => {
            trace::error!(
                "Unrecognized database type \"{}\"",
                &SETTINGS.database.db_type
            );
            DbUri::parse("").unwrap()
        }
    };

    run_now(Database::connect::<ConnectOptions>({
        let mut db_opt = ConnectOptions::new(db_uri.to_string());
        db_opt.max_connections(SETTINGS.database.max_pool_size);
        db_opt.into()
    }))
    .expect_or_log("Failed to connect to database")
});

static DBBACKEND: Lazy<DatabaseBackend> = Lazy::new(|| DBCONN.get_database_backend());

pub async fn query<Q: QueryStatementWriter>(stmt: &mut Q) -> Result<Vec<QueryResult>, DbErr> {
    DBCONN
        .query_all(Statement::from_string(
            *DBBACKEND,
            match *DBBACKEND {
                DatabaseBackend::MySql => stmt.to_string(MysqlQueryBuilder),
                DatabaseBackend::Postgres => stmt.to_string(PostgresQueryBuilder),
                DatabaseBackend::Sqlite => stmt.to_string(SqliteQueryBuilder),
            },
        ))
        .await
}

pub async fn exec<Q: QueryStatementWriter>(stmt: &mut Q) -> Result<Option<QueryResult>, DbErr> {
    DBCONN
        .query_one(Statement::from_string(
            *DBBACKEND,
            match *DBBACKEND {
                DatabaseBackend::MySql => stmt.to_string(MysqlQueryBuilder),
                DatabaseBackend::Postgres => stmt.to_string(PostgresQueryBuilder),
                DatabaseBackend::Sqlite => stmt.to_string(SqliteQueryBuilder),
            },
        ))
        .await
}

pub async fn exec_raw(stmt: String) -> Result<ExecResult, DbErr> {
    DBCONN
        .execute(Statement::from_string(*DBBACKEND, stmt))
        .await
}
