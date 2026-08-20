use crate::AutoDefault;
use crate::core::AnyInfo;
use crate::locale::Lc;

// **< RegionName >*********************************************************************************

/// Interfaz común para las regiones lógicas del `<body>`.
///
/// Una [`RegionName`] representa un contenedor lógico identificado por un nombre de región. Su
/// contenido se obtiene del [`Context`], donde los componentes suelen registrarse usando
/// implementaciones de métodos como [`Contextual::with_child_in()`].
///
/// El contenido de una región viene determinado únicamente por su nombre, no por su tipo. Distintas
/// implementaciones de [`RegionName`] que devuelvan el mismo nombre comparten el mismo conjunto de
/// componentes registrados en el [`Context`]. Un *enum* propio que implemente [`RegionName`] está
/// pensado para **añadir** regiones que PageTop no ofrece (con un nombre propio que no colisione
/// con los de [`CoreRegions`] o [`ReservedRegions`]).
///
/// El tema decide qué regiones mostrar en el `<body>`, normalmente usando una plantilla
/// ([`TemplateName`]) al renderizar la página ([`Page`]).
///
/// Requiere [`AnyInfo`] para que un [`RegionRef`] pueda recuperarse mediante
/// [`AnyCast::downcast_ref()`] hacia su tipo concreto (por ejemplo, para que un tema distinga en
/// [`Theme::handle_component()`] qué variante concreta está renderizando el componente [`Region`]).
///
/// [`Context`]: crate::core::component::Context
/// [`Contextual::with_child_in()`]: crate::core::component::Contextual::with_child_in
/// [`ReservedRegions`]: crate::response::ReservedRegions
/// [`Page`]: crate::response::Page
/// [`AnyCast::downcast_ref()`]: crate::core::AnyCast::downcast_ref
/// [`Theme::handle_component()`]: crate::core::theme::Theme::handle_component
/// [`Region`]: crate::base::component::layout::Region
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
    fn label(&self) -> Lc;
}

/// Referencia estática a una región.
pub type RegionRef = &'static dyn RegionName;

// **< CoreRegions >********************************************************************************

/// Regiones básicas que PageTop proporciona por defecto.
///
/// Comparten sus nombres (`"header"`, `"aside"`, `"content"`, `"footer"`) con otras regiones que
/// implementen [`RegionName`], por lo que comparten también el contenido registrado bajo esos
/// nombres. Por defecto, son las regiones usadas por [`Template`].
///
/// A estas regiones hay que sumar también las regiones internas reservadas por [`ReservedRegions`]
/// (`"page-top"` y `"page-bottom"`), que [`Page::render()`] renderiza en cualquier caso.
///
/// [`Template`]: crate::base::component::layout::Template
/// [`ReservedRegions`]: crate::response::ReservedRegions
/// [`Page::render()`]: crate::response::Page::render
#[derive(AutoDefault)]
pub enum CoreRegions {
    /// Región estándar para la **cabecera** del documento, de nombre `"header"`.
    ///
    /// Suele emplearse para mostrar un logotipo, navegación principal, barras superiores, etc.
    Header,

    /// Región de **contenido secundario**, de nombre `"aside"`.
    ///
    /// Se renderiza por defecto entre `Header` y `Content`. Un tema podría maquetarla, por ejemplo,
    /// como columna lateral junto a `Content` y emplearla para menús secundarios o cualquier otro
    /// contenido complementario al principal.
    Aside,

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

impl RegionName for CoreRegions {
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::Header => "header",
            Self::Aside => "aside",
            Self::Content => "content",
            Self::Footer => "footer",
        }
    }

    #[inline]
    fn label(&self) -> Lc {
        match self {
            Self::Header => Lc::l("region_header"),
            Self::Aside => Lc::l("region_aside"),
            Self::Content => Lc::l("region_content"),
            Self::Footer => Lc::l("region_footer"),
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
    fn label(&self) -> Lc;
}

/// Referencia estática a una plantilla.
pub type TemplateRef = &'static dyn TemplateName;

// **< CoreTemplates >******************************************************************************

/// Plantillas que PageTop proporciona por defecto.
#[derive(AutoDefault)]
pub enum CoreTemplates {
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

impl TemplateName for CoreTemplates {
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Admin => "admin",
        }
    }

    #[inline]
    fn label(&self) -> Lc {
        match self {
            Self::Standard => Lc::l("template-standard"),
            Self::Admin => Lc::l("template-admin"),
        }
    }
}
