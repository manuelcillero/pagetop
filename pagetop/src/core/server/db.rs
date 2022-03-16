use crate::{Lazy, db, run_now, trace};
use crate::config::SETTINGS;

use sea_orm::{ConnectOptions, Database};
use tracing_unwrap::ResultExt;

pub static DBCONN: Lazy<db::DbConn> = Lazy::new(|| {
    trace::info!(
        "Connecting to database \"{}\" using a pool of {} connections",
        &SETTINGS.database.db_name,
        &SETTINGS.database.max_pool_size
    );

    let db_uri = match SETTINGS.database.db_type.as_str() {
        "mysql" | "postgres" => {
            let mut tmp_uri = db::DbUri::parse(format!(
                "{}://{}/{}",
                &SETTINGS.database.db_type,
                &SETTINGS.database.db_host,
                &SETTINGS.database.db_name
            ).as_str()).unwrap();
            tmp_uri.set_username(
                &SETTINGS.database.db_user.as_str()
            ).unwrap();
            // https://github.com/launchbadge/sqlx/issues/1624
            tmp_uri.set_password(
                Some(&SETTINGS.database.db_pass.as_str())
            ).unwrap();
            if SETTINGS.database.db_port != 0 {
                tmp_uri.set_port(
                    Some(SETTINGS.database.db_port)
                ).unwrap();
            }
            tmp_uri
        },
        "sqlite" => db::DbUri::parse(
            format!("{}://{}",
                &SETTINGS.database.db_type,
                &SETTINGS.database.db_name
            ).as_str()).unwrap(),
        _ => {
            trace::error!(
                "Unrecognized database type \"{}\"",
                &SETTINGS.database.db_type
            );
            db::DbUri::parse("").unwrap()
        }
    };

    run_now(
        Database::connect::<ConnectOptions>({
            let mut db_opt = ConnectOptions::new(db_uri.to_string());
            db_opt.max_connections(SETTINGS.database.max_pool_size);
            db_opt.into()
        })
    ).expect_or_log("Failed to connect to database")
});
