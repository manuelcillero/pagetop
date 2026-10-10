//! Definiciones para crear campos de fecha y hora.

use crate::datetime::input;
use crate::prelude::*;

// **< Kind >***************************************************************************************

/// Tipo de dato de un [`form::date::Field`].
///
/// Se aplica al crear el campo con [`Field::date()`], [`Field::time()`] o [`Field::datetime()`].
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    /// Fecha sin hora ([`NaiveDate`]). Es el tipo por defecto.
    #[default]
    Date,
    /// Hora del día ([`NaiveTime`]).
    Time,
    /// Fecha y hora de un instante ([`DateTime<Utc>`](DateTime)), en la zona horaria efectiva.
    DateTime,
}

impl Kind {
    // Sufijo de la clase CSS del contenedor (`form-field-date`...).
    const fn as_str(&self) -> &'static str {
        match self {
            Kind::Date => "date",
            Kind::Time => "time",
            Kind::DateTime => "datetime",
        }
    }
}

// Valor inicial del campo: un dato con su tipo, que se muestra con el formato del idioma, o el
// texto tal como lo escribió el usuario (para volver a mostrarlo, p. ej. si no se pudo leer).
#[derive(AutoDefault, Clone, Debug)]
enum Value {
    #[default]
    None,
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(DateTime<Utc>),
    Text(String),
}

// **< Field >**************************************************************************************

/// Componente para crear un **campo de fecha, de hora o de fecha y hora**.
///
/// Muestra el valor **con el formato del idioma efectivo** del documento, el mismo con el que se
/// muestra el resto de la página y con el que lo leen después los métodos `parse_*` de
/// [`Contextual`]. Por ejemplo, una fecha se escribe `dd/mm/aaaa` en español y `mm/dd/yyyy` en
/// inglés de Estados Unidos, y ese formato se indica en el propio campo (como texto indicativo y
/// como texto de ayuda asociado al campo). Una fecha y hora se muestra y se escribe en la **zona
/// horaria efectiva** del documento ([`Contextual::timezone()`]), normalmente la del usuario, y se
/// lee convertida a UTC.
///
/// Es un campo de texto (`type="text"`), no un selector nativo (`type="date"`): el navegador
/// muestra los selectores nativos con el formato de **su** configuración regional, no con el del
/// idioma de la página, y la página no puede cambiarlo.
///
/// - [`Field::date()`]: fecha sin hora ([`NaiveDate`]), p. ej. una fecha de caducidad. No se
///   convierte de zona horaria.
/// - [`Field::time()`]: hora del día ([`NaiveTime`]).
/// - [`Field::datetime()`]: fecha y hora de un instante ([`DateTime<Utc>`](DateTime)).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # fn build(expires: Option<NaiveDate>) {
/// let field = form::date::Field::date()
///     .with_name("expires")
///     .with_label(Lc::n("Expiry date"))
///     .with_date(expires)
///     .with_required(true);
/// # }
/// ```
///
/// El navegador envía el texto escrito, que se deserializa como `String` y se lee con
/// [`Contextual::parse_date()`], [`Contextual::parse_time()`] o [`Contextual::parse_datetime()`]
/// según el tipo de campo. Si no se puede leer, el error ([`DateInputError`]) tiene un mensaje
/// traducido, y [`with_text()`](Field::with_text) vuelve a mostrar lo que escribió el usuario:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # fn handle(cx: &Context, text: String) {
/// match cx.parse_date(&text) {
///     Ok(expires) => { /* Guardar `expires` (`Option<NaiveDate>`). */ }
///     Err(error) => {
///         let field = form::date::Field::date().with_name("expires").with_text(text);
///         let message = error.message(); // Para mostrarlo junto al formulario.
///     }
/// }
/// # }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Field {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el tipo de dato del campo.
    #[getters(copy)]
    kind: Kind,
    /// Devuelve el nombre del campo.
    name: AttrName,
    #[getters(skip)]
    value: Value,
    /// Devuelve la etiqueta del campo.
    label: Lc,
    /// Devuelve el texto de ayuda del campo.
    help_text: Lc,
    /// Devuelve la configuración de autocompletado del campo; sin ella, se desactiva (`off`).
    autocomplete: Option<form::Autocomplete>,
    /// Devuelve si el campo recibe el foco automáticamente al cargar la página.
    autofocus: bool,
    /// Devuelve si el campo es de sólo lectura.
    readonly: bool,
    /// Devuelve si el campo es obligatorio.
    required: bool,
    /// Devuelve si el campo está deshabilitado.
    disabled: bool,
}

#[async_trait]
impl Component for Field {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &mut Context) {
        if let Some(container_id) = self
            .id()
            .or_else(|| self.name().as_deref().map(|n| util::join!("edit-", n)))
        {
            self.alter_prop(PropsOp::ensure_id(container_id));
        }

