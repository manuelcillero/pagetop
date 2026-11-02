//! Límites de solicitudes por dirección IP: las que envían un correo o crean una cuenta
//! (`user.max_ip_requests`) y los inicios de sesión fallidos (`user.max_ip_failed_logins`).

use pagetop::prelude::*;

use crate::config::SETTINGS;

use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, Ipv6Addr};
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::{Duration as StdDuration, Instant};

// Momentos de las solicitudes de cada dirección dentro de la ventana, y última limpieza.
struct Requests {
    by_ip: HashMap<IpAddr, VecDeque<Instant>>,
    purged_at: Instant,
}

// Cada límite lleva su propia cuenta.
type Log = LazyLock<Mutex<Requests>>;

const fn new_log() -> Log {
    LazyLock::new(|| {
        Mutex::new(Requests {
            by_ip: HashMap::new(),
            purged_at: Instant::now(),
        })
    })
}

static REQUESTS: Log = new_log();
static FAILED_LOGINS: Log = new_log();

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

/// Anota un intento de inicio de sesión de la dirección del cliente y devuelve si se admite, según
/// [`max_ip_failed_logins`](crate::config::UserSettings::max_ip_failed_logins).
///
/// El intento se anota antes de comprobar la contraseña, para que varios intentos simultáneos no
/// puedan superar el límite; si no resulta fallido, se retira con [`forget_login()`].
pub(crate) fn allow_login(request: &HttpRequest) -> bool {
    let Some(ip) = request.client_ip() else {
        return true;
    };
    let allowed = allow_at(
        &mut lock(&FAILED_LOGINS),
        network(ip),
        Instant::now(),
        SETTINGS.max_ip_failed_logins,
        window(SETTINGS.ip_failed_login_window_secs),
    );
    if !allowed {
        trace::warn!("Failed login limit reached for address {ip}");
    }
    allowed
}

/// Retira un intento anotado por [`allow_login()`] que no ha resultado fallido.
pub(crate) fn forget_login(request: &HttpRequest) {
    if let Some(ip) = request.client_ip() {
        forget_at(&mut lock(&FAILED_LOGINS), network(ip));
    }
}

fn lock(log: &Log) -> std::sync::MutexGuard<'_, Requests> {
    log.lock().unwrap_or_else(PoisonError::into_inner)
}

fn window(secs: i64) -> StdDuration {
    StdDuration::from_secs(u64::try_from(secs).unwrap_or(0))
}

// Anota la solicitud si se admite.
fn allow_at(
    requests: &mut Requests,
    ip: IpAddr,
    now: Instant,
    max: u32,
    window: StdDuration,
) -> bool {
    if max == 0 {
        return true;
    }
    let expired = |at: &Instant| now.duration_since(*at) >= window;
    // Sin limpiar, la tabla guardaría todas las direcciones vistas desde el arranque. Basta con
    // hacerlo una vez por ventana: sólo quedan las direcciones con solicitudes recientes.
    if now.duration_since(requests.purged_at) >= window {
        requests
            .by_ip
            .retain(|_, times| times.back().is_some_and(|at| !expired(at)));
        requests.purged_at = now;
    }
    let times = requests.by_ip.entry(ip).or_default();
    while times.front().is_some_and(expired) {
        times.pop_front();
    }
    if times.len() >= usize::try_from(max).unwrap_or(usize::MAX) {
        return false;
    }
    times.push_back(now);
    true
}

// Retira la solicitud más reciente de la dirección. Todas cuentan igual, así que da lo mismo cuál.
fn forget_at(requests: &mut Requests, ip: IpAddr) {
    if let Some(times) = requests.by_ip.get_mut(&ip) {
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

    fn requests(start: Instant) -> Requests {
        Requests {
            by_ip: HashMap::new(),
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
        forget_at(&mut log, ip("192.0.2.1"));
        assert!(allow_at(&mut log, ip("192.0.2.1"), start, 1, WINDOW));
        assert!(!allow_at(&mut log, ip("192.0.2.1"), start, 1, WINDOW));
        // Unknown addresses are ignored.
        forget_at(&mut log, ip("192.0.2.2"));
    }

    #[pagetop::test]
    async fn zero_maximum_never_limits() {
        let start = Instant::now();
        let mut log = requests(start);
        for _ in 0..10 {
            assert!(allow_at(&mut log, ip("192.0.2.1"), start, 0, WINDOW));
        }
        assert!(log.by_ip.is_empty());
    }

    #[pagetop::test]
    async fn addresses_without_recent_requests_are_purged() {
        let start = Instant::now();
        let mut log = requests(start);
        allow_at(&mut log, ip("192.0.2.1"), start, 5, WINDOW);
        allow_at(&mut log, ip("192.0.2.2"), start + WINDOW, 5, WINDOW);
        assert_eq!(log.by_ip.len(), 1);
        assert!(log.by_ip.contains_key(&ip("192.0.2.2")));
    }

    #[pagetop::test]
    async fn ipv6_addresses_count_by_their_64_network() {
        assert_eq!(network(ip("2001:db8:1:2:aaaa::1")), ip("2001:db8:1:2::"));
        assert_eq!(network(ip("192.0.2.1")), ip("192.0.2.1"));
    }
}
