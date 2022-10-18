use crate::config;
use crate::db::*;
use crate::{run_now, trace, LazyStatic};

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend, Statement};
use tracing_unwrap::ResultExt;

pub static DBCONN: LazyStatic<DbConn> = LazyStatic::new(|| {
    trace::info!(
        "Connecting to database \"{}\" using a pool of {} connections",
        config::get("database.db_name"),
        config::get("database.max_pool_size")
    );

    let db_uri = match config::get("database.db_type").as_str() {
        "mysql" | "postgres" => {
            let mut tmp_uri = DbUri::parse(
                format!(
                    "{}://{}/{}",
                    config::get("database.db_type"),
                    config::get("database.db_host"),
                    config::get("database.db_name")
                )
                .as_str(),
            )
            .unwrap();
            tmp_uri
                .set_username(config::get("database.db_user").as_str())
                .unwrap();
            // https://github.com/launchbadge/sqlx/issues/1624
            tmp_uri
                .set_password(Some(config::get("database.db_pass").as_str()))
                .unwrap();
            if config::get_value::<u16>("database.db_port") != 0 {
                tmp_uri.set_port(Some(config::get_value::<u16>("database.db_port"))).unwrap();
            }
            tmp_uri
        }
        "sqlite" => DbUri::parse(
            format!(
                "{}://{}",
                config::get("database.db_type"), &config::get("database.db_name")
            )
            .as_str(),
        )
        .unwrap(),
        _ => {
            trace::error!(
                "Unrecognized database type \"{}\"",
                config::get("database.db_type")
            );
            DbUri::parse("").unwrap()
        }
    };

    run_now(Database::connect::<ConnectOptions>({
        let mut db_opt = ConnectOptions::new(db_uri.to_string());
        db_opt.max_connections(config::get_value::<u32>("database.max_pool_size"));
        db_opt
    }))
    .expect_or_log("Failed to connect to database")
});

static DBBACKEND: LazyStatic<DatabaseBackend> = LazyStatic::new(|| DBCONN.get_database_backend());

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
