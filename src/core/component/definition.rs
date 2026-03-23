use crate::base::action;
use crate::core::component::{ComponentError, Context, Contextual};
use crate::core::theme::ThemeRef;
use crate::core::{AnyInfo, TypeInfo};
use crate::html::{html, Markup};

/// Define la función de renderizado para todos los componentes.
///
/// Este *trait* se implementa automáticamente en cualquier tipo (componente) que implemente
/// [`Component`], por lo que no requiere ninguna codificación manual.
pub trait ComponentRender {
    /// Renderiza el componente usando el contexto proporcionado.
    fn render(&mut self, cx: &mut Context) -> Markup;
}

/// Interfaz común que debe implementar un componente renderizable en PageTop.
///
/// Se recomienda que los componentes declaren sus campos como privados, que deriven
/// [`AutoDefault`](crate::AutoDefault) o implementen [`Default`] para inicializarlos por defecto, y
/// [`Getters`](crate::Getters) para acceder a sus datos. Deberán implementar explícitamente el
/// método [`new()`](Self::new) y podrán sobrescribir los demás métodos para personalizar su
/// comportamiento.
pub trait Component: AnyInfo + ComponentRender + Send + Sync {
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
    /// renderizan o no en función del contexto de renderizado.
    ///
    /// También puede asignarse una función [`FnIsRenderable`](super::FnIsRenderable) a un campo del
    /// componente para permitir que instancias concretas del mismo puedan decidir dinámicamente si
    /// se renderizan o no.
    #[allow(unused_variables)]
    fn is_renderable(&self, cx: &mut Context) -> bool {
        true
    }

    /// Configura el componente justo antes de preparar el renderizado.
    ///
    /// Este método puede sobrescribirse para modificar la estructura interna del componente o el
    /// contexto antes de renderizarlo. Por defecto no hace nada.
    #[allow(unused_variables)]
    fn setup_before_prepare(&mut self, cx: &mut Context) {}

    /// Versión del componente para preparar su propio renderizado.
    ///
    /// Este método forma parte del ciclo de vida de los componentes y se invoca automáticamente
    /// durante el proceso de construcción del documento cuando ningún tema sobrescribe el
    /// renderizado mediante [`Theme::handle_component()`](crate::core::theme::Theme::handle_component).
    ///
    /// Se recomienda obtener los datos del componente a través de sus propios métodos para que los
    /// temas puedan implementar [`Theme::handle_component()`](crate::core::theme::Theme::handle_component)
    /// sin depender de los detalles internos del componente.
    ///
    /// Por defecto, devuelve un [`Markup`] vacío (`Ok(html! {})`).
    ///
    /// En caso de error, devuelve un [`ComponentError`] que puede incluir un marcado alternativo
    /// (*fallback*) para sustituir al componente fallido.
    #[allow(unused_variables)]
    fn prepare_component(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {})
    }
}

/// Implementa [`render()`](ComponentRender::render) para todos los componentes.
///
/// El proceso de renderizado de cada componente sigue esta secuencia:
///
/// 1. Ejecuta [`is_renderable()`](Component::is_renderable) para ver si puede renderizarse en el
///    contexto actual. Si no es así, devuelve un [`Markup`] vacío.
/// 2. Ejecuta [`setup_before_prepare()`](Component::setup_before_prepare) para que el componente
///    pueda ajustar su estructura interna o modificar el contexto.
/// 3. Despacha [`action::component::BeforeRender<C>`](crate::base::action::component::BeforeRender)
///    para que las extensiones puedan hacer ajustes previos.
/// 4. **Prepara el renderizado del componente** recorriendo la cadena de temas (hijo → padre →
///    abuelo…) llamando a [`Theme::handle_component()`](crate::core::theme::Theme::handle_component)
///    en cada nivel hasta que uno devuelva `Some`. Si ninguno lo sobrescribe, llama a
///    [`Component::prepare_component()`](Component::prepare_component) del propio componente.
/// 5. Despacha [`action::component::AfterRender<C>`](crate::base::action::component::AfterRender)
///    para que las extensiones puedan reaccionar con sus últimos ajustes.
/// 6. Despacha [`action::component::TransformMarkup<C>`](crate::base::action::component::TransformMarkup)
///    para que las extensiones puedan modificar el HTML final antes de devolverlo.
/// 7. Devuelve el [`Markup`] resultante.
impl<C: Component> ComponentRender for C {
    fn render(&mut self, cx: &mut Context) -> Markup {
        // Si no es renderizable, devuelve un bloque HTML vacío.
        if !self.is_renderable(cx) {
            return html! {};
        }

        // Configura el componente antes de preparar.
        self.setup_before_prepare(cx);

        // Acciones de las extensiones antes de renderizar el componente.
        action::component::BeforeRender::dispatch(self, cx);

        // Prepara el renderizado: recorre la cadena de temas, luego el componente.
        let prepare = match 'resolve: {
            let mut t: Option<ThemeRef> = Some(cx.theme());
            while let Some(theme) = t {
                if let Some(r) = theme.handle_component(self, cx) {
                    break 'resolve r;
                }
                t = theme.parent();
            }
            self.prepare_component(cx)
        } {
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
