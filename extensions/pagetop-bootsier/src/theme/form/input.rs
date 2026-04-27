//! Definiciones para crear campos de texto de una línea.

use pagetop::prelude::*;

use crate::theme::form;
use crate::LOCALES_BOOTSIER;

use std::fmt;

// **< Kind >***************************************************************************************

/// Tipo de campo para un [`form::input::Field`].
///
/// Determina el tipo de entrada que acepta, así como el comportamiento del navegador al interactuar
/// con el campo. Implícitamente se aplica al crear el control: [`text()`](Field::text),
/// [`password()`](Field::password), [`search()`](Field::search), [`email()`](Field::email),
/// [`telephone()`](Field::telephone) o [`url()`](Field::url).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    /// Entrada de texto genérico (`type="text"`). Es el tipo por defecto.
    #[default]
    Text,
    /// Entrada de una contraseña (`type="password"`). El contenido aparece enmascarado.
    Password,
    /// Campo de búsqueda (`type="search"`). Es un tipo semántico para los cuadros de búsqueda.
    Search,
    /// Entrada de un correo electrónico (`type="email"`). Permite validar el formato del correo.
    Email,
    /// Entrada de un teléfono (`type="tel"`). Activa el teclado de llamadas en móviles.
    Telephone,
    /// Entrada de una URL (`type="url"`). Comprueba que la entrada sea una URL bien formada.
    Url,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Kind::Text => "text",
            Kind::Password => "password",
            Kind::Search => "search",
            Kind::Email => "email",
            Kind::Telephone => "tel",
            Kind::Url => "url",
        })
    }
}

// **< Mode >***************************************************************************************

/// Sugerencia para el teclado virtual de un [`form::input::Field`].
///
/// Indica al navegador qué tipo de teclado virtual mostrar en dispositivos móviles o táctiles al
/// editar el campo. A diferencia del atributo `type` ([`form::input::Kind`]), no restringe los
/// valores aceptados ni activa la validación del navegador; es sólo una sugerencia de presentación.
///
/// Se establece con [`form::input::Field::with_inputmode()`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Suprime el teclado virtual. Útil en campos con teclado personalizado basado en JavaScript.
    None,
    /// Teclado de texto genérico.
    Text,
    /// Teclado decimal, con dígitos y separador decimal.
    Decimal,
    /// Teclado numérico, con sólo dígitos.
    Numeric,
    /// Teclado de teléfono, con dígitos y símbolos `+`, `*` y `#`.
    Tel,
    /// Teclado optimizado para búsquedas (puede incluir tecla de búsqueda).
    Search,
    /// Teclado optimizado para correo electrónico (incluye `@` y `.`).
    Email,
    /// Teclado optimizado para URL (incluye `/`, `.` y `.com`).
    Url,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Mode::None => "none",
            Mode::Text => "text",
            Mode::Decimal => "decimal",
            Mode::Numeric => "numeric",
            Mode::Tel => "tel",
            Mode::Search => "search",
            Mode::Email => "email",
            Mode::Url => "url",
        })
    }
}

// **< Field >**************************************************************************************

