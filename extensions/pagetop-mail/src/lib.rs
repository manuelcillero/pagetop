/*!
<div align="center">

<h1>PageTop Mail</h1>

<p>Envío de correo electrónico para aplicaciones <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-mail?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-mail)
[![Crates.io](https://img.shields.io/crates/v/pagetop-mail.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-mail)
[![Descargas](https://img.shields.io/crates/d/pagetop-mail.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-mail)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-mail#licencia)

</div>

Envía correos en texto plano y HTML, con archivos adjuntos e imágenes incrustadas, por SMTP, con el
programa *sendmail* o a archivos `.eml`, todo configurado en los archivos TOML de la aplicación.
Los correos pueden crearse con plantillas que traducen sus textos al idioma del destinatario, y
comparten un diseño HTML común que la aplicación puede cambiar. Para las pruebas, los correos pueden
guardarse en memoria en lugar de enviarse.

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Guía rápida

**Añade la dependencia** a tu `Cargo.toml`:

```toml
[dependencies]
pagetop-mail = "..."
```

**Declara la extensión** en tu aplicación o en la extensión que envíe correos:

```rust,no_run
use pagetop::prelude::*;

struct MyApp;

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_mail::Mail]
    }
}
```

**Configura el envío** en la sección `[mail]` (todas las opciones en [`config`]):

```toml
[mail]
transport = "smtp"
from      = "Mi Aplicación <no-reply@example.com>"
base_url  = "https://example.com"

[mail.smtp]
host     = "smtp.example.com"
security = "starttls"           # "none", "opportunistic", "starttls" o "tls".
username = "no-reply@example.com"
password = "secreto"            # Mejor en config/local.toml, que no se versiona.
```

Sin configurar, los correos se escriben en las trazas en lugar de enviarse (`transport = "log"`),
así que una aplicación en desarrollo nunca envía correo real por error. La configuración se valida
al arrancar: un ajuste no válido detiene la aplicación con un mensaje que indica el problema.

**Envía un correo** con [`Email`]:

```rust,no_run
use pagetop::prelude::*;
use pagetop_mail::prelude::*;

async fn notify() -> Result<(), MailError> {
    Email::new()
        .with_to("Ana Pérez <ana@example.com>")
        .with_subject("Tu pedido ha salido")
        .with_text("Tu pedido 1234 ha salido hoy.")
        .with_html(html! {
            p { "Tu pedido " strong { "1234" } " ha salido hoy." }
        })
        .send()
        .await
}
```

[`Email::send()`] espera a que el correo se entregue; [`Email::send_later()`] lo envía en segundo
plano y registra en las trazas los errores.

## Plantillas

Una [`MailTemplate`] genera el asunto, el HTML y el texto de un correo a partir de sus datos, con
los textos traducidos mediante [`Lc`] al idioma del contexto que recibe. Así, cada extensión
define sus correos como tipos propios, y el mismo correo se envía a cada destinatario en su idioma:

```rust,no_run
# use pagetop::prelude::*;
# use pagetop_mail::prelude::*;
# include_locales!(LOCALES_SHOP);
struct Welcome {
    name: String,
}

impl MailTemplate for Welcome {
    fn subject(&self, cx: &Context) -> String {
        Lc::t("welcome-subject", &LOCALES_SHOP).lookup(cx).unwrap_or_default()
    }

    fn html(&self, cx: &Context) -> Option<Markup> {
        let greeting = Lc::t("welcome-greeting", &LOCALES_SHOP).with_arg("name", self.name.clone());
        Some(html! {
            p { (greeting.using(cx)) }
            p {
                a href=(absolute_url("/shop")) { (Lc::t("welcome-visit", &LOCALES_SHOP).using(cx)) }
            }
        })
    }
}

async fn welcome(request: HttpRequest) -> Result<(), MailError> {
    // El contexto de la petición: el correo sale en el idioma de quien se acaba de registrar.
    let cx = Context::new(request);
    Email::from_template(&Welcome { name: "Ana".to_string() }, &cx)
        .with_to("ana@example.com")
        .send()
        .await
}
```

## Enlaces en los correos

Los enlaces de un correo deben ser absolutos. [`absolute_url()`] antepone a una ruta la URL pública
de la aplicación, que se configura en `mail.base_url`. No se deduce de la petición, porque la
cabecera `Host` la elige el cliente y un atacante podría hacer que los enlaces de un correo de
recuperación de contraseña apuntaran a su propio servidor.

## Adjuntos e imágenes incrustadas

Un [`Attachment`] se crea con un contenido en memoria o leyendo un archivo, y su tipo MIME se deduce
del nombre. Con [`Attachment::with_content_id()`] se incrusta en el HTML, que lo muestra con
`cid:`:

```rust,no_run
# use pagetop::prelude::*;
# use pagetop_mail::prelude::*;
# async fn example(png: Vec<u8>) -> Result<(), MailError> {
Email::new()
    .with_to("ana@example.com")
    .with_subject("Informe")
    .with_html(html! { img src="cid:logo" alt="Logo"; p { "Adjunto el informe." } })
    .with_attachment(Attachment::new("logo.png", png).with_content_id("logo"))
    .with_attachment(Attachment::from_path("informes/octubre.pdf").await?)
    .send()
    .await
# }
```

## Diseño común del HTML

El HTML de los correos es sólo su contenido: al enviarlos se envuelve en un documento con un diseño
sobrio y neutro. La aplicación puede darle su propia imagen a **todos** los correos, incluidos los
de otras extensiones, registrando la acción [`RenderMailLayout`].

## Vistas previas en el navegador

Con `previews = true` en `[mail]` (sólo en desarrollo), `/mail/preview` lista los correos que las
extensiones declaran con la acción [`DeclareMailPreviews`], y muestra cada uno con datos de ejemplo:
cabeceras, HTML con el diseño común, texto plano y adjuntos, sin enviar nada. Ver [`preview`].

## Texto plano automático

Si un correo sólo tiene HTML, al enviarlo se genera su texto plano a partir del contenido: bloques
separados por líneas, listas con guiones, y la dirección de cada enlace entre paréntesis.

## Cola de envío

Con la *feature* `queue`, los correos se guardan en una cola en la base de datos (con
`pagetop-seaorm`) y el trabajo `mail.queue` de `pagetop-cron` los envía por lotes:

```toml
[dependencies]
pagetop-mail = { version = "...", features = ["queue"] }
```

La cola usa la base de datos que la aplicación configura en `[database]` para `pagetop-seaorm`, con
la *feature* de su motor (`postgres`, `mysql` o `sqlite`), y crea en ella la tabla `mail_queue` al
arrancar. La tabla guarda cada mensaje completo y sin cifrar hasta entregarlo, o durante
`keep_failed_days` si falla: puede contener datos personales y enlaces de un solo uso, también en
las copias de seguridad de la base de datos.

- [`Email::send_later()`] guarda el correo y lo intenta enseguida. Si falla por un error temporal,
  queda en la cola y se reintenta más tarde, también tras un reinicio.
- `Email::queue()` sólo lo guarda: cada ejecución del trabajo envía, uno tras otro, como mucho
  `batch_size` correos, para que los envíos masivos no sobrecarguen el equipo ni el servidor de
  correo.

La espera entre reintentos se duplica en cada intento; un rechazo definitivo del servidor (SMTP
5xx) o el último intento dejan el correo como fallido. Si el servidor no responde, el lote se
interrumpe y los demás correos esperan a la siguiente ejecución, sin gastar intentos. Los ajustes
van en `[mail.queue]`, y el ritmo, en la programación del trabajo:

```toml
[mail.queue]
batch_size = 50

[cron.jobs."mail.queue"]
schedule = "@delay 30s"     # 30 s después de terminar cada lote (por defecto, 1 minuto).
```

## Transportes

Se elige en `mail.transport`:

| Valor        | Uso                                                                         |
|--------------|-----------------------------------------------------------------------------|
| `"smtp"`     | Producción. Servidor SMTP con conexiones reutilizables y cifrado TLS.       |
| `"sendmail"` | Producción, con un servidor de correo local (Postfix, Exim...).             |
| `"file"`     | Desarrollo. Guarda cada correo como `.eml` para abrirlo con un cliente.     |
| `"log"`      | Por defecto. Escribe los correos en las trazas, sin enviarlos.              |
| `"memory"`   | Pruebas. Guarda los correos en memoria para comprobarlos con [`outbox`].    |

En entornos de pruebas con datos reales, `mail.redirect_to` entrega todos los correos a una sola
dirección, sin cambiar sus cabeceras.

Para probar el envío SMTP en desarrollo sin enviar correo real, un servidor como
[Mailpit](https://mailpit.axllent.org) recoge los correos y los muestra en el navegador:

```toml
[mail]
transport = "smtp"

[mail.smtp]
host     = "localhost"
port     = 1025
security = "none"
```
*/

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/assets/favicon.ico"
)]

