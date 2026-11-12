//! Límites de solicitudes por dirección IP: las que envían un correo o crean una cuenta
//! (`user.max_ip_requests`) y los inicios de sesión fallidos (`user.max_ip_failed_logins`), y de
//! los inicios de sesión fallidos por identificador tecleado (`user.max_failed_logins`).

use pagetop::prelude::*;

use crate::config::SETTINGS;

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::net::{IpAddr, Ipv6Addr};
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::{Duration as StdDuration, Instant};

// Claves distintas que se siguen como mucho en cada límite. Sin tope, quien enviara intentos con
// identificadores siempre distintos haría crecer la tabla sin medida; alcanzado, una clave nueva no
// se anota (los límites por dirección IP siguen aplicándose).
const MAX_TRACKED_KEYS: usize = 100_000;

// Momentos de las solicitudes de cada clave dentro de la ventana, y última limpieza.
struct Requests<K> {
    by_key: HashMap<K, VecDeque<Instant>>,
    purged_at: Instant,
}

// Cada límite lleva su propia cuenta.
type Log<K> = LazyLock<Mutex<Requests<K>>>;

const fn new_log<K>() -> Log<K> {
    LazyLock::new(|| {
        Mutex::new(Requests {
            by_key: HashMap::new(),
            purged_at: Instant::now(),
        })
    })
}

static REQUESTS: Log<IpAddr> = new_log();
static FAILED_LOGINS: Log<IpAddr> = new_log();
static FAILED_IDENTIFIERS: Log<String> = new_log();
static FAILED_ANYWHERE: Log<()> = new_log();

/// Anota una solicitud de la dirección del cliente y devuelve si se admite, según
/// [`max_ip_requests`](crate::config::UserSettings::max_ip_requests).
///
/// Una solicitud rechazada no se anota: la dirección vuelve a disponer de solicitudes en cuanto la
/// más antigua sale de la ventana. Las de dirección desconocida se admiten siempre.
pub(crate) fn allow(request: &HttpRequest) -> bool {
    let Some(ip) = request.client_ip() else {
        return true;
    };
    let allowed = allow_at(
        &mut lock(&REQUESTS),
        network(ip),
        Instant::now(),
        SETTINGS.max_ip_requests,
        window(SETTINGS.ip_request_window_secs),
    );
    if !allowed {
        trace::warn!("Request limit reached for address {ip}");
    }
    allowed
}

// Límites que se aplican a un intento de inicio de sesión: por dirección IP (o, sin dirección
// conocida y con un proveedor de autenticación activo, el global) y por identificador tecleado.
pub(crate) struct LoginKeys {
    ip: Option<IpAddr>,
    anywhere: bool,
    identifier: String,
}

impl LoginKeys {
    pub(crate) fn new(request: &HttpRequest, identifier: &str) -> Self {
        let ip = request.client_ip().map(network);
        LoginKeys {
            ip,
            // Sin dirección conocida no hay límite por IP. Con sólo cuentas locales queda el
            // bloqueo de cada cuenta; con un proveedor, cada intento llega además a un servicio
            // externo (p. ej. el directorio, que bloquearía la cuenta), así que se limita en
            // conjunto.
            anywhere: ip.is_none() && crate::provider::active_providers().next().is_some(),
            identifier: crate::identity::login_key(identifier),
        }
    }
}

