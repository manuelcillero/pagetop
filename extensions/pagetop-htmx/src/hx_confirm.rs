//! Confirmación de acciones con un [`Dialog`], en lugar del `confirm()` del navegador.
//!
//! El atributo `hx-confirm` de HTMX pide confirmación antes de enviar una petición con el
//! `confirm()` del navegador: una ventana sin el aspecto del tema, con los botones en el idioma del
//! navegador y no en el de la aplicación. Este módulo la sustituye por un [`Dialog`] del núcleo,
//! que pinta el tema activo como cualquier otro:
//!
//! - [`dialog()`] crea un diálogo con nombre, con su título y su botón de aceptar («Bloquear»,
//!   «Conceder acceso»...). Se añade una vez a la página, **fuera de lo que actualice HTMX**, para
//!   que no lo sustituya. Una página puede tener varios.
//! - [`confirm()`] prepara el botón que pide confirmación en ese diálogo: la pregunta viaja en su
//!   propio `hx-confirm`, y el botón abre el diálogo.
//! - [`dialog_delete()`] y [`confirm_delete()`] hacen lo mismo para el borrado, con un diálogo que
//!   comparten todas las filas de un listado. La pregunta de cada fila lleva el nombre del
//!   registro.
//!
//! Al pulsar el botón, el diálogo muestra su pregunta y la petición queda en espera. Si se acepta,
//! sale la petición del botón tal cual, con su `hx-target` y su `hx-swap`; si se cancela (botón
//! «Cancelar», `Escape` o un clic fuera del diálogo), no se envía nada. Así no hace falta ninguna
//! ruta para preparar la confirmación ni cambiar atributos `hx-*` en el navegador. Si la respuesta
//! sustituye el botón por contenido nuevo (normalmente el listado actualizado), el foco del teclado
//! pasa a ese contenido, porque el botón que lo tenía ha desaparecido.
//!
//! Si la página no incluye el diálogo, el botón pide confirmación como cualquier `hx-confirm` (con
//! el `confirm()` del navegador, salvo que el tema lo sustituya). Sin JavaScript, un botón como
//! [`Button::delete()`] no hace nada (como cualquier petición de HTMX); un botón de enviar dentro
//! de un formulario, en cambio, lo envía sin confirmar, porque el formulario sigue funcionando.
//!
//! Los diálogos no dependen de ningún tema. Se abren y se cierran con el vocabulario
//! `data-dialog-*` de [`Dialog`], que cada tema traduce al suyo, y el script de la extensión sólo
//! copia la pregunta y reanuda la petición. Sólo se carga en las páginas que tienen alguno.
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
//!         let delete = hx_confirm::confirm_delete(
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
//!         .with_child(hx_confirm::dialog_delete())
//!         .render()
//!         .await
//! }
//! ```
//!
//! [`Dialog`]: pagetop::base::component::Dialog

use pagetop::prelude::*;

use crate::{LOCALES_HTMX, hx};

// Los diálogos se llaman `hx-confirm-<nombre>` y su pregunta `hx-confirm-<nombre>-question`. El
// script (`hx-confirm.js`) los busca igual.
const PREFIX: &str = "hx-confirm-";

// Nombre del diálogo del borrado (`id` `hx-confirm-delete`).
const DELETE_DIALOG: &str = "delete";

/// Crea un diálogo de confirmación con nombre.
///
/// Lo abren los botones preparados con [`confirm()`] y el mismo `name`. Muestra `title`, la
/// pregunta del botón pulsado (que leen también los lectores de pantalla al abrirse, con
/// `aria-describedby`) y, en el pie, [`Button::dismiss()`] con el foco inicial (se empieza por la
/// opción segura) y `accept`, con el nombre de la acción («Bloquear»...) y el estilo que le
/// corresponda. `accept` pasa a ser un botón sin envío ([`button::Kind::Plain`]): sólo reanuda la
/// petición retenida y cierra el diálogo.
///
/// `name` forma parte del `id` del diálogo (`hx-confirm-<name>`) y del selector con el que el botón
/// lo abre, así que debe ser único en la página y llevar sólo letras ASCII, dígitos, `-` y `_`.
/// Otro nombre falla en desarrollo y en pruebas; en producción, el botón no encontraría su diálogo.
///
/// Se añade una sola vez a la página, fuera de lo que sustituya HTMX: si el contenido actualizado
/// lo incluyera, cada respuesta pondría un diálogo nuevo con el mismo `id`.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_htmx::prelude::*;
/// let dialog = hx_confirm::dialog(
///     "block",
///     Lc::n("Block account"),
///     Button::plain(Lc::n("Block")).with_style(button::Style::Solid(Intent::Severe)),
/// );
/// let block = hx_confirm::confirm(
///     "block",
///     Button::submit(Lc::n("Block")).with_prop(PropsOp::set(hx::POST, "/users/7/block")),
///     "Block the account «ana»? They will not be able to sign in.",
/// );
/// ```
pub fn dialog(name: impl AsRef<str>, title: Lc, accept: Button) -> Dialog {
    let name = name.as_ref();
    debug_assert_valid_name(name);
    let id = util::join!(PREFIX, name);
    let question_id = util::join!(id.as_str(), "-question");
    Dialog::new()
        .with_id(id)
        .with_title(title)
        .with_prop(PropsOp::set("aria-describedby", question_id.clone()))
        .with_child(Html::with(move |_| html! { p id=(question_id) {} }))
        .with_footer(Button::dismiss().with_autofocus(true))
        .with_footer(
            accept
                .with_kind(button::Kind::Plain)
                .with_prop(PropsOp::set("data-hx-confirm-accept", "true"))
                .with_prop(PropsOp::set("data-dialog-dismiss", "modal")),
        )
}