use pagetop::prelude::*;

use std::sync::LazyLock;

pub mod config;

mod error;
pub use error::MailError;

mod mailbox;
pub use mailbox::Mailbox;

mod attachment;
pub use attachment::Attachment;

mod template;
pub use template::MailTemplate;

mod layout;
pub use layout::{FnRenderMailLayout, RenderMailLayout};

mod email;
pub use email::Email;

pub mod outbox;

pub mod preview;
pub use preview::{DeclareMailPreviews, MailPreviews};

mod text;

mod transport;

#[cfg(feature = "queue")]
mod queue;

include_locales!(LOCALES_MAIL);

/// Prelude de `pagetop-mail`.
pub mod prelude {
    pub use crate::absolute_url;
    pub use crate::{Attachment, Email, MailError, MailTemplate, Mailbox};
    pub use crate::{DeclareMailPreviews, MailPreviews, RenderMailLayout};
}

/// Implementa la extensión `pagetop-mail`.
pub struct Mail;

#[async_trait]
impl Extension for Mail {
    fn name(&self) -> Lc {
        Lc::t("extension_name", &LOCALES_MAIL)
    }

    fn description(&self) -> Lc {
        Lc::t("extension_description", &LOCALES_MAIL)
    }

    #[cfg(feature = "queue")]
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_seaorm::SeaORM, &pagetop_cron::Cron]
    }

    #[cfg(feature = "queue")]
    fn actions(&self) -> Vec<ActionBox> {
        actions![pagetop_cron::DeclareCronJobs::new(queue::declare_jobs)]
    }

    fn settings(&self) -> Option<&'static dyn LoadSettings> {
        Some(&config::SETTINGS)
    }

    async fn initialize(&self) -> Result<(), BoxError> {
        // Construir el transporte al arrancar valida los ajustes; preparar la aplicación varias
        // veces en el mismo proceso (las pruebas lo hacen) reutiliza el ya construido.
        transport::mailer()?.log_startup();
        #[cfg(feature = "queue")]
        queue::install().await?;
        Ok(())
    }

    fn configure_router(&self, router: Router) -> Router {
        if !config::SETTINGS.previews {
            return router;
        }
        router
            .route(preview::PREVIEW_PATH, web::get(preview::list_get))
            .route(
                &util::join!(preview::PREVIEW_PATH, "/{key}"),
                web::get(preview::preview_get),
            )
    }
}

