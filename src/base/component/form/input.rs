//! Definiciones para crear campos de texto de una línea.

use crate::prelude::*;

use std::fmt;

// **< Kind >***************************************************************************************

/// Tipo de campo para un [`form::input::Field`].
///
/// Determina el tipo de entrada que acepta, así como el comportamiento del navegador al interactuar
/// con el campo. Implícitamente se aplica al crear el control usando [`text()`] o [`password()`],
/// [`strict_text()`] o [`strict_password()`], [`search()`], [`email()`], [`telephone()`] o
/// [`url()`].
///
/// [`text()`]: Field::text
/// [`password()`]: Field::password
/// [`strict_text()`]: Field::strict_text
/// [`strict_password()`]: Field::strict_password
/// [`search()`]: Field::search
/// [`email()`]: Field::email
/// [`telephone()`]: Field::telephone
/// [`url()`]: Field::url
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    /// Entrada de texto genérico (`type="text"`). Es el tipo por defecto.
    #[default]
    Text,
    /// Entrada de una contraseña (`type="password"`). El contenido aparece enmascarado.
    Password,
    /// Texto genérico blindado contra el autorrelleno del navegador (`type="text"`).
    ///
    /// Ver [`Field::strict_text()`].
    StrictText,
    /// Contraseña blindada contra el autorrelleno del navegador (`type="text"`).
    ///
    /// Ver [`Field::strict_password()`].
    StrictPassword,
    /// Campo de búsqueda (`type="search"`). Es un tipo semántico para los cuadros de búsqueda.
    Search,
    /// Entrada de un correo electrónico (`type="email"`). Permite validar el formato del correo.
    Email,
    /// Entrada de un teléfono (`type="tel"`). Activa el teclado de llamadas en móviles.
    Telephone,
    /// Entrada de una URL (`type="url"`). Comprueba que la entrada sea una URL bien formada.
    Url,
}

impl Kind {
    /// Devuelve `true` si el tipo aplica el conjunto de medidas contra el autorrelleno del
    /// navegador.
    ///
    /// Ver [`Field::strict_text()`] y [`Field::strict_password()`].
    pub fn is_strict(self) -> bool {
        matches!(self, Kind::StrictText | Kind::StrictPassword)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Kind::Text | Kind::StrictText | Kind::StrictPassword => "text",
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
/// - [`Field::strict_text()`]: texto genérico blindado contra el autorrelleno del navegador.
/// - [`Field::strict_password()`]: contraseña blindada contra el autorrelleno del navegador.
/// - [`Field::search()`]: búsqueda (`type="search"`).
/// - [`Field::email()`]: correo electrónico (`type="email"`).
/// - [`Field::telephone()`]: teléfono (`type="tel"`).
/// - [`Field::url()`]: URL (`type="url"`).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let email = form::input::Field::email()
///     .with_name("email")
///     .with_label(Lc::n("Email address"))
///     .with_placeholder(Lc::n("user@example.com"))
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
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el tipo de campo.
    kind: Kind,
    /// Devuelve el nombre del campo.
    name: AttrName,
    /// Devuelve el valor inicial del campo.
    value: AttrValue,
    /// Devuelve la etiqueta del campo.
    label: Lc,
    /// Devuelve el texto de ayuda del campo.
    help_text: Lc,
    /// Devuelve la longitud mínima permitida en caracteres.
    #[getters(copy)]
    minlength: Option<u16>,
    /// Devuelve la longitud máxima permitida en caracteres.
    #[getters(copy)]
    maxlength: Option<u16>,
    /// Devuelve el texto indicativo del campo.
    placeholder: Lc,
    /// Devuelve la configuración de autocompletado del campo.
    autocomplete: Option<form::Autocomplete>,
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
    #[getters(copy)]
    inputmode: Option<Mode>,
}

#[async_trait]
impl Component for Field {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        if let Some(container_id) = self
            .id()
            .or_else(|| self.name().as_deref().map(|n| util::join!("edit-", n)))
        {
            self.alter_prop(PropsOp::ensure_id(container_id));
        }