/// Crea un diálogo de confirmación de borrado.
///
/// Lo comparten todos los botones de la página preparados con [`confirm_delete()`]. Es un
/// [`dialog()`] con el título traducido «Confirmar la eliminación» y [`Button::delete()`] para
/// aceptar. Se añade una sola vez a la página, **fuera del listado** que actualiza HTMX.
pub fn dialog_delete() -> Dialog {
    dialog(
        DELETE_DIALOG,
        Lc::t("confirm_delete_title", &LOCALES_HTMX),
        Button::delete(),
    )
}

/// Prepara un botón para que pida la confirmación en un diálogo.
///
/// Antes de enviar su petición, `button` pregunta `question` en el diálogo `name` de [`dialog()`].
/// `button` ya lleva la petición (`hx::POST` y, si hace falta, su `hx::TARGET` y su `hx::SWAP`); si
/// es el botón de enviar de un formulario, la petición incluye los campos del formulario.
/// `question` es el texto ya traducido. Añade `hx::CONFIRM` con la pregunta y los atributos
/// `data-dialog-*` que abren el diálogo. `name` sigue las mismas reglas que en [`dialog()`]. Ver el
/// ejemplo de [`dialog()`].
pub fn confirm(name: impl AsRef<str>, button: Button, question: impl Into<String>) -> Button {
    let name = name.as_ref();
    debug_assert_valid_name(name);
    button
        .with_prop(PropsOp::set(hx::CONFIRM, question.into()))
        .with_prop(PropsOp::set("data-hx-confirm", name.to_string()))
        .with_prop(PropsOp::set("data-dialog-toggle", "modal"))
        .with_prop(PropsOp::set(
            "data-dialog-target",
            util::join!("#", PREFIX, name),
        ))
}

/// Prepara un botón para que pida la confirmación en el diálogo de borrado.
///
/// Es [`confirm()`] con el diálogo de [`dialog_delete()`]. `question` lleva el nombre del registro
/// para que no haya dudas de qué se borra («¿Eliminar el rol «Administradores»?»).
///
/// Para los lectores de pantalla, dale también el nombre del registro al propio botón con
/// [`Button::with_hidden_label()`]: todas las filas tienen un «Eliminar» igual. Ver el ejemplo del
/// [módulo](self).
pub fn confirm_delete(button: Button, question: impl Into<String>) -> Button {
    confirm(DELETE_DIALOG, button, question)
}

// El nombre forma el `id` del diálogo y el selector `#hx-confirm-<nombre>` con el que lo abre el
// tema: con otros caracteres (`.`, `:`, un espacio...) el botón no encontraría su diálogo.
fn debug_assert_valid_name(name: &str) {
    debug_assert!(
        !name.is_empty()
            && name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'),
        "confirmation dialog name `{name}` must only have ASCII letters, digits, `-` and `_`"
    );
}

// Carga el script sólo en las páginas que tienen algún diálogo de confirmación.
pub(crate) fn add_confirm_script(dialog: &mut Dialog, cx: &mut Context) {
    if dialog
        .id()
        .as_deref()
        .is_some_and(|id| id.starts_with(PREFIX))
    {
        cx.alter_assets(AssetsOp::add_javascript(
            JavaScript::defer("/htmx/js/hx-confirm.min.js").with_version(env!("CARGO_PKG_VERSION")),
        ));
    }
}
