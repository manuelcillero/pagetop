//! Perform redirections in HTTP.
//!
//! **URL redirection**, also known as *URL forwarding*, is a technique to give more than one URL
//! address to a web resource. HTTP has a response called ***HTTP redirect*** for this operation
//! (see [Redirections in HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP/Redirections)).
//!
//! There are several types of redirects, sorted into three categories:
//!
//!   * **Permanent redirections**. These redirections are meant to last forever. They imply that
//!     the original URL should no longer be used, and replaced with the new one. Search engine
//!     robots, RSS readers, and other crawlers will update the original URL for the resource.
//!
//!   * **Temporary redirections**. Sometimes the requested resource can't be accessed from its
//!     canonical location, but it can be accessed from another place. In this case, a temporary
//!     redirect can be used. Search engine robots and other crawlers don't memorize the new,
//!     temporary URL. Temporary redirections are also used when creating, updating, or deleting
//!     resources, to show temporary progress pages.
//!
//!   * **Special redirections**.

use crate::service::HttpResponse;

pub struct Redirect;

impl Redirect {
    /// Permanent redirection. Status Code **301**. GET methods unchanged. Others may or may not be
    /// changed to GET. Typical for reorganization of a website.
    pub fn moved(redirect_to_url: &str) -> HttpResponse {
        HttpResponse::MovedPermanently()
            .append_header(("Location", redirect_to_url))
            .finish()
    }

    /// Permanent redirection. Status Code **308**. Method and body not changed. Typical for
    /// reorganization of a website, with non-GET links/operations.
    pub fn permanent(redirect_to_url: &str) -> HttpResponse {
        HttpResponse::PermanentRedirect()
            .append_header(("Location", redirect_to_url))
            .finish()
    }

    /// Temporary redirection. Status Code **302**. GET methods unchanged. Others may or may not be
    /// changed to GET. Used when the web page is temporarily unavailable for unforeseen reasons.
    pub fn found(redirect_to_url: &str) -> HttpResponse {
        HttpResponse::Found()
            .append_header(("Location", redirect_to_url))
            .finish()
    }

    /// Temporary redirection. Status Code **303**. GET methods unchanged. Others changed to GET
    /// (body lost). Used to redirect after a PUT or a POST, so that refreshing the result page
    /// doesn't re-trigger the operation.
    pub fn see_other(redirect_to_url: &str) -> HttpResponse {
        HttpResponse::SeeOther()
            .append_header(("Location", redirect_to_url))
            .finish()
    }

    /// Temporary redirection. Status Code **307**. Method and body not changed. The web page is
    /// temporarily unavailable for unforeseen reasons. Better than [`found()`](found) when non-GET
    /// operations are available on the site.
    pub fn temporary(redirect_to_url: &str) -> HttpResponse {
        HttpResponse::TemporaryRedirect()
            .append_header(("Location", redirect_to_url))
            .finish()
    }

    /// Special redirection. Status Code **304**. Redirects a page to the locally cached copy (that
    /// was stale). Sent for revalidated conditional requests. Indicates that the cached response is
    /// still fresh and can be used.
    pub fn not_modified(redirect_to_url: &str) -> HttpResponse {
        HttpResponse::NotModified()
            .append_header(("Location", redirect_to_url))
            .finish()
    }
}