/// Anota un intento de inicio de sesión y devuelve si se admite: según
/// [`max_ip_failed_logins`](crate::config::UserSettings::max_ip_failed_logins) para la dirección
/// del cliente y según [`max_failed_logins`](crate::config::UserSettings::max_failed_logins) para
/// el identificador tecleado, exista o no la cuenta.
///
/// El intento se anota antes de comprobar la contraseña, para que varios intentos simultáneos no
/// puedan superar el límite; si resulta correcto, se retira con [`forget_login()`]. Rechazado, no
/// se anota en ningún límite.
pub(crate) fn allow_login(keys: &LoginKeys) -> bool {
    let now = Instant::now();
    let ip_window = window(SETTINGS.ip_failed_login_window_secs);
    let ip_allowed = match keys.ip {
        Some(ip) => allow_at(
            &mut lock(&FAILED_LOGINS),
            ip,
            now,
            SETTINGS.max_ip_failed_logins,
            ip_window,
        ),
        None if keys.anywhere => allow_at(
            &mut lock(&FAILED_ANYWHERE),
            (),
            now,
            SETTINGS.max_ip_failed_logins,
            ip_window,
        ),
        None => true,
    };
    if !ip_allowed {
        match keys.ip {
            Some(ip) => trace::warn!("Failed login limit reached for address {ip}"),
            None => trace::warn!("Failed login limit reached for requests of unknown address"),
        }
        return false;
    }
    let max = u32::try_from(SETTINGS.max_failed_logins).unwrap_or(0);
    let identifier_allowed = allow_at(
        &mut lock(&FAILED_IDENTIFIERS),
        keys.identifier.clone(),
        now,
        max,
        window(SETTINGS.failed_login_window_secs),
    );
    if !identifier_allowed {
        trace::warn!("Failed login limit reached for an identifier");
        forget_ip(keys);
    }
    identifier_allowed
}

/// Retira un intento anotado por [`allow_login()`] con la contraseña correcta. Como un acceso
/// correcto pone a cero el recuento de fallos de la cuenta, también borra el del identificador.
pub(crate) fn forget_login(keys: &LoginKeys) {
    forget_ip(keys);
    lock(&FAILED_IDENTIFIERS).by_key.remove(&keys.identifier);
}

/// Retira el intento anotado por [`allow_login()`] para el identificador, pero no el de la
/// dirección IP. Para un intento que no se ha podido comprobar (un proveedor que no responde).
pub(crate) fn forget_identifier(keys: &LoginKeys) {
    forget_at(&mut lock(&FAILED_IDENTIFIERS), &keys.identifier);
}

/// Borra los fallos anotados para el nombre de usuario y el email de una cuenta. Se llama al
/// restablecer su contraseña, que también levanta el bloqueo de la cuenta.
pub(crate) fn forget_account(username: &str, email: &str) {
    let mut failed = lock(&FAILED_IDENTIFIERS);
    failed.by_key.remove(&crate::identity::login_key(username));
    failed.by_key.remove(&crate::identity::login_key(email));
}

fn forget_ip(keys: &LoginKeys) {
    match keys.ip {
        Some(ip) => forget_at(&mut lock(&FAILED_LOGINS), &ip),
        None if keys.anywhere => forget_at(&mut lock(&FAILED_ANYWHERE), &()),
        None => {}
    }
}

