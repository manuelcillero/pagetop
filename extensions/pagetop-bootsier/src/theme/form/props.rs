use pagetop::prelude::*;

use std::borrow::Cow;
use std::fmt;

// **< Autocomplete >*******************************************************************************

/// Valor del atributo HTML `autocomplete`.
///
/// Según la [especificación](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#autofill)
/// oficial este valor puede ser:
///
/// - `on` / `off`, o
/// - una lista ordenada de tokens predefinidos separados por espacios.
///
/// Las variantes de `Autocomplete` permiten:
///
/// - Generar valores canónicos `on`/`off` ([`Autocomplete::On`], [`Autocomplete::Off`]).
/// - Generar una lista de tokens en formato texto ([`Autocomplete::Custom`]). Los valores creados
///   mediante [`Autocomplete::custom()`] se normalizan con [`util::normalize_ascii_or_empty()`].
///
/// Las entradas no válidas que lleguen a [`Autocomplete::custom()`] se degradan a
/// [`Autocomplete::On`] (valor canónico y seguro).
#[derive(Clone, Debug, PartialEq)]
pub enum Autocomplete {
    /// Genera `autocomplete="on"`.
    On,
    /// Genera `autocomplete="off"`.
    Off,
    /// Genera un valor personalizado (se espera en formato canónico).
    ///
    /// Normalmente contiene una lista de tokens separados por espacios (p. ej. `"username"` o
    /// `"username webauthn"`).
    Custom(CowStr),
}

impl Autocomplete {
    // --< Field token >----------------------------------------------------------------------------

    /// Genera `autocomplete="<field>"` usando un campo predefinido.
    #[inline]
    pub fn field(field: AutofillField) -> Self {
        Self::Custom(Cow::Borrowed(field.as_str()))
    }

    // --< Sections >-------------------------------------------------------------------------------

    /// Construye `autocomplete` usando un nombre de sección y un campo predefinido.
    ///
    /// Genera `autocomplete="section-<name> <field>"`.
    ///
    /// Si `name` contiene espacios tras normalizar con [`util::normalize_ascii()`] (o si no es
    /// ASCII / queda vacío), se ignora la sección y se genera solo el campo (`<field>`).
    pub fn section(name: impl AsRef<str>, field: AutofillField) -> Self {
        match util::normalize_ascii(name.as_ref()) {
            Ok(n) if !n.as_ref().contains(' ') => {
                Self::custom(util::join!("section-", n.as_ref(), " ", field.as_str()))
            }
            _ => Self::field(field),
        }
    }

    // --< Common fields >--------------------------------------------------------------------------

    /// Genera `autocomplete="username"`.
    pub fn username() -> Self {
        Self::field(AutofillField::Username)
    }

    /// Genera `autocomplete="username webauthn"` (Passkeys / WebAuthn).
    pub fn username_webauthn() -> Self {
        Self::custom("username webauthn")
    }

    /// Genera `autocomplete="email"`.
    pub fn email() -> Self {
        Self::field(AutofillField::Email)
    }

    /// Genera `autocomplete="current-password"`.
    pub fn current_password() -> Self {
        Self::field(AutofillField::CurrentPassword)
    }

    /// Genera `autocomplete="current-password webauthn"` (Passkeys / WebAuthn).
    pub fn current_password_webauthn() -> Self {
        Self::custom("current-password webauthn")
    }

    /// Genera `autocomplete="new-password"`.
    pub fn new_password() -> Self {
        Self::field(AutofillField::NewPassword)
    }

    /// Genera `autocomplete="one-time-code"`.
    pub fn otp() -> Self {
        Self::field(AutofillField::OneTimeCode)
    }

    // --< Address contexts >-----------------------------------------------------------------------

    /// Contexto de dirección de envío. Genera `autocomplete="shipping <field>"`.
    pub fn shipping(field: AutofillField) -> Self {
        Self::Custom(Cow::Owned(util::join!("shipping ", field.as_str())))
    }

