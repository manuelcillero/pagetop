//! Datos de demostración: 25 roles y 48 usuarios ficticios para explorar la administración.
//!
//! Sólo se compila con la feature `demo-data`. Se ejecuta una vez desde
//! `Extension::initialize()`, después de `auth::seed_initial_data()`. Si el rol
//! `demo_role_01` ya existe, la siembra se omite para no duplicar datos en reinicios.

use pagetop::datetime::Utc;

use pagetop_seaorm::db::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter, Set, dbconn};

use crate::account::UserStatus;
use crate::auth;
use crate::entity::{role, user};
use crate::error::AuthError;
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

    let role_ids = match create_demo_roles().await {
        Ok(ids) => ids,
        Err(e) => {
            eprintln!(
                "pagetop-user demo-data error: failed to create roles: {}",
                e
            );
            return;
        }
    };

    match create_demo_users(&role_ids).await {
        Ok(()) => println!(
            "\npagetop-user: demo data created ({ROLE_COUNT} roles, {USER_COUNT} users).\n  \
             password: {DEMO_PASSWORD}\n"
        ),
        Err(e) => eprintln!(
            "pagetop-user demo-data error: failed to create users: {}",
            e
        ),
    }
}

async fn create_demo_roles() -> Result<Vec<i32>, AuthError> {
    let now = Utc::now().naive_utc();
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
        let result = role::Entity::insert(new_role).exec(dbconn()).await?;
        role_ids.push(result.last_insert_id);
    }
    Ok(role_ids)
}

async fn create_demo_users(role_ids: &[i32]) -> Result<(), AuthError> {
    let hash = password::hash_password(DEMO_PASSWORD)?;
    let now = Utc::now().naive_utc();

    for n in 1..=USER_COUNT {
        let new_user = user::ActiveModel {
            id: ActiveValue::NotSet,
            username: Set(format!("demo_user_{n:02}")),
            email: Set(format!("demo_user_{n:02}@example.com")),
            email_verified_at: Set(Some(now)),
            password_hash: Set(hash.clone()),
            status: Set(UserStatus::Active.as_i16()),
            language: Set(None),
            timezone: Set(None),
            display_name: Set(Some(format!("Demo User {n:02}"))),
            last_login_at: Set(None),
            last_access_at: Set(None),
            failed_login_count: Set(0),
            locked_until: Set(None),
            is_admin: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
        };
        let result = user::Entity::insert(new_user).exec(dbconn()).await?;
        let user_id = result.last_insert_id;

        auth::assign_role(user_id, crate::AUTHENTICATED_ROLE_ID).await?;

        // Reparte los usuarios de forma cíclica entre los roles de demostración.
        let role_id = role_ids[(n - 1) % role_ids.len()];
        auth::assign_role(user_id, role_id).await?;
    }
    Ok(())
}
