use pagetop::prelude::*;

use std::borrow::Cow;
use std::fmt;

// **< CheckboxKind >*******************************************************************************

/// Variante visual para [`form::Checkbox`](crate::theme::form::Checkbox) en un formulario.
///
/// Determina si el control se renderiza como una casilla de verificación estándar o como un
/// interruptor (*toggle switch*).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum CheckboxKind {
    /// Casilla de verificación estándar. Es el tipo por defecto.
    #[default]
    Check,
    /// Interruptor de encendido/apagado.
    Switch,
    // TODO: Añadir variante `NativeSwitch` cuando el atributo `switch` de la propuesta WHATWG
    // (https://github.com/whatwg/html/issues/9546) sea estándar y tenga soporte amplio. Safari ya
    // lo soporta. También se añadiría el constructor `Checkbox::native_switch()`.
}

// **< Autocomplete / AutofillField >***************************************************************

/// Configuración para el autocompletado de controles en un formulario.
///
/// Indica al navegador si puede sugerir o rellenar automáticamente el valor del control usando
/// datos que el usuario haya introducido antes (credenciales guardadas, datos de contacto, etc.).
///
/// Lo habitual es usar uno de los **métodos predefinidos**, que generan el token canónico adecuado
/// para cada tipo de dato:
///
/// - Identidad y credenciales: [`username()`](Autocomplete::username),
///   [`email()`](Autocomplete::email), [`current_password()`](Autocomplete::current_password),
///   [`new_password()`](Autocomplete::new_password), [`otp()`](Autocomplete::otp).
/// - Token o tokens directos: [`token(field)`](Autocomplete::token) con una variante de
///   [`AutofillField`].
/// - Direcciones: [`shipping(field)`](Autocomplete::shipping),
///   [`billing(field)`](Autocomplete::billing).
/// - Datos de contacto: [`home(field)`](Autocomplete::home), [`work(field)`](Autocomplete::work),
///   [`mobile(field)`](Autocomplete::mobile), [`fax(field)`](Autocomplete::fax),
///   [`pager(field)`](Autocomplete::pager).
/// - Sección personalizada: [`section(name, field)`](Autocomplete::section).
///
/// Para activar o inhibir el autocompletado sin especificar el tipo de dato basta con usar las
/// variantes [`form::Autocomplete::On`](Autocomplete::On) o
/// [`form::Autocomplete::Off`](Autocomplete::Off). Para combinaciones no cubiertas por los métodos
/// anteriores, [`custom()`](Autocomplete::custom) acepta cualquier cadena ASCII válida.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// // Correo electrónico con sugerencia semántica del navegador.
/// let ac = form::Autocomplete::email();
///
/// // Contraseña nueva en un formulario de registro.
/// let ac = form::Autocomplete::new_password();
///
/// // Teléfono de contacto del trabajo.
/// let ac = form::Autocomplete::work(form::AutofillField::Tel);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Autocomplete {
    /// Genera `autocomplete="on"`.
    On,
    /// Genera `autocomplete="off"`.
    Off,
    /// Contiene el valor literal del atributo `autocomplete` tal como se enviará al navegador.
    ///
    /// Debe contener un token o lista de tokens separados por espacios (p. ej. `"username"` o
    /// `"username webauthn"`).
    Custom(CowStr),
}

impl Autocomplete {
    // --< Token >----------------------------------------------------------------------------------

    /// Genera `autocomplete` a partir del token o tokens del [`AutofillField`] indicado.
    #[inline]
    pub fn token(field: AutofillField) -> Self {
        Self::Custom(Cow::Borrowed(field.as_str()))
    }

    // --< Secciones >------------------------------------------------------------------------------

    /// Construye `autocomplete` con un prefijo de sección y un token o tokens del
    /// [`form::AutofillField`](AutofillField) indicado.
    ///
    /// Genera `autocomplete="section-<name> <field>"`. Si `name` no es ASCII o contiene espacios,
    /// se ignora la sección y se genera sólo el token indicado.
    ///
    /// El prefijo `section-*` sirve para distinguir entre varios grupos del mismo tipo en una misma
    /// página (p. ej. una dirección de envío y otra de facturación).
    pub fn section(name: impl AsRef<str>, field: AutofillField) -> Self {
        match util::normalize_ascii(name.as_ref()) {
            Ok(n) if !n.as_ref().contains(' ') => {
                Self::custom(util::join!("section-", n.as_ref(), " ", field.as_str()))
            }
            _ => Self::token(field),
        }
    }

    // --< Comunes >--------------------------------------------------------------------------------

    /// Genera `autocomplete="username"`.
    pub fn username() -> Self {
        Self::token(AutofillField::Username)
    }

    /// Genera `autocomplete="username webauthn"` (Passkeys / WebAuthn).
    pub fn username_webauthn() -> Self {
        Self::custom("username webauthn")
    }

