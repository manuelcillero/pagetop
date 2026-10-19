//! Nombre de usuario y email que identifican una cuenta: normalización y validación.
//!
//! Las reglas públicas se documentan en `IdentityPolicy`. Aquí se separan dos cosas:
//!
//! - **Buscar y comparar**, igual con cualquier política: `*_key` (sin espacios en los extremos,
//!   NFKC en el nombre de usuario y en minúsculas) y el esqueleto de confusión del nombre
//!   (UTS #39), guardados en columnas propias con índice único. Así se puede cambiar de política
//!   sin que dejen de encontrarse las cuentas existentes.
//!
//!   Del esqueleto se guarda su *hash* SHA-256: sólo se compara por igualdad, y puede ser mucho
//!   más largo que el nombre (cada sílaba coreana se descompone en hasta siete caracteres), así
//!   que no cabría en una columna de longitud fija con índice único.
//! - **Aceptar un nombre nuevo**, lo único que depende de la política.

use pagetop::prelude::*;
use pagetop_mail::Mailbox;
use pagetop_seaorm::db::Set;

use unicode_ident::{is_xid_continue, is_xid_start};
use unicode_normalization::UnicodeNormalization;
use unicode_security::{GeneralSecurityProfile, MixedScript, skeleton};

use sha2::{Digest, Sha256};

use crate::LOCALES_USER;
use crate::config::IdentityPolicy;
use crate::entity::user;
use crate::error::AuthError;

// Longitud mínima del nombre de usuario, con cualquier política. `u16` porque también limita el
// campo del formulario (`minlength`), igual que las máximas de `username_max_chars()`.
pub(crate) const USERNAME_MIN_CHARS: u16 = 3;

// Longitud máxima de una dirección de correo, la que admite SMTP.
pub(crate) const EMAIL_MAX_CHARS: u16 = 254;

// Longitud máxima del nombre de usuario según la política. La de `Permissive` es la de Drupal.
pub(crate) fn username_max_chars(policy: IdentityPolicy) -> u16 {
    match policy {
        IdentityPolicy::Strict | IdentityPolicy::Unicode => 32,
        IdentityPolicy::Permissive => 60,
    }
}

// **< Username >***********************************************************************************

// Nombre de usuario listo para guardar: el valor tal como se escribió (normalizado), su clave de
// búsqueda y el *hash* de su esqueleto de confusión.
#[derive(Debug, Getters)]
pub(crate) struct Username {
    value: String,
    key: String,
    skeleton: String,
}

impl Username {
    // Guarda el nombre de usuario en el modelo.
    pub(crate) fn set_on(self, model: &mut user::ActiveModel) {
        model.username = Set(self.value);
        model.username_key = Set(self.key);
        model.username_skeleton = Set(self.skeleton);
    }
}

// Normaliza y valida un nombre de usuario con la política dada.
//
// `current_key` es la clave del nombre actual de la cuenta, si se está editando: un nombre que no
// cambia (salvo en mayúsculas) no se valida de nuevo, para que una cuenta creada con una política
// más abierta pueda seguir guardando su perfil.
pub(crate) fn username(
    raw: &str,
    policy: IdentityPolicy,
    current_key: Option<&str>,
) -> Result<Username, AuthError> {
    let value = raw.trim().nfkc().collect::<String>();
    let key = value.to_lowercase();
    if current_key != Some(key.as_str()) && !accepts(&value, policy) {
        return Err(AuthError::InvalidUsername);
    }
    let skeleton = skeleton_hash(&key);
    Ok(Username {
        value,
        key,
        skeleton,
    })
}

// Clave para buscar una cuenta por nombre de usuario con lo que escribe el usuario, que no se
// valida: una entrada no válida simplemente no encuentra ninguna cuenta.
pub(crate) fn username_key(raw: &str) -> String {
    raw.trim().nfkc().collect::<String>().to_lowercase()
}

// *Hash* SHA-256, en hexadecimal, del esqueleto de confusión (UTS #39) de la clave del nombre.
fn skeleton_hash(key: &str) -> String {
    let digest = Sha256::digest(skeleton(key).collect::<String>().as_bytes());
    let mut hex = String::with_capacity(64);
    for byte in digest {
        hex.push(char::from(b"0123456789abcdef"[usize::from(byte >> 4)]));
        hex.push(char::from(b"0123456789abcdef"[usize::from(byte & 0x0f)]));
    }
    hex
}