        // Clases CSS del contenedor del campo de texto.
        if self.kind().is_strict() {
            self.alter_prop(PropsOp::prepend_classes("form-field-strict"));
        }
        self.alter_prop(PropsOp::prepend_classes(util::join!(
            "form-field form-field-",
            self.kind().to_string()
        )));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let container_id = self.id();
        let input_id = container_id.as_deref().map(|id| util::join!(id, "-input"));
        let input_class = if *self.plaintext() {
            "form-control-plaintext"
        } else {
            "form-control"
        };
        let strict = self.kind().is_strict();
        let masked = *self.kind() == Kind::StrictPassword;
        let autocomplete = if strict {
            Some(&form::Autocomplete::Off)
        } else {
            self.autocomplete()
        };

        Ok(html! {
            div (self.props()) {
                @if let Some(label) = self.label().lookup(cx) {
                    label for=[input_id.as_deref()] class="form-label" {
                        (label)
                        @if *self.required() {
                            span
                                class="form-required"
                                title=[Lc::l("field_required").lookup(cx)]
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
                    name=[self.name().as_deref()]
                    value=[self.value().as_deref()]
                    minlength=[self.minlength()]
                    maxlength=[self.maxlength()]
                    placeholder=[self.placeholder().lookup(cx)]
                    inputmode=[self.inputmode()]
                    autocomplete=[autocomplete]
                    spellcheck=[strict.then_some("false")]
                    autocorrect=[strict.then_some("off")]
                    style=[masked.then_some("-webkit-text-security: disc; text-security: disc;")]
                    autofocus[*self.autofocus()]
                    readonly[*self.readonly() || *self.plaintext() || strict]
                    onfocus=[strict.then_some("this.removeAttribute('readonly')")]
                    required[*self.required()]
                    disabled[*self.disabled()];
                @if let Some(description) = self.help_text().lookup(cx) {
                    div class="form-text" { (description) }
                }
            }
        })
    }
}

#[builder_impl]
impl Field {
    /// Crea un campo de **texto genérico** (`type="text"`).
    ///
    /// Es el tipo por defecto. Adecuado para nombres, apellidos, ciudades y cualquier entrada
    /// textual sin restricciones de formato específicas.
    pub fn text() -> Self {
        Self::default()
    }

    /// Crea un campo de **contraseña** (`type="password"`).
    ///
    /// El navegador oculta los caracteres introducidos. Se recomienda usar con
    /// [`with_autocomplete()`](Self::with_autocomplete) para permitir autorrellenar con una
    /// contraseña guardada o dejar al usuario recibir sugerencias o crear una nueva.
    pub fn password() -> Self {
        Self {
            kind: Kind::Password,
            ..Default::default()
        }
    }

    /// Crea un campo de **texto genérico blindado** contra el autorrelleno del navegador.
    ///
    /// Algunos navegadores rellenan los campos de texto con datos ya guardados (como usuario,
    /// email, etc.) aplicando heurísticas por nombre o posición, incluso con `autocomplete="off"`.
    /// Este método fuerza `autocomplete="off"`, desactiva corrección ortográfica y autocorrección,
    /// y permanece de sólo lectura hasta que el usuario hace foco (ya que normalmente el navegador
    /// respeta el `readonly` en la carga aunque ignore `autocomplete="off"`).
    ///
    /// Soporte por motor de navegador (Blink: Chrome, Edge, Opera; Gecko: Firefox; WebKit: Safari):
    ///
    /// | Medida                        | Blink      | Gecko      | WebKit                   |
    /// |-------------------------------|------------|------------|--------------------------|
    /// | `autocomplete="off"` ignorado | Sí         | Sí         | Lo respeta mejor         |
    /// | `spellcheck="false"`          | Sí         | Sí         | Sí                       |
    /// | `autocorrect="off"`           | Sin efecto | Sin efecto | Sí (origen del atributo) |
    ///
    /// El truco `readonly` + `onfocus` compensa específicamente la agresividad de Chrome/Blink al
    /// autorrellenar campos nada más cargar la página; su fiabilidad varía entre versiones y no
    /// está documentada de forma oficial. Ninguna de estas medidas es infalible frente a gestores
    /// de contraseñas de terceros, que usan heurísticas propias.
    ///
    /// Ver también [`Field::strict_password()`].
    pub fn strict_text() -> Self {
        Self {
            kind: Kind::StrictText,
            ..Default::default()
        }
    }

    /// Crea un campo de **contraseña blindada** contra el autorrelleno del navegador.
    ///
    /// Aplica las mismas medidas que [`Field::strict_text()`] y, además, enmascara el contenido
    /// visualmente vía CSS (usando `-webkit-text-security`) en lugar de usar `type="password"`. Al
    /// no ser un campo de contraseña para el navegador, evita el gestor nativo de contraseñas, el
    /// icono de mostrar/ocultar y el aviso de guardar la contraseña.
    ///
    /// Útil en formularios de acceso donde ese comportamiento no es deseable (como puestos
    /// compartidos o paneles administrativos sensibles).
    ///
    /// La propiedad `-webkit-text-security` está soportada de forma nativa en Blink (Chrome, Edge,
    /// Opera) y en WebKit (Safari, su origen); Firefox lo soporta desde la versión 114 (2023) con
    /// el mismo nombre.
    pub fn strict_password() -> Self {
        Self {
            kind: Kind::StrictPassword,
            ..Default::default()
        }
    }

    /// Crea un campo de **búsqueda** (`type="search"`).
    ///
    /// Semánticamente equivalente a `text` pero optimizado para búsquedas: algunos navegadores
    /// añaden un botón para borrar el contenido.
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

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece el nombre del campo (atributo `name`).
    ///
    /// Sin él, el valor del campo no se transmite al servidor al enviar el formulario. Para
    /// deserializar el campo en el servidor es recomendable establecer un `name` explícito.
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece el valor inicial del campo.
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        self.value.alter_str(value);
        self
    }

    /// Establece la etiqueta visible del campo (usa [`Lc::none()`] para quitarla).
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }

