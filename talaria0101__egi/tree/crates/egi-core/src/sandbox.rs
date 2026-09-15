//! The narrow contract a sandbox backend implements.
//!
//! Sized to exactly what a backend does: start a confined agent and hand back
//! its pipes, say what it can enforce on this host, tear a session down, and
//! find sandboxes a previous run left behind. It is not a plugin system.

/// Where a session's project appears to the agent, under every backend.
///
/// The agent is never shown a host path. A path carries the operator's name
/// and the shape of their machine.
pub const WORKSPACE_PATH: &str = "/workspace";

/// Where a session's own state directory appears to the agent.
pub const STATE_PATH: &str = "/state";

/// The agent's home, inside the session state it is allowed to write.
pub const AGENT_HOME: &str = "/state/home";

/// Label marking every sandbox this system owns, for discovery and cleanup.
pub const SYSTEM_LABEL: &str = "egi.system";

/// Label carrying the session a sandbox belongs to.
pub const SESSION_LABEL: &str = "egi.session";

/// Prefix for the name given to a session's sandbox.
pub const SANDBOX_NAME_PREFIX: &str = "egi-";

/// The sandbox name for a session, used for discovery and teardown.
#[must_use]
pub fn sandbox_name(session_id: &str) -> String {
    format!("{SANDBOX_NAME_PREFIX}{session_id}")
}

/// What a backend can and cannot enforce on this host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CapabilityReport {
    /// Guarantees the backend cannot enforce here. Reported at startup, and
    /// fatal when the configuration requires full enforcement.
    pub gaps: Vec<String>,
    /// Facts worth stating that are not gaps.
    pub notes: Vec<String>,
}

/// Whether `host` is permitted by an allowlist of names and `*.` wildcards.
///
/// A lone `*` admits any host. A bare name matches only itself. A
/// `*.example.com` rule matches any subdomain of `example.com` but not the
/// bare `example.com`. The comparison is case-folded, because a hostname is.
#[must_use]
pub fn host_allowed(host: &str, allow: &[String]) -> bool {
    let host = host.trim().to_lowercase();
    if host.is_empty() {
        return false;
    }
    for rule in allow {
        if rule == "*" {
            return true;
        }
        if let Some(suffix) = rule.strip_prefix("*.") {
            let suffix = format!(".{suffix}").to_lowercase();
            if host.len() > suffix.len() && host.ends_with(suffix.as_str()) {
                return true;
            }
        } else if host == rule.to_lowercase() {
            return true;
        }
    }
    false
}

/// The host and port a `CONNECT` line asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectTarget {
    /// Host the session asked to reach.
    pub host: String,
    /// Port the session asked to open.
    pub port: u16,
}

/// Reads the target out of an HTTP `CONNECT` request line.
///
/// The line is `CONNECT host:port HTTP/1.1`. Anything else, a missing port,
/// or a port out of range is refused by returning `None` rather than
/// guessing, since a target the broker had to guess at is one it cannot
/// claim to have checked.
#[must_use]
pub fn parse_connect(request_line: &str) -> Option<ConnectTarget> {
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?;
    let authority = parts.next()?;
    if !method.eq_ignore_ascii_case("CONNECT") {
        return None;
    }
    let colon = authority.rfind(':')?;
    if colon == 0 || colon == authority.len() - 1 {
        return None;
    }
    let host = &authority[..colon];
    if host.contains('/') || host.contains('[') {
        return None;
    }
    let port: u16 = authority[colon + 1..].parse().ok()?;
    if port == 0 {
        return None;
    }
    Some(ConnectTarget {
        host: host.to_owned(),
        port,
    })
}

/// Ports the broker will open upstream.
pub const ALLOWED_UPSTREAM_PORTS: &[u16] = &[443];

#[cfg(test)]
mod tests {
    use super::*;

    fn allow(rules: &[&str]) -> Vec<String> {
        rules.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn bare_name_matches_only_itself() {
        assert!(host_allowed("github.com", &allow(&["github.com"])));
        assert!(!host_allowed("api.github.com", &allow(&["github.com"])));
        assert!(!host_allowed(
            "github.com.evil.com",
            &allow(&["github.com"])
        ));
    }

    #[test]
    fn wildcard_matches_subdomains_but_not_the_apex() {
        let rules = allow(&["*.githubusercontent.com"]);
        assert!(host_allowed("raw.githubusercontent.com", &rules));
        assert!(host_allowed("a.b.githubusercontent.com", &rules));
        assert!(!host_allowed("githubusercontent.com", &rules));
    }

    #[test]
    fn star_admits_any_host_and_empty_host_refuses() {
        assert!(host_allowed("anything.example", &allow(&["*"])));
        assert!(!host_allowed("", &allow(&["*"])));
        assert!(!host_allowed("", &allow(&["github.com"])));
    }

    #[test]
    fn matching_is_case_folded() {
        assert!(host_allowed("GitHub.COM", &allow(&["github.com"])));
    }

    #[test]
    fn parse_connect_reads_host_and_port() {
        let target = parse_connect("CONNECT github.com:443 HTTP/1.1").unwrap();
        assert_eq!(target.host, "github.com");
        assert_eq!(target.port, 443);
    }

    #[test]
    fn parse_connect_refuses_anything_else() {
        assert_eq!(parse_connect("GET https://github.com/ HTTP/1.1"), None);
        assert_eq!(parse_connect("CONNECT github.com HTTP/1.1"), None);
        assert_eq!(parse_connect("CONNECT github.com:0 HTTP/1.1"), None);
        assert_eq!(parse_connect("CONNECT github.com:99999 HTTP/1.1"), None);
        assert_eq!(parse_connect("CONNECT :443 HTTP/1.1"), None);
        assert_eq!(parse_connect("CONNECT [::1]:443 HTTP/1.1"), None);
    }

    #[test]
    fn sandbox_name_carries_the_prefix() {
        assert_eq!(sandbox_name("abc"), "egi-abc");
    }
}
