use crate::async_trait;
use crate::base::action;
use crate::core::component::{ComponentError, Context, Contextual};
use crate::core::theme::ThemeRef;
use crate::core::{AnyInfo, TypeInfo};
use crate::html::{Markup, html};

/// Habilita el clonado de componentes.
///
/// Se implementa automáticamente para todo tipo que implemente [`Component`] y [`Clone`]. El método
/// [`clone_box`](Self::clone_box) devuelve un clon del componente original encapsulado en un
/// `Box<dyn Component>`, lo que permite clonar componentes sin conocer su tipo concreto en tiempo
/// de compilación.
pub trait ComponentClone {
    /// Devuelve un clon del componente encapsulado en un [`Box<dyn Component>`].
    fn clone_box(&self) -> Box<dyn Component>;
}

/// Define la función de renderizado para todos los componentes.
///
/// Este trait se implementa automáticamente en cualquier tipo (componente) que implemente
/// [`Component`], por lo que no requiere ninguna codificación manual.
#[async_trait]
pub trait ComponentRender {
    /// Renderiza el componente usando el contexto proporcionado.
    async fn render(&mut self, cx: &mut Context) -> Markup;
}

/// Interfaz común que debe implementar un componente renderizable en PageTop.
///
/// Se recomienda que los componentes declaren sus campos como privados, que deriven
/// [`AutoDefault`](crate::AutoDefault) o implementen [`Default`] para inicializarlos por defecto, y
/// [`Getters`](crate::Getters) para acceder a sus datos. Deberán implementar explícitamente el
/// método [`new()`](Self::new) y podrán sobrescribir los demás métodos para personalizar su
/// comportamiento.
///
/// # Requisito: derivar `Clone`
///
/// Todo tipo que implemente `Component` **debe** derivar también [`Clone`]. Aunque el compilador
/// no lo exige directamente (hacerlo rompería la seguridad de objeto de `dyn Component`),
/// [`ComponentClone`] se implementa automáticamente mediante una *impl blanket* sólo para los tipos
/// que sean `Component + Clone + 'static`. Sin `Clone`, habría que implementar [`ComponentClone`] a
/// mano, y el componente no podría registrarse en [`InRegion`](crate::core::theme::InRegion).
#[async_trait]
pub trait Component: AnyInfo + ComponentClone + ComponentRender + Send + Sync {
    /// Crea una nueva instancia del componente.
    ///
    /// Por convención suele devolver `Self::default()`.
    fn new() -> Self
    where
        Self: Sized;

