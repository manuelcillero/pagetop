//! API para añadir y gestionar nuevos temas.
//!
//! Un tema es la *piel* de la aplicación: define estilos, tipografías, espaciados o comportamientos
//! interactivos. Para ello utiliza plantillas ([`Template`]) que describen cómo maquetar el cuerpo
//! del documento a partir de regiones ([`Region`]). Cada región es un contenedor lógico
//! identificado por un nombre para agrupar y renderizar componentes.
//!
//! Una página ([`Page`](crate::response::Page)) es un documento HTML completo. Implementa
//! [`Contextual`](crate::core::component::Contextual) para gestionar su propio [`Context`], donde
//! mantiene el tema activo, la plantilla seleccionada y los componentes asociados a cada región a
//! renderizar.
//!
//! Además, PageTop permite crear **temas hijo** que refinan el comportamiento de su tema padre. Un
//! tema hijo hereda automáticamente todos los métodos del padre y puede sobrescribirlos
//! selectivamente. Por ejemplo, puede redefinir el renderizado de un componente a través de
//! [`Theme::handle_component()`] sin cambiar el resto del comportamiento heredado. Un tema hijo
//! puede ser a su vez padre de otro, basta declararlo cada vez con [`Theme::parent()`].
//!
//! # Cómo crear un tema nuevo
//!
//! Un tema mínimo es una extensión que implementa [`Extension`](crate::core::extension::Extension)
//! y también [`Theme`] para que [`Extension::theme()`](crate::core::extension::Extension::theme)
//! devuelva `Some(&Self)`. Basta con un `impl Theme for MyTheme {}` vacío, ya que todos los
//! métodos de [`Theme`] tienen implementación por defecto.
//!
//! Un tema puede personalizarse en tres pasos, cada uno necesario sólo si lo que ofrece PageTop por
//! defecto no basta:
//!
//! 1. **Definir regiones propias**. Por defecto PageTop define [`DefaultRegions`], con tres
//!    regiones (`Header`, `Content` y `Footer`) que usan la implementación por defecto de
//!    [`Region::render()`]. Un tema puede definir un *enum* propio que implemente [`Region`] para
//!    exponer sus propias regiones (por ejemplo, una barra lateral) o para cambiar cómo se muestra
//!    el contenido de una región ya existente (identificada por su nombre).
//! 2. **Definir plantillas propias**. Por defecto existe [`DefaultTemplates`], con dos plantillas
//!    (`Standard` y `Admin`) que usan la implementación por defecto de [`Template::render()`] para
//!    renderizar [`DefaultRegions::Header`], [`DefaultRegions::Content`] y
//!    [`DefaultRegions::Footer`], en este orden. Un tema puede definir un *enum* propio que
//!    implemente [`Template`] para crear nuevas plantillas, maquetar las regiones de otra forma,
//!    cambiar su orden o envolverlas en contenedores adicionales.
//! 3. **Elegir las plantillas predeterminadas**. Por un lado, la plantilla por defecto vía
//!    [`Theme::default_template()`] y, por otro, la plantilla para las páginas de administración,
//!    [`Theme::admin_template()`]. De esta forma, las páginas creadas con `Page::new()` usarán
//!    automáticamente la plantilla `default_template()` del tema activo, y las páginas creadas con
//!    `Page::admin()` usarán la de `admin_template()`, sin tener que llamar manualmente a
//!    [`with_template()`](crate::core::component::Contextual::with_template). Un tema que no
//!    sobrescriba estos métodos sigue usando las plantillas por defecto de PageTop.
//!
//! Las páginas de error (403, 404, y otros errores fatales) no tienen una plantilla propia: se
//! renderizan con la plantilla ya activa en la página, para que el usuario no pierda el contexto de
//! navegación del sitio. Los temas pueden personalizar su contenido sobrescribiendo
//! [`Theme::error_403()`], [`Theme::error_404()`] o [`Theme::error_fatal()`], sin necesidad de una
//! plantilla distinta.
//!
//! El resto del comportamiento de un tema (renderizado del `<head>`, o intervención en el
//! renderizado de componentes concretos con [`Theme::handle_component()`]) se sobrescribe de forma
//! independiente de estos tres pasos y no es necesario para tener un tema funcional.
//!
//! # Componentes que se procesan en todas las páginas
//!
//! Los componentes añadidos a una página con
//! [`with_child_in()`](crate::core::component::Contextual::with_child_in) sólo existen para esa
//! petición concreta: hay que volver a añadirlos cada vez que se construya la página. [`InRegion`]
//! resuelve el caso contrario: un componente que se debe procesar en todas las páginas, o en todas
//! las de un tema concreto, sin tener que registrarlo en el código de cada página.
//!
//! `InRegion` registra el componente una sola vez, normalmente al arrancar la aplicación o al
//! inicializar una extensión, y a partir de ahí se procesa automáticamente en todas las páginas que
//! correspondan:
//!
//! ```rust,no_run
//! # use pagetop::prelude::*;
//! InRegion::Global(&DefaultRegions::Footer).add(PoweredBy::new());
//! ```
//!
//! El componente se guarda como **prototipo**: cada página recibe un clon fresco en el momento del
//! renderizado, de modo que su `setup()` siempre parte de un estado inicial limpio y no acumula
//! mutaciones entre peticiones.
//!
//! Como cualquier otro componente, antes de renderizarse pasa por
//! [`is_renderable()`](crate::core::component::Component::is_renderable), el primer paso del
//! [ciclo de renderizado](crate::core::component::ComponentRender). Esto permite registrarlo una
//! sola vez y que decida por sí mismo cuándo mostrarse, por ejemplo según la ruta de la petición o
//! si el usuario actual está autenticado.

