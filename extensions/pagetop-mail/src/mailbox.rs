//! Direcciones de correo con nombre opcional ([`Mailbox`]).

use pagetop::prelude::*;

use crate::MailError;

use std::fmt;

/// Dirección de correo, con el nombre opcional de su titular.
///
/// Se crea a partir de una cadena con la dirección sola (`"ana@example.com"`) o con nombre
/// (`"Ana Pérez <ana@example.com>"`, o con el nombre entre comillas), o de una tupla
/// `(nombre, dirección)`. Ésta última es la forma segura cuando el nombre procede de datos de
/// usuario, porque puede contener comas, comillas u otros caracteres especiales.
///
/// La dirección no se valida al crearla, sino al enviar el correo, que falla con
/// [`MailError::InvalidAddress`] si no es válida.
///
/// # Ejemplos
///
/// ```rust
/// # use pagetop_mail::prelude::*;
/// let ana = Mailbox::from("Ana Pérez <ana@example.com>");
/// assert_eq!(ana.name(), Some(&"Ana Pérez".to_string()));
/// assert_eq!(ana.address(), "ana@example.com");
///
/// let luis = Mailbox::from(("Luis, el de soporte", "luis@example.com"));
/// assert_eq!(luis.name(), Some(&"Luis, el de soporte".to_string()));
///
/// let anon = Mailbox::from("no-reply@example.com");
/// assert_eq!(anon.name(), None);
/// ```
#[derive(AutoDefault, Clone, Debug, Eq, Getters, PartialEq)]
pub struct Mailbox {
    /// Nombre del titular, si lo tiene.
    name: Option<String>,
    /// Dirección de correo.
    address: String,
}

impl Mailbox {
    /// Crea una dirección de correo sin nombre.
    pub fn new(address: impl Into<String>) -> Self {
        Mailbox {
            name: None,
            address: address.into().trim().to_owned(),
        }
    }

    /// Crea una dirección de correo con el nombre de su titular. Un nombre vacío equivale a no
    /// indicarlo, y los caracteres de control del nombre (saltos de línea, tabuladores...) se
    /// cambian por espacios.
    pub fn with_name(name: impl Into<String>, address: impl Into<String>) -> Self {
        // `lettre` provoca un `panic!` al componer una cabecera con un nombre que tenga caracteres
        // de control, y el nombre puede proceder de datos de usuario.
        let name: String = name
            .into()
            .chars()
            .map(|c| if c.is_control() { ' ' } else { c })
            .collect();
        Mailbox {
            name: util::non_blank(&name).map(str::to_owned),
            ..Mailbox::new(address)
        }
    }

    /// Comprueba que la dirección sea válida, con el mismo criterio que se aplica al enviar.
    ///
    /// Sirve para validar una dirección al recibirla (p. ej. de un formulario), antes de guardarla.
    /// Falla con [`MailError::InvalidAddress`] si no es válida.
    ///
    /// ```rust
    /// # use pagetop_mail::prelude::*;
    /// assert!(Mailbox::new("ana@example.com").validate().is_ok());
    /// assert!(Mailbox::new("ana.example.com").validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), MailError> {
        self.to_lettre().map(drop)
    }

    // Convierte la dirección al tipo de `lettre`, validándola.
    pub(crate) fn to_lettre(&self) -> Result<lettre::message::Mailbox, MailError> {
        let address = self.address.parse::<lettre::Address>().map_err(|source| {
            MailError::InvalidAddress {
                address: self.address.clone(),
                source,
            }
        })?;
        Ok(lettre::message::Mailbox::new(self.name.clone(), address))
    }
}

impl From<&str> for Mailbox {
    fn from(value: &str) -> Self {
        let value = value.trim();
        // "Nombre <dirección>": la dirección es lo que va entre los últimos `<` y `>`.
        if let Some(rest) = value.strip_suffix('>')
            && let Some((name, address)) = rest.rsplit_once('<')
        {
            let name = name.trim();
            let name = name
                .strip_prefix('"')
                .and_then(|n| n.strip_suffix('"'))
                .unwrap_or(name);
            return Mailbox::with_name(name, address);
        }
        Mailbox::new(value)
    }
}

impl From<String> for Mailbox {
    fn from(value: String) -> Self {
        Mailbox::from(value.as_str())
    }
}

impl From<&String> for Mailbox {
    fn from(value: &String) -> Self {
        Mailbox::from(value.as_str())
    }
}

impl<N: Into<String>, A: Into<String>> From<(N, A)> for Mailbox {
    fn from((name, address): (N, A)) -> Self {
        Mailbox::with_name(name, address)
    }
}

/// Muestra la dirección como `Nombre <dirección>`, o sólo la dirección si no tiene nombre. Es una
/// representación para trazas y pruebas, no la codificación de las cabeceras del correo.
impl fmt::Display for Mailbox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{name} <{}>", self.address),
            None => f.write_str(&self.address),
        }
    }
}
