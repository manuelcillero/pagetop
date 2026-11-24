use crate::prelude::*;

use serde::Deserialize;

/// Indica si un formulario se envió con [`Button::save_and_new()`](super::Button::save_and_new).
///
/// Se añade con `#[serde(flatten)]` a los datos del formulario de alta. Tras guardar el registro,
/// [`is_requested()`](Self::is_requested) decide si se vuelve al listado («Guardar») o a un
/// formulario vacío para crear otro («Guardar y nuevo»), con el aviso de
/// [`notice()`](Self::notice):
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct ItemInput {
///     name: String,
///     #[serde(flatten)]
///     save_and_new: button::SaveAndNew,
/// }
///
/// async fn new_post(request: HttpRequest, web::Form(input): web::Form<ItemInput>) -> Response {
///     // ... guardar `input.name` ...
///     let cx = Context::new(request);
///     if input.save_and_new.is_requested() {
///         Redirect::see_other(cx.route("/items/new")).with_message(
///             &cx,
///             MessageLevel::Info,
///             button::SaveAndNew::notice(),
///         )
///     } else {
///         Redirect::see_other(cx.route("/items"))
///     }
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Deserialize)]
pub struct SaveAndNew {
    // Mismo nombre que el atributo `name` del botón: sólo llega si se pulsó.
    save_and_new: Option<String>,
}

impl SaveAndNew {
    /// Devuelve `true` si el formulario se envió con el botón «Guardar y nuevo».
    pub fn is_requested(&self) -> bool {
        self.save_and_new.is_some()
    }

    /// Aviso para el formulario vacío tras guardar: «Guardado. Puedes crear otro registro.».
    pub fn notice() -> Lc {
        Lc::l("button_save_and_new_notice")
    }
}
