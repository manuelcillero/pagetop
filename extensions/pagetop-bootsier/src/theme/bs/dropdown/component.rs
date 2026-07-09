use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;
use crate::theme::*;

/// Componente para crear un **menú desplegable**.
///
/// Renderiza un botón (único o desdoblado, ver [`with_button_split()`](Self::with_button_split))
/// con un menú desplegable de elementos [`dropdown::Item`](crate::theme::bs::dropdown::Item), que
/// se muestra u oculta según la interacción del usuario. Admite variaciones para el tamaño y el
/// color del botón, también para la dirección de apertura, alineación o política de cierre.
///
/// Si no tiene título (ver [`with_title()`](Self::with_title)) se muestra únicamente la lista de
/// elementos sin ningún botón para interactuar.
///
/// Si este componente se usa en un menú [`Nav`](crate::theme::bs::Nav) (ver
/// [`nav::Item::dropdown()`](crate::theme::bs::nav::Item::dropdown)) sólo se tendrán en cuenta **el
/// título** (si no existe le asigna uno por defecto) y **la lista de elementos**; el resto de
/// propiedades no afectarán a su representación en [`Nav`](crate::theme::bs::Nav).
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let dd = bs::Dropdown::new()
///     .with_title(L10n::n("Menu"))
///     .with_button_color(class::ButtonColor::solid(token::Color::Secondary))
///     .with_auto_close(bs::dropdown::AutoClose::ClickableInside)
///     .with_direction(bs::dropdown::Direction::Dropend)
///     .with_item(bs::dropdown::Item::link(L10n::n("Home"), |_| "/".into()))
///     .with_item(bs::dropdown::Item::link_blank(L10n::n("Doc"), |_| "https://docs.rs".into()))
///     .with_item(bs::dropdown::Item::divider())
///     .with_item(bs::dropdown::Item::header(L10n::n("User session")))
///     .with_item(bs::dropdown::Item::button(L10n::n("Sign out")));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Dropdown {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el título del menú desplegable.
    title: L10n,
    /// Devuelve el tamaño configurado del botón.
    button_size: class::ButtonSize,
    /// Devuelve el color/estilo configurado del botón.
    button_color: class::ButtonColor,
    /// Devuelve si se debe desdoblar (*split*) el botón (botón de acción + *toggle*).
    button_split: bool,
    /// Devuelve si el botón del menú está integrado en un grupo de botones.
    button_grouped: bool,
    /// Devuelve la política de cierre automático del menú desplegado.
    auto_close: bs::dropdown::AutoClose,
    /// Devuelve la dirección de despliegue configurada.
    direction: bs::dropdown::Direction,
    /// Devuelve la configuración de alineación horizontal del menú desplegable.
    menu_align: bs::dropdown::MenuAlign,
    /// Devuelve la posición configurada para el menú desplegable.
    menu_position: bs::dropdown::MenuPosition,
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
        self.alter_prop(PropsOp::prepend_classes(
            self.direction().to_class(*self.button_grouped()),
        ));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // Si no hay elementos en el menú, no se prepara.
        let items = self.items().render(cx).await;
        if items.is_empty() {
            return Ok(html! {});
        }

        // Título opcional para el menú desplegable.
        let title = self.title().using(cx);

        Ok(html! {
            div (self.props()) {
                @if !title.is_empty() {
                    @let btn_base = {
                        let mut classes = String::from("btn");
                        self.button_size().push_to(&mut classes);
                        self.button_color().push_to(&mut classes);
                        classes
                    };
                    @let pos = self.menu_position();
                    @let offset = pos.data_offset();
                    @let reference = pos.data_reference();
                    @let auto_close = self.auto_close().opt_str();
                    @let menu_classes = {
                        let mut classes = "dropdown-menu".to_string();
                        self.menu_align().push_to(&mut classes);
                        classes
                    };

                    // Renderizado en modo split (dos botones) o simple (un botón).
                    @if *self.button_split() {
                        // Botón principal (acción/etiqueta).
                        @let btn = html! {
                            button
                                type="button"
                                class=(&btn_base)
                            {
                                (title)
                            }
                        };
                        // Botón *toggle* que abre/cierra el menú asociado.
                        @let btn_toggle_classes =
                            util::join!(&btn_base, " dropdown-toggle dropdown-toggle-split");
                        @let btn_toggle = html! {
                            button
                                type="button"
                                class=(&btn_toggle_classes)
                                data-bs-toggle="dropdown"
                                data-bs-offset=[offset]
                                data-bs-reference=[reference]
                                data-bs-auto-close=[auto_close]
                                aria-expanded="false"
                            {
                                span class="visually-hidden" {
                                    (L10n::t("dropdown_toggle", &LOCALES_BOOTSIER).using(cx))
                                }
                            }
                        };
                        // Orden según dirección (en `dropstart` el *toggle* se sitúa antes).
                        @match self.direction() {
                            bs::dropdown::Direction::Dropstart => {
                                (btn_toggle)
                                ul class=(&menu_classes) { (items) }
                                (btn)
                            }
                            _ => {
                                (btn)
                                (btn_toggle)
                                ul class=(&menu_classes) { (items) }
                            }
                        }
                    } @else {
                        // Botón único con funcionalidad de *toggle*.
                        @let btn_toggle_classes = util::join!(&btn_base, " dropdown-toggle");
                        button
                            type="button"
                            class=(&btn_toggle_classes)
                            data-bs-toggle="dropdown"
                            data-bs-offset=[offset]
                            data-bs-reference=[reference]
                            data-bs-auto-close=[auto_close]
                            aria-expanded="false"
                        {
                            (title)
                        }
                        ul class=(&menu_classes) { (items) }
                    }
                } @else {
                    // Sin botón: sólo el listado como menú contextual.
                    ul class="dropdown-menu" { (items) }
                }
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
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title = title;
        self
    }

    /// Ajusta el tamaño del botón.
    #[builder_fn]
    pub fn with_button_size(mut self, size: class::ButtonSize) -> Self {
        self.button_size = size;
        self
    }

    /// Define el color/estilo del botón.
    #[builder_fn]
    pub fn with_button_color(mut self, color: class::ButtonColor) -> Self {
        self.button_color = color;
        self
    }

    /// Activa/desactiva el modo *split* (botón de acción + *toggle*).
    #[builder_fn]
    pub fn with_button_split(mut self, split: bool) -> Self {
        self.button_split = split;
        self
    }

    /// Indica si el botón del menú está integrado en un grupo de botones.
    #[builder_fn]
    pub fn with_button_grouped(mut self, grouped: bool) -> Self {
        self.button_grouped = grouped;
        self
    }

    /// Establece la política de cierre automático del menú desplegable.
    #[builder_fn]
    pub fn with_auto_close(mut self, auto_close: bs::dropdown::AutoClose) -> Self {
        self.auto_close = auto_close;
        self
    }

    /// Establece la dirección de despliegue del menú.
    #[builder_fn]
    pub fn with_direction(mut self, direction: bs::dropdown::Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Configura la alineación horizontal (con posible comportamiento *responsive* adicional).
    #[builder_fn]
    pub fn with_menu_align(mut self, align: bs::dropdown::MenuAlign) -> Self {
        self.menu_align = align;
        self
    }

    /// Configura la posición del menú.
    #[builder_fn]
    pub fn with_menu_position(mut self, position: bs::dropdown::MenuPosition) -> Self {
        self.menu_position = position;
        self
    }

    /// Añade un nuevo elemento al menú o modifica la lista de elementos del menú con una operación
    /// [`ChildOp`].
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