    /// Contexto de dirección de facturación. Genera `autocomplete="billing <field>"`.
    pub fn billing(field: AutofillField) -> Self {
        Self::Custom(Cow::Owned(util::join!("billing ", field.as_str())))
    }

    // --< Contact hints >--------------------------------------------------------------------------

    /// Detalle de contacto: `autocomplete="home <field>"`.
    pub fn home(field: AutofillField) -> Self {
        Self::Custom(Cow::Owned(util::join!("home ", field.as_str())))
    }

    /// Detalle de contacto: `autocomplete="work <field>"`.
    pub fn work(field: AutofillField) -> Self {
        Self::Custom(Cow::Owned(util::join!("work ", field.as_str())))
    }

    /// Detalle de contacto: `autocomplete="mobile <field>"`.
    pub fn mobile(field: AutofillField) -> Self {
        Self::Custom(Cow::Owned(util::join!("mobile ", field.as_str())))
    }

    /// Detalle de contacto: `autocomplete="fax <field>"`.
    pub fn fax(field: AutofillField) -> Self {
        Self::Custom(Cow::Owned(util::join!("fax ", field.as_str())))
    }

    /// Detalle de contacto: `autocomplete="pager <field>"`.
    pub fn pager(field: AutofillField) -> Self {
        Self::Custom(Cow::Owned(util::join!("pager ", field.as_str())))
    }

    // --< Custom tokens >--------------------------------------------------------------------------

    /// Crea un `autocomplete` con texto libre (se espera en formato canónico).
    ///
    /// Esta función acepta una cadena con `on`/`off` o una lista de tokens separados por espacios:
    ///
    /// - Rechaza entradas no ASCII.
    /// - Recorta separadores ASCII al inicio/fin.
    /// - Compacta secuencias de separadores ASCII en un único espacio.
    /// - Convierte a minúsculas.
    ///
    /// - Si el valor normalizado es `"on"` o `"off"`, devuelve [`Autocomplete::On`] o
    ///   [`Autocomplete::Off`].
    /// - Si el valor es inválido (vacío tras normalizar o contiene bytes no ASCII), devuelve
    ///   [`Autocomplete::On`].
    pub fn custom(autocomplete: impl Into<CowStr>) -> Self {
        let value: CowStr = autocomplete.into();
        let raw = value.as_ref();

        // Normaliza la entrada.
        let Some(normalized) = util::normalize_ascii_or_empty(raw, "Autocomplete::custom") else {
            return Self::On;
        };
        let autocomplete = normalized.as_ref();

        // Identifica valores especiales.
        if autocomplete == "on" {
            return Self::On;
        }
        if autocomplete == "off" {
            return Self::Off;
        }

        // Mantiene el `Cow` original si no cambia nada (no reserva espacio).
        if autocomplete == raw {
            return Self::Custom(value);
        }
        // En otro caso asigna espacio para la normalización.
        Self::Custom(Cow::Owned(normalized.into_owned()))
    }
}

