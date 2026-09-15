//! Provider contracts: how the daemon reaches a model without handing the
//! agent a credential.
//!
//! The agent never holds a real credential. The daemon brokers every model
//! call at the boundary: the session offers a nonce worth only what the
//! broker will do with it, and the broker puts the real credential on at
//! this end.

/// Where a model is reached, and what pays for it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Endpoint {
    /// Provider base URL, without a trailing slash.
    pub base_url: String,
    /// Model id the provider routes on.
    pub model: String,
}

/// Failure talking to a model, in words worth showing in a thread.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderError(pub String);

impl core::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ProviderError {}
