//! Core types for egi: config, sandbox contracts, provider contracts.
//!
//! This crate holds the types every other crate builds on. It performs no IO
//! and owns no runtime: backends, brokers, and chat adapters depend on it,
//! never the reverse.

pub mod config;
pub mod provider;
pub mod sandbox;
