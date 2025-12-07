//! API para añadir y gestionar nuevos temas.
//!
//! Los temas son extensiones que implementan [`Extension`](crate::core::extension::Extension) y
//! también [`Theme`], de modo que [`Extension::theme()`](crate::core::extension::Extension::theme)
//! permita identificar y registrar los temas disponibles.
//!
//! Un tema es la *piel* de la aplicación: define estilos, tipografías, espaciados o comportamientos
//! interactivos. Para ello utiliza plantillas ([`Template`]) que describen cómo maquetar el cuerpo
//! del documento a partir de varias regiones ([`Region`]). Cada región es un contenedor lógico
//! identificado por un nombre, cuyo contenido se obtiene del [`Context`] de la página.
//!
//! Una página ([`Page`](crate::response::page::Page)) representa un documento HTML completo.
//! Implementa [`Contextual`](crate::core::component::Contextual) para gestionar su propio
//! [`Context`], donde mantiene el tema activo, la plantilla seleccionada y los componentes
//! asociados a cada región.
//!
//! De este modo, temas y extensiones colaboran sobre una estructura común: las aplicaciones
//! registran componentes en el [`Context`], las plantillas organizan las regiones y las páginas
//! generan el documento HTML resultante.
//!
//! Los temas pueden definir sus propias implementaciones de [`Template`] y [`Region`] (por ejemplo,
//! mediante *enums* adicionales) para añadir nuevas plantillas o exponer regiones específicas.

use crate::core::component::Context;
use crate::html::{html, Markup};
use crate::locale::L10n;
use crate::{util, AutoDefault};

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
/// forma diferente. Por ejemplo, [`DefaultRegion::Header`] y `BootsierRegion::Header` mostrarían
/// los mismos componentes si ambas devuelven el nombre `"header"`, pero podrían maquetarse de
/// manera distinta.
///
/// El tema decide qué regiones mostrar en el cuerpo del documento, normalmente usando una plantilla
/// ([`Template`]) al renderizar la página ([`Page`](crate::response::page::Page)).
pub trait Region {
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
    fn render(&'static self, cx: &mut Context) -> Markup
    where
        Self: Sized,
    {
        html! {
            @let region = cx.render_region(self);
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

// **< DefaultRegion >******************************************************************************

/// Regiones básicas que PageTop proporciona por defecto.
///
/// Estas regiones comparten sus nombres (`"header"`, `"content"`, `"footer"`) con cualquier región
/// equivalente definida por otros temas, por lo que comparten también el contenido registrado bajo
/// esos nombres.
#[derive(AutoDefault)]
pub enum DefaultRegion {
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

impl Region for DefaultRegion {
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
/// de una página ([`Page`](crate::response::page::Page)). El tema utiliza esta información para
/// determinar qué regiones ([`Region`]) deben renderizarse y en qué orden.
pub trait Template {
    /// Renderiza el contenido de la plantilla.
    ///
    /// Por defecto, renderiza las regiones básicas de [`DefaultRegion`] en este orden:
    /// [`DefaultRegion::Header`], [`DefaultRegion::Content`] y [`DefaultRegion::Footer`].
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
    fn render(&'static self, cx: &mut Context) -> Markup {
        html! {
            (DefaultRegion::Header.render(cx))
            (DefaultRegion::Content.render(cx))
            (DefaultRegion::Footer.render(cx))
        }
    }
}

/// Referencia estática a una plantilla.
pub type TemplateRef = &'static dyn Template;

// **< DefaultTemplate >****************************************************************************

/// Plantillas que PageTop proporciona por defecto.
#[derive(AutoDefault)]
pub enum DefaultTemplate {
    /// Plantilla predeterminada.
    ///
    /// Utiliza la implementación por defecto de [`Template::render()`] y se emplea cuando no se
    /// selecciona ninguna otra plantilla explícitamente.
    #[default]
    Standard,

    /// Plantilla de error.
    ///
    /// Se utiliza para páginas de error u otros estados excepcionales. Por defecto utiliza la misma
    /// implementación de [`Template::render()`] que [`Self::Standard`].
    Error,
}

impl Template for DefaultTemplate {}

// **< Definitions >********************************************************************************

mod definition;
pub use definition::{Theme, ThemeRef};

mod regions;
pub(crate) use regions::ChildrenInRegions;
pub use regions::InRegion;

pub(crate) mod all;