use crate::async_trait;
use crate::core::component::Context;
use crate::html::{Markup, html};
use crate::locale::L10n;
use crate::{AutoDefault, util};

// **< Region >*************************************************************************************

/// Interfaz común para las regiones lógicas de un documento.
///
/// Una `Region` representa un contenedor lógico identificado por un nombre de región. Su contenido
/// se obtiene del [`Context`], donde los componentes suelen registrarse usando implementaciones de
/// métodos como [`Contextual::with_child_in()`](crate::core::component::Contextual::with_child_in).
///
/// El contenido de una región viene determinado únicamente por su nombre, no por su tipo. Distintas
/// implementaciones de [`Region`] que devuelvan el mismo nombre compartirán el mismo conjunto de
/// componentes registrados en el [`Context`], aunque cada región puede renderizar ese contenido de
/// forma diferente. Por ejemplo, [`DefaultRegions::Header`] y `BootsierRegions::Header` mostrarían
/// los mismos componentes si ambas devuelven el nombre `"header"`, pero podrían maquetarse de
/// manera distinta.
///
/// El tema decide qué regiones mostrar en el cuerpo del documento, normalmente usando una plantilla
/// ([`Template`]) al renderizar la página ([`Page`](crate::response::Page)).
#[async_trait]
pub trait Region: Send + Sync {
    /// Devuelve el nombre de la región.
    ///
    /// Este nombre es el identificador lógico de la región y se usa como clave en el [`Context`]
    /// para recuperar y renderizar el contenido registrado bajo ese nombre. Cualquier
    /// implementación de [`Region`] que devuelva el mismo nombre compartirá el mismo conjunto de
    /// componentes.
    ///
    /// En la implementación predeterminada de [`Self::render()`] también se utiliza para construir
    /// las clases del contenedor de la región (`"region region-<name>"`).
    fn name(&self) -> &'static str;

    /// Devuelve un *texto localizado* como etiqueta de accesibilidad asociada a la región.
    ///
    /// En la implementación predeterminada de [`Self::render()`], este valor se usa como
    /// `aria-label` del contenedor de la región.
    fn label(&self) -> L10n;

    /// Renderiza el contenedor de la región.
    ///
    /// Por defecto, recupera del [`Context`] el contenido de la región y, si no está vacío, lo
    /// envuelve en un `<div>` con clases `"region region-<name>"` y un `aria-label` basado en el
    /// *texto localizado* de la etiqueta asociada a la región:
    ///
    /// ```html
    /// <div class="region region-<name>" role="region" aria-label="<label>">
    ///     <!-- Componentes de la región "name" -->
    /// </div>
    /// ```
    ///
    /// Se puede sobrescribir este método para modificar la estructura del contenedor, las clases
    /// utilizadas o la semántica del marcado generado para cada región.
    async fn render(&self, cx: &mut Context) -> Markup {
        html! {
            @let region = cx.render_region_named(self.name()).await;
            @if !region.is_empty() {
                div
                    class=(util::join!("region region-", self.name()))
                    role="region"
                    aria-label=[self.label().lookup(cx)]
                {
                    (region)
                }
            }
        }
    }
}

