use crate::prelude::*;

use std::fmt;

/// Componente que renderiza el cuerpo de una plantilla de regiones.
///
/// La composición por defecto usa el componente [`Region`](crate::base::component::layout::Region)
/// para mostrar, en este orden, las regiones [`CoreRegion::Header`], [`CoreRegion::Content`] y
/// [`CoreRegion::Footer`].
///
/// No incluye las regiones reservadas
/// [`ReservedRegion::PageTop`](crate::response::ReservedRegion::PageTop) y
/// [`ReservedRegion::PageBottom`](crate::response::ReservedRegion::PageBottom) porque el propio
/// [`Page::render()`](crate::response::Page::render) las añade antes y después del resultado de
/// [`Theme::render_page_body()`](crate::core::theme::Theme::render_page_body) para que se
/// rendericen siempre, independientemente de la plantilla que se use.
///
/// Si un tema necesita maquetar una plantilla determinada de forma distinta, puede capturar este
/// componente en [`Theme::handle_component()`](crate::core::theme::Theme::handle_component) y hacer
/// [`downcast_ref()`](crate::core::AnyCast::downcast_ref) sobre el [`TemplateRef`] que devuelve
/// [`Self::template()`], para compararlo con la variante deseada.
///
/// Como cualquier otro componente, participa también en el despacho de las
/// [acciones de componentes](crate::base::action::component) para que otras extensiones puedan
/// intervenir en su renderizado.
#[derive(Clone, Getters)]
pub struct Template {
    /// Devuelve la plantilla subyacente.
    #[getters(copy)]
    template: TemplateRef,
}

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Template")
            .field("template", &self.template().name())
            .finish()
    }
}

impl Default for Template {
    fn default() -> Self {
        Template {
            template: &CoreTemplate::Standard,
        }
    }
}

#[async_trait]
impl Component for Template {
    fn new() -> Self {
        Self::default()
    }

    /// Devuelve el nombre de la plantilla subyacente como identificador del componente.
    fn id(&self) -> Option<String> {
        Some(self.template().name().to_owned())
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {
            (layout::Region::header().render(cx).await)
            (layout::Region::default().render(cx).await)
            (layout::Region::footer().render(cx).await)
        })
    }
}

impl Template {
    /// Define el componente que renderizará [`CoreTemplate::Admin`].
    pub fn admin() -> Self {
        Template {
            template: &CoreTemplate::Admin,
        }
    }

    /// Define el componente que renderizará la plantilla indicada.
    pub fn of(template: TemplateRef) -> Self {
        Template { template }
    }
}
