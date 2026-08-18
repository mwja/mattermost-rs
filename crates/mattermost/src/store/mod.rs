//! This module provides lightweight caching mechanism for standardised
//! resources, with cache keys and loader functions that you can use for on-disk
//! caching.
//!
//! A [Store] caches a given [Resource] in memory, and on disk, then let's you
//! re-request the same resource again.
//!
//! This is being actively developed around the needs of the desktop app, so
//! expect this API to change between patch versions until 0.1.0.
mod resources;

pub use resources::{AllChannelCategories, AllChannels, AllTeams, ChannelPosts, UserById};

use std::{
    any::Any,
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Serialize, de::DeserializeOwned};

use crate::MattermostClient;

pub trait Resource: Send + 'static {
    type Value: Serialize + DeserializeOwned + Clone + Send + Sync + 'static;
    type Error: std::fmt::Display + Send + 'static;

    /// Memory-cache key and the on-disk relative path
    fn cache_key(&self) -> String;

    fn fetch(
        &self,
        client: &MattermostClient,
    ) -> impl Future<Output = Result<Self::Value, Self::Error>> + Send;
}

/// Caches [`Resource`] values in memory, then on disk. Network fallback is done
/// by the caller (see `AsyncAppContextStoreExt` in the desktop app)
pub struct Store {
    memory: HashMap<String, Box<dyn Any + Send + Sync>>,
    cache_dir: PathBuf,
}

impl Store {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            memory: HashMap::new(),
            cache_dir,
        }
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.cache_dir.clone()
    }

    pub fn memory_get<T: Clone + Send + Sync + 'static>(&self, key: &str) -> Option<T> {
        self.memory.get(key)?.downcast_ref::<T>().cloned()
    }

    pub fn memory_insert<T: Send + Sync + 'static>(&mut self, key: String, value: T) {
        self.memory.insert(key, Box::new(value));
    }

    pub fn read_disk<T: DeserializeOwned>(cache_dir: &Path, key: &str) -> Option<T> {
        let path = cache_dir.join(key).with_extension("mp");
        let bytes = std::fs::read(path).ok()?;
        rmp_serde::from_slice(&bytes).ok()
    }

    pub fn write_disk<T: Serialize>(cache_dir: &Path, key: &str, value: &T) {
        let path = cache_dir.join(key).with_extension("mp");
        if let Some(parent) = path.parent()
            && std::fs::create_dir_all(parent).is_err()
        {
            return;
        }
        if let Ok(bytes) = rmp_serde::to_vec(value) {
            let _ = std::fs::write(path, bytes);
        }
    }
}