// Si la política acepta el nombre, ya normalizado.
//
// Nunca se admite `@`: un nombre de usuario no puede coincidir con el email de otra cuenta, que
// también sirve para iniciar sesión. Con `Unicode` y `Permissive`, las letras y dígitos son las
// clases que definen los identificadores de Rust (XID, UAX #31), que incluyen las marcas
// combinables de escrituras como el devanagari; `identifier_allowed()` descarta los caracteres
// que UTS #39 desaconseja (invisibles, obsoletos...) y `is_single_script()` la mezcla de alfabetos.
fn accepts(name: &str, policy: IdentityPolicy) -> bool {
    let symbols: &[char] = match policy {
        IdentityPolicy::Strict | IdentityPolicy::Unicode => &['.', '_', '-'],
        IdentityPolicy::Permissive => &['.', '_', '-', ' ', '\'', '+'],
    };
    let length = usize::from(USERNAME_MIN_CHARS)..=usize::from(username_max_chars(policy));
    if !length.contains(&name.chars().count()) {
        return false;
    }
    let Some(first) = name.chars().next() else {
        return false;
    };
    match policy {
        IdentityPolicy::Strict => {
            first.is_ascii_alphanumeric()
                && name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || symbols.contains(&c))
        }
        IdentityPolicy::Unicode | IdentityPolicy::Permissive => {
            let letter_or_digit = |c: char| {
                c.identifier_allowed()
                    && (is_xid_start(c) || (is_xid_continue(c) && c.is_numeric()))
            };
            letter_or_digit(first)
                && name
                    .chars()
                    .all(|c| symbols.contains(&c) || (c.identifier_allowed() && is_xid_continue(c)))
                && name.is_single_script()
                && !name.contains("  ")
        }
    }
}

// **< Email >**************************************************************************************

// Email listo para guardar: el valor tal como se escribió y su clave de búsqueda.
#[derive(Debug, Getters)]
pub(crate) struct Email {
    value: String,
    key: String,
}

impl Email {
    // Guarda el email en el modelo.
    pub(crate) fn set_on(self, model: &mut user::ActiveModel) {
        model.email = Set(self.value);
        model.email_key = Set(self.key);
    }
}

// Normaliza y valida un email, igual con cualquier política: el formato que exige `pagetop-mail`
// para poder enviarle correos. Se guarda tal como se escribe; la clave en minúsculas evita cuentas
// duplicadas por mayúsculas (la parte local las distingue según el estándar, pero ningún
// proveedor real lo aplica).
pub(crate) fn email(raw: &str) -> Result<Email, AuthError> {
    let value = raw.trim().to_owned();
    let key = email_key(&value);
    // También la clave: en minúsculas, algunas letras ocupan dos caracteres (`İ` -> `i̇`).
    let max = usize::from(EMAIL_MAX_CHARS);
    if value.chars().count() > max
        || key.chars().count() > max
        || Mailbox::new(value.as_str()).validate().is_err()
    {
        return Err(AuthError::InvalidEmail);
    }
    Ok(Email { key, value })
}

// Clave para buscar una cuenta por email con lo que escribe el usuario, sin validarlo.
pub(crate) fn email_key(raw: &str) -> String {
    raw.trim().to_lowercase()
}

// **< Textos >*************************************************************************************