// Una tabla envenenada sigue siendo válida: cada operación la deja consistente antes de cualquier
// punto en que pudiera fallar.
fn lock<K>(log: &'static Log<K>) -> std::sync::MutexGuard<'static, Requests<K>> {
    log.lock().unwrap_or_else(PoisonError::into_inner)
}

fn window(secs: i64) -> StdDuration {
    StdDuration::from_secs(u64::try_from(secs).unwrap_or(0))
}

// Anota la solicitud si se admite.
fn allow_at<K: Eq + Hash>(
    requests: &mut Requests<K>,
    key: K,
    now: Instant,
    max: u32,
    window: StdDuration,
) -> bool {
    if max == 0 {
        return true;
    }
    let expired = |at: &Instant| now.duration_since(*at) >= window;
    // Sin limpiar, la tabla guardaría todas las claves vistas desde el arranque. Basta con hacerlo
    // una vez por ventana (o al llenarse): sólo quedan las claves con solicitudes recientes.
    let full = requests.by_key.len() >= MAX_TRACKED_KEYS;
    if full || now.duration_since(requests.purged_at) >= window {
        requests
            .by_key
            .retain(|_, times| times.back().is_some_and(|at| !expired(at)));
        requests.purged_at = now;
    }
    if requests.by_key.len() >= MAX_TRACKED_KEYS && !requests.by_key.contains_key(&key) {
        return true;
    }
    let times = requests.by_key.entry(key).or_default();
    while times.front().is_some_and(expired) {
        times.pop_front();
    }
    if times.len() >= usize::try_from(max).unwrap_or(usize::MAX) {
        return false;
    }
    times.push_back(now);
    true
}

// Retira la solicitud más reciente de la clave. Todas cuentan igual, así que da lo mismo cuál.
fn forget_at<K: Eq + Hash>(requests: &mut Requests<K>, key: &K) {
    if let Some(times) = requests.by_key.get_mut(key) {
        times.pop_back();
    }
}

// Una dirección IPv6 cuenta por su red `/64`: un mismo equipo o red local suele disponer de una red
// entera y podría cambiar de dirección en cada solicitud.
fn network(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V4(_) => ip,
        IpAddr::V6(v6) => {
            let mut segments = v6.segments();
            segments[4..].fill(0);
            IpAddr::V6(Ipv6Addr::from(segments))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOW: StdDuration = StdDuration::from_secs(60);

    fn requests(start: Instant) -> Requests<IpAddr> {
        Requests {
            by_key: HashMap::new(),
            purged_at: start,
        }
    }

    fn ip(value: &str) -> IpAddr {
        value.parse().unwrap()
    }

    #[pagetop::test]
    async fn requests_over_the_limit_are_rejected_until_the_oldest_expires() {
        let start = Instant::now();
        let mut log = requests(start);
        let at = |secs| start + StdDuration::from_secs(secs);
        assert!(allow_at(&mut log, ip("192.0.2.1"), at(0), 2, WINDOW));
        assert!(allow_at(&mut log, ip("192.0.2.1"), at(10), 2, WINDOW));
        assert!(!allow_at(&mut log, ip("192.0.2.1"), at(20), 2, WINDOW));
        // Other addresses keep their own count.
        assert!(allow_at(&mut log, ip("192.0.2.2"), at(20), 2, WINDOW));
        // A rejected request is not counted: the first one expires at 60 s.
        assert!(allow_at(&mut log, ip("192.0.2.1"), at(60), 2, WINDOW));
        assert!(!allow_at(&mut log, ip("192.0.2.1"), at(61), 2, WINDOW));
    }

    #[pagetop::test]
    async fn forgotten_requests_free_their_place() {
        let start = Instant::now();
        let mut log = requests(start);
        assert!(allow_at(&mut log, ip("192.0.2.1"), start, 1, WINDOW));
        forget_at(&mut log, &ip("192.0.2.1"));
        assert!(allow_at(&mut log, ip("192.0.2.1"), start, 1, WINDOW));
        assert!(!allow_at(&mut log, ip("192.0.2.1"), start, 1, WINDOW));
        // Unknown addresses are ignored.
        forget_at(&mut log, &ip("192.0.2.2"));
    }

    #[pagetop::test]
    async fn zero_maximum_never_limits() {
        let start = Instant::now();
        let mut log = requests(start);
        for _ in 0..10 {
            assert!(allow_at(&mut log, ip("192.0.2.1"), start, 0, WINDOW));
        }
        assert!(log.by_key.is_empty());
    }

    #[pagetop::test]
    async fn addresses_without_recent_requests_are_purged() {
        let start = Instant::now();
        let mut log = requests(start);
        allow_at(&mut log, ip("192.0.2.1"), start, 5, WINDOW);
        allow_at(&mut log, ip("192.0.2.2"), start + WINDOW, 5, WINDOW);
        assert_eq!(log.by_key.len(), 1);
        assert!(log.by_key.contains_key(&ip("192.0.2.2")));
    }

    #[pagetop::test]
    async fn ipv6_addresses_count_by_their_64_network() {
        assert_eq!(network(ip("2001:db8:1:2:aaaa::1")), ip("2001:db8:1:2::"));
        assert_eq!(network(ip("192.0.2.1")), ip("192.0.2.1"));
    }

    #[pagetop::test]
    async fn keys_of_any_type_are_limited_and_forgotten() {
        let start = Instant::now();
        let mut log: Requests<String> = Requests {
            by_key: HashMap::new(),
            purged_at: start,
        };
        assert!(allow_at(&mut log, "ana".to_owned(), start, 1, WINDOW));
        assert!(!allow_at(&mut log, "ana".to_owned(), start, 1, WINDOW));
        assert!(allow_at(&mut log, "bea".to_owned(), start, 1, WINDOW));
        forget_at(&mut log, &"ana".to_owned());
        assert!(allow_at(&mut log, "ana".to_owned(), start, 1, WINDOW));
    }
}
