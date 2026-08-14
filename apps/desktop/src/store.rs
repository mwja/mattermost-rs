use std::path::PathBuf;

use gpui::{App, AsyncApp, Global, Task};
use mattermost::store::{Resource, Store};

use crate::client::AsyncAppContextClientExt;

struct SharedStore(Store);

impl Global for SharedStore {}

pub fn init(cx: &mut App, cache_dir: PathBuf) {
    cx.set_global(SharedStore(Store::new(cache_dir)));
}

pub trait AsyncAppContextStoreExt {
    /// Loads a [`Resource`], trying memory, then disk, then the network.
    fn load<R: Resource>(&self, resource: R) -> Task<Result<R::Value, R::Error>>;
}

impl AsyncAppContextStoreExt for AsyncApp {
    fn load<R: Resource>(&self, resource: R) -> Task<Result<R::Value, R::Error>> {
        let key = resource.cache_key();

        // 1. Memory — synchronous, no thread hop at all.
        if let Some(hit) =
            self.read_global::<SharedStore, _>(|store, _| store.0.memory_get::<R::Value>(&key))
        {
            return Task::ready(Ok(hit));
        }

        let cache_dir = self.read_global::<SharedStore, _>(|store, _| store.0.cache_dir());

        self.spawn(async move |cx| {
            let (disk_key, disk_dir) = (key.clone(), cache_dir.clone());
            let disk_hit = cx
                .background_executor()
                .spawn(async move { Store::read_disk::<R::Value>(&disk_dir, &disk_key) })
                .await;

            if let Some(value) = disk_hit {
                cx.update_global::<SharedStore, _>(|store, _| {
                    store.0.memory_insert(key.clone(), value.clone())
                });
                return Ok(value);
            }

            let client = cx.get_client().expect("no client available");
            let value = gpui_tokio_bridge::spawn(cx, async move { resource.fetch(&client).await })
                .await
                .expect("tokio task panicked")?;

            let (write_key, write_dir, write_value) =
                (key.clone(), cache_dir.clone(), value.clone());
            cx.background_executor()
                .spawn(async move { Store::write_disk(&write_dir, &write_key, &write_value) })
                .await;

            cx.update_global::<SharedStore, _>(|store, _| {
                store.0.memory_insert(key, value.clone())
            });

            Ok(value)
        })
    }
}
