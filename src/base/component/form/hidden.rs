use crate::prelude::*;

/// Componente para crear un **campo oculto** del formulario.
///
/// Renderiza un campo sin ningún marcado visible. Su valor se envía al servidor junto con el resto
/// del formulario, aunque el usuario no lo vea en la página.
///
/// Es útil para transportar datos de estado, identificadores o cualquier valor que deba incluirse
/// en el envío sin mostrarse. **No es un dato protegido**: el usuario puede verlo en el código de
/// la página y cambiarlo antes de enviar el formulario, así que el servidor debe validarlo como
/// cualquier otro campo.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let item = form::Hidden::new()
///     .with_name("item_id")
///     .with_value("42");
///
/// // Equivalente, cuando `name` y `value` se conocen de antemano:
/// let item = form::Hidden::field("item_id", "42");
/// ```
///
/// Al enviar el formulario el navegador transmite `name=valor`. En el servidor se deserializa
/// como `String`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     item_id: String,
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Hidden {
    /// Devuelve el nombre del campo oculto.
    name: AttrName,
    /// Devuelve el valor del campo oculto.
    value: AttrValue,
}

#[async_trait]
impl Component for Hidden {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, _cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {
            input
                type="hidden"
                name=[self.name().as_deref()]
                value=[self.value().as_deref()];
        })
    }
}

#[builder_impl]
impl Hidden {
    /// Crea un campo oculto con nombre y valor (atributos `name` y `value`) ya establecidos.
    ///
    /// Equivale a `Hidden::new().with_name(name).with_value(value)`.
    pub fn field(name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        Self::default().with_name(name).with_value(value)
    }

    // **< Hidden BUILDER >*************************************************************************

    /// Establece el nombre del campo oculto (atributo `name`).
    ///
    /// Sin él, el valor del campo no se transmite al servidor al enviar el formulario. Para
    /// deserializar el campo en el servidor es recomendable establecer un `name` explícito.
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece el valor del campo oculto (atributo `value`).
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        self.value.alter_str(value);
        self
    }
}
