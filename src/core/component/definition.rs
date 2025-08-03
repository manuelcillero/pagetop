use crate::base::action;
use crate::core::{AnyInfo, TypeInfo};
use crate::html::{html, Context, Markup, PrepareMarkup, Render};

/// Define la función de renderizado para todos los componentes.
///
/// Este *trait* se implementa automáticamente en cualquier tipo (componente) que implemente
/// [`Component`], por lo que no requiere ninguna codificación manual.
pub trait ComponentRender {
    /// Renderiza el componente usando el contexto proporcionado.
    fn render(&mut self, cx: &mut Context) -> Markup;
}

/// Interfaz común que debe implementar un componente renderizable en `PageTop`.
///
/// Se recomienda que los componentes deriven [`AutoDefault`](crate::AutoDefault). También deben
/// implementar explícitamente el método [`new()`](Self::new) y pueden sobrescribir los otros
/// métodos para personalizar su comportamiento.
pub trait Component: AnyInfo + ComponentRender + Send + Sync {
    /// Crea una nueva instancia del componente.
    fn new() -> Self
    where
        Self: Sized;

    /// Devuelve el nombre del componente.
    ///
    /// Por defecto se obtiene del nombre corto del tipo usando [`TypeInfo::ShortName`].
    fn name(&self) -> &'static str {
        TypeInfo::ShortName.of::<Self>()
    }

    /// Devuelve una descripción opcional del componente.
    ///
    /// Por defecto, no se proporciona ninguna descripción (`None`).
    fn description(&self) -> Option<String> {
        None
    }

    /// Devuelve un identificador opcional para el componente.
    ///
    /// Este identificador puede usarse para referenciar el componente en el HTML. Por defecto, no
    /// tiene ningún identificador (`None`).
    fn id(&self) -> Option<String> {
        None
    }

    /// Configura el componente justo antes de preparar el renderizado.
    ///
    /// Este método puede sobrescribirse para modificar la estructura interna del componente o el
    /// contexto antes de preparar la renderización del componente. Por defecto no hace nada.
    #[allow(unused_variables)]
    fn setup_before_prepare(&mut self, cx: &mut Context) {}

    /// Devuelve una representación estructurada del componente lista para renderizar.
    ///
    /// Este método forma parte del ciclo de vida de los componentes y se invoca automáticamente
    /// durante el proceso de construcción del documento. Puede sobrescribirse para generar
    /// dinámicamente el contenido HTML con acceso al contexto de renderizado.
    ///
    /// Por defecto, devuelve [`PrepareMarkup::None`].
    #[allow(unused_variables)]
    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::None
    }
}

/// Implementa [`render()`](ComponentRender::render) para todos los componentes.
///
/// Y para cada componente ejecuta la siguiente secuencia:
///
/// 1. Despacha [`action::component::IsRenderable`](crate::base::action::component::IsRenderable)
///    para ver si se puede renderizar. Si no es así, devuelve un [`Markup`] vacío.
/// 2. Ejecuta [`setup_before_prepare()`](Component::setup_before_prepare) para que el componente
///    pueda ajustar su estructura interna o modificar el contexto.
/// 3. Despacha [`action::theme::BeforeRender<C>`](crate::base::action::theme::BeforeRender) para
///    que el tema pueda hacer ajustes en el componente o el contexto.
/// 4. Despacha [`action::component::BeforeRender<C>`](crate::base::action::component::BeforeRender)
///    para que otras extensiones puedan hacer ajustes.
/// 5. **Prepara el renderizado del componente**:
///    - Despacha [`action::theme::PrepareRender<C>`](crate::base::action::theme::PrepareRender)
///      para permitir al tema preparar un renderizado diferente al predefinido.
///    - Si no es así, ejecuta [`prepare_component()`](Component::prepare_component) para preparar
///      el renderizado predefinido del componente.
/// 6. Despacha [`action::theme::AfterRender<C>`](crate::base::action::theme::AfterRender) para
///    que el tema pueda hacer sus últimos ajustes.
/// 7. Despacha [`action::component::AfterRender<C>`](crate::base::action::component::AfterRender)
///    para que otras extensiones puedan hacer sus últimos ajustes.
/// 8. Finalmente devuelve un [`Markup`] del renderizado preparado en el paso 5.
impl<C: Component> ComponentRender for C {
    fn render(&mut self, cx: &mut Context) -> Markup {
        // Si no es renderizable, devuelve un bloque HTML vacío.
        if !action::component::IsRenderable::dispatch(self, cx) {
            return html! {};
        }

        // Configura el componente antes de preparar.
        self.setup_before_prepare(cx);

        // Acciones específicas del tema antes de renderizar el componente.
        action::theme::BeforeRender::dispatch(self, cx);

        // Acciones de las extensiones antes de renderizar el componente.
        action::component::BeforeRender::dispatch(self, cx);

        // Prepara el renderizado del componente.
        let prepare = action::theme::PrepareRender::dispatch(self, cx);
        let prepare = if prepare.is_empty() {
            self.prepare_component(cx)
        } else {
            prepare
        };

        // Acciones específicas del tema después de renderizar el componente.
        action::theme::AfterRender::dispatch(self, cx);

        // Acciones de las extensiones después de renderizar el componente.
        action::component::AfterRender::dispatch(self, cx);

        // Devuelve el marcado final.
        prepare.render()
    }
}
