use crate::prelude::*;

/// Componente para crear un **menú desplegable**.
///
/// Renderiza un botón (único o desdoblado, ver [`with_button_split()`](Self::with_button_split))
/// que despliega u oculta una lista de elementos [`dropdown::Item`](super::Item).
///
/// Sin título (ver [`with_title()`](Self::with_title)), se muestra únicamente la lista de
/// elementos, sin ningún botón para interactuar, sólo un menú contextual estático.
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let dd = dropdown::Dropdown::new()
///     .with_title(Lc::n("Menu"))
///     .with_item(dropdown::Item::link(Lc::n("Home"), "/"))
///     .with_item(dropdown::Item::link_blank(Lc::n("Doc"), "https://docs.rs"))
///     .with_item(dropdown::Item::divider())
///     .with_item(dropdown::Item::header(Lc::n("User session")))
///     .with_item(dropdown::Item::button(Lc::n("Sign out")));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Dropdown {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el título del menú desplegable.
    title: Lc,
    /// Devuelve si el botón se desdobla (*split*) en botón de acción más *toggle*.
    button_split: bool,
    /// Devuelve el tamaño visual del botón.
    #[getters(copy)]
    button_size: button::Size,
    /// Devuelve el estilo visual del botón.
    #[getters(copy)]
    button_style: button::Style,
    /// Devuelve la lista de elementos del menú.
    items: Children,
}

#[async_trait]
impl Component for Dropdown {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes("dropdown"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // Si no hay elementos en el menú, no se prepara.
        let items = self.items().render(cx).await;
        if items.is_empty() {
            return Ok(html! {});
        }

        let title = self.title().using(cx);

        // Sin título: menú contextual estático, sin botón ni comportamiento de apertura/cierre.
        if title.is_empty() {
            return Ok(html! {
                ul class="dropdown-menu" { (items) }
            });
        }

        // Sin tamaño ni estilo (caso más común), evita construir dinámicamente las clases.
        let (button_classes, toggle_classes): (CowStr, CowStr) =
            if matches!(self.button_size(), button::Size::None)
                && matches!(self.button_style(), button::Style::None)
            {
                (
                    CowStr::from("dropdown-button"),
                    CowStr::from("dropdown-toggle"),
                )
            } else {
                use button::{Size, Style};

                let size_class = match self.button_size() {
                    Size::None => "",
                    Size::Small => " button-sm",
                    Size::Large => " button-lg",
                };
                let style_class = match self.button_style() {
                    Style::None => String::new(),
                    Style::Solid(intent) => util::join!(" button-", intent.color(cx)),
                    Style::Outline(intent) => util::join!(" button-outline-", intent.color(cx)),
                    Style::Link => " button-link".to_string(),
                };
                (
                    util::join!("dropdown-button", size_class, &style_class).into(),
                    util::join!("dropdown-toggle", size_class, &style_class).into(),
                )
            };

        let toggle_label = Lc::l("dropdown_toggle").using(cx);

        Ok(html! {
            div (self.props()) {
                @if *self.button_split() {
                    button type="button" class=(&button_classes) { (&title) }
                    button
                        type="button"
                        class=(&toggle_classes)
                        aria-haspopup="true"
                        aria-expanded="false"
                    {
                        span class="visually-hidden" { (toggle_label) }
                    }
                } @else {
                    button
                        type="button"
                        class=(&toggle_classes)
                        aria-haspopup="true"
                        aria-expanded="false"
                    {
                        (&title)
                    }
                }
                ul class="dropdown-menu" { (items) }
            }
        })
    }
}

impl Dropdown {
    // **< Dropdown BUILDER >***********************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece el título del menú desplegable.
    #[builder_fn]
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Activa/desactiva el modo *split* (botón de acción más *toggle*).
    #[builder_fn]
    pub fn with_button_split(mut self, split: bool) -> Self {
        self.button_split = split;
        self
    }

    /// Establece el tamaño visual del botón (usa [`button::Size::None`] para quitarlo).
    #[builder_fn]
    pub fn with_button_size(mut self, size: button::Size) -> Self {
        self.button_size = size;
        self
    }

    /// Establece el estilo visual del botón (usa [`button::Style::None`] para quitarlo).
    #[builder_fn]
    pub fn with_button_style(mut self, style: button::Style) -> Self {
        self.button_style = style;
        self
    }

    /// Añade un nuevo elemento al menú o modifica la lista de elementos del menú con una operación
    /// [`TypedOp`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    ///
    /// let dd = dropdown::Dropdown::new()
    ///     .with_item(dropdown::Item::link(Lc::n("Option"), "/path/to/option"))
    ///     .with_item(TypedOp::AddMany(vec![
    ///         dropdown::Item::link(Lc::n("Other"), "/path/to/other"),
    ///         dropdown::Item::divider(),
    ///         dropdown::Item::link(Lc::n("Home"), "/"),
    ///     ]));
    /// ```
    #[builder_fn]
    pub fn with_item(mut self, op: impl Into<TypedOp<dropdown::Item>>) -> Self {
        self.items.alter_child(op.into());
        self
    }
}
