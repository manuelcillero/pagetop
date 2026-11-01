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

// Hash que se guarda de un token ligado a `binding` (ver `create_bound_token()`). Sin `binding`, el
// de `hash_token()`. El token es base64url, sin saltos de línea: el separador no es ambiguo.
fn bound_hash(token: &str, binding: &str) -> String {
    if binding.is_empty() {
        hash_token(token)
    } else {
        hash_token(&util::join!(token, "\n", binding))
    }
}

// **< TokenKind >**********************************************************************************

/// Tipos de token que emite `pagetop-user`.
#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
    PasswordReset,
    EmailVerification,
    /// Confirma la nueva dirección pedida en el perfil (`pending_email`).
    EmailChange,
}

impl TokenKind {
    pub fn as_str(self) -> &'static str {
        match self {
            TokenKind::PasswordReset => "password_reset",
            TokenKind::EmailVerification => "email_verification",
            TokenKind::EmailChange => "email_change",
        }
    }

    /// TTL del token en segundos según su tipo.
    pub fn ttl_secs(self) -> i64 {
        match self {
            TokenKind::PasswordReset => 3600,          // 1 hora
            TokenKind::EmailVerification => 86400 * 3, // 3 días
            TokenKind::EmailChange => 86400,           // 1 día
        }
    }

    // Límite de tokens de este tipo que puede pedir una cuenta: cuántos (0 = sin límite) y en
    // cuántos segundos.
    fn limit(self) -> (u32, i64) {
        match self {
            TokenKind::PasswordReset => (
                SETTINGS.max_password_resets,
                SETTINGS.password_reset_window_secs,
            ),
            TokenKind::EmailChange => (
                SETTINGS.max_email_changes,
                SETTINGS.email_change_window_secs,
            ),
            TokenKind::EmailVerification => (0, 0),
        }
    }

    // Segundos que se conserva un token desde su creación: hasta que caduca y mientras cuenta para
    // el límite de su tipo.
    fn keep_secs(self) -> i64 {
        self.ttl_secs().max(self.limit().1)
    }
}

// **< create_token >*******************************************************************************

/// Crea un token en BD y devuelve el valor en claro para enviarlo al usuario.
///
/// Anula los tokens anteriores del mismo tipo que el usuario tenga sin consumir, de modo que sólo
/// vale el último enlace enviado. Falla con [`AuthError::TooManyRequests`] si el usuario ya ha
/// pedido todos los que admite el límite de su tipo:
/// [`max_password_resets`](crate::config::UserSettings::max_password_resets) en
/// [`password_reset_window_secs`](crate::config::UserSettings::password_reset_window_secs) los de
/// recuperación de contraseña, y
/// [`max_email_changes`](crate::config::UserSettings::max_email_changes) en
/// [`email_change_window_secs`](crate::config::UserSettings::email_change_window_secs) los de
/// cambio de email. Cuentan todos los creados en ese tiempo, consumidos o anulados también, pero no
/// los rechazados; los de verificación de email no tienen límite. Rechazado, los anteriores siguen
/// valiendo.
pub async fn create_token(user_id: i32, kind: TokenKind) -> Result<String, AuthError> {
    create_bound_token(user_id, kind, "").await
}

