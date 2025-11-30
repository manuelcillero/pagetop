use pagetop::prelude::*;

// **< ItemKind >***********************************************************************************

/// Tipos de [`dropdown::Item`](crate::theme::dropdown::Item) disponibles en un menú desplegable
/// [`Dropdown`](crate::theme::Dropdown).
///
/// Define internamente la naturaleza del elemento y su comportamiento al mostrarse o interactuar
/// con él.
#[derive(AutoDefault)]
pub enum ItemKind {
    /// Elemento vacío, no produce salida.
    #[default]
    Void,
    /// Etiqueta sin comportamiento interactivo.
    Label(L10n),
    /// Elemento de navegación. Opcionalmente puede abrirse en una nueva ventana y estar
    /// inicialmente deshabilitado.
    Link {
        label: L10n,
        path: FnPathByContext,
        blank: bool,
        disabled: bool,
    },
    /// Acción ejecutable en la propia página, sin navegación asociada. Inicialmente puede estar
    /// deshabilitado.
    Button { label: L10n, disabled: bool },
    /// Título o encabezado que separa grupos de opciones.
    Header(L10n),
    /// Separador visual entre bloques de elementos.
    Divider,
}

// **< Item >***************************************************************************************

/// Representa un **elemento individual** de un menú desplegable
/// [`Dropdown`](crate::theme::Dropdown).
///
/// Cada instancia de [`dropdown::Item`](crate::theme::dropdown::Item) se traduce en un componente
/// visible que puede comportarse como texto, enlace, botón, encabezado o separador, según su
/// [`ItemKind`].
///
/// Permite definir identificador, clases de estilo adicionales o tipo de interacción asociada,
/// manteniendo una interfaz común para renderizar todos los elementos del menú.
#[derive(AutoDefault, Getters)]
pub struct Item {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS asociadas al elemento.
    classes: AttrClasses,
    /// Devuelve el tipo de elemento representado.
    item_kind: ItemKind,
}

impl Component for Item {
    fn new() -> Self {
        Item::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.item_kind() {
            ItemKind::Void => PrepareMarkup::None,

            ItemKind::Label(label) => PrepareMarkup::With(html! {
                li id=[self.id()] class=[self.classes().get()] {
                    span class="dropdown-item-text" {
                        (label.using(cx))
                    }
                }
            }),

            ItemKind::Link {
                label,
                path,
                blank,
                disabled,
            } => {
                let path = path(cx);
                let current_path = cx.request().map(|request| request.path());
                let is_current = !*disabled && (current_path == Some(path));

                let mut classes = "dropdown-item".to_string();
                if is_current {
                    classes.push_str(" active");
                }
                if *disabled {
                    classes.push_str(" disabled");
                }

                let href = (!disabled).then_some(path);
                let target = (!disabled && *blank).then_some("_blank");
                let rel = (!disabled && *blank).then_some("noopener noreferrer");

                let aria_current = (href.is_some() && is_current).then_some("page");
                let aria_disabled = disabled.then_some("true");
                let tabindex = disabled.then_some("-1");

                PrepareMarkup::With(html! {
                    li id=[self.id()] class=[self.classes().get()] {
                        a
                            class=(classes)
                            href=[href]
                            target=[target]
                            rel=[rel]
                            aria-current=[aria_current]
                            aria-disabled=[aria_disabled]
                            tabindex=[tabindex]
                        {
                            (label.using(cx))
                        }
                    }
                })
            }

            ItemKind::Button { label, disabled } => {
                let mut classes = "dropdown-item".to_string();
                if *disabled {
                    classes.push_str(" disabled");
                }

                let aria_disabled = disabled.then_some("true");
                let disabled_attr = disabled.then_some("disabled");

                PrepareMarkup::With(html! {
                    li id=[self.id()] class=[self.classes().get()] {
                        button
                            class=(classes)
                            type="button"
                            aria-disabled=[aria_disabled]
                            disabled=[disabled_attr]
                        {
                            (label.using(cx))
                        }
                    }
                })
            }

            ItemKind::Header(label) => PrepareMarkup::With(html! {
                li id=[self.id()] class=[self.classes().get()] {
                    h6 class="dropdown-header" {
                        (label.using(cx))
                    }
                }
            }),

            ItemKind::Divider => PrepareMarkup::With(html! {
                li id=[self.id()] class=[self.classes().get()] { hr class="dropdown-divider" {} }
            }),
        }
    }
}

impl Item {
    /// Crea un elemento de tipo texto, mostrado sin interacción.
    pub fn label(label: L10n) -> Self {
        Item {
            item_kind: ItemKind::Label(label),
            ..Default::default()
        }
    }

    /// Crea un enlace para la navegación.
    pub fn link(label: L10n, path: FnPathByContext) -> Self {
        Item {
            item_kind: ItemKind::Link {
                label,
                path,
                blank: false,
                disabled: false,
            },
            ..Default::default()
        }
    }

    /// Crea un enlace deshabilitado que no permite la interacción.
    pub fn link_disabled(label: L10n, path: FnPathByContext) -> Self {
        Item {
            item_kind: ItemKind::Link {
                label,
                path,
                blank: false,
                disabled: true,
            },
            ..Default::default()
        }
    }

    /// Crea un enlace que se abre en una nueva ventana o pestaña.
    pub fn link_blank(label: L10n, path: FnPathByContext) -> Self {
        Item {
            item_kind: ItemKind::Link {
                label,
                path,
                blank: true,
                disabled: false,
            },
            ..Default::default()
        }
    }

    /// Crea un enlace inicialmente deshabilitado que se abriría en una nueva ventana.
    pub fn link_blank_disabled(label: L10n, path: FnPathByContext) -> Self {
        Item {
            item_kind: ItemKind::Link {
                label,
                path,
                blank: true,
                disabled: true,
            },
            ..Default::default()
        }
    }

    /// Crea un botón de acción local, sin navegación asociada.
    pub fn button(label: L10n) -> Self {
        Item {
            item_kind: ItemKind::Button {
                label,
                disabled: false,
            },
            ..Default::default()
        }
    }

    /// Crea un botón deshabilitado.
    pub fn button_disabled(label: L10n) -> Self {
        Item {
            item_kind: ItemKind::Button {
                label,
                disabled: true,
            },
            ..Default::default()
        }
    }

    /// Crea un encabezado para un grupo de elementos dentro del menú.
    pub fn header(label: L10n) -> Self {
        Item {
            item_kind: ItemKind::Header(label),
            ..Default::default()
        }
    }

    /// Crea un separador visual entre bloques de elementos.
    pub fn divider() -> Self {
        Item {
            item_kind: ItemKind::Divider,
            ..Default::default()
        }
    }

    // **< Item BUILDER >***************************************************************************

    /// Establece el identificador único (`id`) del elemento.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al elemento.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }
}
