use crate::AutoDefault;
use crate::core::component::{Context, Contextual};

/// Intención semántica de un componente visual.
///
/// Describe *qué comunica* un componente, y cada tema traduce la intención a su propia paleta de
/// colores. Define un vocabulario con variantes por énfasis (`Primary`, `Neutral`), un canal
/// informativo (`Info`) y una escala de severidad (`Success`, `Warning`, `Severe`).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Intent {
    /// Énfasis principal, suele ser la acción que más destaca a la vista.
    #[default]
    Primary,
    /// Presencia sin énfasis, una opción discreta o normal.
    Neutral,
    /// Informa de algo que merece leerse, ajeno a la escala de severidad. Encaja en notas o avisos
    /// informativos.
    Info,
    /// Destaca algo que ha ido bien, un estado favorable o un camino afirmativo. En una alerta
    /// podría indicar la confirmación de una operación correcta o un estado saludable; en un botón,
    /// la acción que confirma o aprueba algo positivamente.
    Success,
    /// Precaución, representa algo que requiere atención o cuidado. En una alerta, sería un
    /// problema no bloqueante o un aviso; en un botón, una acción que conviene meditar sin llegar a
    /// ser destructiva.
    Warning,
    /// Algo va mal, es peligroso o destructivo. En una alerta sería un error o un fallo bloqueante;
    /// en un botón, la acción destructiva o irreversible (p. ej. eliminar).
    Severe,
}

impl Intent {
    /// Devuelve el nombre del color asociado a la intención en el tema activo del contexto actual.
    ///
    /// Atajo de [`Theme::intent_color()`](crate::core::theme::Theme::intent_color) a través de
    /// [`Context::theme()`]. La intención no tiene una cadena propia, es el tema activo quien
    /// decide su traducción.
    pub fn color(&self, cx: &Context) -> &'static str {
        cx.theme().intent_color(*self)
    }
}
