<div align="center">

<h1>PageTop User</h1>

<p>Gestión de usuarios, autenticación, roles y permisos para <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-user?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-user)
[![Crates.io](https://img.shields.io/crates/v/pagetop-user.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-user)
[![Descargas](https://img.shields.io/crates/d/pagetop-user.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-user)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-user#licencia)

</div>

Añade cuentas de usuario a una aplicación PageTop: registro, inicio de sesión con sesiones en la
base de datos, perfil propio, recuperación de contraseña y verificación del email por correo, y
control de acceso con roles y permisos. Incluye las páginas de administración de usuarios, roles y
permisos sobre `pagetop-admin`, hechas sólo con componentes del núcleo para que funcionen con
cualquier tema.

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Guía rápida

**Añade la dependencia** a tu `Cargo.toml`:

```toml
[dependencies]
pagetop-user = "..."
```

Los usuarios y las sesiones se guardan con `pagetop-seaorm`: la aplicación elige el motor de base
de datos con su *feature* (`postgres`, `mysql` o `sqlite`) y configura la conexión en la sección
`[database]`.

**Declara la extensión** como dependencia de tu aplicación o de la extensión que la requiera
(`&pagetop_user::User`, como en el ejemplo de más abajo). Al arrancar crea sus tablas y, si no hay
ninguna cuenta, el primer administrador, con los datos de `[user.seed]`:

```toml
[user.seed]
admin_username = "admin"
admin_email    = "admin@example.com"
# Sin 'admin_password', se genera una aleatoria y se muestra una sola vez al crearlo.
```

**Comprueba los permisos** en tus *handlers*. El usuario actual se inyecta automáticamente en el
contexto de cada `Page` al crearla con `Page::new()`, y las funciones del núcleo, como
`has_permission()`, lo consultan:

```rust,no_run
use pagetop::prelude::*;
use pagetop_user::prelude::*;

#[derive(Clone, Copy, Debug)]
enum MyPermission {
    SeeDashboard,
}

impl Permission for MyPermission {
    fn key(&self) -> CowStr {
        match self {
            Self::SeeDashboard => "myapp:see_dashboard".into(),
        }
    }
}

struct MyApp;

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_user::User]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/dashboard", web::get(dashboard))
    }
}

async fn dashboard(request: HttpRequest) -> Result<Markup, ErrorPage> {
    if !has_permission(&request, &MyPermission::SeeDashboard) {
        return Err(ErrorPage::NotFound(Some(request)));
    }
    Page::new(request)
        .with_child(Html::with(|_| html! { h1 { "Panel" } }))
        .render().await
}
```

## Páginas

| Ruta                      | Uso                                                          |
|---------------------------|--------------------------------------------------------------|
| `/user/login`             | Inicio de sesión, con el nombre de usuario o el email.       |
| `/user/register`          | Registro, si lo permite `allow_registration`.                |
| `/user`                   | Perfil propio: editarlo, cambiar la contraseña o el email.   |
| `/user/password/reset`    | Recuperación de la contraseña por correo.                    |
| `/admin/user/users`       | Administración de usuarios: alta, edición, roles, bloqueo... |
| `/admin/user/roles`       | Administración de roles y de sus permisos.                   |
| `/admin/user/permissions` | Catálogo de los permisos que declaran las extensiones.       |

## Nombres de usuario y email

Cada cuenta tiene un nombre de usuario y un email, obligatorios y únicos; se puede iniciar sesión
con cualquiera de los dos. Qué nombres de usuario se aceptan lo decide `identity_policy` en la
sección `[user]`:

```toml
[user]
identity_policy = "unicode"     # "strict" (por defecto), "unicode" o "permissive".
```

Con cualquier política, un nombre de usuario nunca contiene `@`, no se distinguen mayúsculas (`Ana`
y `ana` son el mismo usuario) y no se admiten nombres que se confunden a simple vista con otro
existente (`rnaria` si existe `maria`, o `аdmin` con una `а` cirílica). Las reglas completas de
cada política, y las del email, están en `IdentityPolicy` (módulo `config`).

## Correos

La recuperación de contraseña, la confirmación de una nueva dirección y, con
`require_email_verification`, la verificación de las cuentas nuevas envían un correo con un enlace
de un solo uso mediante `pagetop-mail`. Además, se avisa a la dirección de la cuenta cuando
cambia su contraseña (la cambie el usuario o un administrador) y cuando se pide cambiar su
dirección. Configura el envío en la sección `[mail]`, incluidos el remitente (`mail.from`), que
estos correos no indican, y la URL pública de los enlaces (`mail.base_url`): sin configurar, los
correos se escriben en las trazas en lugar de enviarse.

La recuperación de contraseña sólo se envía a cuentas activas. Mientras una dirección no está
verificada podría ser de otra persona, así que sus correos no incluyen nada que elija quien creó
la cuenta, ni el nombre de usuario ni el nombre visible. Una dirección queda verificada al
confirmarla desde el enlace del correo de verificación (abrirlo no basta: hay que pulsar un botón,
porque los servidores de correo abren los enlaces para analizarlos) o cuando la fija un
administrador.

Para cambiar su dirección, el usuario indica la nueva y su contraseña actual. La nueva no se aplica
hasta que se confirma desde el enlace que se le envía, y entonces queda verificada; mientras tanto,
la cuenta conserva la anterior, que recibe un aviso. Restablecer la contraseña descarta el cambio
pendiente: es lo que el aviso pide hacer a quien no lo ha pedido. También lo descarta un
administrador que fije otra dirección. Cada enlace sirve sólo para la dirección a la que se envió,
y los enlaces sin usar de recuperación y de verificación dejan de valer cuando cambia la dirección
de la cuenta; los de recuperación, también cuando cambia su contraseña.

Cambiar la contraseña o el email exige la contraseña actual, y sus fallos cuentan para el bloqueo
por intentos fallidos (`max_failed_logins`), como los del inicio de sesión.

Para que nadie pueda usar la aplicación para enviar correos a direcciones ajenas, las recuperaciones
de contraseña y los cambios de dirección se limitan por cuenta (`max_password_resets` y
`max_email_changes`), y ellos y los registros, también por dirección IP (`max_ip_requests`).

Los inicios de sesión fallidos se limitan también por dirección IP (`max_ip_failed_logins`), para
frenar a quien prueba unas pocas contraseñas habituales en muchas cuentas sin llegar a bloquear
ninguna. Detrás de un *proxy* inverso, los límites por IP requieren configurar
`server.client_ip_header`.

## Créditos

Este *crate* guarda las contraseñas con el algoritmo Argon2 mediante
[argon2](https://github.com/RustCrypto/password-hashes), de RustCrypto, y detecta los nombres de
usuario que se confunden con otro con
[unicode-security](https://github.com/unicode-rs/unicode-security). Ambos se distribuyen bajo
licencia MIT o Apache 2.0, a elegir.

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
