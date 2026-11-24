//! Confirmación del borrado en un [`Dialog`] compartido por todo el listado.
//!
//! El atributo `hx-confirm` de HTMX pide confirmación antes de enviar una petición con el
//! `confirm()` del navegador: una ventana sin el aspecto del tema, con los botones en el idioma del
//! navegador y no en el de la aplicación. Este módulo la sustituye por un único [`Dialog`] del
//! núcleo para toda la página, que pinta el tema activo como cualquier otro:
//!
//! - [`dialog()`] crea el diálogo. Se añade una vez a la página, **fuera del listado**, para que
//!   HTMX no lo sustituya al actualizarlo.
//! - [`confirm()`] prepara el botón de cada fila: la pregunta (con el nombre del registro) viaja en
//!   su propio `hx-confirm`, y el botón abre el diálogo compartido.
//!
//! Al pulsar el botón de una fila, el diálogo muestra su pregunta y la petición queda en espera. Si
//! se acepta, sale la petición de esa fila tal cual, con su `hx-target` y su `hx-swap`; si se
//! cancela (botón «Cancelar», `Escape` o un clic fuera del diálogo), no se envía nada. Así no hace
//! falta ninguna ruta para preparar la confirmación ni cambiar atributos `hx-*` en el navegador.
//! Si la respuesta sustituye el botón por contenido nuevo (normalmente el listado actualizado), el
//! foco del teclado pasa a ese contenido, porque el botón que lo tenía ha desaparecido.
//!
//! Si la página no incluye el diálogo, el botón pide confirmación como cualquier `hx-confirm` (con
//! el `confirm()` del navegador, salvo que el tema lo sustituya). Sin JavaScript, el botón no hace
//! nada (como cualquier petición de HTMX): nunca borra sin confirmar.
//!
//! El diálogo no depende de ningún tema. Se abre y se cierra con el vocabulario `data-dialog-*` de
//! [`Dialog`], que cada tema traduce al suyo, y el script de la extensión sólo copia la pregunta y
//! reanuda la petición. Sólo se carga en las páginas que tienen el diálogo.
//!
//! # Ejemplo
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_htmx::prelude::*;
//!
//! async fn plots(request: HttpRequest) -> Result<Markup, ErrorPage> {
//!     let mut table = Table::new().with_column(Lc::n("Plot")).with_column(Lc::none());
//!     for (id, name) in [(1, "A1"), (2, "B2")] {
//!         let delete_route = util::join!("/plots/", id.to_string(), "/delete");
//!         let delete = hx_confirm::confirm(
//!             Button::delete()
//!                 .with_size(button::Size::Small)
//!                 .with_hidden_label(Lc::n(name))
//!                 .with_prop(PropsOp::set(hx::POST, delete_route))
//!                 .with_prop(PropsOp::set(hx::TARGET, "#plots"))
//!                 .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML)),
//!             util::join!("Delete the plot «", name, "»?"),
//!         );
//!         table.alter_row(table::Row::new().with_cell(name).with_cell(delete));
//!     }
//!     Page::new(request)
//!         .with_child(ListView::new().with_id("plots").with_table(table))
//!         .with_child(hx_confirm::dialog())
//!         .render()
//!         .await
//! }
//! ```
//!
//! [`Dialog`]: pagetop::base::component::Dialog

use pagetop::prelude::*;

use crate::{LOCALES_HTMX, hx};

// `id` del diálogo compartido y de su pregunta. El script (`hx-confirm.js`) usa los mismos.
const DIALOG_ID: &str = "hx-confirm-dialog";
const QUESTION_ID: &str = "hx-confirm-dialog-question";

/// Crea el diálogo de confirmación compartido por todos los botones de la página preparados con
/// [`confirm()`].
///
/// Tiene el título traducido «Confirmar la eliminación», la pregunta del botón pulsado (que leen
/// también los lectores de pantalla al abrirse, con `aria-describedby`) y, en el pie,
/// [`Button::dismiss()`] con el foco inicial (en un borrado, el foco empieza en la opción segura) y
/// [`Button::delete()`] para aceptar.
///
/// Se añade una sola vez a la página, fuera del listado que actualiza HTMX: si el listado lo
/// incluyera, cada respuesta pondría un diálogo nuevo con el mismo `id`.
pub fn dialog() -> Dialog {
    Dialog::new()
        .with_id(DIALOG_ID)
        .with_title(Lc::t("confirm_delete_title", &LOCALES_HTMX))
        .with_prop(PropsOp::set("aria-describedby", QUESTION_ID))
        .with_child(Html::with(|_| html! { p id=(QUESTION_ID) {} }))
        .with_footer(Button::dismiss().with_autofocus(true))
        .with_footer(
            Button::delete()
                .with_prop(PropsOp::set("data-hx-confirm-accept", "true"))
                .with_prop(PropsOp::set("data-dialog-dismiss", "modal")),
        )
}

/// Prepara `button` para que pida confirmación con `question` en el diálogo de [`dialog()`] antes
/// de enviar su petición.
///
/// `button` ya lleva la petición que borra (`hx::POST` con su `hx::TARGET` y su `hx::SWAP`), y
/// `question` es el texto ya traducido, con el nombre del registro para que no haya dudas de qué se
/// borra («¿Eliminar el rol «Administradores»?»). Añade `hx::CONFIRM` con la pregunta y los
/// atributos `data-dialog-*` que abren el diálogo.
///
/// Para los lectores de pantalla, dale también el nombre del registro al propio botón con
/// [`Button::with_hidden_label()`]: todas las filas tienen un «Eliminar» igual. Ver el ejemplo del
/// [módulo](self).
pub fn confirm(button: Button, question: impl Into<String>) -> Button {
    button
        .with_prop(PropsOp::set(hx::CONFIRM, question.into()))
        .with_prop(PropsOp::set("data-hx-confirm", "true"))
        .with_prop(PropsOp::set("data-dialog-toggle", "modal"))
        .with_prop(PropsOp::set(
            "data-dialog-target",
            util::join!("#", DIALOG_ID),
        ))
}

// Carga el script sólo en las páginas que tienen el diálogo compartido.
pub(crate) fn add_confirm_script(dialog: &mut Dialog, cx: &mut Context) {
    if dialog.id().as_deref() == Some(DIALOG_ID) {
        cx.alter_assets(AssetsOp::add_javascript(
            JavaScript::defer("/htmx/js/hx-confirm.min.js").with_version(env!("CARGO_PKG_VERSION")),
        ));
    }
}