/// Componente para crear un **campo de texto de una línea**.
///
/// Renderiza los tipos más habituales en formularios:
///
/// - [`Field::text()`]: campo de texto genérico (`type="text"`, por defecto).
/// - [`Field::password()`]: contraseña (`type="password"`).
/// - [`Field::search()`]: búsqueda (`type="search"`).
/// - [`Field::email()`]: correo electrónico (`type="email"`).
/// - [`Field::telephone()`]: teléfono (`type="tel"`).
/// - [`Field::url()`]: URL (`type="url"`).
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// let email = form::input::Field::email()
///     .with_name("email")
///     .with_label(L10n::n("Email address"))
///     .with_placeholder(L10n::n("user@example.com"))
///     .with_autocomplete(Some(form::Autocomplete::email()))
///     .with_required(true);
/// ```
///
/// Al enviar el formulario el navegador transmite `name=valor`. Un campo de texto siempre envía su
/// valor, incluso si está vacío. En el servidor se deserializa como `String`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     email: String, // Siempre presente; cadena vacía si el usuario no escribió nada.
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Field {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS del contenedor del campo.
    classes: Classes,
    /// Devuelve el tipo de campo.
    kind: Kind,
    /// Devuelve el nombre del campo.
    name: AttrName,
    /// Devuelve el valor inicial del campo.
    value: AttrValue,
    /// Devuelve la etiqueta del campo.
    label: Attr<L10n>,
    /// Devuelve el texto de ayuda del campo.
    help_text: Attr<L10n>,
    /// Devuelve la longitud mínima permitida en caracteres.
    minlength: Attr<u16>,
    /// Devuelve la longitud máxima permitida en caracteres.
    maxlength: Attr<u16>,
    /// Devuelve el texto indicativo del campo.
    placeholder: Attr<L10n>,
    /// Devuelve la configuración de autocompletado del campo.
    autocomplete: Attr<form::Autocomplete>,
    /// Devuelve si el campo recibe el foco automáticamente al cargar la página.
    autofocus: bool,
    /// Devuelve si el campo es de sólo lectura.
    readonly: bool,
    /// Devuelve si el campo es obligatorio.
    required: bool,
    /// Devuelve si el campo está deshabilitado.
    disabled: bool,
    /// Devuelve si el campo se muestra como texto plano sin bordes ni fondo.
    plaintext: bool,
    /// Devuelve la sugerencia de teclado virtual para el campo.
    inputmode: Attr<Mode>,
}

impl Component for Field {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            util::join!("form-field form-field-", self.kind().to_string()),
        );
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let container_id = self
            .id()
            .or_else(|| self.name().get().map(|n| util::join!("edit-", n)));
        let input_id = container_id.as_deref().map(|id| util::join!(id, "-input"));
        let input_class = if *self.plaintext() {
            "form-control-plaintext"
        } else {
            "form-control"
        };
        Ok(html! {
            div id=[container_id.as_deref()] class=[self.classes().get()] {
                @if let Some(label) = self.label().lookup(cx) {
                    label for=[input_id.as_deref()] class="form-label" {
                        (label)
                        @if *self.required() {
                            span
                                class="form-required"
                                title=(L10n::t("input_required", &LOCALES_BOOTSIER).using(cx))
                            {
                                "*"
                            }
                        }
                    }
                }
                input
                    type=(self.kind())
                    id=[input_id.as_deref()]
                    class=(input_class)
                    name=[self.name().get()]
                    value=[self.value().get()]
                    minlength=[self.minlength().get()]
                    maxlength=[self.maxlength().get()]
                    placeholder=[self.placeholder().lookup(cx)]
                    inputmode=[self.inputmode().get()]
                    autocomplete=[self.autocomplete().get()]
                    autofocus[*self.autofocus()]
                    readonly[*self.readonly() || *self.plaintext()]
                    required[*self.required()]
                    disabled[*self.disabled()];
                @if let Some(description) = self.help_text().lookup(cx) {
                    div class="form-text" { (description) }
                }
            }
        })
    }
}

impl Field {
    /// Crea un campo de **texto genérico** (`type="text"`).
    ///
    /// Es el tipo por defecto. Adecuado para nombres, apellidos, ciudades y cualquier entrada
    /// textual sin restricciones de formato específicas.
    pub fn text() -> Self {
        Field::default()
    }

    /// Crea un campo de **contraseña** (`type="password"`).
    ///
    /// El navegador oculta los caracteres introducidos. Se recomienda usar con
    /// [`with_autocomplete()`](Field::with_autocomplete) para indicar si acepta la contraseña
    /// actual o una nueva.
    pub fn password() -> Self {
        Self {
            kind: Kind::Password,
            ..Default::default()
        }
    }

    /// Crea un campo de **búsqueda** (`type="search"`).
    ///
    /// Semánticamente equivalente a `text` pero optimizado para búsquedas: algunos
    /// navegadores añaden un botón para borrar el contenido.
    pub fn search() -> Self {
        Self {
            kind: Kind::Search,
            ..Default::default()
        }
    }

    /// Crea un campo de **correo electrónico** (`type="email"`).
    ///
    /// El navegador valida el formato de la dirección antes de enviar el formulario. En
    /// dispositivos móviles muestra un teclado adaptado para introducir direcciones de correo.
    pub fn email() -> Self {
        Self {
            kind: Kind::Email,
            ..Default::default()
        }
    }

