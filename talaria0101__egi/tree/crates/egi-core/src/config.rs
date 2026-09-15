//! Configuration: what the operator wrote, checked before anything runs.
//!
//! The daemon refuses to start on a config it cannot enforce. Every field
//! below is the operator's statement of intent; the sandbox backend reports
//! what it can hold, and a gap stops startup unless the operator accepted it
//! in advance.

/// Which backend confines sessions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SandboxBackend {
    /// Confine sessions as host processes with Landlock, seccomp, namespaces.
    #[default]
    Bailey,
    /// Run each session in a rootless container.
    Podman,
}

/// How much network a session gets. `None` disables it entirely.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NetworkMode {
    /// Provider reachable, host unreachable, broker allowlist enforced.
    #[default]
    Restricted,
    /// No network at all, whatever else is named.
    None,
}

/// How outbound is bounded for a session that has a network.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EgressMode {
    /// Force every connection through a broker the daemon runs outside the
    /// sandbox. The broker permits only allowlisted hosts and injects the
    /// provider credential itself, so the key never enters the sandbox.
    #[default]
    Proxy,
    /// Port-only rule: a session may reach any host on an allowed port.
    /// It cannot tell the model provider from anywhere else on 443.
    Open,
}

/// What a session may reach outbound, and how that is enforced.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EgressConfig {
    /// Whether egress is brokered (`Proxy`) or port-only (`Open`).
    pub mode: EgressMode,
    /// Hosts the broker permits under `Proxy` mode, on top of the provider.
    /// A leading `*.` matches subdomains. A lone `*` admits any host,
    /// keeping the broker as an audit pass-through that still gates the
    /// port and logs every connection but restricts no host.
    pub allow: Vec<String>,
}

/// What a session may consume, and what the backend enforces.
#[derive(Clone, Debug, PartialEq)]
pub struct SandboxConfig {
    /// Which backend confines sessions.
    pub backend: SandboxBackend,
    /// Refuse to start when the backend cannot enforce every configured
    /// guarantee on this host, rather than reporting the gap and continuing.
    pub require_full_enforcement: bool,
    /// Network exposure granted to a session.
    pub network: NetworkMode,
    /// Ports a session may open outbound, when the network is not `None`.
    /// The default is HTTPS alone, which is all a model provider needs.
    pub egress_ports: Vec<u16>,
    /// What a session may reach outbound, and whether it is brokered.
    pub egress: EgressConfig,
    /// Container image the podman backend runs. Inert under bailey.
    pub image: String,
    /// Memory ceiling per session, such as `4g`.
    pub memory: String,
    /// CPU ceiling per session, in cores.
    pub cpus: f64,
    /// Process count ceiling per session.
    pub pids: u32,
    /// Largest single file a session may write, such as `1g`.
    pub file_max: String,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            backend: SandboxBackend::Bailey,
            require_full_enforcement: true,
            network: NetworkMode::Restricted,
            egress_ports: vec![443],
            egress: EgressConfig::default(),
            image: String::new(),
            memory: "4g".to_owned(),
            cpus: 2.0,
            pids: 512,
            file_max: "1g".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_hold_the_tightest_shape() {
        let config = SandboxConfig::default();
        assert_eq!(config.backend, SandboxBackend::Bailey);
        assert!(config.require_full_enforcement);
        assert_eq!(config.network, NetworkMode::Restricted);
        assert_eq!(config.egress.mode, EgressMode::Proxy);
        assert_eq!(config.egress_ports, vec![443]);
    }
}