/// Devuelve la URL absoluta de una ruta de la aplicación, para los enlaces de los correos.
///
/// Antepone a la ruta la URL pública configurada en `mail.base_url` o, si no hay ninguna, la
/// dirección y el puerto de escucha del servidor (sólo útil en desarrollo). Una ruta que ya es una
/// URL externa (`https://...`) se devuelve sin cambios.
///
/// Si la ruta debe conservar el idioma de la petición en curso, constrúyela con
/// [`Context::route()`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_mail::prelude::*;
/// // Con `base_url = "https://example.com"`.
/// assert_eq!(absolute_url("/user/verify"), "https://example.com/user/verify");
/// ```
pub fn absolute_url(path: impl Into<RoutePath>) -> String {
    let path = path.into().to_string();
    if util::url_looks_external(&path) {
        return path;
    }
    if path.starts_with('/') {
        util::join!(base_url(), path)
    } else {
        util::join!(base_url(), "/", path)
    }
}

// URL pública de la aplicación, sin la barra final.
fn base_url() -> &'static str {
    static BASE_URL: LazyLock<String> = LazyLock::new(|| {
        match config::SETTINGS
            .base_url
            .as_deref()
            .and_then(util::non_blank)
        {
            Some(url) => url.trim_end_matches('/').to_owned(),
            None => {
                let server = &global::SETTINGS.server;
                util::join!(
                    "http://",
                    &server.bind_address,
                    ":",
                    server.bind_port.to_string()
                )
            }
        }
    });
    &BASE_URL
}
