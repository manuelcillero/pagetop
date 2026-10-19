//! Hashing de contraseñas con Argon2id.

use argon2::{
    Argon2, ParamsBuilder, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::SaltString,
};
use rand_core::OsRng;

use std::sync::LazyLock;

use crate::config::SETTINGS;
use crate::error::AuthError;

/// Genera el hash PHC de una contraseña usando Argon2id con los parámetros configurados.
pub fn hash_password(plain: &str) -> Result<String, AuthError> {
    let params = ParamsBuilder::new()
        .m_cost(SETTINGS.password.argon2_m_cost)
        .t_cost(SETTINGS.password.argon2_t_cost)
        .p_cost(SETTINGS.password.argon2_p_cost)
        .build()
        .map_err(|e| AuthError::PasswordHash(e.to_string()))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(plain.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AuthError::PasswordHash(e.to_string()))
}

/// Verifica que `plain` corresponde al hash PHC almacenado.
///
/// Devuelve `false` si el hash está malformado o la contraseña no coincide.
pub fn verify_password(plain: &str, phc: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(phc) else {
        return false;
    };
    Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok()
}

// Versiones de `hash_password()` y `verify_password()` para las peticiones: el cálculo se hace en
// el *pool* de hilos bloqueantes de Tokio. Con los costes por defecto tarda decenas de milisegundos,
// que retendrían un hilo del *runtime* asíncrono y retrasarían las demás peticiones (p. ej. durante
// un intento de fuerza bruta contra el inicio de sesión).
pub(crate) async fn hash_password_async(plain: &str) -> Result<String, AuthError> {
    let plain = plain.to_owned();
    tokio::task::spawn_blocking(move || hash_password(&plain))
        .await
        .map_err(|err| AuthError::PasswordHash(err.to_string()))?
}

pub(crate) async fn verify_password_async(plain: &str, phc: &str) -> bool {
    let (plain, phc) = (plain.to_owned(), phc.to_owned());
    tokio::task::spawn_blocking(move || verify_password(&plain, &phc))
        .await
        .unwrap_or(false)
}

// Verifica la contraseña contra un *hash* que no corresponde a ninguna cuenta. Cuando no existe la
// cuenta, iguala el tiempo de respuesta al de una contraseña errónea: si no, la diferencia
// revelaría qué cuentas existen.
pub(crate) async fn verify_password_without_account(plain: &str) {
    static DUMMY_HASH: LazyLock<String> =
        LazyLock::new(|| hash_password("pagetop-user-dummy-password").unwrap_or_default());
    verify_password_async(plain, DUMMY_HASH.as_str()).await;
}

/// Indica si el hash necesita actualizarse: no es de Argon2id o se cifró con otros parámetros de
/// coste que los configurados.
pub fn needs_rehash(phc: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(phc) else {
        return true;
    };
    let cost = |name: &str| parsed.params.get_decimal(name);
    let password = &SETTINGS.password;
    parsed.algorithm != argon2::Algorithm::Argon2id.ident()
        || cost("m") != Some(password.argon2_m_cost)
        || cost("t") != Some(password.argon2_t_cost)
        || cost("p") != Some(password.argon2_p_cost)
}

/// Valida la longitud mínima, en caracteres, y devuelve error si no se cumple.
pub fn validate_strength(plain: &str) -> Result<(), AuthError> {
    let min = SETTINGS.password.min_length;
    if plain.chars().count() < min {
        Err(AuthError::PasswordTooShort(min))
    } else {
        Ok(())
    }
}

/// Comprueba que `password` y `confirm_password` coinciden.
pub fn passwords_match(password: &str, confirm_password: &str) -> Result<(), AuthError> {
    if password == confirm_password {
        Ok(())
    } else {
        Err(AuthError::PasswordMismatch)
    }
}
