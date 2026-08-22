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
//! Como `parent()` se resuelve en tiempo de ejecución, PageTop no puede descartar en compilación
//! referencias circulares (un tema acaba siendo padre de sí mismo, directa o transitivamente). Ese
//! ciclo provocaría un bucle infinito en [`Theme::handle_component()`] o un desbordamiento de pila
//! en los métodos predefinidos de `Theme` que delegan recursivamente en el padre. Para evitarlo,
//! PageTop recorre la cadena de cada tema al registrarlo y **aborta el arranque de la aplicación**
//! si detecta una referencia circular.
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
//! 1. **Definir regiones nuevas**. Por defecto, PageTop define [`CoreRegions`] (`Header`, `Aside`,
//!    `Content`, `Footer`) como regiones de plantilla siempre disponibles, y [`ReservedRegions`]
//!    (`PageTop`, `PageBottom`) como regiones reservadas que se renderizan al margen de cualquier
//!    plantilla. Un tema puede definir su propio *enum* que implemente [`RegionName`] para
//!    **añadir** nuevas regiones que PageTop no ofrece (como barras laterales, regiones específicas
//!    para menús, sliders, cabeceras hero, etc.). No es necesario redefinir las de [`CoreRegions`]
//!    ni las de [`ReservedRegions`], que ya existen y se asume que cualquier tema respeta.
//! 2. **Definir plantillas nuevas**. Por defecto existe [`CoreTemplates`], con las plantillas
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
//!    ejemplo, [`CoreTemplates`] o el propio *enum* del tema). `pagetop-bootsier` hace exactamente
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
//! InRegion::Global(&CoreRegions::Footer).add(PoweredBy::new());
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
//!
//! [`ReservedRegions`]: crate::response::ReservedRegions

mod intent;
pub use intent::Intent;

mod layout;
pub use layout::{CoreRegions, RegionName, RegionRef};
pub use layout::{CoreTemplates, TemplateName, TemplateRef};

mod definition;
pub use definition::{Theme, ThemeRef};

mod regions;
pub(crate) use regions::ChildrenInRegions;
pub use regions::InRegion;

pub(crate) mod all;
