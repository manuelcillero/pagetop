//! Diseño común del HTML de los correos y acción [`RenderMailLayout`] para cambiarlo.

use pagetop::prelude::*;

use crate::{Email, LOCALES_MAIL};

use std::ops::ControlFlow;

/// Tipo de función que genera el documento HTML completo de un correo.
///
/// Recibe:
///
/// - `cx`: contexto con el idioma del correo, para traducir los textos del diseño.
/// - `email`: el correo, p. ej. para usar su asunto como título del documento.
/// - `content`: el contenido HTML del correo, que el diseño debe incluir sin modificar.
pub type FnRenderMailLayout = fn(cx: &Context, email: &Email, content: &Markup) -> Markup;

/// Acción para cambiar el diseño común del HTML de los correos.
///
/// Todos los correos con contenido HTML ([`Email::with_html()`] o una
/// [`MailTemplate`](crate::MailTemplate)) se envuelven al enviarlos en un documento HTML completo:
/// cabecera, estilos y pie. El diseño por defecto es sobrio y neutro: un recuadro blanco de hasta
/// 600 píxeles de ancho sobre fondo gris, con el nombre de la aplicación al pie.
///
/// Una aplicación (o una extensión) registra esta acción para dar a **todos** los correos su propia
/// imagen, incluidos los que envían otras extensiones como `pagetop-user`. Si hay varias, se aplica
/// sólo la de menor peso.
///
/// Los clientes de correo ignoran las hojas de estilo externas y muchos ignoran también los estilos
/// del `<head>`: lo seguro es escribir los estilos en línea (`style="..."`) y maquetar con tablas.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_mail::prelude::*;
///
/// fn my_layout(cx: &Context, email: &Email, content: &Markup) -> Markup {
///     html! {
///         (DOCTYPE)
///         html lang=(cx.langid().to_string()) {
///             head {
///                 meta charset="utf-8";
///                 title { (email.subject()) }
///             }
///             body style="font-family: Georgia, serif;" {
///                 img src="https://example.com/logo.png" alt="Example" width="120";
///                 (content)
///             }
///         }
///     }
/// }
///
/// struct MyApp;
///
/// #[async_trait]
/// impl Extension for MyApp {
///     fn dependencies(&self) -> Vec<ExtensionRef> {
///         vec![&pagetop_mail::Mail]
///     }
///
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![RenderMailLayout::new(my_layout)]
///     }
/// }
/// ```
///
/// [`Email::with_html()`]: crate::Email::with_html
pub struct RenderMailLayout {
    f: FnRenderMailLayout,
    weight: Weight,
}

impl ActionDispatcher for RenderMailLayout {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl RenderMailLayout {
    /// Registra una función que genera el documento HTML de los correos.
    pub fn new(f: FnRenderMailLayout) -> Self {
        RenderMailLayout { f, weight: 0 }
    }

    /// Opcional. Si varias extensiones registran esta acción, se aplica la de menor peso. Se pueden
    /// usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}

// Genera el documento HTML del correo con la acción de menor peso o, sin ninguna, con el diseño por
// defecto.
pub(crate) fn render(cx: &Context, email: &Email, content: &Markup) -> Markup {
    let mut document = None;
    try_dispatch_actions(|action: &RenderMailLayout| {
        document = Some((action.f)(cx, email, content));
        ControlFlow::Break(())
    });
    document.unwrap_or_else(|| default_layout(cx, email, content))
}

// Estilos en línea del diseño por defecto.
const PAGE_STYLE: &str = "margin: 0; padding: 0; background-color: #f4f4f5;";
const CARD_STYLE: &str = "max-width: 600px; background-color: #ffffff; border-radius: 6px;";
const CONTENT_STYLE: &str = concat!(
    "font-family: -apple-system, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; ",
    "padding: 32px; font-size: 16px; line-height: 1.5; color: #1f2937;"
);
const FOOTER_STYLE: &str = concat!(
    "font-family: -apple-system, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; ",
    "margin: 16px 0 0; font-size: 12px; color: #6b7280;"
);

fn default_layout(cx: &Context, email: &Email, content: &Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang=(cx.langid().to_string()) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="color-scheme" content="light";
                title { (email.subject()) }
            }
            body style=(PAGE_STYLE) {
                table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0"
                    style=(PAGE_STYLE)
                {
                    tr {
                        td align="center" style="padding: 24px 12px;" {
                            table role="presentation" width="100%" cellpadding="0" cellspacing="0"
                                border="0" style=(CARD_STYLE)
                            {
                                tr {
                                    td style=(CONTENT_STYLE) { (content) }
                                }
                            }
                            p style=(FOOTER_STYLE) {
                                (Lc::t("mail_footer", &LOCALES_MAIL)
                                    .with_arg("app", &global::SETTINGS.app.name)
                                    .using(cx))
                            }
                        }
                    }
                }
            }
        }
    }
}
