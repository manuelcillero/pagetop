use crate::prelude::*;

/// Etiqueta de un campo de formulario, con la marca opcional de campo obligatorio.
///
/// Muestra la etiqueta del campo y, si es obligatorio ([`with_required()`](Self::with_required)),
/// le añade un asterisco con un texto emergente traducido (`field_required`). Se renderiza con
/// [`render()`](Self::render), que genera un `<label>`, o nada si el texto es [`Lc::none()`]. La
/// clase predeterminada (`form-label`, o `form-check-label` en [`check()`](Self::check)) puede
/// cambiarse con [`with_class()`](Self::with_class).
///
/// Se puede crear con:
///
/// - [`new()`](Self::new): para añadir la etiqueta a un control, enlazada con su atributo `for`.
/// - [`check()`](Self::check): para el texto de una casilla de verificación, junto a ella.
/// - [`group()`](Self::group): para el nombre de un grupo de controles (casillas, botones de
///   opción), con un identificador para enlazarlo desde el grupo con `aria-labelledby`.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # fn render(label: &Lc, input_id: Option<&str>, required: bool, cx: &Context) -> Markup {
/// let label = form::parts::Label::new(label, input_id).with_required(required);
/// html! {
///     (label.render(cx))
///     input type="text" id=[input_id] required[required];
/// }
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Label<'a> {
    text: &'a Lc,
    for_id: Option<&'a str>,
    id: Option<String>,
    class: CowStr,
    required: bool,
}

impl<'a> Label<'a> {
    /// Etiqueta del control con identificador `control_id` (atributo `for`), con la clase
    /// `form-label`.
    pub fn new(label: &'a Lc, control_id: Option<&'a str>) -> Self {
        Self::build(label, control_id, None, "form-label")
    }

    /// Texto de una casilla de verificación con identificador `control_id`, que se coloca junto a
    /// ella, con la clase `form-check-label`.
    pub fn check(label: &'a Lc, control_id: Option<&'a str>) -> Self {
        Self::build(label, control_id, None, "form-check-label")
    }

    /// Nombre de un grupo de controles cuyo contenedor tiene el identificador `group_id`, con la
    /// clase `form-label`. No se enlaza con `for` (no hay un único control), sino que recibe el
    /// identificador `{group_id}-label` para que el contenedor lo enlace con `aria-labelledby`.
    pub fn group(label: &'a Lc, group_id: &str) -> Self {
        // Sin texto no se genera la etiqueta, así que tampoco hay identificador al que enlazar.
        let id = (!label.is_none()).then(|| util::join!(group_id, "-label"));
        Self::build(label, None, id, "form-label")
    }

    fn build(
        label: &'a Lc,
        for_id: Option<&'a str>,
        id: Option<String>,
        class: &'static str,
    ) -> Self {
        Self {
            text: label,
            for_id,
            id,
            class: CowStr::Borrowed(class),
            required: false,
        }
    }

    /// Devuelve el identificador para `aria-labelledby` (sólo en [`group()`](Self::group)), o
    /// `None` si el texto es [`Lc::none()`].
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Renderiza la etiqueta traducida al idioma del contexto.
    pub fn render(&self, cx: &Context) -> Markup {
        // Se genera el elemento aunque falte la traducción, para que `id()` no quede sin destino.
        html! {
            @if !self.text.is_none() {
                label for=[self.for_id] id=[self.id()] class=(self.class) {
                    (self.text.lookup(cx).unwrap_or_default())
                    @if self.required {
                        span
                            class="form-required"
                            title=[Lc::l("field_required").lookup(cx)]
                        {
                            "*"
                        }
                    }
                }
            }
        }
    }
}

#[builder_impl]
impl<'a> Label<'a> {
    /// Sustituye la clase de la etiqueta (por defecto, `form-label`, o `form-check-label` en
    /// [`check()`](Self::check)) por otra u otras, separadas por espacios; útil para un tema cuyo
    /// marcado use otras clases.
    pub fn with_class(mut self, class: impl Into<CowStr>) -> Self {
        self.class = class.into();
        self
    }

    /// Indica si el campo es obligatorio, para añadir el asterisco a la etiqueta. El atributo
    /// `required` lo pone el propio control.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}
