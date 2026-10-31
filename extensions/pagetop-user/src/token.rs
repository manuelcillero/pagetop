//! Generación y verificación de tokens de un solo uso (reset de contraseña, verificación de
//! email...).

use pagetop::prelude::*;
use pagetop_seaorm::db::query::Expr;
use pagetop_seaorm::db::{
    ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set, dbconn,
};

use crate::config::SETTINGS;
use crate::entity::user_token;
use crate::error::AuthError;

use sha2::{Digest, Sha256};

// **< Generación de tokens >***********************************************************************

/// Genera un token URL-safe de 43 caracteres (32 bytes -> base64url sin padding).
pub fn generate_token() -> String {
    use rand_core::{OsRng, RngCore};
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    // base64url sin padding (a=, == al final) para usar en URLs de forma segura.
    use base64ct::{Base64UrlUnpadded, Encoding};
    Base64UrlUnpadded::encode_string(&bytes)
}

/// Calcula el hash SHA-256 de un token y lo devuelve en hexadecimal (64 chars).
///
/// El hash es lo que se almacena en BD; el token en claro se envía al usuario por email.
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let digest = hasher.finalize();
    // Convertir a hex sin depender de crates adicionales.
    use std::fmt::Write;
    let mut s = String::with_capacity(64);
    for b in digest {
        write!(s, "{:02x}", b).unwrap();
    }
    s
}

// **< TokenKind >**********************************************************************************

/// Tipos de token que emite `pagetop-user`.
#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
    PasswordReset,
    EmailVerification,
}

impl TokenKind {
    pub fn as_str(self) -> &'static str {
        match self {
            TokenKind::PasswordReset => "password_reset",
            TokenKind::EmailVerification => "email_verification",
        }
    }

    /// TTL del token en segundos según su tipo.
    pub fn ttl_secs(self) -> i64 {
        match self {
            TokenKind::PasswordReset => 3600,          // 1 hora
            TokenKind::EmailVerification => 86400 * 3, // 3 días
        }
    }

    // Segundos que se conserva un token desde su creación: hasta que caduca y, los de recuperación,
    // mientras cuentan para el límite de solicitudes.
    fn keep_secs(self) -> i64 {
        match self {
            TokenKind::PasswordReset => self.ttl_secs().max(SETTINGS.password_reset_window_secs),
            TokenKind::EmailVerification => self.ttl_secs(),
        }
    }
}

// **< create_token >*******************************************************************************

/// Crea un token en BD y devuelve el valor en claro para enviarlo al usuario.
///
/// Anula los tokens del mismo tipo que el usuario tenga sin consumir, de modo que sólo vale el
/// último enlace enviado.
pub async fn create_token(user_id: i32, kind: TokenKind) -> Result<String, AuthError> {
    let now = Utc::now();

    // Los anteriores se anulan en lugar de borrarse para que sigan contando en
    // `count_tokens_since()`; sólo se borran los que ya no pueden usarse ni cuentan.
    user_token::Entity::update_many()
        .col_expr(user_token::Column::ConsumedAt, Expr::value(now))
        .col_expr(user_token::Column::UpdatedAt, Expr::value(now))
        .filter(user_token::Column::UserId.eq(user_id))
        .filter(user_token::Column::Kind.eq(kind.as_str()))
        .filter(user_token::Column::ConsumedAt.is_null())
        .exec(dbconn())
        .await?;
    user_token::Entity::delete_many()
        .filter(user_token::Column::UserId.eq(user_id))
        .filter(user_token::Column::Kind.eq(kind.as_str()))
        .filter(user_token::Column::CreatedAt.lt(now - Duration::seconds(kind.keep_secs())))
        .exec(dbconn())
        .await?;

    let token = generate_token();
    let expires_at = now + Duration::seconds(kind.ttl_secs());

    let new_token = user_token::ActiveModel {
        id: ActiveValue::NotSet,
        user_id: Set(user_id),
        kind: Set(kind.as_str().to_owned()),
        token_hash: Set(hash_token(&token)),
        expires_at: Set(expires_at),
        consumed_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };
    user_token::Entity::insert(new_token).exec(dbconn()).await?;
    Ok(token)
}

/// Cuenta los tokens del tipo indicado creados para el usuario desde `since`, consumidos o no.
///
/// Los tokens se conservan al menos durante su tiempo de vida o, en los de recuperación de
/// contraseña, durante
/// [`password_reset_window_secs`](crate::config::UserSettings::password_reset_window_secs) si es
/// mayor. Con un `since` más antiguo el recuento puede quedarse corto.
pub async fn count_tokens_since(
    user_id: i32,
    kind: TokenKind,
    since: DateTime<Utc>,
) -> Result<u64, AuthError> {
    Ok(user_token::Entity::find()
        .filter(user_token::Column::UserId.eq(user_id))
        .filter(user_token::Column::Kind.eq(kind.as_str()))
        .filter(user_token::Column::CreatedAt.gte(since))
        .count(dbconn())
        .await?)
}

// **< is_valid_token / consume_token >*************************************************************

/// Comprueba, sin consumirlo, que el token en claro sea válido para el usuario: existe, es del tipo
/// indicado, no ha caducado y no se ha consumido.
pub async fn is_valid_token(user_id: i32, token: &str, kind: TokenKind) -> Result<bool, AuthError> {
    let row = user_token::Entity::find()
        .filter(user_token::Column::TokenHash.eq(hash_token(token)))
        .filter(user_token::Column::UserId.eq(user_id))
        .filter(user_token::Column::Kind.eq(kind.as_str()))
        .filter(user_token::Column::ConsumedAt.is_null())
        .filter(user_token::Column::ExpiresAt.gt(Utc::now()))
        .one(dbconn())
        .await?;
    Ok(row.is_some())
}

/// Consume el token en claro del usuario si es válido (ver [`is_valid_token()`]).
///
/// Falla con [`AuthError::InvalidToken`] si no lo es. Se marca como consumido en una sola
/// sentencia condicionada a que siga sin consumir: dos peticiones simultáneas con el mismo enlace
/// no pueden usarlo las dos.
pub async fn consume_token(user_id: i32, token: &str, kind: TokenKind) -> Result<(), AuthError> {
    let now = Utc::now();
    let result = user_token::Entity::update_many()
        .col_expr(user_token::Column::ConsumedAt, Expr::value(now))
        .col_expr(user_token::Column::UpdatedAt, Expr::value(now))
        .filter(user_token::Column::TokenHash.eq(hash_token(token)))
        .filter(user_token::Column::UserId.eq(user_id))
        .filter(user_token::Column::Kind.eq(kind.as_str()))
        .filter(user_token::Column::ConsumedAt.is_null())
        .filter(user_token::Column::ExpiresAt.gt(now))
        .exec(dbconn())
        .await?;
    if result.rows_affected == 1 {
        Ok(())
    } else {
        Err(AuthError::InvalidToken)
    }
}
