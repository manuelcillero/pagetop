//! Límite de solicitudes por dirección IP (`user.max_ip_requests`).

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

static REQUESTS: LazyLock<Mutex<Requests>> = LazyLock::new(|| {
    Mutex::new(Requests {
        by_ip: HashMap::new(),
        purged_at: Instant::now(),
    })
});

/// Anota una solicitud de la dirección del cliente y devuelve si se admite, según
/// [`max_ip_requests`](crate::config::UserSettings::max_ip_requests).
///
/// Una solicitud rechazada no se anota: la dirección vuelve a disponer de solicitudes en cuanto la
/// más antigua sale de la ventana. Las de dirección desconocida se admiten siempre.
pub(crate) fn allow(request: &HttpRequest) -> bool {
    let Some(ip) = request.client_ip() else {
        return true;
    };
    let window =
        StdDuration::from_secs(u64::try_from(SETTINGS.ip_request_window_secs).unwrap_or(0));
    let allowed = allow_at(
        &mut REQUESTS.lock().unwrap_or_else(PoisonError::into_inner),
        network(ip),
        Instant::now(),
        SETTINGS.max_ip_requests,
        window,
    );
    if !allowed {
        trace::warn!("Request limit reached for address {ip}");
    }
    allowed
}

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
