use crate::prelude::*;

/// Componente estructural que renderiza el contenido de una región del documento.
///
/// `Region` actúa como un contenedor lógico asociado a un nombre de región. Su contenido se obtiene
/// del contexto de renderizado ([`Context`]), donde los componentes suelen registrarse con métodos
/// como [`Contextual::with_child_in()`]. Cada región puede integrarse posteriormente en el cuerpo
/// del documento mediante [`Template`], normalmente desde una página ([`Page`]).
#[derive(AutoDefault)]
pub struct Region {
    #[default(AttrName::new(Self::DEFAULT))]
    name: AttrName,
    #[default(L10n::l("region-content"))]
    label: L10n,
}

impl Component for Region {
    fn new() -> Self {
        Region::default()
    }

    fn id(&self) -> Option<String> {
        self.name.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let Some(name) = self.name().get() else {
            return PrepareMarkup::None;
        };
        let output = cx.render_region(&name);
        if output.is_empty() {
            return PrepareMarkup::None;
        }
        PrepareMarkup::With(html! {
            div
                id=[self.id()]
                class=(join!("region region-", &name))
                role="region"
                aria-label=[self.label().lookup(cx)]
            {
                (output)
            }
        })
    }
}

impl Region {
    /// Región especial situada al **inicio del documento**.
    ///
    /// Su función es proporcionar un punto estable donde las extensiones puedan inyectar contenido
    /// global antes de renderizar el resto de regiones principales (cabecera, contenido, etc.).
    ///
    /// No suele utilizarse en los temas como una región “visible” dentro del maquetado habitual,
    /// sino como punto de anclaje para elementos auxiliares, marcadores técnicos, inicializadores o
    /// contenido de depuración que deban situarse en la parte superior del documento.
    ///
    /// Se considera una región **reservada** para este tipo de usos globales.
    pub const PAGETOP: &str = "page-top";

    /// Región estándar para la **cabecera** del documento.
    ///
    /// Suele emplearse para mostrar un logotipo, navegación principal, barras superiores, etc.
    pub const HEADER: &str = "header";

    /// Región principal de **contenido**.
    ///
    /// Es la región donde se espera que se renderice el contenido principal de la página (p. ej.
    /// cuerpo de la ruta actual, bloques centrales, vistas principales, etc.). En muchos temas será
    /// la región mínima imprescindible para que la página tenga sentido.
    pub const CONTENT: &str = "content";

    /// Región estándar para el **pie de página**.
    ///
    /// Suele contener información legal, enlaces secundarios, créditos, etc.
    pub const FOOTER: &str = "footer";

    /// Región especial situada al **final del documento**.
    ///
    /// Pensada para proporcionar un punto estable donde las extensiones puedan inyectar contenido
    /// global después de renderizar el resto de regiones principales (cabecera, contenido, etc.).
    ///
    /// No suele utilizarse en los temas como una región “visible” dentro del maquetado habitual,
    /// sino como punto de anclaje para elementos auxiliares asociados a comportamientos dinámicos
    /// que deban situarse en la parte inferior del documento.
    ///
    /// Igual que [`Self::PAGETOP`], se considera una región **reservada** para este tipo de usos
    /// globales.
    pub const PAGEBOTTOM: &str = "page-bottom";

    /// Región por defecto que se asigna cuando no se especifica ningún nombre.
    ///
    /// Por diseño, la región por defecto es la de contenido principal ([`Self::CONTENT`]), de
    /// manera que un tema sencillo pueda limitarse a definir una sola región funcional.
    pub const DEFAULT: &str = Self::CONTENT;

    /// Prepara una región para el nombre indicado.
    ///
    /// El valor de `name` se utiliza como nombre de la región y como identificador (`id`) del
    /// contenedor. Al renderizarse, este componente mostrará el contenido registrado en el contexto
    /// bajo ese nombre.
    pub fn named(name: impl AsRef<str>) -> Self {
        Region {
            name: AttrName::new(name),
            label: L10n::default(),
        }
    }

    /// Prepara una región para el nombre indicado con una etiqueta de accesibilidad.
    ///
    /// El valor de `name` se utiliza como nombre de la región y como identificador (`id`) del
    /// contenedor, mientras que `label` será el texto localizado que se usará como `aria-label` del
    /// contenedor.
    pub fn labeled(name: impl AsRef<str>, label: L10n) -> Self {
        Region {
            name: AttrName::new(name),
            label,
        }
    }

    // **< Region BUILDER >*************************************************************************

    /// Establece o modifica el nombre de la región.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_value(name);
        self
    }

    /// Establece la etiqueta localizada de la región.
    ///
    /// Esta etiqueta se utiliza como `aria-label` del contenedor predefinido `<div role="region">`,
    /// lo que mejora la accesibilidad para lectores de pantalla y otras tecnologías de apoyo.
    #[builder_fn]
    pub fn with_label(mut self, label: L10n) -> Self {
        self.label = label;
        self
    }

    // **< Region GETTERS >*************************************************************************

    /// Devuelve el nombre de la región.
    pub fn name(&self) -> &AttrName {
        &self.name
    }

    /// Devuelve la etiqueta localizada asociada a la región.
    pub fn label(&self) -> &L10n {
        &self.label
    }
}
