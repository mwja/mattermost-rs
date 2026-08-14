//! Mattermost API client library for Rust.
//!
//! This crate is currently being developed in parallel with the
//! [desktop application](https://github.com/mwja/mattermost-rs) and is not yet
//! intended to be used by itself.
//!
//! That is not to say it can't be, but expect breaking changes until this
//! library and the desktop app are stabilised.
//!
//! For standardised access, caching etc, consider using the [store] module to
//! access resources, rather than using the [client] directly. These provided
//! standard cache keys, in-memory cache and disk cache.

pub mod client;
pub mod store;

pub use client::MattermostClient;
