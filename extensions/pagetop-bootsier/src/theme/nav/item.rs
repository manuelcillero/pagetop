use pagetop::prelude::*;

use crate::prelude::*;
use crate::LOCALES_BOOTSIER;

// **< ItemKind >***********************************************************************************

/// Tipos de [`nav::Item`](crate::theme::nav::Item) disponibles en un menú
/// [`Nav`](crate::theme::Nav).
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
    /// Elemento que despliega un menú [`Dropdown`].
    Dropdown(Typed<Dropdown>),
}

// **< Item >***************************************************************************************

/// Representa un **elemento individual** de un menú [`Nav`](crate::theme::Nav).
///
/// Cada instancia de [`nav::Item`](crate::theme::nav::Item) se traduce en un componente visible que
/// puede comportarse como texto, enlace, botón o menú desplegable según su [`ItemKind`].
///
/// Permite definir identificador, clases de estilo adicionales o tipo de interacción asociada,
/// manteniendo una interfaz común para renderizar todos los elementos del menú.
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Item {
    id       : AttrId,
    classes  : AttrClasses,
    item_kind: ItemKind,
}

impl Component for Item {
    fn new() -> Self {
        Item::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            if matches!(self.item_kind(), ItemKind::Dropdown(_)) {
                "nav-item dropdown"
            } else {
                "nav-item"
            },
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.item_kind() {
            ItemKind::Void => PrepareMarkup::None,

            ItemKind::Label(label) => PrepareMarkup::With(html! {
                li id=[self.id()] class=[self.classes().get()] {
                    span {
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
                let is_current = !*disabled && current_path.map_or(false, |p| p == path);

                let mut classes = "nav-link".to_string();
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

                PrepareMarkup::With(html! {
                    li id=[self.id()] class=[self.classes().get()] {
                        a
                            class=(classes)
                            href=[href]
                            target=[target]
                            rel=[rel]
                            aria-current=[aria_current]
                            aria-disabled=[aria_disabled]
                        {
                            (label.using(cx))
                        }
                    }
                })
            }

            ItemKind::Dropdown(menu) => {
                if let Some(dd) = menu.borrow() {
                    let items = dd.items().render(cx);
                    if items.is_empty() {
                        return PrepareMarkup::None;
                    }
                    let title = dd.title().lookup(cx).unwrap_or_else(|| {
                        L10n::t("dropdown", &LOCALES_BOOTSIER)
                            .lookup(cx)
                            .unwrap_or_else(|| "Dropdown".to_string())
                    });
                    PrepareMarkup::With(html! {
                        li id=[self.id()] class=[self.classes().get()] {
                            a
                                class="nav-link dropdown-toggle"
                                data-bs-toggle="dropdown"
                                href="#"
                                role="button"
                                aria-expanded="false"
                            {
                                (title)
                            }
                            ul class="dropdown-menu" {
                                (items)
                            }
                        }
                    })
                } else {
                    PrepareMarkup::None
                }
            }
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

    /// Crea un elemento de navegación que contiene un menú desplegable [`Dropdown`].
    ///
    /// Sólo se tienen en cuenta **el título** (si no existe le asigna uno por defecto) y **la lista
    /// de elementos** del [`Dropdown`]; el resto de propiedades del componente no afectarán a su
    /// representación en [`Nav`].
    pub fn dropdown(menu: Dropdown) -> Self {
        Item {
            item_kind: ItemKind::Dropdown(Typed::with(menu)),
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

    // **< Item GETTERS >***************************************************************************

    /// Devuelve las clases CSS asociadas al elemento.
    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    /// Devuelve el tipo de elemento representado por este elemento.
    pub fn item_kind(&self) -> &ItemKind {
        &self.item_kind
    }
}
