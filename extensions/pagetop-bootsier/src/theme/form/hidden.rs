use pagetop::prelude::*;

/// Componente para crear un **campo oculto** del formulario.
///
/// Renderiza un campo sin ningún marcado visible. Su valor se envía al servidor junto con el resto
/// del formulario, pero el usuario no puede verlo ni modificarlo.
///
/// Es útil para transportar datos de estado, tokens CSRF, identificadores o cualquier valor que
/// deba incluirse en el envío sin ser accesible al usuario.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// let token = form::Hidden::new()
///     .with_name("csrf_token")
///     .with_value("a1b2c3d4e5");
/// ```
///
/// Al enviar el formulario el navegador transmite `name=valor`. En el servidor se deserializa
/// como `String`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     csrf_token: String,
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Hidden {
    /// Devuelve el nombre del campo oculto.
    name: AttrName,
    /// Devuelve el valor del campo oculto.
    value: AttrValue,
}

impl Component for Hidden {
    fn new() -> Self {
        Self::default()
    }

    fn prepare(&self, _cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {
            input
                type="hidden"
                name=[self.name().get()]
                value=[self.value().get()];
        })
    }
}

impl Hidden {
    // **< Hidden BUILDER >*************************************************************************

    /// Establece el nombre del campo oculto (atributo `name`).
    ///
    /// Sin él, el valor del campo no se transmite al servidor al enviar el formulario. Para
    /// deserializar el campo en el servidor es recomendable establecer un `name` explícito.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece el valor del campo oculto (atributo `value`).
    #[builder_fn]
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        self.value.alter_str(value);
        self
    }
}