impl fmt::Display for Autocomplete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Autocomplete::On => f.write_str("on"),
            Autocomplete::Off => f.write_str("off"),
            Autocomplete::Custom(c) => f.write_str(c),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AutofillField {
    // Identidad / cuenta
    /// Nombre completo.
    Name,
    /// Tratamiento o título (p. ej. "Sr.", "Sra.", "Dra.").
    HonorificPrefix,
    /// Nombre de pila.
    GivenName,
    /// Nombre adicional (p. ej. segundo nombre).
    AdditionalName,
    /// Apellidos.
    FamilyName,
    /// Sufijo honorífico (p. ej. "Jr.", "PhD").
    HonorificSuffix,
    /// Apodo.
    Nickname,
    /// Identificador de usuario (login).
    Username,

    // Credenciales
    /// Contraseña actual.
    CurrentPassword,
    /// Nueva contraseña.
    NewPassword,
    /// Código de un solo uso (OTP).
    OneTimeCode,

    // Organización
    /// Cargo o título dentro de una organización.
    OrganizationTitle,
    /// Nombre de la organización.
    Organization,

    // Contacto
    /// Correo electrónico.
    Email,
    /// Teléfono.
    Tel,
    /// Prefijo/código de país del teléfono (incluye `+`).
    TelCountryCode,
    /// Teléfono sin el código de país.
    TelNational,
    /// Código de área (si aplica).
    TelAreaCode,
    /// Teléfono sin código de país ni de área.
    TelLocal,
    /// Prefijo local (primera parte tras el área).
    TelLocalPrefix,
    /// Sufijo local (segunda parte tras el área).
    TelLocalSuffix,
    /// Extensión interna.
    TelExtension,
    /// URL.
    Url,
    /// Referencia de mensajería instantánea (URL).
    Impp,

    // Dirección (muy habitual en formularios)
    /// Dirección postal completa (una sola línea/textarea).
    StreetAddress,
    /// Línea 1 de dirección.
    AddressLine1,
    /// Línea 2 de dirección.
    AddressLine2,
    /// Línea 3 de dirección.
    AddressLine3,
    /// Nivel administrativo 4 (el más específico).
    AddressLevel4,
    /// Nivel administrativo 3.
    AddressLevel3,
    /// Nivel administrativo 2 (p. ej. ciudad/municipio).
    AddressLevel2,
    /// Nivel administrativo 1 (p. ej. provincia/estado).
    AddressLevel1,
    /// Código postal.
    PostalCode,
    /// País (código o token `country`).
    Country,
    /// Nombre del país.
    CountryName,

    // Pago (si algún día lo necesitas)
    /// Nombre del titular de la tarjeta.
    CcName,
    /// Nombre de pila del titular de la tarjeta.
    CcGivenName,
    /// Nombre adicional del titular de la tarjeta.
    CcAdditionalName,
    /// Apellidos del titular de la tarjeta.
    CcFamilyName,
    /// Número de tarjeta.
    CcNumber,
    /// Fecha de caducidad (completa).
    CcExp,
    /// Mes de caducidad.
    CcExpMonth,
    /// Año de caducidad.
    CcExpYear,
    /// Código de seguridad (CVC/CVV).
    CcCsc,
    /// Tipo de tarjeta (p. ej. visa/mastercard).
    CcType,

    // Transacción / preferencias
    /// Moneda preferida para la transacción (código ISO 4217).
    TransactionCurrency,
    /// Cantidad de la transacción (número).
    TransactionAmount,
    /// Idioma preferido (BCP 47).
    Language,

    // Otros datos personales (según necesidad del producto)
    /// Fecha de nacimiento completa.
    Bday,
    /// Día de nacimiento.
    BdayDay,
    /// Mes de nacimiento.
    BdayMonth,
    /// Año de nacimiento.
    BdayYear,
    /// Sexo (según el valor que el UA tenga guardado).
    Sex,
    /// Foto (URL o referencia, según UA).
    Photo,
}

impl AutofillField {
    /// Devuelve el token exacto definido por HTML para `autocomplete`.
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            AutofillField::Name => "name",
            AutofillField::HonorificPrefix => "honorific-prefix",
            AutofillField::GivenName => "given-name",
            AutofillField::AdditionalName => "additional-name",
            AutofillField::FamilyName => "family-name",
            AutofillField::HonorificSuffix => "honorific-suffix",
            AutofillField::Nickname => "nickname",
            AutofillField::Username => "username",

            AutofillField::CurrentPassword => "current-password",
            AutofillField::NewPassword => "new-password",
            AutofillField::OneTimeCode => "one-time-code",

            AutofillField::OrganizationTitle => "organization-title",
            AutofillField::Organization => "organization",

            AutofillField::Email => "email",
            AutofillField::Tel => "tel",
            AutofillField::TelCountryCode => "tel-country-code",
            AutofillField::TelNational => "tel-national",
            AutofillField::TelAreaCode => "tel-area-code",
            AutofillField::TelLocal => "tel-local",
            AutofillField::TelLocalPrefix => "tel-local-prefix",
            AutofillField::TelLocalSuffix => "tel-local-suffix",
            AutofillField::TelExtension => "tel-extension",
            AutofillField::Url => "url",
            AutofillField::Impp => "impp",