// Reglas del nombre de usuario de la política, como texto de ayuda de los formularios.
pub(crate) fn username_help(policy: IdentityPolicy) -> Lc {
    let key = match policy {
        IdentityPolicy::Strict => "help-username-strict",
        IdentityPolicy::Unicode => "help-username-unicode",
        IdentityPolicy::Permissive => "help-username-permissive",
    };
    Lc::t(key, &LOCALES_USER)
        .with_number("min", u32::from(USERNAME_MIN_CHARS))
        .with_number("max", u32::from(username_max_chars(policy)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accepted(raw: &str, policy: IdentityPolicy) -> bool {
        username(raw, policy, None).is_ok()
    }

    fn skeleton_of(raw: &str) -> String {
        username(raw, IdentityPolicy::Permissive, Some(&username_key(raw)))
            .unwrap()
            .skeleton
    }

    const ALWAYS_REJECTED: &[&str] = &[
        "ab",
        "_ana",
        ".ana",
        "ana@example.com",
        "ana@casa",
        "<a>",
        "a\u{200B}na", // Zero-width space.
        "ana\u{202E}", // Right-to-left override.
    ];

    #[pagetop::test]
    async fn strict_accepts_only_ascii_identifiers() {
        let name = username("  Ana.Perez_01 ", IdentityPolicy::Strict, None).unwrap();
        assert_eq!(name.value, "Ana.Perez_01", "the value keeps its case");
        assert_eq!(name.key, "ana.perez_01");
        assert!(accepted("a-b", IdentityPolicy::Strict));
        // Full-width letters are normalized to ASCII (NFKC).
        assert_eq!(
            username("Ａｎａ", IdentityPolicy::Strict, None)
                .unwrap()
                .value,
            "Ana"
        );
        let too_long = "a".repeat(33);
        for name in
            ALWAYS_REJECTED
                .iter()
                .copied()
                .chain(["peña", "аna", "ana perez", too_long.as_str()])
        {
            assert!(
                !accepted(name, IdentityPolicy::Strict),
                "{name:?} must be rejected"
            );
        }
    }

    #[pagetop::test]
    async fn unicode_accepts_any_single_script() {
        for name in [
            "josé",
            "peña",
            "Müller",
            "σοφία",
            "Иван",
            "山田太郎",
            "山田やまだ",
            "नमस्ते", // Devanagari, with combining vowel signs.
            "محمد",
            "ana_01",
        ] {
            assert!(
                accepted(name, IdentityPolicy::Unicode),
                "{name:?} must be accepted"
            );
        }
        // A combining accent is composed (NFKC): both spellings are the same user.
        assert_eq!(username_key("jose\u{301}"), username_key("josé"));
        let too_long = "a".repeat(33);
        for name in ALWAYS_REJECTED.iter().copied().chain([
            "pаypal",
            "ana perez",
            "O'Brien",
            too_long.as_str(),
        ]) {
            assert!(
                !accepted(name, IdentityPolicy::Unicode),
                "{name:?} must be rejected"
            );
        }
    }

    #[pagetop::test]
    async fn permissive_adds_spaces_apostrophes_and_plus() {
        for name in ["Ana Pérez", "O'Brien", "ana+test", "José María"] {
            assert!(
                accepted(name, IdentityPolicy::Permissive),
                "{name:?} must be accepted"
            );
        }
        let too_long = "a".repeat(61);
        for name in
            ALWAYS_REJECTED
                .iter()
                .copied()
                .chain(["ana  perez", "pаypal", too_long.as_str()])
        {
            assert!(
                !accepted(name, IdentityPolicy::Permissive),
                "{name:?} must be rejected"
            );
        }
    }

    #[pagetop::test]
    async fn confusable_names_share_their_skeleton() {
        assert_eq!(skeleton_of("admin"), skeleton_of("аdmin")); // Cyrillic "а".
        assert_eq!(skeleton_of("maria"), skeleton_of("rnaria"));
        assert_eq!(skeleton_of("Ana"), skeleton_of("ana"));
        assert_ne!(skeleton_of("ana"), skeleton_of("eva"));
    }

    // Each Hangul syllable expands to up to seven characters in the skeleton: only its hash fits
    // the column, whatever the name.
    #[pagetop::test]
    async fn longest_names_fit_their_columns() {
        let name = username(&"꽦".repeat(60), IdentityPolicy::Permissive, None).unwrap();
        assert_eq!(name.skeleton.len(), 64);
        assert!(name.key.chars().count() <= 255);
    }

    #[pagetop::test]
    async fn unchanged_names_skip_the_policy() {
        let current = username_key("Ana Pérez");
        assert!(username("ana pérez", IdentityPolicy::Strict, Some(&current)).is_ok());
        assert!(username("Eva Pérez", IdentityPolicy::Strict, Some(&current)).is_err());
    }

    #[pagetop::test]
    async fn emails_keep_their_case_and_compare_without_it() {
        let email = email(" Ana@Example.COM ").unwrap();
        assert_eq!(email.value, "Ana@Example.COM");
        assert_eq!(email.key, "ana@example.com");
        assert_eq!(email_key("ANA@example.com "), email.key);
        let too_long = format!("{}@example.com", "a".repeat(250));
        for invalid in [
            "",
            "ana",
            "ana@",
            "Ana <ana@example.com>",
            too_long.as_str(),
        ] {
            assert!(
                matches!(super::email(invalid), Err(AuthError::InvalidEmail)),
                "{invalid:?} must be rejected"
            );
        }
    }
}
