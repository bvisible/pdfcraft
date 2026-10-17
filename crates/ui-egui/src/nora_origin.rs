// //// Neoffice — added file (no upstream equivalent): where the Nora pane of the browser build may send its calls.

//! Where the Nora pane's calls may go, and which of them may carry the page's credentials. The rules are common to
//! Neoffice's editors:
//! - `?nora` with no value (or `1`, `true`) is the page's own origin (`location.origin`);
//! - `?nora=<origin>` is taken only when it is the page's own origin (compared as the browser compares origins: case,
//!   a trailing slash and the scheme's default port make no difference), or when the page and that origin are both
//!   loopback (`127.0.0.1`, `localhost`, `[::1]`: the workshop's stand-in server); any other origin the address names
//!   is ignored, and the calls go to the page's own origin. Another origin can only come from
//!   `window.pdfcraftNora.origin`, which the hosting page sets;
//! - the page's credentials (a Frappe page's CSRF token, `window.pdfcraftNora.headers`) go only with a call to the
//!   page's own origin.
//!
//! So a link can't send what the person writes to Nora, what she reads of their document and the pictures of its
//! pages, with their session's token, to a server the link names. The rules hold no browser type, so they are tested
//! natively; the browser build (`apps/pdfcraft-web/src/nora.rs`) applies them.

/// The origin Nora's calls go to when the page's address has `nora=<param>` (`param` as `URLSearchParams` decodes it,
/// empty for a bare `?nora`), for a page whose own origin is `page` (`location.origin`). `Ok("")` is the page's own
/// origin when it has none a call could name (`null`, a `file:` page): the calls then go to relative addresses. `Err`
/// says why an origin the address names is not taken; the caller then uses the page's own origin.
pub fn from_param(param: &str, page: &str) -> Result<String, String> {
    let own = || parse(page).map(|origin| origin.to_string()).unwrap_or_default();
    let param = param.trim();
    if matches!(param, "" | "1" | "true") {
        return Ok(own());
    }
    match (parse(param), parse(page)) {
        (Some(named), Some(page)) if named == page => Ok(own()),
        (Some(named), Some(page)) if is_loopback(&named.host) && is_loopback(&page.host) => Ok(named.to_string()),
        _ => Err(format!(
            "the page's address names {param:?} for Nora: only the page's own origin, or a loopback one from a loopback page, \
             is taken from the address, so the calls go to the page's own origin; a hosting page names another one in \
             `window.pdfcraftNora.origin`"
        )),
    }
}

/// Whether a call to `target` (an origin, or `""` for the page's own) may carry the page's credentials: only a call to
/// the page's own origin.
pub fn is_page_origin(target: &str, page: &str) -> bool {
    if target.trim().is_empty() {
        return true;
    }
    match (parse(target), parse(page)) {
        (Some(target), Some(page)) => target == page,
        _ => false,
    }
}

/// An http(s) origin: scheme, host and port, compared as the browser compares origins.
#[derive(Debug, PartialEq, Eq)]
struct Origin {
    scheme: String,
    host: String,
    port: u16,
}

impl Origin {
    fn default_port(&self) -> u16 {
        if self.scheme == "https" { 443 } else { 80 }
    }
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.port == self.default_port() {
            write!(f, "{}://{}", self.scheme, self.host)
        } else {
            write!(f, "{}://{}:{}", self.scheme, self.host, self.port)
        }
    }
}

/// `scheme://host[:port]`, with at most one `/` after it and nothing else (no user, path, query or fragment); `None`
/// otherwise, `null` included.
fn parse(text: &str) -> Option<Origin> {
    let text = text.trim();
    let text = text.strip_suffix('/').unwrap_or(text);
    let (scheme, rest) = text.split_once("://")?;
    let scheme = scheme.to_ascii_lowercase();
    let default_port = match scheme.as_str() {
        "http" => 80,
        "https" => 443,
        _ => return None,
    };
    if rest.is_empty() || rest.contains(['/', '?', '#', '@', '\\', ' ']) {
        return None;
    }
    let (host, port) = match rest.strip_prefix('[') {
        // An IPv6 address, in brackets.
        Some(after) => {
            let (inner, tail) = after.split_once(']')?;
            let port = if tail.is_empty() { default_port } else { tail.strip_prefix(':')?.parse().ok()? };
            (format!("[{}]", inner.to_ascii_lowercase()), port)
        }
        None => match rest.rsplit_once(':') {
            Some((host, port)) => (host.to_ascii_lowercase(), port.parse().ok()?),
            None => (rest.to_ascii_lowercase(), default_port),
        },
    };
    if host.is_empty() || host == "[]" || (!host.starts_with('[') && host.contains(':')) {
        return None;
    }
    Some(Origin { scheme, host, port })
}

