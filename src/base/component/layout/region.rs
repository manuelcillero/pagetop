use crate::prelude::*;

use std::fmt;

/// Componente que renderiza una región del `<body>`.
///
/// No recibe ningún contenido de quien lo construye. Lo obtiene directamente del [`Context`] en el
/// momento de renderizarse (ver [`Context::render_region()`]). Si la región no tiene contenido, no
/// se renderiza nada.
///
/// Si un tema necesita maquetar una región determinada de forma distinta, puede capturar este
/// componente en [`Theme::handle_component()`](crate::core::theme::Theme::handle_component) y hacer
/// [`downcast_ref()`](crate::core::AnyCast::downcast_ref) sobre el [`RegionRef`] que devuelve
/// [`Self::region()`], para compararlo con la variante deseada.
///
/// Como cualquier otro componente, participa también en el despacho de las
/// [acciones de componentes](crate::base::action::component) para que otras extensiones puedan
/// intervenir en su renderizado.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// struct Sidebar;
///
/// impl RegionName for Sidebar {
///     fn name(&self) -> &'static str {
///         "sidebar"
///     }
///
///     fn label(&self) -> L10n {
///         L10n::n("Sidebar")
///     }
/// }
///
/// let header = layout::Region::header();
/// let sidebar = layout::Region::of(&Sidebar);
/// ```
#[derive(Clone, Getters)]
pub struct Region {
    /// Devuelve la región subyacente.
    #[getters(copy)]
    region: RegionRef,
}

impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Region")
            .field("region", &self.region().name())
            .finish()
    }
}

impl Default for Region {
    fn default() -> Self {
        Region {
            region: &CoreRegion::Content,
        }
    }
}

#[async_trait]
impl Component for Region {
    fn new() -> Self {
        Self::default()
    }

    /// Devuelve el nombre de la región subyacente como identificador del componente.
    fn id(&self) -> Option<String> {
        Some(self.region().name().to_owned())
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let name = self.region().name();
        let content = cx.render_region(self.region()).await;
        Ok(html! {
            @if !content.is_empty() {
                div
                    id=[self.id()]
                    class=(util::join!("region region-", name))
                    role="region"
                    aria-label=[self.region().label().lookup(cx)]
                {
                    (content)
                }
            }
        })
    }
}

impl Region {
    /// Define el componente que renderizará [`CoreRegion::Header`].
    pub fn header() -> Self {
        Region {
            region: &CoreRegion::Header,
        }
    }

    /// Define el componente que renderizará [`CoreRegion::Footer`].
    pub fn footer() -> Self {
        Region {
            region: &CoreRegion::Footer,
        }
    }

    /// Define el componente que renderizará la región indicada.
    pub fn of(region: RegionRef) -> Self {
        Region { region }
    }
}