    /// Crea un campo de **teléfono** (`type="tel"`).
    ///
    /// No impone ninguna restricción de formato (los formatos de teléfono varían por país), pero
    /// en dispositivos móviles muestra el teclado numérico de llamadas.
    pub fn telephone() -> Self {
        Self {
            kind: Kind::Telephone,
            ..Default::default()
        }
    }

    /// Crea un campo de **URL** (`type="url"`).
    ///
    /// El navegador valida que el valor sea una URL bien formada antes de enviar el formulario.
    pub fn url() -> Self {
        Self {
            kind: Kind::Url,
            ..Default::default()
        }
    }

    // **< Field BUILDER >**************************************************************************

    /// Establece el identificador único (`id`) del contenedor del campo.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al contenedor del campo.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
        self
    }

    /// Establece el nombre del campo (atributo `name`).
    ///
    /// Sin él, el valor del campo no se transmite al servidor al enviar el formulario. Para
    /// deserializar el campo en el servidor es recomendable establecer un `name` explícito.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece el valor inicial del campo.
    #[builder_fn]
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        self.value.alter_str(value);
        self
    }

    /// Establece o elimina la etiqueta visible del campo (basta pasar `None` para quitarla).
    #[builder_fn]
    pub fn with_label(mut self, label: impl Into<Option<L10n>>) -> Self {
        self.label.alter_opt(label.into());
        self
    }

    /// Establece o elimina el texto de ayuda del campo (basta pasar `None` para quitarlo).
    #[builder_fn]
    pub fn with_help_text(mut self, help_text: impl Into<Option<L10n>>) -> Self {
        self.help_text.alter_opt(help_text.into());
        self
    }

    /// Establece la longitud mínima permitida en caracteres (`None` para no imponer mínimo).
    #[builder_fn]
    pub fn with_minlength(mut self, minlength: Option<u16>) -> Self {
        self.minlength.alter_opt(minlength);
        self
    }

    /// Establece la longitud máxima permitida en caracteres (`None` para no imponer límite).
    #[builder_fn]
    pub fn with_maxlength(mut self, maxlength: Option<u16>) -> Self {
        self.maxlength.alter_opt(maxlength);
        self
    }

    /// Establece o elimina el texto indicativo del campo (`None` para quitarlo).
    ///
    /// Este texto aparece en el mismo campo y desaparece en cuanto el usuario empieza a escribir.
    /// Al ser texto visible para el usuario se acepta [`L10n`] para poder localizarlo.
    #[builder_fn]
    pub fn with_placeholder(mut self, placeholder: impl Into<Option<L10n>>) -> Self {
        self.placeholder.alter_opt(placeholder.into());
        self
    }

    /// Establece la configuración de autocompletado del campo.
    ///
    /// Usar los métodos de [`form::Autocomplete`] para los valores más habituales (p. ej.
    /// [`Autocomplete::email()`](form::Autocomplete::email) o
    /// [`Autocomplete::current_password()`](form::Autocomplete::current_password)).
    #[builder_fn]
    pub fn with_autocomplete(mut self, autocomplete: Option<form::Autocomplete>) -> Self {
        self.autocomplete.alter_opt(autocomplete);
        self
    }

    /// Establece si el campo recibe el foco automáticamente al cargar la página.
    #[builder_fn]
    pub fn with_autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    /// Establece si el campo es de sólo lectura.
    #[builder_fn]
    pub fn with_readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Establece si el campo es obligatorio.
    #[builder_fn]
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Establece si el campo está deshabilitado.
    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Establece si el campo se muestra como texto plano (sin bordes ni fondo).
    ///
    /// Útil para mostrar un valor no editable en pantalla que sí se envía al servidor con el
    /// formulario.
    #[builder_fn]
    pub fn with_plaintext(mut self, plaintext: bool) -> Self {
        self.plaintext = plaintext;
        self
    }

    /// Establece el modo de entrada sugerido para el teclado virtual en dispositivos móviles.
    ///
    /// A diferencia del atributo `type` ([`form::input::Kind`]), no restringe los valores aceptados
    /// ni activa la validación del navegador; es sólo una sugerencia de presentación.
    #[builder_fn]
    pub fn with_inputmode(mut self, inputmode: Option<Mode>) -> Self {
        self.inputmode.alter_opt(inputmode);
        self
    }
}
