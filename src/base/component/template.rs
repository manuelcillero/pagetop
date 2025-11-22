use crate::prelude::*;

/// Componente estructural para renderizar plantillas de contenido.
///
/// `Template` describe cómo se compone el cuerpo del documento a partir de varias regiones lógicas
/// ([`Region`]). En función de su nombre, decide qué regiones se renderizan y en qué orden.
///
/// Normalmente se invoca desde una página ([`Page`]), que consulta el nombre de plantilla guardado
/// en el [`Context`] y delega en `Template` la composición de las regiones que forman el cuerpo del
/// documento.
///
/// Los temas pueden sobrescribir este componente para exponer sus propias plantillas o adaptar las
/// plantillas predeterminadas.
#[derive(AutoDefault)]
pub struct Template {
    #[default(AttrName::new(Self::DEFAULT))]
    name: AttrName,
}

impl Component for Template {
    fn new() -> Self {
        Template::default()
    }

    fn id(&self) -> Option<String> {
        self.name.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let Some(name) = self.name().get() else {
            return PrepareMarkup::None;
        };
        match name.as_str() {
            Self::DEFAULT | Self::ERROR => PrepareMarkup::With(html! {
                (Region::labeled(Region::HEADER, L10n::l("region-header")).render(cx))
                (Region::default().render(cx))
                (Region::labeled(Region::FOOTER, L10n::l("region-footer")).render(cx))
            }),
            _ => PrepareMarkup::None,
        }
    }
}

impl Template {
    /// Nombre de la plantilla predeterminada.
    ///
    /// Por defecto define una estructura básica con las regiones [`Region::HEADER`],
    /// [`Region::CONTENT`] y [`Region::FOOTER`], en ese orden. Esta plantilla se usa cuando no se
    /// selecciona ninguna otra de forma explícita (ver [`Contextual::with_template()`]).
    pub const DEFAULT: &str = "default";

    /// Nombre de la plantilla de error.
    ///
    /// Se utiliza para páginas de error u otros estados excepcionales. Por defecto reutiliza
    /// la misma estructura que [`Self::DEFAULT`], pero permite a temas y extensiones distinguir
    /// el contexto de error para aplicar estilos o contenidos específicos.
    pub const ERROR: &str = "error";

    /// Selecciona la plantilla asociada al nombre indicado.
    ///
    /// El valor de `name` se utiliza como nombre de la plantilla y como identificador (`id`) del
    /// componente.
    pub fn named(name: impl AsRef<str>) -> Self {
        Template {
            name: AttrName::new(name),
        }
    }

    // **< Template BUILDER >***********************************************************************

    /// Establece o modifica el nombre de la plantilla seleccionada.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_value(name);
        self
    }

    // **< Template GETTERS >***********************************************************************

    /// Devuelve el nombre de la plantilla seleccionada.
    pub fn name(&self) -> &AttrName {
        &self.name
    }
}
