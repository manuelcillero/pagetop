use pagetop::trace;

use crate::config;
use crate::db::{DbConn, DbUri};

use std::sync::LazyLock;

use sea_orm::{ConnectOptions, Database};

pub use futures::executor::block_on as run_now;

pub static DBCONN: LazyLock<DbConn> = LazyLock::new(|| {
    trace::info!(
        "Connecting to database \"{}\" using a pool of {} connections",
        &config::SETTINGS.database.db_name,
        &config::SETTINGS.database.max_pool_size
    );

    let db_uri = match config::SETTINGS.database.db_type.as_str() {
        "mysql" | "postgres" => {
            let mut tmp_uri = DbUri::parse(
                format!(
                    "{}://{}/{}",
                    &config::SETTINGS.database.db_type,
                    &config::SETTINGS.database.db_host,
                    &config::SETTINGS.database.db_name
                )
                .as_str(),
            )
            .unwrap();
            tmp_uri
                .set_username(config::SETTINGS.database.db_user.as_str())
                .unwrap();
            // https://github.com/launchbadge/sqlx/issues/1624
            tmp_uri
                .set_password(Some(config::SETTINGS.database.db_pass.as_str()))
                .unwrap();
            if config::SETTINGS.database.db_port != 0 {
                tmp_uri
                    .set_port(Some(config::SETTINGS.database.db_port))
                    .unwrap();
            }
            tmp_uri
        }
        "sqlite" => DbUri::parse(
            format!(
                "{}://{}",
                &config::SETTINGS.database.db_type,
                &config::SETTINGS.database.db_name
            )
            .as_str(),
        )
        .unwrap(),
        _ => {
            trace::error!(
                "Unrecognized database type \"{}\"",
                &config::SETTINGS.database.db_type
            );
            DbUri::parse("").unwrap()
        }
    };

    run_now(Database::connect::<ConnectOptions>({
        let mut db_opt = ConnectOptions::new(db_uri.to_string());
        db_opt.max_connections(config::SETTINGS.database.max_pool_size);
        db_opt
    }))
    .unwrap_or_else(|_| panic!("Failed to connect to database"))
});
