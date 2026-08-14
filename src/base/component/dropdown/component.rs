use crate::prelude::*;

/// Componente para crear un **menú de acciones desplegable**.
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
    /// Devuelve si el botón se desdobla (*split*) en botón de acción + *toggle*.
    button_split: bool,
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

        let toggle_label = Lc::l("dropdown_toggle").using(cx);

        Ok(html! {
            div (self.props()) {
                @if *self.button_split() {
                    button type="button" class="dropdown-button" { (&title) }
                    button
                        type="button"
                        class="dropdown-toggle"
                        aria-haspopup="true"
                        aria-expanded="false"
                    {
                        span class="visually-hidden" { (toggle_label) }
                    }
                } @else {
                    button
                        type="button"
                        class="dropdown-toggle"
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

    /// Activa/desactiva el modo *split* (botón de acción + *toggle*).
    #[builder_fn]
    pub fn with_button_split(mut self, split: bool) -> Self {
        self.button_split = split;
        self
    }

    /// Añade un nuevo elemento al menú o modifica la lista de elementos del menú con una
    /// operación [`ChildOp`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// dropdown.with_item(dropdown::Item::link("Opción", "/ruta"));
    /// dropdown.with_item(ChildOp::AddMany(vec![
    ///     dropdown::Item::link(...).into(),
    ///     dropdown::Item::divider().into(),
    ///     dropdown::Item::link(...).into(),
    /// ]));
    /// ```
    #[builder_fn]
    pub fn with_item(mut self, op: impl Into<ChildOp>) -> Self {
        self.items.alter_child(op.into());
        self
    }
}