/// Como [`create_token()`], pero el token sólo vale junto con `binding` (p. ej. la dirección a la
/// que se envía): [`is_valid_bound_token()`] y [`consume_bound_token()`] lo rechazan con otro
/// valor. Así un enlace no sirve para lo que haya cambiado después de enviarlo.
pub async fn create_bound_token(
    user_id: i32,
    kind: TokenKind,
    binding: &str,
) -> Result<String, AuthError> {
    let now = Utc::now();
    let token = generate_token();
    let new_token = user_token::ActiveModel {
        id: ActiveValue::NotSet,
        user_id: Set(user_id),
        kind: Set(kind.as_str().to_owned()),
        token_hash: Set(bound_hash(&token, binding)),
        expires_at: Set(now + Duration::seconds(kind.ttl_secs())),
        consumed_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };
    let id = user_token::Entity::insert(new_token)
        .exec(dbconn())
        .await?
        .last_insert_id;

    // Se cuenta después de guardarlo y contándolo a él: si varias peticiones simultáneas guardan el
    // suyo antes de contar, la última en contar los ve todos, así que nunca se admiten más de las
    // que permite el límite (como mucho, se rechaza alguna de más). El rechazado se borra para que
    // no cuente.
    let (max, window_secs) = kind.limit();
    if max > 0 {
        let count = user_token::Entity::find()
            .filter(user_token::Column::UserId.eq(user_id))
            .filter(user_token::Column::Kind.eq(kind.as_str()))
            .filter(user_token::Column::CreatedAt.gte(now - Duration::seconds(window_secs)))
            .count(dbconn())
            .await?;
        if count > u64::from(max) {
            user_token::Entity::delete_by_id(id).exec(dbconn()).await?;
            return Err(AuthError::TooManyRequests);
        }
    }

    // Sólo los anteriores a éste: si dos peticiones simultáneas se anularan entre sí, no valdría
    // ningún enlace. Se anulan en lugar de borrarse para que sigan contando para el límite; sólo se
    // borran los que ya no pueden usarse ni cuentan.
    user_token::Entity::update_many()
        .col_expr(user_token::Column::ConsumedAt, Expr::value(now))
        .col_expr(user_token::Column::UpdatedAt, Expr::value(now))
        .filter(user_token::Column::UserId.eq(user_id))
        .filter(user_token::Column::Kind.eq(kind.as_str()))
        .filter(user_token::Column::ConsumedAt.is_null())
        .filter(user_token::Column::Id.lt(id))
        .exec(dbconn())
        .await?;
    user_token::Entity::delete_many()
        .filter(user_token::Column::UserId.eq(user_id))
        .filter(user_token::Column::Kind.eq(kind.as_str()))
        .filter(user_token::Column::CreatedAt.lt(now - Duration::seconds(kind.keep_secs())))
        .exec(dbconn())
        .await?;
    Ok(token)
}

/// Anula los tokens del tipo indicado que el usuario tenga sin consumir: sus enlaces dejan de
/// valer.
pub async fn revoke_tokens(user_id: i32, kind: TokenKind) -> Result<(), AuthError> {
    let now = Utc::now();
    user_token::Entity::update_many()
        .col_expr(user_token::Column::ConsumedAt, Expr::value(now))
        .col_expr(user_token::Column::UpdatedAt, Expr::value(now))
        .filter(user_token::Column::UserId.eq(user_id))
        .filter(user_token::Column::Kind.eq(kind.as_str()))
        .filter(user_token::Column::ConsumedAt.is_null())
        .exec(dbconn())
        .await?;
    Ok(())
}

// **< is_valid_token / consume_token >*************************************************************

/// Comprueba, sin consumirlo, que el token en claro sea válido para el usuario: existe, es del tipo
/// indicado, no ha caducado y no se ha consumido.
pub async fn is_valid_token(user_id: i32, token: &str, kind: TokenKind) -> Result<bool, AuthError> {
    is_valid_bound_token(user_id, token, kind, "").await
}

/// Como [`is_valid_token()`], para un token creado con [`create_bound_token()`] y `binding`.
pub async fn is_valid_bound_token(
    user_id: i32,
    token: &str,
    kind: TokenKind,
    binding: &str,
) -> Result<bool, AuthError> {
    let row = user_token::Entity::find()
        .filter(user_token::Column::TokenHash.eq(bound_hash(token, binding)))
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
    consume_bound_token(user_id, token, kind, "").await
}

/// Como [`consume_token()`], para un token creado con [`create_bound_token()`] y `binding`.
pub async fn consume_bound_token(
    user_id: i32,
    token: &str,
    kind: TokenKind,
    binding: &str,
) -> Result<(), AuthError> {
    let now = Utc::now();
    let result = user_token::Entity::update_many()
        .col_expr(user_token::Column::ConsumedAt, Expr::value(now))
        .col_expr(user_token::Column::UpdatedAt, Expr::value(now))
        .filter(user_token::Column::TokenHash.eq(bound_hash(token, binding)))
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
