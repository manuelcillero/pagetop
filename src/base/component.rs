//! Componentes nativos proporcionados por PageTop.
//!
//! Conviene destacar que PageTop distingue entre:
//!
//! - **Componentes estructurales** que definen el esqueleto de un documento HTML, como [`Template`]
//!   y [`Region`], utilizados por [`Page`](crate::response::page::Page) para generar la estructura
//!   final.
//! - **Componentes de contenido** (menús, barras, tarjetas, etc.), que se incluyen en las regiones
//!   gestionadas por los componentes estructurales.
//!
//! El componente [`Template`] describe cómo maquetar el cuerpo del documento a partir de varias
//! regiones lógicas ([`Region`]). En función de la plantilla seleccionada, determina qué regiones
//! se renderizan y en qué orden. Por ejemplo, la plantilla predeterminada [`Template::DEFAULT`]
//! utiliza las regiones [`Region::HEADER`], [`Region::CONTENT`] y [`Region::FOOTER`].
//!
//! Un componente [`Region`] es un contenedor lógico asociado a un nombre de región. Su contenido se
//! obtiene del [`Context`](crate::core::component::Context), donde los componentes se registran
//! mediante [`Contextual::with_child_in()`](crate::core::component::Contextual::with_child_in) y
//! otros mecanismos similares, y se integra en el documento a través de [`Template`].
//!
//! Por su parte, una página ([`Page`](crate::response::page::Page)) representa un documento HTML
//! completo. Implementa [`Contextual`](crate::core::component::Contextual) para mantener su propio
//! [`Context`](crate::core::component::Context), donde gestiona el tema activo, la plantilla
//! seleccionada y los componentes asociados a cada región, y se encarga de generar la estructura
//! final de la página.
//!
//! De este modo, temas y extensiones colaboran sobre una estructura común: las aplicaciones
//! registran componentes en el [`Context`](crate::core::component::Context), las plantillas
//! organizan las regiones y las páginas generan el documento HTML resultante.
//!
//! Los temas pueden sobrescribir [`Template`] para exponer nuevas plantillas o adaptar las
//! predeterminadas, y lo mismo con [`Region`] para añadir regiones adicionales o personalizar su
//! representación.

mod html;
pub use html::Html;

mod region;
pub use region::Region;

mod template;
pub use template::Template;

mod block;
pub use block::Block;

mod intro;
pub use intro::{Intro, IntroOpening};

mod poweredby;
pub use poweredby::PoweredBy;