/// Referencia estática a una región.
pub type RegionRef = &'static dyn Region;

// **< DefaultRegions >*****************************************************************************

/// Regiones básicas que PageTop proporciona por defecto.
///
/// Estas regiones comparten sus nombres (`"header"`, `"content"`, `"footer"`) con cualquier región
/// equivalente definida por otros temas, por lo que comparten también el contenido registrado bajo
/// esos nombres.
#[derive(AutoDefault)]
pub enum DefaultRegions {
    /// Región estándar para la **cabecera** del documento, de nombre `"header"`.
    ///
    /// Suele emplearse para mostrar un logotipo, navegación principal, barras superiores, etc.
    Header,

    /// Región principal de **contenido**, de nombre `"content"`.
    ///
    /// Es la región donde se renderiza el contenido principal del documento. En general será la
    /// región mínima imprescindible para que una página tenga sentido.
    #[default]
    Content,

    /// Región estándar para el **pie de página**, de nombre `"footer"`.
    ///
    /// Suele contener información legal, enlaces secundarios, créditos, etc.
    Footer,
}

impl Region for DefaultRegions {
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::Header => "header",
            Self::Content => "content",
            Self::Footer => "footer",
        }
    }

    #[inline]
    fn label(&self) -> L10n {
        match self {
            Self::Header => L10n::l("region-header"),
            Self::Content => L10n::l("region-content"),
            Self::Footer => L10n::l("region-footer"),
        }
    }
}

// **< Template >***********************************************************************************

/// Interfaz común para definir plantillas de contenido.
///
/// Una `Template` puede proporcionar una o más variantes para decidir la composición del `<body>`
/// de una página ([`Page`](crate::response::Page)). El tema utiliza esta información para
/// determinar qué regiones ([`Region`]) deben renderizarse y en qué orden.
#[async_trait]
pub trait Template: Send + Sync {
    /// Renderiza el contenido de la plantilla.
    ///
    /// Por defecto, renderiza las regiones básicas de [`DefaultRegions`] en este orden:
    /// [`DefaultRegions::Header`], [`DefaultRegions::Content`] y [`DefaultRegions::Footer`].
    ///
    /// Se puede sobrescribir este método para:
    ///
    /// - Cambiar el conjunto de regiones que se renderizan según variantes de la plantilla.
    /// - Alterar el orden de dichas regiones.
    /// - Envolver las regiones en contenedores adicionales.
    /// - Implementar distribuciones específicas (por ejemplo, con barras laterales).
    ///
    /// Este método se invoca normalmente desde [`Theme::render_page_body()`] para generar el
    /// contenido del `<body>` de una página según la plantilla devuelta por el contexto de la
    /// propia página ([`Contextual::template()`](crate::core::component::Contextual::template())).
    async fn render(&self, cx: &mut Context) -> Markup {
        html! {
            (DefaultRegions::Header.render(cx).await)
            (DefaultRegions::Content.render(cx).await)
            (DefaultRegions::Footer.render(cx).await)
        }
    }
}

/// Referencia estática a una plantilla.
pub type TemplateRef = &'static dyn Template;

// **< DefaultTemplates >***************************************************************************

/// Plantillas que PageTop proporciona por defecto.
#[derive(AutoDefault)]
pub enum DefaultTemplates {
    /// Plantilla predeterminada.
    ///
    /// Utiliza la implementación por defecto de [`Template::render()`] y se emplea cuando no se
    /// selecciona ninguna otra plantilla explícitamente.
    #[default]
    Standard,

    /// Plantilla para la **interfaz de administración**.
    ///
    /// Se utiliza para páginas de administración o paneles de control. Por defecto utiliza la misma
    /// implementación de [`Template::render()`] que [`Self::Standard`].
    Admin,
}

