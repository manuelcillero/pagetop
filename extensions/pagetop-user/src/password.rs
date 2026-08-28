//! Hashing de contraseñas con Argon2id.

use argon2::{
    Argon2, ParamsBuilder, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::SaltString,
};
use rand_core::OsRng;

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

/// Indica si el hash necesita actualizarse (parámetros de coste han cambiado).
pub fn needs_rehash(phc: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(phc) else {
        return true;
    };
    // Compara el coste de memoria con el configurado.
    if let Some(m_cost) = parsed.params.get_str("m") {
        let current: u32 = m_cost.parse().unwrap_or(0);
        if current != SETTINGS.password.argon2_m_cost {
            return true;
        }
    }
    false
}

/// Valida la longitud mínima y devuelve error si no se cumple.
pub fn validate_strength(plain: &str) -> Result<(), AuthError> {
    let min = SETTINGS.password.min_length;
    if plain.len() < min {
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