    /// Genera `autocomplete="email"`.
    pub fn email() -> Self {
        Self::token(AutofillField::Email)
    }

    /// Genera `autocomplete="current-password"`.
    pub fn current_password() -> Self {
        Self::token(AutofillField::CurrentPassword)
    }

    /// Genera `autocomplete="current-password webauthn"` (Passkeys / WebAuthn).
    pub fn current_password_webauthn() -> Self {
        Self::custom("current-password webauthn")
    }

    /// Genera `autocomplete="new-password"`.
    pub fn new_password() -> Self {
        Self::token(AutofillField::NewPassword)
    }

    /// Genera `autocomplete="one-time-code"`.
    pub fn otp() -> Self {
        Self::token(AutofillField::OneTimeCode)
    }

    // --< Direcciones >----------------------------------------------------------------------------

    /// Contexto de dirección de envío. Genera `autocomplete="shipping <field>"`.
    pub fn shipping(field: AutofillField) -> Self {
        Self::Custom(Cow::Owned(util::join!("shipping ", field.as_str())))
    }

    /// Contexto de dirección de facturación. Genera `autocomplete="billing <field>"`.
    pub fn billing(field: AutofillField) -> Self {
        Self::Custom(Cow::Owned(util::join!("billing ", field.as_str())))
    }

    // --< Contacto >-------------------------------------------------------------------------------

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

    // --< Tokens personalizados >------------------------------------------------------------------

    /// Crea un valor de `autocomplete` a partir de una cadena de texto libre.
    ///
    /// Normaliza la entrada recortando espacios extra, compactando separadores y convirtiendo a
    /// minúsculas. Si el resultado es `"on"` u `"off"`, devuelve la variante correspondiente; si la
    /// entrada contiene caracteres no ASCII o queda vacía tras normalizar, devuelve
    /// [`form::Autocomplete::On`](Autocomplete::On).
    ///
    /// Para los casos habituales se recomienda usar los métodos predefinidos de
    /// [`form::Autocomplete`](Autocomplete).
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

/// Tokens para el autocompletado de formularios con [`form::Autocomplete`](Autocomplete).
///
/// Representa los tokens de autorrelleno (*autofill field*) definidos por la
/// [especificación WHATWG](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#autofill-field)
/// para el atributo `autocomplete`. Cada variante corresponde exactamente a un token canónico
/// de dicha especificación.
///
/// Los valores se usan en combinación con [`form::Autocomplete`](Autocomplete) para construir el
/// valor completo del atributo `autocomplete` de un control de formulario. Los métodos de
/// [`form::Autocomplete`](Autocomplete) como [`token()`](Autocomplete::token),
/// [`email()`](Autocomplete::email), [`shipping()`](Autocomplete::shipping) o
/// [`section()`](Autocomplete::section) aceptan variantes de `AutofillField` para generar el token
/// correspondiente.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let ac = form::Autocomplete::token(form::AutofillField::Username);
/// let ac = form::Autocomplete::shipping(form::AutofillField::StreetAddress);
/// let ac = form::Autocomplete::section("job", form::AutofillField::Email);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AutofillField {
    // --< Identidad / cuenta >---------------------------------------------------------------------
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

    // --< Credenciales >---------------------------------------------------------------------------
    /// Contraseña actual.
    CurrentPassword,
    /// Nueva contraseña.
    NewPassword,
    /// Código de un solo uso (OTP).
    OneTimeCode,

    // --< Organización >---------------------------------------------------------------------------
    /// Cargo o título dentro de una organización.
    OrganizationTitle,
    /// Nombre de la organización.
    Organization,

    // --< Contacto >-------------------------------------------------------------------------------
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
    /// URL personal o de contacto.
    Url,
    /// Referencia de mensajería instantánea (URL).
    Impp,

    // --< Dirección >------------------------------------------------------------------------------
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
    /// País (el navegador rellena el código de país).
    Country,
    /// Nombre del país.
    CountryName,

    // --< Pago >-----------------------------------------------------------------------------------
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

    // --< Transacción / preferencias >-------------------------------------------------------------
    /// Moneda preferida para la transacción (código ISO 4217).
    TransactionCurrency,
    /// Cantidad de la transacción (número).
    TransactionAmount,
    /// Idioma preferido (BCP 47).
    Language,

    // --< Datos personales >-----------------------------------------------------------------------
    /// Fecha de nacimiento completa.
    Bday,
    /// Día de nacimiento.
    BdayDay,
    /// Mes de nacimiento.
    BdayMonth,
    /// Año de nacimiento.
    BdayYear,
    /// Sexo (valor libre guardado por el navegador).
    Sex,
    /// Foto (URL o referencia guardada por el navegador).
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
