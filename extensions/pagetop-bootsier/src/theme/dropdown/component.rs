use pagetop::prelude::*;

use crate::prelude::*;
use crate::LOCALES_BOOTSIER;

/// Componente para crear un **menú desplegable**.
///
/// Renderiza un botón (único o desdoblado, ver [`with_button_split()`](Self::with_button_split))
/// para mostrar un menú desplegable de elementos [`dropdown::Item`], que se muestra/oculta según la
/// interacción del usuario. Admite variaciones de tamaño/color del botón, también dirección de
/// apertura, alineación o política de cierre.
///
/// Si no tiene título (ver [`with_title()`](Self::with_title)) se muestra únicamente la lista de
/// elementos sin ningún botón para interactuar.
///
/// Si este componente se usa en un menú [`Nav`] (ver [`nav::Item::dropdown()`]) sólo se tendrán en
/// cuenta **el título** (si no existe le asigna uno por defecto) y **la lista de elementos**; el
/// resto de propiedades no afectarán a su representación en [`Nav`].
///
/// Ver ejemplo en el módulo [`dropdown`].
/// Si no contiene elementos, el componente **no se renderiza**.
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Dropdown {
    id            : AttrId,
    classes       : AttrClasses,
    title         : L10n,
    button_size   : ButtonSize,
    button_color  : ButtonColor,
    button_split  : bool,
    button_grouped: bool,
    auto_close    : dropdown::AutoClose,
    direction     : dropdown::Direction,
    menu_align    : dropdown::MenuAlign,
    menu_position : dropdown::MenuPosition,
    items         : Children,
}

impl Component for Dropdown {
    fn new() -> Self {
        Dropdown::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            self.direction().class_with(self.button_grouped()),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        // Si no hay elementos en el menú, no se prepara.
        let items = self.items().render(cx);
        if items.is_empty() {
            return PrepareMarkup::None;
        }

        // Título opcional para el menú desplegable.
        let title = self.title().using(cx);

        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] {
                @if !title.is_empty() {
                    @let mut btn_classes = AttrClasses::new({
                        let mut classes = "btn".to_string();
                        self.button_size().push_class(&mut classes);
                        self.button_color().push_class(&mut classes);
                        classes
                    });
                    @let pos = self.menu_position();
                    @let offset = pos.data_offset();
                    @let reference = pos.data_reference();
                    @let auto_close = self.auto_close.as_str();
                    @let menu_classes = AttrClasses::new({
                        let mut classes = "dropdown-menu".to_string();
                        self.menu_align().push_class(&mut classes);
                        classes
                    });

                    // Renderizado en modo split (dos botones) o simple (un botón).
                    @if self.button_split() {
                        // Botón principal (acción/etiqueta).
                        @let btn = html! {
                            button
                                type="button"
                                class=[btn_classes.get()]
                            {
                                (title)
                            }
                        };
                        // Botón *toggle* que abre/cierra el menú asociado.
                        @let btn_toggle = html! {
                            button
                                type="button"
                                class=[btn_classes.alter_value(
                                    ClassesOp::Add, "dropdown-toggle dropdown-toggle-split"
                                ).get()]
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
                            dropdown::Direction::Dropstart => {
                                (btn_toggle)
                                ul class=[menu_classes.get()] { (items) }
                                (btn)
                            }
                            _ => {
                                (btn)
                                (btn_toggle)
                                ul class=[menu_classes.get()] { (items) }
                            }
                        }
                    } @else {
                        // Botón único con funcionalidad de *toggle*.
                        button
                            type="button"
                            class=[btn_classes.alter_value(
                                ClassesOp::Add, "dropdown-toggle"
                            ).get()]
                            data-bs-toggle="dropdown"
                            data-bs-offset=[offset]
                            data-bs-reference=[reference]
                            data-bs-auto-close=[auto_close]
                            aria-expanded="false"
                        {
                            (title)
                        }
                        ul class=[menu_classes.get()] { (items) }
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

    /// Establece el identificador único (`id`) del menú desplegable.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al menú desplegable.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
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
    pub fn with_button_size(mut self, size: ButtonSize) -> Self {
        self.button_size = size;
        self
    }

    /// Define el color/estilo del botón.
    #[builder_fn]
    pub fn with_button_color(mut self, color: ButtonColor) -> Self {
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
    pub fn with_auto_close(mut self, auto_close: dropdown::AutoClose) -> Self {
        self.auto_close = auto_close;
        self
    }

    /// Establece la dirección de despliegue del menú.
    #[builder_fn]
    pub fn with_direction(mut self, direction: dropdown::Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Configura la alineación horizontal (con posible comportamiento *responsive* adicional).
    #[builder_fn]
    pub fn with_menu_align(mut self, align: dropdown::MenuAlign) -> Self {
        self.menu_align = align;
        self
    }

    /// Configura la posición del menú.
    #[builder_fn]
    pub fn with_menu_position(mut self, position: dropdown::MenuPosition) -> Self {
        self.menu_position = position;
        self
    }

    /// Añade un nuevo elemento hijo al menú.
    #[inline]
    pub fn add_item(mut self, item: dropdown::Item) -> Self {
        self.items.add(Child::with(item));
        self
    }

    /// Modifica la lista de elementos (`children`) aplicando una operación [`TypedOp`].
    #[builder_fn]
    pub fn with_items(mut self, op: TypedOp<dropdown::Item>) -> Self {
        self.items.alter_typed(op);
        self
    }

    // **< Dropdown GETTERS >***********************************************************************

    /// Devuelve las clases CSS asociadas al menú desplegable.
    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    /// Devuelve el título del menú desplegable.
    pub fn title(&self) -> &L10n {
        &self.title
    }

    /// Devuelve el tamaño configurado del botón.
    pub fn button_size(&self) -> &ButtonSize {
        &self.button_size
    }

    /// Devuelve el color/estilo configurado del botón.
    pub fn button_color(&self) -> &ButtonColor {
        &self.button_color
    }

    /// Devuelve si se debe desdoblar (*split*) el botón (botón de acción + *toggle*).
    pub fn button_split(&self) -> bool {
        self.button_split
    }

    /// Devuelve si el botón del menú está integrado en un grupo de botones.
    pub fn button_grouped(&self) -> bool {
        self.button_grouped
    }

    /// Devuelve la política de cierre automático del menú desplegado.
    pub fn auto_close(&self) -> &dropdown::AutoClose {
        &self.auto_close
    }

    /// Devuelve la dirección de despliegue configurada.
    pub fn direction(&self) -> &dropdown::Direction {
        &self.direction
    }

    /// Devuelve la configuración de alineación horizontal del menú desplegable.
    pub fn menu_align(&self) -> &dropdown::MenuAlign {
        &self.menu_align
    }

    /// Devuelve la posición configurada para el menú desplegable.
    pub fn menu_position(&self) -> &dropdown::MenuPosition {
        &self.menu_position
    }

    /// Devuelve la lista de elementos (`children`) del menú.
    pub fn items(&self) -> &Children {
        &self.items
    }
}
