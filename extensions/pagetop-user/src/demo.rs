//! Datos de demostración: 25 roles y 48 usuarios ficticios para explorar la administración.
//!
//! Sólo se compila con la feature `demo-data`. Se ejecuta una vez desde
//! `Extension::initialize()`, después de `auth::seed_initial_data()`. Si el rol
//! `demo_role_01` ya existe, la siembra se omite para no duplicar datos en reinicios. Roles y
//! usuarios se crean en una sola transacción: si algo falla, no queda nada sembrado y se vuelve a
//! intentar en el siguiente arranque.

use pagetop::prelude::*;
use pagetop_seaorm::db::*;

use crate::account::UserStatus;
use crate::config::SETTINGS;
use crate::entity::{role, user, user_role};
use crate::error::AuthError;
use crate::identity;
use crate::password;

const ROLE_COUNT: usize = 25;
const USER_COUNT: usize = 48;
const DEMO_PASSWORD: &str = "Demo12345!";

/// Crea los roles y usuarios de demostración si todavía no existen.
pub(crate) async fn seed_demo_data() {
    let already_seeded = matches!(
        role::Entity::find()
            .filter(role::Column::MachineName.eq("demo_role_01"))
            .one(dbconn())
            .await,
        Ok(Some(_))
    );
    if already_seeded {
        return;
    }

    let hash = match password::hash_password(DEMO_PASSWORD) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("pagetop-user demo-data error: failed to hash password: {e}");
            return;
        }
    };

    let result = dbconn()
        .transaction::<_, _, AuthError>(|txn| {
            Box::pin(async move {
                let role_ids = create_demo_roles(txn).await?;
                create_demo_users(txn, &role_ids, &hash).await
            })
        })
        .await
        .map_err(flatten_txn_err);

    match result {
        Ok(()) => println!(
            "\npagetop-user: demo data created ({ROLE_COUNT} roles, {USER_COUNT} users).\n  \
             password: {DEMO_PASSWORD}\n"
        ),
        Err(e) => eprintln!("pagetop-user demo-data error: failed to create demo data: {e}"),
    }
}

async fn create_demo_roles(txn: &DatabaseTransaction) -> Result<Vec<i32>, AuthError> {
    let now = Utc::now();
    let mut role_ids = Vec::with_capacity(ROLE_COUNT);
    for n in 1..=ROLE_COUNT {
        let new_role = role::ActiveModel {
            id: ActiveValue::NotSet,
            machine_name: Set(format!("demo_role_{n:02}")),
            label: Set(format!("Demo Role {n:02}")),
            description: Set(Some(
                "Rol de demostración generado por la feature demo-data.".into(),
            )),
            // Los pesos 0 y 1 los ocupan los roles de sistema (anonymous, authenticated).
            weight: Set(n as i32 + 1),
            locked: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
        };
        let result = role::Entity::insert(new_role).exec(txn).await?;
        role_ids.push(result.last_insert_id);
    }
    Ok(role_ids)
}

async fn create_demo_users(
    txn: &DatabaseTransaction,
    role_ids: &[i32],
    hash: &str,
) -> Result<(), AuthError> {
    let now = Utc::now();

    for n in 1..=USER_COUNT {
        let username =
            identity::username(&format!("demo_user_{n:02}"), SETTINGS.identity_policy, None)?;
        let email = identity::email(&format!("demo_user_{n:02}@example.com"))?;
        let mut new_user = user::ActiveModel {
            id: ActiveValue::NotSet,
            email_verified_at: Set(Some(now)),
            password_hash: Set(hash.to_owned()),
            status: Set(UserStatus::Active.as_i16()),
            language: Set(None),
            timezone: Set(None),
            theme: Set(None),
            display_name: Set(Some(format!("Demo User {n:02}"))),
            about: Set(None),
            last_login_at: Set(None),
            last_access_at: Set(None),
            failed_login_count: Set(0),
            locked_until: Set(None),
            is_admin: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        username.set_on(&mut new_user);
        email.set_on(&mut new_user);
        let result = user::Entity::insert(new_user).exec(txn).await?;
        let user_id = result.last_insert_id;

        // Reparte los usuarios de forma cíclica entre los roles de demostración.
        user_role::Entity::insert(user_role::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_ids[(n - 1) % role_ids.len()]),
        })
        .exec_without_returning(txn)
        .await?;
    }
    Ok(())
}
