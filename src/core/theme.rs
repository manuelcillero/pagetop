//! API para añadir y gestionar nuevos temas.
//!
//! Un tema es la *piel* de la aplicación: define estilos, tipografías, espaciados o comportamientos
//! interactivos. Usa plantillas ([`Template`](crate::base::component::layout::Template)) para
//! maquetar los contenidos en base a regiones ([`Region`](crate::base::component::layout::Region)).
//! Cada región es un contenedor lógico identificado por un nombre para agrupar y renderizar
//! componentes.
//!
//! Una página ([`Page`](crate::response::Page)) es un documento HTML completo. Implementa
//! [`Contextual`](crate::core::component::Contextual) para gestionar su propio
//! [`Context`](crate::core::component::Context), donde mantiene el tema activo, la plantilla
//! seleccionada y los componentes asociados a cada región a renderizar.
//!
//! # Temas hijo, herencia y componentes
//!
//! PageTop permite crear **temas hijo** que refinan el comportamiento de su tema padre. Un tema
//! hijo hereda automáticamente todos los métodos del padre y puede sobrescribirlos selectivamente.
//! Esta herencia sólo determina qué implementación de sus métodos se usa cuando el tema hijo no los
//! sobrescribe (como el renderizado del `<body>` y del `<head>`, los recursos incorporados, el uso
//! de [`Theme::handle_component()`], las páginas de error, etc.). Un tema hijo puede ser a su vez
//! padre de otro, basta declararlo cada vez con [`Theme::parent()`].
//!
//! Sin embargo, no dice nada sobre los componentes. Aunque un tema puede exportar su propio
//! catálogo de componentes, realmente no pertenecen como tal a ningún tema ni dependen de esa
//! cadena de herencia. Una extensión puede existir únicamente para aportar un componente genérico
//! (por ejemplo, un editor de texto enriquecido) pensado para usarse en cualquier aplicación, con
//! independencia del tema activo. Que un tema decida capturar ese componente en
//! [`Theme::handle_component()`] para adaptarlo es una decisión propia del tema, no una relación de
//! parentesco: cualquier tema de la cadena de herencia puede interceptar cualquier componente,
//! venga de la extensión que venga, sin que exista ningún vínculo de diseño previo entre ambos.
//!
//! Lo que sí es responsabilidad del tema activo es garantizar que el componente disponga de los
//! recursos que necesita para verse y comportarse correctamente: sus propios estilos y JavaScript,
//! si los aporta, o los que ofrezca el tema. El componente genera su marcado igual aunque esos
//! recursos falten, pero el resultado seguramente no lucirá ni funcionará como se espera.
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
//! 1. **Definir regiones nuevas**. Por defecto, PageTop define [`CoreRegion`] (`Header`, `Content`,
//!    `Footer`) como las regiones de plantilla que se asumen siempre disponibles, y
//!    [`ReservedRegion`](crate::response::ReservedRegion) (`PageTop`, `PageBottom`) como las
//!    regiones reservadas que se renderizan al margen de cualquier plantilla. Un tema puede definir
//!    su propio *enum* que implemente [`RegionName`] para **añadir** nuevas regiones que PageTop no
//!    ofrece (por ejemplo, una barra lateral). No es necesario redefinir las de [`CoreRegion`] ni
//!    las de [`ReservedRegion`](crate::response::ReservedRegion), que ya existen y se asume que
//!    cualquier tema respeta.
//! 2. **Definir plantillas nuevas**. Por defecto existe [`CoreTemplate`], con las plantillas
//!    `Standard` y `Admin` que usan `Page::new()` y `Page::admin()` respectivamente, y que son
//!    siempre las mismas: no hay un método de `Theme` para elegir una plantilla predeterminada
//!    distinta. Un tema puede definir su propio *enum* que implemente [`TemplateName`] para
//!    **añadir** plantillas que PageTop no ofrece, y no para redefinir `Standard`/`Admin`.
//! 3. **Cambiar cómo se renderiza** una región, una plantilla o un componente ya existente, se hace
//!    capturando el componente ([`Region`](crate::base::component::layout::Region) o
//!    [`Template`](crate::base::component::layout::Template), o el componente que sea) en
//!    [`Theme::handle_component()`]. En el caso de regiones y plantillas, para distinguir *qué*
//!    región o plantilla concreta envuelve el componente, sin comparar cadenas, basta con encadenar
//!    el *getter* correspondiente
//!    ([`Region::region()`](crate::base::component::layout::Region::region) o
//!    [`Template::template()`](crate::base::component::layout::Template::template)) con
//!    [`AnyCast::downcast_ref()`](crate::core::AnyCast::downcast_ref) hacia el tipo concreto (por
//!    ejemplo, [`CoreTemplate`] o el propio *enum* del tema). `pagetop-bootsier` hace exactamente
//!    esto para maquetar `Standard` y `Admin` de forma distinta, sin necesitar sus propias
//!    variantes de plantilla.
//!
//! Para forzar una plantilla completamente distinta en una página concreta, se puede llamar
//! manualmente a [`with_template()`](crate::core::component::Contextual::with_template).
//!
//! Las páginas de error (403, 404, y otros errores fatales) no tienen una plantilla propia: se
//! renderizan con la plantilla ya activa en la página, para que el usuario no pierda el contexto de
//! navegación del sitio. Los temas pueden personalizar su contenido sobrescribiendo
//! [`Theme::error_403()`], [`Theme::error_404()`] o [`Theme::error_fatal()`], sin necesidad de una
//! plantilla distinta.
//!
//! El resto del comportamiento de un tema (por ejemplo, el renderizado del `<head>`) se sobrescribe
//! de forma independiente de estos tres pasos y no es necesario para tener un tema funcional.
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
//! InRegion::Global(&CoreRegion::Footer).add(PoweredBy::new());
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