#[async_trait]
impl Template for DefaultTemplates {}

// **< render_component! >**************************************************************************

/// Sobrescribe el renderizado de componentes en
/// [`Theme::handle_component()`](crate::core::theme::Theme::handle_component).
///
/// Evalúa `$component` contra cada tipo de componente listado en orden. En cuanto encuentra
/// coincidencia, devuelve `Some(Ok(markup))` o `Some(Err(e))` según el resultado de la expresión
/// asociada. Si ningún tipo coincide, devuelve `None` para que el sistema continúe con la cadena de
/// herencia o con el renderizado por defecto del propio componente.
///
/// # Ejemplo
///
/// ```rust,ignore
/// fn handle_component(
///     &self,
///     component: &dyn Component,
///     cx: &mut Context,
/// ) -> Option<Result<Markup, ComponentError>> {
///     render_component!(component, {
///         Button  => |btn| { Ok(html! { button.btn.btn-primary { (btn.label()) } }) },
///         Heading => |h| self.render_heading(h, cx),
///     })
/// }
///
/// fn render_heading(&self, h: &Heading, cx: &mut Context) -> Result<Markup, ComponentError> {
///     Ok(html! { h2.display-4 { (h.text()) } })
/// }
/// ```
#[macro_export]
macro_rules! render_component {
    ($component:expr, { $($type:ty => |$var:ident| $body:expr),* $(,)? }) => {
        'render_component: {
            // Reborrow explícito como referencia compartida para que `downcast_ref` funcione
            // correctamente con `&mut dyn Component` (limitación del compilador con trait objects).
            let __c = &*($component);
            $(
                if let Some($var) = __c.downcast_ref::<$type>() {
                    break 'render_component Some($body);
                }
            )*
            None
        }
    };
}

// **< setup_component! >***************************************************************************

/// Muta un componente dentro de
/// [`Theme::handle_component()`](crate::core::theme::Theme::handle_component).
///
/// Evalúa `$component` contra cada tipo de componente listado en orden. En cuanto encuentra
/// coincidencia, ejecuta el bloque asociado y detiene la evaluación. Si ningún tipo coincide, no
/// hace nada.
///
/// Usa acceso mutable al componente mediante [`downcast_mut`](crate::core::AnyCast::downcast_mut),
/// lo que permite modificar su estado. El tema puede devolver `None` tras la mutación para que otro
/// nivel de la cadena se encargue del renderizado.
///
/// # Ejemplos
///
/// Solo mutación: el tema ajusta el componente y delega el renderizado al siguiente nivel:
///
/// ```rust,ignore
/// fn handle_component(
///     &self,
///     component: &mut dyn Component,
///     cx: &mut Context,
/// ) -> Option<Result<Markup, ComponentError>> {
///     setup_component!(component, { Button => |btn| { btn.add_class("btn-primary"); } });
///     None
/// }
/// ```
///
/// Mutación y renderizado combinados: el `Button` se muta y se renderiza aquí; el `Heading` se
/// muta pero continúa la cadena para que otro nivel lo renderice:
///
/// ```rust,ignore
/// fn handle_component(
///     &self,
///     component: &mut dyn Component,
///     cx: &mut Context,
/// ) -> Option<Result<Markup, ComponentError>> {
///     setup_component!(component, {
///         Button  => |btn| { btn.add_class("btn-primary"); },
///         Heading => |h|   { h.add_class("display-4"); },
///     });
///     render_component!(component, {
///         Button => |btn| Ok(html! { button.btn { (btn.label()) } }),
///     })
/// }
/// ```
#[macro_export]
macro_rules! setup_component {
    ($component:expr, { $($type:ty => |$var:ident| $body:expr),* $(,)? }) => {
        'setup_component: {
            $(
                if let Some($var) = ($component).downcast_mut::<$type>() {
                    $body;
                    break 'setup_component;
                }
            )*
        }
    };
}

// **< Definitions >********************************************************************************

mod definition;
pub use definition::{Theme, ThemeRef};

mod regions;
pub(crate) use regions::ChildrenInRegions;
pub use regions::InRegion;

pub(crate) mod all;
