//! Vistas previas de los correos en el navegador ([`DeclareMailPreviews`]).
//!
//! Con `previews = true` en la sección `[mail]`, la aplicación ofrece en [`PREVIEW_PATH`] la lista
//! de los correos que declaran las extensiones, y una página para cada uno con sus cabeceras, el
//! HTML tal como se enviaría (con el diseño común), su texto plano y sus adjuntos. Así se diseñan y
//! revisan los correos sin enviarlos.
//!
//! Las vistas previas usan datos de ejemplo y no envían nada, pero muestran las plantillas a
//! cualquiera que conozca la ruta: **actívalas sólo en desarrollo**.
//!
//! Cada vista previa se genera con el contexto de la petición, así que se puede ver en otro idioma
//! añadiendo `?lang=` a la ruta (p. ej. `/mail/preview/user.password_reset?lang=en-US`), si la
//! negociación de idioma lo permite.

use pagetop::prelude::*;

use crate::{Email, LOCALES_MAIL, transport};

/// Ruta de la lista de vistas previas. Cada correo se ve en `{PREVIEW_PATH}/{clave}`.
pub const PREVIEW_PATH: &str = "/mail/preview";

/// Tipo de función que genera un correo de ejemplo para su vista previa.
///
/// Recibe el contexto de la petición, con su idioma, y devuelve el correo como se enviaría, con
/// datos de ejemplo. Para que la vista previa siga el idioma de la petición, el correo debe crearse
/// con [`Email::from_template()`] o indicar el idioma con [`Email::with_langid()`].
pub type FnMailPreview = fn(cx: &Context) -> Email;

/// Tipo de función que declara vistas previas en un [`MailPreviews`].
pub type FnDeclareMailPreviews = fn(previews: &mut MailPreviews);

/// Vistas previas declaradas por las extensiones. Ver [`DeclareMailPreviews`].
#[derive(AutoDefault)]
pub struct MailPreviews {
    previews: Vec<MailPreview>,
}

struct MailPreview {
    key: &'static str,
    title: Lc,
    build: FnMailPreview,
}

impl MailPreviews {
    /// Añade la vista previa de un correo.
    ///
    /// - `key`: identificador único, que forma parte de la ruta (p. ej. `"shop.order_shipped"`).
    ///   Conviene empezarlo por el nombre de la extensión.
    /// - `title`: título descriptivo para la lista.
    /// - `build`: función que genera el correo con datos de ejemplo.
    pub fn add(&mut self, key: &'static str, title: Lc, build: FnMailPreview) {
        self.previews.push(MailPreview { key, title, build });
    }

    // Recoge las vistas previas de todas las extensiones, ordenadas por su clave.
    fn collect() -> Self {
        let mut previews = MailPreviews::default();
        dispatch_actions(|action: &DeclareMailPreviews| (action.f)(&mut previews));
        previews.previews.sort_by_key(|preview| preview.key);
        previews
    }
}

/// Acción para declarar vistas previas de correos.
///
/// Cada extensión que envía correos puede registrar esta acción para que sus correos se puedan
/// revisar en el navegador con datos de ejemplo (ver la [documentación del módulo](self)).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_mail::prelude::*;
///
/// fn order_shipped(cx: &Context) -> Email {
///     Email::new()
///         .with_to("cliente@example.com")
///         .with_subject("Tu pedido ha salido")
///         .with_html(html! { p { "Tu pedido " strong { "1234" } " ha salido hoy." } })
///         .with_langid(cx)
/// }
///
/// fn declare_previews(previews: &mut MailPreviews) {
///     previews.add("shop.order_shipped", Lc::n("Pedido enviado"), order_shipped);
/// }
///
/// struct Shop;
///
/// #[async_trait]
/// impl Extension for Shop {
///     fn dependencies(&self) -> Vec<ExtensionRef> {
///         vec![&pagetop_mail::Mail]
///     }
///
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![DeclareMailPreviews::new(declare_previews)]
///     }
/// }
/// ```
pub struct DeclareMailPreviews {
    f: FnDeclareMailPreviews,
    weight: Weight,
}

impl ActionDispatcher for DeclareMailPreviews {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareMailPreviews {
    /// Registra una función que declara vistas previas.
    pub fn new(f: FnDeclareMailPreviews) -> Self {
        DeclareMailPreviews { f, weight: 0 }
    }

