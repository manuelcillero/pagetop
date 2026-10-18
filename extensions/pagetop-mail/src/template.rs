//! Plantillas de correo ([`MailTemplate`]).

use pagetop::prelude::*;

/// Plantilla de correo: asunto, contenido HTML y texto plano generados a partir de unos datos.
///
/// Una plantilla es un tipo propio con los datos que necesita el correo (nombre del destinatario,
/// enlaces, importes...). Sus métodos reciben un [`Context`] con el idioma del destinatario, así
/// que los textos se escriben con [`Lc`] y el HTML con [`html!`], igual que en un componente, y
/// las fechas se formatean con los métodos de [`Contextual`].
///
/// El correo se crea con [`Email::from_template()`](crate::Email::from_template), que llama a los
/// tres métodos. El HTML es sólo el **contenido**: al enviarlo se envuelve en el diseño común de
/// los correos (ver [`RenderMailLayout`](crate::RenderMailLayout)). El texto plano es opcional: si
/// la plantilla sólo genera HTML, al enviar el correo se obtiene a partir de él. Escríbelo sólo si
/// el generado no queda bien.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_mail::prelude::*;
///
/// include_locales!(LOCALES_SHOP);
///
/// struct OrderShipped {
///     customer: String,
///     order: u32,
///     tracking_url: String,
/// }
///
/// impl MailTemplate for OrderShipped {
///     fn subject(&self, cx: &Context) -> String {
///         Lc::t("mail-shipped-subject", &LOCALES_SHOP)
///             .with_arg("order", self.order.to_string())
///             .lookup(cx)
///             .unwrap_or_default()
///     }
///
///     fn html(&self, cx: &Context) -> Option<Markup> {
///         let greeting =
///             Lc::t("mail-greeting", &LOCALES_SHOP).with_arg("name", self.customer.clone());
///         let track = Lc::t("mail-track", &LOCALES_SHOP);
///         Some(html! {
///             p { (greeting.using(cx)) }
///             p { a href=(self.tracking_url) { (track.using(cx)) } }
///         })
///     }
///
///     fn text(&self, cx: &Context) -> Option<String> {
///         let greeting = Lc::t("mail-greeting", &LOCALES_SHOP)
///             .with_arg("name", self.customer.clone())
///             .lookup(cx)
///             .unwrap_or_default();
///         Some(util::join!(greeting, "\n\n", &self.tracking_url))
///     }
/// }
///
/// async fn notify(language: &impl LangId) -> Result<(), MailError> {
///     let template = OrderShipped {
///         customer: "Ana".to_string(),
///         order: 1234,
///         tracking_url: absolute_url("/orders/1234"),
///     };
///     let cx = Context::default().with_langid(language);
///     Email::from_template(&template, &cx)
///         .with_to("ana@example.com")
///         .send()
///         .await
/// }
/// ```
pub trait MailTemplate {
    /// Asunto del correo, en el idioma del contexto.
    fn subject(&self, cx: &Context) -> String;

    /// Contenido HTML del correo, sin el documento que lo envuelve. Por defecto, ninguno.
    fn html(&self, _cx: &Context) -> Option<Markup> {
        None
    }

    /// Texto plano del correo. Por defecto, ninguno: al enviarlo se genera a partir del HTML.
    fn text(&self, _cx: &Context) -> Option<String> {
        None
    }
}