        // Clases CSS del contenedor del campo.
        self.alter_prop(PropsOp::prepend_classes(util::join!(
            "form-field form-field-",
            self.kind().as_str()
        )));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let container_id = self.id();
        let input_id = container_id.as_deref().map(|id| util::join!(id, "-input"));
        let tz = cx.timezone();
        let value = match (self.kind(), &self.value) {
            (_, Value::Text(text)) => Some(text.clone()),
            (Kind::Date, Value::Date(date)) => Some(input::format_date(*date, cx)),
            (Kind::Time, Value::Time(time)) => Some(input::format_time(*time, cx)),
            (Kind::DateTime, Value::DateTime(dt)) => Some(input::format_datetime(*dt, cx, tz)),
            _ => None,
        };
        let hint = match self.kind() {
            Kind::Date => input::date_hint(cx),
            Kind::Time => input::time_hint(cx),
            Kind::DateTime => input::datetime_hint(cx),
        };
        let format = Lc::l("input_format_help").with_arg("format", hint.clone());
        let format = form::parts::Help::suffixed(&format, input_id.as_deref(), "-format");
        let help = form::parts::Help::new(self.help_text(), input_id.as_deref());
        let label = form::parts::Label::new(self.label(), input_id.as_deref())
            .with_required(*self.required());
        // El formato y, si lo hay, el texto de ayuda describen el campo, en ese orden.
        let described_by = util::join_pair!(
            format.id().unwrap_or_default(),
            " ",
            help.id().unwrap_or_default()
        );
        // Sin autocompletado explícito se desactiva: el navegador sugeriría fechas escritas antes
        // en otros formularios, que rara vez son las que se quieren introducir.
        let autocomplete = self.autocomplete().unwrap_or(&form::Autocomplete::Off);
        Ok(html! {
            div (self.props().unpack(cx)) {
                (label.render(cx))
                input
                    type="text"
                    id=[input_id.as_deref()]
                    class="form-control"
                    name=[self.name().as_deref()]
                    value=[value]
                    placeholder=(hint)
                    aria-describedby=[util::non_blank(&described_by)]
                    autocomplete=(autocomplete)
                    autofocus[*self.autofocus()]
                    readonly[*self.readonly()]
                    required[*self.required()]
                    disabled[*self.disabled()];
                (format.render(cx))
                (help.render(cx))
            }
        })
    }
}

#[builder_impl]
impl Field {
    /// Crea un campo de **fecha sin hora** ([`NaiveDate`]), p. ej. una fecha de nacimiento o de
    /// caducidad. Se lee con [`Contextual::parse_date()`].
    pub fn date() -> Self {
        Self::default()
    }

    /// Crea un campo de **hora del día** ([`NaiveTime`]). Se lee con
    /// [`Contextual::parse_time()`].
    pub fn time() -> Self {
        Self {
            kind: Kind::Time,
            ..Default::default()
        }
    }

    /// Crea un campo de **fecha y hora de un instante** ([`DateTime<Utc>`](DateTime)), que el
    /// usuario ve y escribe en su hora local. Se lee con [`Contextual::parse_datetime()`].
    pub fn datetime() -> Self {
        Self {
            kind: Kind::DateTime,
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
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece el nombre del campo (atributo `name`).
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece la fecha inicial de un campo de [`Field::date()`] (`None` para dejarlo vacío). En
    /// un campo de otro tipo se ignora.
    pub fn with_date(mut self, date: impl Into<Option<NaiveDate>>) -> Self {
        self.value = date.into().map_or(Value::None, Value::Date);
        self
    }

    /// Establece la hora inicial de un campo de [`Field::time()`] (`None` para dejarlo vacío). En
    /// un campo de otro tipo se ignora.
    pub fn with_time(mut self, time: impl Into<Option<NaiveTime>>) -> Self {
        self.value = time.into().map_or(Value::None, Value::Time);
        self
    }

    /// Establece el instante inicial de un campo de [`Field::datetime()`] (`None` para dejarlo
    /// vacío). Se muestra en la zona horaria efectiva del documento. En un campo de otro tipo se
    /// ignora.
    pub fn with_datetime(mut self, dt: impl Into<Option<DateTime<Utc>>>) -> Self {
        self.value = dt.into().map_or(Value::None, Value::DateTime);
        self
    }

    /// Establece el texto inicial tal cual, sin interpretarlo. Útil para volver a mostrar lo que
    /// escribió el usuario cuando no se ha podido leer.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.value = Value::Text(text.into());
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

    /// Establece la configuración de autocompletado del campo (`None` para desactivarlo, que es lo
    /// predeterminado). Por ejemplo, para una fecha de nacimiento:
    /// `form::Autocomplete::token(form::AutofillField::Bday)`.
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
}