    /// Devuelve el nombre del componente.
    ///
    /// Por defecto se obtiene del nombre corto del tipo usando [`TypeInfo::ShortName`].
    fn name(&self) -> &'static str {
        TypeInfo::ShortName.of::<Self>()
    }

    /// Devuelve una descripción del componente, si existe.
    ///
    /// Por defecto, no se proporciona ninguna descripción (`None`).
    fn description(&self) -> Option<String> {
        None
    }

    /// Devuelve el identificador del componente, si existe.
    ///
    /// Este identificador puede usarse para referenciar el componente en el HTML. Por defecto, no
    /// tiene ningún identificador (`None`).
    fn id(&self) -> Option<String> {
        None
    }

    /// Indica si el componente es renderizable.
    ///
    /// Por defecto, todos los componentes son renderizables (`true`). Sin embargo, este método
    /// puede sobrescribirse para decidir dinámicamente si los componentes de este tipo se
    /// renderizan o no en función del contexto de renderizado. Recibe sólo una referencia
    /// compartida al contexto porque su único propósito es consultar datos, no modificarlos.
    ///
    /// También **puede asignarse una función [`FnIsRenderable`](super::FnIsRenderable) a un campo
    /// del componente** para permitir que instancias concretas del mismo puedan decidir
    /// dinámicamente si se renderizan o no.
    #[allow(unused_variables)]
    fn is_renderable(&self, cx: &Context) -> bool {
        true
    }

    /// Indica si el renderizado del componente se aplaza hasta haber renderizado el resto de la
    /// página.
    ///
    /// Si devuelve `true` y el componente forma parte del `<body>` de una página
    /// ([`Page::render()`]), su [ciclo de renderizado](ComponentRender) completo se aplaza hasta
    /// que se ha renderizado el resto del `<body>`, y el resultado se coloca en el mismo lugar que
    /// ocupa el componente. Se aplaza el *momento* del renderizado, no la *posición*. El componente
    /// se muestra en su sitio, no al final de la página. Si no se trata de una página, se
    /// renderiza en el momento, como cualquier otro componente.
    ///
    /// Mientras tanto, en su lugar queda una marca en forma de comentario HTML usando el patrón
    /// `<!-- placeholder ... -->`. Los componentes que lo contienen ven la marca y no su HTML
    /// final, y la tratan como contenido aunque el componente acabe sin generar nada. Sus acciones
    /// y transformaciones no pueden alterar el resultado del componente, sólo la marca, que deben
    /// conservar. Si finalmente el componente no genera nada, la marca se queda en la página. Un
    /// contenedor que sólo tenga la marca cumple el selector CSS `:empty`, así que los estilos
    /// pueden ocultarlo (ver [`Region`]). Si una transformación de un componente que lo contiene
    /// quita la marca (por ejemplo, un minificador que elimine los comentarios), el resultado del
    /// componente no se muestra y se registra un aviso en la traza.
    ///
    /// Los componentes aplazados se renderizan en el orden en que se encontraron al renderizar el
    /// `<body>` (normalmente, el orden en que aparecen en la página). Un componente aplazado no
    /// debe añadir mensajes para el usuario: si se renderiza después del componente [`Messages`]
    /// que encabeza la región [`CoreRegions::Content`], no se mostrarán. Un componente aplazado
    /// dentro de otro también aplazado no espera: se renderiza en el momento, con el que lo
    /// contiene.
    ///
    /// Por defecto, ningún componente se aplaza (`false`).
    ///
    /// [`Page::render()`]: crate::response::Page::render
    /// [`Region`]: crate::base::component::layout::Region
    /// [`Messages`]: crate::base::component::Messages
    /// [`CoreRegions::Content`]: crate::core::theme::CoreRegions::Content
    fn is_deferred(&self) -> bool {
        false
    }

    /// Configura el estado interno del componente antes de generar el marcado.
    ///
    /// Segundo paso del [ciclo de renderizado](ComponentRender). Se ejecuta tras comprobar
    /// [`is_renderable()`] y antes de la acción [`BeforeRender`] y de [`prepare()`]. Por defecto no
    /// hace nada.
    ///
    /// Está pensado para **normalizar el estado interno** del componente antes de renderizarlo. Por
    /// ejemplo, calcular clases CSS, ajustar valores de campos, derivar atributos a partir del
    /// contexto, etc. Recibe `&mut Context` donde también puede aplicar ajustes que dependan del
    /// propio componente (p. ej. estilos o *assets*); cualquier extensión que intercepte
    /// [`BeforeRender`] verá ya aplicados esos cambios, porque `setup()` se ejecuta antes. Se
    /// desaconseja utilizar para operaciones de E/S o consultas a base de datos; es
    /// intencionadamente síncrono.
    ///
    /// La frontera con [`prepare()`] es de **sincronía**, no de qué puede modificar cada uno. Los
    /// dos reciben `&mut Context` (ambos pueden realizar ajustes en él), pero sólo `setup()` recibe
    /// además `&mut self` (`prepare()` sólo recibe `&self`, así que no puede normalizar el propio
    /// componente). La carga y el acceso a datos siguen correspondiendo a `prepare()`, que es
    /// `async` precisamente para permitir E/S; `setup()` se mantiene síncrono a propósito. Esta
    /// separación es deliberada y no debe fusionarse.
    ///
    /// [`is_renderable()`]: Self::is_renderable
    /// [`prepare()`]: Self::prepare
    /// [`BeforeRender`]: crate::base::action::component::BeforeRender
    #[allow(unused_variables)]
    fn setup(&mut self, cx: &mut Context) {}

    /// Genera el marcado HTML del componente cuando ningún tema lo sobrescribe.
    ///
    /// Este es el cuarto paso del [ciclo de renderizado](ComponentRender) tras llamar al
    /// [`setup()`] del componente y despachar la acción [`BeforeRender`] que atiende los cambios de
    /// otras extensiones antes de renderizar. Se invoca sólo si ningún tema en la cadena devuelve
    /// `Some` en [`Theme::render_component()`] para este componente.
    ///
    /// Es `async`, a diferencia de [`setup()`], para permitir a los componentes realizar aquí sus
    /// llamadas asíncronas, como consultas a base de datos, peticiones a servicios externos, o
    /// cualquier operación de E/S, que necesiten para preparar su contenido.
    ///
    /// Se recomienda obtener los datos del componente a través de sus propios métodos para que los
    /// temas puedan implementar [`Theme::setup_component()`]/[`Theme::render_component()`] sin
    /// depender de los detalles internos.
    ///
    /// Los campos que representen contenido no deben almacenar [`Markup`] ya generado, sino que
    /// guardarán el dato en bruto, por ejemplo [`Lc`] para textos traducibles, o un componente
    /// anidado como [`Html`], o cualquier otro `Component`, normalmente a través de [`Embed`] o
    /// [`Child`]; y será este método quien lo convierta en HTML en el momento del renderizado, no
    /// quien construye la instancia.
    ///
    /// Por defecto, devuelve un [`Markup`] vacío (`Ok(html! {})`). En caso de error, devuelve un
    /// [`ComponentError`] que puede incluir un marcado alternativo (*fallback*).
    ///
    /// [`setup()`]: Self::setup
    /// [`Lc`]: crate::locale::Lc
    /// [`Theme::setup_component()`]: crate::core::theme::Theme::setup_component
    /// [`Theme::render_component()`]: crate::core::theme::Theme::render_component
    /// [`Embed`]: crate::core::component::Embed
    /// [`Child`]: crate::core::component::Child
    /// [`BeforeRender`]: crate::base::action::component::BeforeRender
    /// [`Html`]: crate::base::component::Html
    #[allow(unused_variables)]
    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {})
    }
}

