use crate::prelude::*;

/// Componente genérico que representa un bloque de contenido.
///
/// Los bloques se utilizan como contenedores de otros componentes o contenidos, con un título
/// opcional y un cuerpo que sólo se renderiza si existen componentes hijos (*children*).
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Block {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el título del bloque.
    title: L10n,
    /// Devuelve la lista de componentes hijo del bloque.
    children: Children,
}

#[async_trait]
impl Component for Block {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, cx: &Context) {
        // Asegura que el bloque tiene un identificador único.
        self.alter_prop(PropsOp::ensure_id(cx.build_id::<Self>(1)));

        // Todos los bloques tienen la clase CSS `block` por defecto.
        self.alter_prop(PropsOp::prepend_classes("block"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let block_body = self.children().render(cx).await;

        if block_body.is_empty() {
            return Ok(html! {});
        }

        Ok(html! {
            div (self.props()) {
                @if let Some(title) = self.title().lookup(cx) {
                    h2 class="block-title" { span { (title) } }
                }
                div class="block-body" { (block_body) }
            }
        })
    }
}

impl Block {
    // **< Block BUILDER >**************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece el título del bloque.
    #[builder_fn]
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title = title;
        self
    }

    /// Añade un nuevo componente al bloque o modifica la lista de componentes (`children`) con una
    /// operación [`ChildOp`].
    #[builder_fn]
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}
