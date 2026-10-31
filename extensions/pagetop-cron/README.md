<div align="center">

<h1>PageTop Cron</h1>

<p>Programación de tareas para aplicaciones <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-cron?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-cron)
[![Crates.io](https://img.shields.io/crates/v/pagetop-cron.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-cron)
[![Descargas](https://img.shields.io/crates/d/pagetop-cron.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-cron)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-cron#licencia)

</div>

Ejecuta trabajos periódicos en segundo plano mientras la aplicación está en marcha: cada noche a
las 4:30, cada diez minutos, el último día de cada mes, o unos segundos después de terminar la
ejecución anterior. Cada extensión declara sus trabajos en el código con una acción, y quien
administra la aplicación decide en los archivos de configuración cómo se ejecutan, y puede cambiar
cuándo se ejecuta cada trabajo o desactivarlo sin recompilar.

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Guía rápida

**Añade la dependencia** a tu `Cargo.toml`:

```toml
[dependencies]
pagetop-cron = "..."
```

**Escribe el trabajo** como una función asíncrona que recibe un `JobContext` y devuelve un
`JobResult`. Con `?` puede propagar cualquier error, que se registra en las trazas:

```rust,no_run
use pagetop::prelude::*;
use pagetop_cron::prelude::*;

async fn cleanup(_cx: JobContext) -> JobResult {
    // Borra los carritos abandonados hace más de 30 días...
    Ok(())
}
```

**Declara el trabajo** con la acción `DeclareCronJobs` en la extensión que lo aporta, que depende
de `pagetop-cron`:

```rust,no_run
use pagetop::prelude::*;
use pagetop_cron::prelude::*;

fn declare_jobs(jobs: &mut CronJobs) {
    jobs.add(
        Job::new("shop.cleanup", Schedule::cron("30 4 * * *"), cleanup)
            .with_title(Lc::n("Remove abandoned carts"))
            .with_timeout(Duration::minutes(15)),
    );
}

struct Shop;

#[async_trait]
impl Extension for Shop {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_cron::Cron]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![DeclareCronJobs::new(declare_jobs)]
    }
}
```

Y ya está: al ejecutar la aplicación con `Application::run()`, el trabajo se ejecuta cada día a
las 4:30 en la zona horaria de la aplicación (`[app].timezone`). La clave (`"shop.cleanup"`)
identifica el trabajo en las trazas, en la configuración y al ejecutarlo bajo demanda; conviene
empezarla por el nombre de la extensión.

## Cuándo se ejecuta un trabajo

`Schedule` ofrece tres formas de programarlo:

```rust,no_run
use pagetop::prelude::*;
use pagetop_cron::prelude::*;

// Según una expresión cron: cada día a las 4:30.
Schedule::cron("30 4 * * *");
// Cada cinco minutos desde que *empieza* la ejecución anterior (ritmo fijo).
Schedule::fixed_rate(Duration::minutes(5));
// Treinta segundos después de que *termina* la ejecución anterior (nunca se solapan).
Schedule::fixed_delay(Duration::seconds(30));
```

Las expresiones cron siguen la sintaxis clásica de cinco campos, con un sexto campo opcional para
los segundos y un séptimo, también opcional, para el año, y admiten además `L`, `W` y `#` (último
día del mes, día laborable más cercano, enésimo día de la semana) y apodos como `@daily`. Se
interpretan en la zona horaria de la aplicación y respetan los cambios de hora. Todos los
detalles, con ejemplos, en `Schedule`.

Para procesar colas o tareas pendientes, `fixed_delay()` suele ser la mejor opción: cada ejecución
procesa lo que dejó la anterior y nunca coinciden dos.

## Opciones de cada trabajo

`Job` ofrece además:

| Método                | Uso                                                              |
|-----------------------|------------------------------------------------------------------|
| `with_title()`        | Título descriptivo, traducible, para listados y paneles.         |
| `with_timeout()`      | Tiempo máximo de cada ejecución; si lo supera, se cancela.       |
| `with_overlap()`      | Qué hacer si la ejecución anterior sigue en curso (`Overlap`).   |
| `with_timezone()`     | Zona horaria propia para su expresión cron.                      |
| `with_run_on_start()` | Se ejecuta también al arrancar la aplicación.                    |

Si llega la hora de un trabajo y su ejecución anterior aún no ha terminado, por defecto se omite la
nueva (`Overlap::Skip`); con `Overlap::Allow` se ejecuta a la vez.

La función del trabajo puede ser también un cierre. Accede a los ajustes y servicios de la
aplicación igual que un *handler*, p. ej. con los `SETTINGS` de cada extensión o con `dbconn()` de
`pagetop-seaorm`:

```rust,no_run
use pagetop::prelude::*;
use pagetop_cron::prelude::*;

let job = Job::new("app.heartbeat", Schedule::fixed_rate(Duration::minutes(1)), |cx| async move {
    trace::info!("Heartbeat from \"{}\" ({:?})", cx.key(), cx.trigger());
    Ok(())
});
```

Un `panic!` dentro de un trabajo no afecta a la aplicación ni a sus siguientes ejecuciones: se
registra como un fallo más.

## Modos de ejecución

Quien administra la aplicación elige cómo se ejecutan los trabajos con `mode`, en la sección
`[cron]` (todas las opciones en `config`):

| `mode`       | Los trabajos se ejecutan...                                             |
|--------------|-------------------------------------------------------------------------|
| `"internal"` | Por defecto. Con el planificador de la propia aplicación.               |
| `"external"` | Cuando un programa externo llama a `POST /cron` con la clave secreta.   |
| `"off"`      | Nunca de forma automática; sólo bajo demanda con `run_now()`.           |

### Una sola instancia: `"internal"`

Con `"internal"`, cada proceso de la aplicación lleva su propio reloj y lanza los trabajos cuando
les toca. Con un solo proceso siempre en marcha (un servidor, un servicio de *systemd*, un
contenedor) es justo lo que se necesita, y no hay que configurar nada.

### Varias instancias

Si la misma aplicación se ejecuta en varios procesos a la vez, cada uno lanza todos los trabajos
con `"internal"`:

```text
               +--> instancia A (internal) --> limpieza a las 4:30
balanceador ---+--> instancia B (internal) --> limpieza a las 4:30
               +--> instancia C (internal) --> limpieza a las 4:30
```

La limpieza se hace tres veces, y un boletín se enviaría tres veces. Las instancias no se ponen de
acuerdo entre sí, así que hay dos formas de evitarlo:

- **`"internal"` en una instancia y `"off"` en las demás.** Es lo más sencillo si las instancias
  se configuran a mano, pero esa instancia queda distinta del resto: si se detiene, nadie ejecuta
  los trabajos.
- **`"external"` en todas.** Ninguna tiene reloj propio: un único reloj fuera de la aplicación
  llama periódicamente a `POST /cron`, y el balanceador entrega cada llamada a una sola instancia,
  que ejecuta lo programado desde la llamada anterior:

```text
crontab (en una sola máquina) --POST /cron--> balanceador --> instancia B ejecuta lo pendiente
```

### Cuándo usar `"external"`

- **Réplicas idénticas o autoescalado** (Kubernetes, Docker Swarm, varias máquinas tras un
  balanceador). Todas arrancan con la misma imagen y la misma configuración, así que no se puede
  dejar una distinta. Con `"external"` todas son iguales y cualquiera puede atender la llamada.
- **Plataformas que detienen la aplicación cuando no hay tráfico** (escalado a cero). Si nadie
  visita el sitio de madrugada, no queda ningún proceso vivo y el reloj interno de las 4:30 no
  existe; algunas plataformas, además, sólo dan CPU mientras se atiende una petición. La llamada
  externa despierta la aplicación y le da tiempo para ejecutar lo pendiente. Estas plataformas
  suelen ofrecer su propio planificador para hacerla.
- **Planificar desde las herramientas de operaciones** que ya se usan, como un *CronJob* de
  Kubernetes o un temporizador de *systemd*. La llamada responde `200 OK` aunque falle algún
  trabajo, para que un reintento no repita los demás; el resultado de cada uno viene en el cuerpo
  de la respuesta y, para avisar de los fallos, está la acción `AfterJobRun`.

Con una sola instancia siempre en marcha, `"external"` no aporta nada: añade piezas (un `crontab`,
una clave secreta, una ruta expuesta) y pierde precisión. Mejor `"internal"`.

### Configurar `"external"`

```toml
[cron]
mode                  = "external"
trigger_token         = "..."   # Clave secreta propia, mejor en config/local.toml o en
                                # PAGETOP_CRON__TRIGGER_TOKEN.
trigger_interval_secs = 60      # Una llamada por minuto (el valor por defecto).
```

La clave debe tener al menos 16 caracteres (letras, dígitos, `-` o `_`), o la aplicación no
arranca; `openssl rand -hex 32` genera una adecuada. Viaja en la cabecera
`Authorization: Bearer <clave>`, no en la ruta, porque los *proxies* y los balanceadores suelen
registrar las rutas de las peticiones. Para que tampoco aparezca en la lista de procesos del
sistema, `curl` puede leer la cabecera de un archivo que sólo pueda leer el usuario del `crontab`:

```text
# crontab del sistema: comprueba los trabajos cada minuto. El archivo contiene la línea
# "Authorization: Bearer <clave>".
* * * * * curl -fsS -X POST -H @/etc/miapp/cron-header https://example.com/cron > /dev/null
```

La llamada responde cuando terminan los trabajos que ejecuta, con `200 OK` y una línea por trabajo
con su resultado (`succeeded`, `failed`, `timed out` o `panicked`); sin la clave correcta responde
`404 Not Found`, igual que una ruta que no existe. Esperar mantiene viva la aplicación en las
plataformas que sólo dan CPU mientras se atiende una petición, así que el tiempo de espera del
programa externo debe superar lo que tarde el trabajo más largo; conviene limitarlo con
`with_timeout()`. Si el programa externo corta antes, los trabajos siguen igualmente, pero en esas
plataformas pueden quedarse sin CPU.

Cada llamada ejecuta lo programado en el último minuto, sea cual sea la instancia que la recibe y
aunque acabe de arrancar: una llamada a las 4:30:20 ejecuta lo programado desde las 4:29:00
(excluido) hasta las 4:30:00. Así ninguna instancia necesita recordar qué hicieron las demás, pero
el programa externo debe llamar una vez, y sólo una, cada minuto: si falta una llamada se pierden
sus ejecuciones, y si sobra, dos instancias pueden repetirlas. Para llamar con otra frecuencia
(p. ej. cada cuarto de hora, en una plataforma que despierta la aplicación en cada llamada),
indícala en `trigger_interval_secs` (`900`) y programa los trabajos en múltiplos de ella. La
precisión es la del intervalo: con una llamada por minuto, un trabajo programado cada diez
segundos se ejecuta una vez por minuto.

Una llamada de más suele ser un reintento. La respuesta es `200 OK` aunque falle algún trabajo, así
que el programa externo sólo reintenta tras un error de red o si se agota su tiempo de espera; pero
si el reintento llega a otra instancia, ésta ejecuta de nuevo lo mismo, porque las instancias no
comparten estado. Desactiva los reintentos del programa externo (p. ej. `backoffLimit: 0` en un
*CronJob* de Kubernetes) y, si un trabajo no debe repetirse nunca, que lo compruebe él mismo (p. ej.
en la base de datos) antes de hacer nada.

## Ajustar un trabajo desde la configuración

Cada trabajo puede cambiarse por su clave, sin tocar el código, en la subsección
`[cron.jobs."clave"]` (ver `JobSettings`):

```toml
[cron.jobs."shop.cleanup"]
schedule     = "0 5 * * SUN"            # Sólo los domingos a las 5:00.
timeout_secs = 1800                     # Tiempo máximo de 30 minutos (0 = sin límite).

[cron.jobs."shop.reminders"]
enabled = false                         # Desactivado: sólo se ejecuta bajo demanda.
```

La programación admite la misma sintaxis que `Schedule::parse()`, incluidos `"@every 10m"` y
`"@delay 30s"`. Una programación no válida detiene el arranque indicando el trabajo, y una clave
que no corresponde a ningún trabajo se ignora con un aviso en las trazas.

## Ejecutar y consultar los trabajos

`run_now()` ejecuta un trabajo en el acto y devuelve su `JobRun`: útil para un botón
«Ejecutar ahora» de un panel de administración, para una orden de mantenimiento o para las
pruebas. `jobs()` y `job()` devuelven su `JobStatus`: programación efectiva, próxima
ejecución, resultado de la última y número de ejecuciones, fallos y omisiones.

Para reaccionar a cada ejecución terminada (registrarla, enviar métricas, avisar cuando falla un
trabajo), registra la acción `AfterJobRun`.

## Detener la aplicación

Al detener la aplicación (Ctrl+C o `SIGTERM`), el planificador deja de lanzar ejecuciones y espera
a que terminen las que están en curso, hasta `cron.shutdown_timeout_secs` (30 s por defecto). Un
trabajo largo puede comprobar `JobContext::is_stopping()` para dejar su trabajo en un punto
seguro antes de que se agote ese tiempo.

## Pruebas

El planificador sólo se pone en marcha con `Application::run()` (ver `Extension::start()`), nunca
en las pruebas que preparan la aplicación y obtienen su servidor con `Application::test()`. En
ellas, los trabajos se ejecutan cuando la prueba quiere con `run_now()`:

```rust,no_run
use pagetop::prelude::*;

#[pagetop::test]
async fn cleanup_job_succeeds() {
    Application::prepare(&Shop).await;
    let run = pagetop_cron::run_now("shop.cleanup").await.unwrap();
    assert!(run.outcome().is_success(), "{}", run.outcome());
}
```

## Qué no hace

Para mantenerse sencilla, la extensión no guarda nada fuera de la memoria:

- **No recupera las ejecuciones perdidas.** Si la aplicación estaba detenida cuando tocaba una
  ejecución, no se hace al volver a arrancar; la siguiente llega según su programación. Para
  trabajos que no deben perder ninguna, combina la programación con `with_run_on_start()`, o
  guarda en la base de datos qué queda pendiente y procésalo en cada ejecución.
- **No es una cola de trabajos.** Para tareas puntuales que deben sobrevivir a un reinicio (un
  correo que no se pudo enviar, una importación...), cada extensión guarda las pendientes en su
  propia tabla y declara un trabajo periódico que las procesa.
- **No coordina varias instancias** por sí misma; ver «Varias instancias».

## Créditos

Las expresiones cron se interpretan con [croner](https://github.com/hexagon/croner-rust), la misma
biblioteca que usa *tokio-cron-scheduler*, distribuida bajo licencia
[MIT](https://github.com/hexagon/croner-rust/blob/master/LICENSE).

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
