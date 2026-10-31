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

**Configura el envío** en la sección `[mail]` (todas las opciones en el módulo `config`):

```toml
[mail]
transport = "smtp"
from      = "Mi Aplicación <no-reply@example.com>"
base_url  = "https://example.com"

[mail.smtp]
host     = "smtp.example.com"
security = "starttls"           # "none", "opportunistic", "starttls" o "tls".
username = "no-reply@example.com"
password = "secreto"            # Mejor en config/local.toml, que no se versiona, o en la
                                # variable de entorno PAGETOP_MAIL__SMTP__PASSWORD.
```

Sin configurar, los correos se escriben en las trazas en lugar de enviarse (`transport = "log"`),
así que una aplicación en desarrollo nunca envía correo real por error. La configuración se valida
al arrancar: un ajuste no válido detiene la aplicación con un mensaje que indica el problema.

**Envía un correo** con `Email`:

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

`Email::send()` espera a que el correo se entregue; `Email::send_later()` lo envía en segundo plano
y registra en las trazas los errores.

## Plantillas

Una `MailTemplate` genera el asunto, el HTML y el texto de un correo a partir de sus datos, con los
textos traducidos mediante `Lc` al idioma del contexto que recibe. Así, cada extensión define sus
correos como tipos propios, y el mismo correo se envía a cada destinatario en su idioma:

```rust,no_run
use pagetop::prelude::*;
use pagetop_mail::prelude::*;

include_locales!(LOCALES_SHOP);

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

Los enlaces de un correo deben ser absolutos. `absolute_url()` antepone a una ruta la URL pública de
la aplicación, que se configura en `mail.base_url`. No se deduce de la petición, porque la cabecera
`Host` la elige el cliente y un atacante podría hacer que los enlaces de un correo de recuperación
de contraseña apuntaran a su propio servidor.

## Adjuntos e imágenes incrustadas

Un `Attachment` se crea con un contenido en memoria o leyendo un archivo, y su tipo MIME se deduce
del nombre. Con `Attachment::with_content_id()` se incrusta en el HTML, que lo muestra con `cid:`:

```rust,no_run
use pagetop::prelude::*;
use pagetop_mail::prelude::*;

async fn report(png: Vec<u8>) -> Result<(), MailError> {
    Email::new()
        .with_to("ana@example.com")
        .with_subject("Informe")
        .with_html(html! { img src="cid:logo" alt="Logo"; p { "Adjunto el informe." } })
        .with_attachment(Attachment::new("logo.png", png).with_content_id("logo"))
        .with_attachment(Attachment::from_path("informes/octubre.pdf").await?)
        .send()
        .await
}
```

## Diseño común del HTML

El HTML de los correos es sólo su contenido: al enviarlos se envuelve en un documento con un diseño
sobrio y neutro. La aplicación puede darle su propia imagen a **todos** los correos, incluidos los
de otras extensiones, registrando la acción `RenderMailLayout`.

## Vistas previas en el navegador

Con `previews = true` en `[mail]` (sólo en desarrollo), `/mail/preview` lista los correos que las
extensiones declaran con la acción `DeclareMailPreviews`, y muestra cada uno con datos de ejemplo:
cabeceras, HTML con el diseño común, texto plano y adjuntos, sin enviar nada.

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

- `Email::send_later()` guarda el correo y lo intenta enseguida. Si falla por un error temporal,
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
| `"memory"`   | Pruebas. Guarda los correos en memoria para comprobarlos con `outbox`.      |

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

## Créditos

Este *crate* se apoya en [lettre](https://lettre.rs), la biblioteca de correo de referencia en Rust,
distribuida bajo licencia [MIT](https://github.com/lettre/lettre/blob/master/LICENSE).

## Advertencia

**PageTop** es un proyecto personal para aprender [Rust](https://www.rust-lang.org/es) y conocer su
ecosistema. Su API está sujeta a cambios frecuentes. No se recomienda su uso en producción, al menos
hasta que se libere la versión **1.0.0**.

## Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
