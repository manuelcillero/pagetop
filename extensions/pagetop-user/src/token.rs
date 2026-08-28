//! Generación y verificación de tokens de un solo uso (reset de contraseña, verificación de
//! email...).

use pagetop::datetime::{Duration, Utc};
use sha2::{Digest, Sha256};

use pagetop_seaorm::db::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, Set, dbconn,
};

use crate::entity::user_token;
use crate::error::AuthError;

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
}

// **< create_token >*******************************************************************************

/// Crea un token en BD y devuelve el valor en claro para enviarlo al usuario.
///
/// Si el usuario ya tiene un token del mismo tipo vigente, lo reemplaza.
pub async fn create_token(user_id: i32, kind: TokenKind) -> Result<String, AuthError> {
    // Invalidar tokens anteriores del mismo tipo para este usuario.
    user_token::Entity::delete_many()
        .filter(user_token::Column::UserId.eq(user_id))
        .filter(user_token::Column::Kind.eq(kind.as_str()))
        .exec(dbconn())
        .await?;

    let token = generate_token();
    let now = Utc::now().naive_utc();
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

// **< consume_token >******************************************************************************

/// Verifica que el token en claro sea válido (existe, no ha expirado y no ha sido consumido)
/// y lo marca como consumido. Devuelve el `user_id` asociado.
pub async fn consume_token(token: &str, kind: TokenKind) -> Result<i32, AuthError> {
    let hash = hash_token(token);
    let now = Utc::now().naive_utc();

    let row = user_token::Entity::find()
        .filter(user_token::Column::TokenHash.eq(&hash))
        .filter(user_token::Column::Kind.eq(kind.as_str()))
        .one(dbconn())
        .await?
        .ok_or(AuthError::InvalidToken)?;

    if row.consumed_at.is_some() || row.expires_at < now {
        return Err(AuthError::InvalidToken);
    }

    let user_id = row.user_id;
    let mut active: user_token::ActiveModel = row.into();
    active.consumed_at = Set(Some(now));
    active.updated_at = Set(now);
    active.update(dbconn()).await?;

    Ok(user_id)
}