/// The loopback hosts the workshop's stand-in server runs on.
fn is_loopback(host: &str) -> bool {
    matches!(host, "127.0.0.1" | "localhost" | "[::1]")
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = "https://drive.example";
    const LOOPBACK_PAGE: &str = "http://127.0.0.1:8771";

    #[test]
    fn a_bare_nora_is_the_page_s_own_origin() {
        for param in ["", "1", "true", "  "] {
            assert_eq!(from_param(param, PAGE), Ok(PAGE.to_string()), "{param:?}");
        }
        assert_eq!(from_param("", "https://drive.example:443/"), Ok(PAGE.to_string()), "written as the browser writes it");
        assert_eq!(from_param("", "null"), Ok(String::new()), "a page without an origin of its own calls relative addresses");
    }

    #[test]
    fn the_page_s_own_origin_may_be_named_however_it_is_written() {
        for param in ["https://drive.example", "https://drive.example/", "HTTPS://Drive.Example", "https://drive.example:443"] {
            assert_eq!(from_param(param, PAGE), Ok(PAGE.to_string()), "{param:?}");
        }
        assert_eq!(from_param("http://127.0.0.1:8771/", LOOPBACK_PAGE), Ok(LOOPBACK_PAGE.to_string()));
    }

    #[test]
    fn any_other_origin_the_address_names_is_refused() {
        for param in [
            "https://elsewhere.example",
            "http://drive.example",
            "https://drive.example:8443",
            "https://drive.example.elsewhere.example",
            "https://drive.example@elsewhere.example",
            "https://elsewhere.example/https://drive.example",
            "https://elsewhere.example?https://drive.example",
            "//elsewhere.example",
            "javascript:alert(1)",
            "elsewhere.example",
            "ftp://drive.example",
            "http://127.0.0.1:8773",
            "https://[::1]",
        ] {
            let refused = from_param(param, PAGE);
            assert!(refused.as_ref().is_err_and(|why| why.contains("own origin")), "{param:?} was taken: {refused:?}");
        }
    }

    #[test]
    fn a_loopback_page_may_name_a_loopback_origin_for_the_workshop_s_mock() {
        for (param, taken) in [
            ("http://127.0.0.1:8773", "http://127.0.0.1:8773"),
            ("http://localhost:8773", "http://localhost:8773"),
            ("http://[::1]:8773", "http://[::1]:8773"),
            ("http://LOCALHOST:8773/", "http://localhost:8773"),
        ] {
            assert_eq!(from_param(param, LOOPBACK_PAGE), Ok(taken.to_string()), "{param:?}");
        }
        assert!(from_param("http://[::1]:8773", "http://localhost:8080").is_ok());
        for param in
            ["https://elsewhere.example", "http://127.0.0.1.elsewhere.example:8773", "http://127.0.0.2:8773", "http://localhost@elsewhere.example"]
        {
            assert!(from_param(param, LOOPBACK_PAGE).is_err(), "{param:?}");
        }
    }

    #[test]
    fn the_page_s_credentials_go_only_to_its_own_origin() {
        assert!(is_page_origin("", PAGE), "a relative call is the page's own");
        assert!(is_page_origin("https://drive.example", PAGE));
        assert!(is_page_origin("https://drive.example/", PAGE));
        assert!(is_page_origin("HTTPS://DRIVE.EXAMPLE:443", PAGE));
        assert!(!is_page_origin("https://elsewhere.example", PAGE));
        assert!(!is_page_origin("http://drive.example", PAGE), "another scheme is another origin");
        assert!(!is_page_origin("http://127.0.0.1:8773", LOOPBACK_PAGE), "another port is another origin, loopback or not");
        assert!(!is_page_origin("not an origin", PAGE));
        assert!(!is_page_origin("https://drive.example", "null"));
    }
}