// *************************************************************************************************

impl<T: Component + Clone + 'static> ComponentClone for T {
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

// *************************************************************************************************

/// Implementa [`render()`] para todos los componentes.
///
/// Si el componente se aplaza ([`is_deferred()`]) y se está renderizando el `<body>` de una página,
/// devuelve una marca que [`Page::render()`] sustituirá por el resultado de este mismo proceso una
/// vez renderizado el resto del `<body>` (si no genera nada, la marca se queda). Si no, el proceso
/// de renderizado de cada componente sigue esta secuencia:
///
/// 1. Ejecuta [`is_renderable()`] para ver si puede renderizarse en el contexto actual. Si no es
///    así, devuelve un [`Markup`] vacío.
/// 2. Ejecuta [`setup()`] para que el componente pueda ajustar su estado interno y, de forma
///    síncrona, el propio [`Context`].
/// 3. Despacha [`action::component::BeforeRender<C>`] para que las extensiones puedan hacer ajustes
///    previos.
/// 4. Prepara el renderizado del componente, recorre la cadena de temas (hijo > padre > abuelo...)
///    llamando en cada nivel primero a [`Theme::setup_component()`] y después a
///    [`Theme::render_component()`], hasta que uno devuelva `Some`. Si ninguno lo sobrescribe,
///    llama al [`Component::prepare()`] del propio componente.
/// 5. Despacha [`action::component::AfterRender<C>`] para que las extensiones puedan reaccionar con
///    sus últimos ajustes.
/// 6. Finalmente despacha [`action::component::TransformMarkup<C>`] para que las extensiones puedan
///    trabajar sobre el HTML final para modificarlo antes de devolverlo.
/// 7. Devuelve el [`Markup`] resultante.
///
/// [`render()`]: ComponentRender::render
/// [`Page::render()`]: crate::response::Page::render
/// [`is_renderable()`]: Component::is_renderable
/// [`is_deferred()`]: Component::is_deferred
/// [`setup()`]: Component::setup
/// [`Component::prepare()`]: Component::prepare
/// [`action::component::BeforeRender<C>`]: crate::base::action::component::BeforeRender
/// [`action::component::AfterRender<C>`]: crate::base::action::component::AfterRender
/// [`action::component::TransformMarkup<C>`]: crate::base::action::component::TransformMarkup
/// [`Theme::setup_component()`]: crate::core::theme::Theme::setup_component
/// [`Theme::render_component()`]: crate::core::theme::Theme::render_component
#[async_trait]
impl<C: Component> ComponentRender for C {
    async fn render(&mut self, cx: &mut Context) -> Markup {
        // Si se aplaza, deja una marca y se renderiza al terminar el resto del <body>.
        if self.is_deferred()
            && let Some(mark) = cx.defer(&*self)
        {
            return mark;
        }

        // Si no es renderizable, devuelve un bloque HTML vacío.
        if !self.is_renderable(cx) {
            return html! {};
        }

        // Configura el componente (y, de forma síncrona, el contexto) antes de preparar.
        self.setup(cx);

        // Acciones de las extensiones antes de renderizar el componente.
        action::component::BeforeRender::dispatch(self, cx);

        // Prepara el renderizado: recorre la cadena de temas, luego el componente.
        let result = 'resolve: {
            let mut t: Option<ThemeRef> = Some(cx.theme());
            while let Some(theme) = t {
                theme.setup_component(self, cx);
                if let Some(r) = theme.render_component(self, cx).await {
                    break 'resolve r;
                }
                t = theme.parent();
            }
            self.prepare(cx).await
        };
        let prepare = match result {
            Ok(markup) => markup,
            Err(error) => {
                crate::trace::error!(
                    path = cx.request().map(|r| r.path()).unwrap_or("<unknown>"),
                    component = self.name(),
                    id = self.id().as_deref().unwrap_or("<not set>"),
                    "render failed, using fallback: {}",
                    error.message()
                );
                error.into_fallback()
            }
        };

        // Acciones de las extensiones después de renderizar el componente.
        action::component::AfterRender::dispatch(self, cx);

        // Acciones de las extensiones que transforman el HTML final antes de devolverlo.
        action::component::TransformMarkup::dispatch(self, cx, prepare)
    }
}