use crate::AutoDefault;
use crate::core::AnyInfo;
use crate::locale::L10n;

// **< RegionName >*********************************************************************************

/// Interfaz común para las regiones lógicas del `<body>`.
///
/// Una `RegionName` representa un contenedor lógico identificado por un nombre de región. Su
/// contenido se obtiene del [`Context`](crate::core::component::Context), donde los componentes
/// suelen registrarse usando implementaciones de métodos como
/// [`Contextual::with_child_in()`](crate::core::component::Contextual::with_child_in).
///
/// El contenido de una región viene determinado únicamente por su nombre, no por su tipo. Distintas
/// implementaciones de [`RegionName`] que devuelvan el mismo nombre comparten el mismo conjunto de
/// componentes registrados en el [`Context`](crate::core::component::Context). Un *enum* propio que
/// implemente [`RegionName`] está pensado para **añadir** regiones que PageTop no ofrece (con un
/// nombre propio que no colisione con los de [`CoreRegion`] o
/// [`ReservedRegion`](crate::response::ReservedRegion)).
///
/// El tema decide qué regiones mostrar en el `<body>`, normalmente usando una plantilla
/// ([`TemplateName`]) al renderizar la página ([`Page`](crate::response::Page)).
///
/// Requiere [`AnyInfo`] para que un [`RegionRef`] pueda recuperarse mediante
/// [`AnyCast::downcast_ref()`](crate::core::AnyCast::downcast_ref) hacia su tipo concreto (por
/// ejemplo, para que un tema distinga en
/// [`Theme::handle_component()`](crate::core::theme::Theme::handle_component) qué variante
/// concreta está renderizando el componente [`Region`](crate::base::component::layout::Region)).
pub trait RegionName: Send + Sync + AnyInfo {
    /// Devuelve el nombre de la región.
    ///
    /// Este nombre es el identificador lógico de la región y se usa como clave en el
    /// [`Context`](crate::core::component::Context) para recuperar y renderizar el contenido
    /// registrado bajo ese nombre. Cualquier implementación de [`RegionName`] que devuelva el mismo
    /// nombre compartirá el mismo conjunto de componentes.
    fn name(&self) -> &'static str;

    /// Devuelve un *texto localizado* como etiqueta de accesibilidad asociada a la región.
    ///
    /// En la implementación predeterminada de [`Region`](crate::base::component::layout::Region),
    /// este valor se usa como `aria-label` del contenedor de la región.
    fn label(&self) -> L10n;
}

/// Referencia estática a una región.
pub type RegionRef = &'static dyn RegionName;

// **< CoreRegion >*********************************************************************************

/// Regiones básicas que PageTop proporciona por defecto.
///
/// Comparten sus nombres (`"header"`, `"content"`, `"footer"`) con otras regiones que implementen
/// [`RegionName`], por lo que comparten también el contenido registrado bajo esos nombres. Por
/// defecto, son las regiones usadas por [`Template`](crate::base::component::layout::Template).
///
/// A estas regiones hay que sumar también las regiones internas reservadas por
/// [`ReservedRegion`](crate::response::ReservedRegion) (`"page-top"` y `"page-bottom"`), que
/// [`Page::render()`](crate::response::Page::render) renderiza en cualquier caso.
#[derive(AutoDefault)]
pub enum CoreRegion {
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

impl RegionName for CoreRegion {
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

// **< TemplateName >*******************************************************************************

/// Interfaz común para las plantillas lógicas de una página.
///
/// Representa una variante identificada por un nombre. Un tema puede usar este nombre para decidir
/// la composición del cuerpo de una página ([`Page`](crate::response::Page)), es decir, qué
/// regiones ([`RegionName`]) renderizar y en qué orden.
///
/// Requiere [`AnyInfo`] por el mismo motivo que [`RegionName`], para que un [`TemplateRef`] pueda
/// recuperarse mediante [`AnyCast::downcast_ref()`](crate::core::AnyCast::downcast_ref) hacia su
/// tipo concreto (por ejemplo, para que un tema distinga en
/// [`Theme::handle_component()`](crate::core::theme::Theme::handle_component) qué variante concreta
/// está renderizando el componente [`Template`](crate::base::component::layout::Template)).
pub trait TemplateName: Send + Sync + AnyInfo {
    /// Devuelve el nombre de la plantilla.
    fn name(&self) -> &'static str;

    /// Devuelve un *texto localizado* como etiqueta descriptiva de la plantilla.
    fn label(&self) -> L10n;
}

/// Referencia estática a una plantilla.
pub type TemplateRef = &'static dyn TemplateName;

// **< CoreTemplate >*******************************************************************************

/// Plantillas que PageTop proporciona por defecto.
#[derive(AutoDefault)]
pub enum CoreTemplate {
    /// Plantilla predeterminada, de nombre `"standard"`.
    ///
    /// Se emplea cuando no se selecciona ninguna otra plantilla explícitamente.
    #[default]
    Standard,

    /// Plantilla para la **interfaz de administración**, de nombre `"admin"`.
    ///
    /// Se utiliza para páginas de administración o paneles de control.
    Admin,
}

impl TemplateName for CoreTemplate {
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Admin => "admin",
        }
    }

    #[inline]
    fn label(&self) -> L10n {
        match self {
            Self::Standard => L10n::l("template-standard"),
            Self::Admin => L10n::l("template-admin"),
        }
    }
}

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