    /// Establece el texto de ayuda del campo (usa [`Lc::none()`] para quitarlo).
    pub fn with_help_text(mut self, help_text: Lc) -> Self {
        self.help_text = help_text;
        self
    }

    /// Establece la longitud mínima permitida en caracteres (`None` para no imponer mínimo).
    pub fn with_minlength(mut self, minlength: impl Into<Option<u16>>) -> Self {
        self.minlength = minlength.into();
        self
    }

    /// Establece la longitud máxima permitida en caracteres (`None` para no imponer límite).
    pub fn with_maxlength(mut self, maxlength: impl Into<Option<u16>>) -> Self {
        self.maxlength = maxlength.into();
        self
    }

    /// Establece el texto indicativo del campo (usa [`Lc::none()`] para quitarlo).
    ///
    /// Este texto aparece en el mismo campo y desaparece en cuanto el usuario empieza a escribir.
    /// Al ser texto visible para el usuario se acepta [`Lc`] para poder localizarlo.
    pub fn with_placeholder(mut self, placeholder: Lc) -> Self {
        self.placeholder = placeholder;
        self
    }

    /// Establece la configuración de autocompletado del campo.
    ///
    /// Usar los métodos de [`form::Autocomplete`] para los valores más habituales (p. ej.
    /// [`Autocomplete::email()`](form::Autocomplete::email) o
    /// [`Autocomplete::current_password()`](form::Autocomplete::current_password)).
    pub fn with_autocomplete(
        mut self,
        autocomplete: impl Into<Option<form::Autocomplete>>,
    ) -> Self {
        self.autocomplete = autocomplete.into();
        self
    }

    /// Establece si el campo recibe el foco automáticamente al cargar la página.
    pub fn with_autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    /// Establece si el campo es de sólo lectura.
    pub fn with_readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Establece si el campo es obligatorio.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Establece si el campo está deshabilitado.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Establece si el campo se muestra como texto plano (sin bordes ni fondo).
    ///
    /// Útil para mostrar un valor no editable en pantalla que sí se envía al servidor con el
    /// formulario. El efecto visual depende del tema activo.
    pub fn with_plaintext(mut self, plaintext: bool) -> Self {
        self.plaintext = plaintext;
        self
    }

    /// Establece el modo de entrada sugerido para el teclado virtual en dispositivos móviles.
    ///
    /// A diferencia del atributo `type` ([`form::input::Kind`]), no restringe los valores aceptados
    /// ni activa la validación del navegador; es sólo una sugerencia de presentación.
    pub fn with_inputmode(mut self, inputmode: impl Into<Option<Mode>>) -> Self {
        self.inputmode = inputmode.into();
        self
    }
}
