use crate::prelude::*;

use std::fmt;

/// Componente que renderiza una región del `<body>`.
///
/// No recibe ningún contenido de quien lo construye. Lo obtiene directamente del [`Context`] en el
/// momento de renderizarse (ver [`Context::render_region()`]). Salvo `Content` (ver más abajo), si
/// la región no tiene contenido, no renderiza nada.
///
/// Si un tema necesita maquetar una región determinada de forma distinta, puede capturar este
/// componente en [`Theme::render_component()`](crate::core::theme::Theme::render_component) y
/// hacer [`downcast_ref()`](crate::core::AnyCast::downcast_ref) sobre el [`RegionRef`] que devuelve
/// [`Self::region()`], y compararlo con la variante deseada.
///
/// Por defecto, el renderizado de [`CoreRegions::Content`] añade al inicio un componente
/// [`Messages`] aplazado (ver [`Messages::with_deferred()`]) para mostrar los mensajes pendientes
/// al usuario. Por eso, en una página, `Content` se renderiza siempre, aunque esté vacía y no haya
/// mensajes, ya que queda la marca del componente aplazado como un comentario HTML. Como los
/// elementos que sólo contienen comentarios cumplen el selector CSS `:empty`, un tema puede ocultar
/// las regiones vacías usando la regla `.region:empty { display: none; }`.
///
/// Para llevar los mensajes a otro lugar, un tema puede capturar este componente, renderizar
/// `Content` sin el [`Messages`] y añadirlo a otra región o directamente a la plantilla, con
/// [`Messages::with_deferred()`] para que siga recogiendo todos los mensajes. Si se captura
/// `Content` por otro motivo, hay que incluir el [`Messages`] para no perder los mensajes.
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
///     fn label(&self) -> Lc {
///         Lc::n("Sidebar")
///     }
/// }
///
/// let header = layout::Region::header();
/// let sidebar = layout::Region::of(&Sidebar);
/// ```
///
/// [`Messages`]: crate::base::component::Messages
/// [`Messages::with_deferred()`]: crate::base::component::Messages::with_deferred
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
            region: &CoreRegions::Content,
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
        // Por el nombre, como el contenido de las regiones (ver `RegionName`).
        let content = if name == CoreRegions::Content.name() {
            html! {
                (Messages::new().with_deferred(true).render(cx).await)
                (cx.render_region(self.region()).await)
            }
        } else {
            cx.render_region(self.region()).await
        };
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
    /// Define el componente que renderizará [`CoreRegions::Header`].
    pub fn header() -> Self {
        Region {
            region: &CoreRegions::Header,
        }
    }

    /// Define el componente que renderizará [`CoreRegions::Aside`].
    pub fn aside() -> Self {
        Region {
            region: &CoreRegions::Aside,
        }
    }

    /// Define el componente que renderizará [`CoreRegions::Content`].
    ///
    /// Equivale a [`Self::new()`] o [`Self::default()`], que ya usan esta región por defecto.
    pub fn content() -> Self {
        Region {
            region: &CoreRegions::Content,
        }
    }

    /// Define el componente que renderizará [`CoreRegions::Footer`].
    pub fn footer() -> Self {
        Region {
            region: &CoreRegions::Footer,
        }
    }

    /// Define el componente que renderizará la región indicada.
    pub fn of(region: RegionRef) -> Self {
        Region { region }
    }
}
