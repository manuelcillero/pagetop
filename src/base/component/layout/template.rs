use crate::prelude::*;

use std::fmt;

/// Componente que renderiza el cuerpo de una plantilla de regiones.
///
/// La composición por defecto usa el componente [`Region`] para mostrar, en este orden, las
/// regiones [`CoreRegions::Header`], [`CoreRegions::Aside`], [`CoreRegions::Content`] y
/// [`CoreRegions::Footer`], envueltas en un contenedor `div.wrapper` que un tema puede maquetar a
/// su gusto (por ejemplo, usando CSS Grid, para que `Aside` se muestre como columna lateral junto a
/// `Content`). Si `Aside`, o cualquier otra región, no tiene contenido, no se renderiza.
///
/// No incluye las regiones reservadas ([`ReservedRegions::PageTop`] y
/// [`ReservedRegions::PageBottom`]) porque el propio [`Page::render()`] las añade antes y después
/// del resultado de [`Theme::render_page_body()`] para que se rendericen siempre,
/// independientemente de la plantilla que se use.
///
/// Si un tema necesita maquetar una plantilla determinada de forma distinta, puede capturar este
/// componente en [`Theme::handle_component()`] y hacer [`downcast_ref()`] sobre el [`TemplateRef`]
/// que devuelve [`Self::template()`], para compararlo con la variante deseada.
///
/// Como cualquier otro componente, participa también en el despacho de las
/// [acciones de componentes](crate::base::action::component) para que otras extensiones puedan
/// intervenir en su renderizado.
///
/// [`Region`]: crate::base::component::layout::Region
/// [`ReservedRegions::PageTop`]: crate::response::ReservedRegions::PageTop
/// [`ReservedRegions::PageBottom`]: crate::response::ReservedRegions::PageBottom
/// [`Page::render()`]: crate::response::Page::render
/// [`Theme::render_page_body()`]: crate::core::theme::Theme::render_page_body
/// [`Theme::handle_component()`]: crate::core::theme::Theme::handle_component
/// [`downcast_ref()`]: crate::core::AnyCast::downcast_ref
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
            template: &CoreTemplates::Standard,
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
        let body = html! {
            (layout::Region::header().render(cx).await)
            (layout::Region::aside().render(cx).await)
            (layout::Region::content().render(cx).await)
            (layout::Region::footer().render(cx).await)
        };

        if body.is_empty() {
            return Ok(html! {});
        }

        Ok(html! {
            div.wrapper {
                (body)
            }
        })
    }
}

impl Template {
    /// Define el componente que renderizará [`CoreTemplates::Standard`].
    ///
    /// Equivale a [`Self::new()`] o [`Self::default()`], que ya usan esta plantilla por defecto.
    pub fn standard() -> Self {
        Template {
            template: &CoreTemplates::Standard,
        }
    }

    /// Define el componente que renderizará [`CoreTemplates::Admin`].
    pub fn admin() -> Self {
        Template {
            template: &CoreTemplates::Admin,
        }
    }

    /// Define el componente que renderizará la plantilla indicada.
    pub fn of(template: TemplateRef) -> Self {
        Template { template }
    }
}