    /// Opcional. Las acciones de menor peso se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}

// **< Handlers >***********************************************************************************

// GET {PREVIEW_PATH} - Lista de las vistas previas.
pub(crate) async fn list_get(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_title(Lc::t("preview_title", &LOCALES_MAIL))
        .with_child(Html::with(|cx| {
            let previews = MailPreviews::collect();
            html! {
                h1 { (Lc::t("preview_title", &LOCALES_MAIL).using(cx)) }
                @if previews.previews.is_empty() {
                    p { (Lc::t("preview_empty", &LOCALES_MAIL).using(cx)) }
                } @else {
                    ul {
                        @for preview in &previews.previews {
                            li {
                                a href=(cx.route(preview_path(preview.key))) {
                                    (preview.title.using(cx))
                                }
                                " "
                                code { (preview.key) }
                            }
                        }
                    }
                }
            }
        }))
        .render()
        .await
}

// GET {PREVIEW_PATH}/{key} - Vista previa de un correo.
pub(crate) async fn preview_get(
    request: HttpRequest,
    web::Path(key): web::Path<String>,
) -> Result<Markup, ErrorPage> {
    let previews = MailPreviews::collect();
    let Some(preview) = previews.previews.iter().find(|p| p.key == key) else {
        return Err(ErrorPage::NotFound(Some(request)));
    };
    let cx = Context::new(request.clone());
    let email = transport::prepare_preview((preview.build)(&cx));
    let title = preview.title.clone();
    Page::new(request)
        .with_title(title.clone())
        .with_child(Html::with(move |cx| render_preview(cx, &title, &email)))
        .render()
        .await
}

fn preview_path(key: &str) -> String {
    util::join!(PREVIEW_PATH, "/", key)
}

fn render_preview(cx: &mut Context, title: &Lc, email: &Email) -> Markup {
    let none = Lc::t("preview_none", &LOCALES_MAIL);
    let list = |mailboxes: &[crate::Mailbox]| {
        let mut list = String::new();
        for mailbox in mailboxes {
            if !list.is_empty() {
                list.push_str(", ");
            }
            list.push_str(&mailbox.to_string());
        }
        list
    };
    let headers = [
        ("preview_from", email.from().map(ToString::to_string)),
        (
            "preview_reply_to",
            email.reply_to().map(ToString::to_string),
        ),
        ("preview_to", Some(list(email.to()))),
        ("preview_cc", Some(list(email.cc()))),
        ("preview_bcc", Some(list(email.bcc()))),
        ("preview_subject", Some(email.subject().to_owned())),
    ];
    html! {
        p { a href=(cx.route(PREVIEW_PATH)) { (Lc::t("preview_back", &LOCALES_MAIL).using(cx)) } }
        h1 { (title.using(cx)) }
        dl {
            @for (label, value) in headers {
                dt { (Lc::t(label, &LOCALES_MAIL).using(cx)) }
                dd {
                    @match value.filter(|v| !v.is_empty()) {
                        Some(value) => (value),
                        None => em { (none.using(cx)) },
                    }
                }
            }
            dt { (Lc::t("preview_attachments", &LOCALES_MAIL).using(cx)) }
            dd {
                @if email.attachments().is_empty() {
                    em { (none.using(cx)) }
                } @else {
                    ul {
                        @for attachment in email.attachments() {
                            li {
                                (attachment.filename()) " (" (attachment.content_type()) ", "
                                (attachment.body().len()) " B"
                                @if let Some(id) = attachment.content_id() {
                                    ", cid:" (id)
                                }
                                ")"
                            }
                        }
                    }
                }
            }
        }
        @if let Some(html) = email.html() {
            h2 { (Lc::t("preview_html", &LOCALES_MAIL).using(cx)) }
            // `sandbox` vacío aísla el correo: sin scripts, sin formularios y sin navegar desde él.
            iframe
                title=[Lc::t("preview_html", &LOCALES_MAIL).lookup(cx)]
                srcdoc=(html.as_str())
                sandbox=""
                style="width: 100%; height: 640px; border: 1px solid #d1d5db;" {}
        }
        @if let Some(text) = email.text() {
            h2 { (Lc::t("preview_text", &LOCALES_MAIL).using(cx)) }
            pre style="white-space: pre-wrap;" { (text) }
        }
    }
}