            AutofillField::StreetAddress => "street-address",
            AutofillField::AddressLine1 => "address-line1",
            AutofillField::AddressLine2 => "address-line2",
            AutofillField::AddressLine3 => "address-line3",
            AutofillField::AddressLevel4 => "address-level4",
            AutofillField::AddressLevel3 => "address-level3",
            AutofillField::AddressLevel2 => "address-level2",
            AutofillField::AddressLevel1 => "address-level1",
            AutofillField::PostalCode => "postal-code",
            AutofillField::Country => "country",
            AutofillField::CountryName => "country-name",

            AutofillField::CcName => "cc-name",
            AutofillField::CcGivenName => "cc-given-name",
            AutofillField::CcAdditionalName => "cc-additional-name",
            AutofillField::CcFamilyName => "cc-family-name",
            AutofillField::CcNumber => "cc-number",
            AutofillField::CcExp => "cc-exp",
            AutofillField::CcExpMonth => "cc-exp-month",
            AutofillField::CcExpYear => "cc-exp-year",
            AutofillField::CcCsc => "cc-csc",
            AutofillField::CcType => "cc-type",

            AutofillField::TransactionCurrency => "transaction-currency",
            AutofillField::TransactionAmount => "transaction-amount",
            AutofillField::Language => "language",

            AutofillField::Bday => "bday",
            AutofillField::BdayDay => "bday-day",
            AutofillField::BdayMonth => "bday-month",
            AutofillField::BdayYear => "bday-year",
            AutofillField::Sex => "sex",
            AutofillField::Photo => "photo",
        }
    }
}

// **< InputType >**********************************************************************************

#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum InputType {
    #[default]
    Textfield,
    Password,
    Search,
    Email,
    Telephone,
    Url,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            InputType::Textfield => "text",
            InputType::Password => "password",
            InputType::Search => "search",
            InputType::Email => "email",
            InputType::Telephone => "tel",
            InputType::Url => "url",
        })
    }
}

// **< Method >*************************************************************************************

/// Método HTTP usado por un formulario ([`Form`](crate::theme::Form)) para el envío de los datos.
///
/// En HTML, el atributo `method` del formulario indica **cómo** se envían los datos:
///
/// - **GET**: los pares `name=value` se codifican en la **URL** añadiendo una cadena de consulta
///   como `?a=1&b=2`. Es el método por defecto en HTML cuando no se especifica. Suele ser apropiado
///   para **búsquedas** o formularios que no modifican datos ni el estado del sistema.
///
/// - **POST**: los datos se envían en el **cuerpo** de la petición (*request body*). Es apropiado
///   para acciones que **modifican el estado** o cuando hay formularios grandes. Es el **método por
///   defecto** en PageTop.
///
/// # Consideraciones prácticas
///
/// - **Visibilidad y privacidad**: con GET los datos quedan visibles en la URL (historial, *logs*,
///   marcadores). No se recomienda para datos sensibles. Con POST no van en la URL, pero **no se
///   cifran** por sí mismos; por eso es esencial el uso de HTTPS.
/// - **Tamaño**: GET está limitado por la longitud máxima de URL que acepten el navegador y el
///   servidor. POST es más flexible para cargas grandes.
/// - **Ficheros**: la subida de ficheros requiere `method="post"` y un `enctype` adecuado
///   (habitualmente `multipart/form-data`).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Method {
    /// Envía los datos en el cuerpo de la petición.
    ///
    /// Es el **método por defecto** en PageTop. Recomendado para operaciones que modifican el
    /// estado o para envíos grandes.
    #[default]
    Post,

    /// Envía los datos en la URL como una cadena *query*.
    ///
    /// Recomendado para búsquedas y operaciones que no modifican datos ni el estado del sistema.
    Get,
}
