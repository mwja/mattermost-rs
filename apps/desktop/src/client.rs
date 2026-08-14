use anyhow::{Result, anyhow};
use gpui::{App, AsyncApp, Context, Global};
use mattermost::MattermostClient;

/// Wrapper around [MattermostClient] to make it a global resource in the
/// application context.
pub(crate) struct SharedClient {
    client: Option<MattermostClient>,
}

impl Global for SharedClient {}

pub trait AppContextClientExt {
    fn get_client(&self) -> Option<&MattermostClient>;
    fn set_client(&mut self, client: Option<MattermostClient>);
}
pub trait AsyncAppContextClientExt {
    fn get_client(&self) -> Result<MattermostClient>;
    fn set_client(&self, client: Option<MattermostClient>) -> Result<()>;
}

impl<'a, T> AppContextClientExt for Context<'a, T> {
    fn get_client(&self) -> Option<&MattermostClient> {
        self.global::<SharedClient>().client.as_ref()
    }

    fn set_client(&mut self, client: Option<MattermostClient>) {
        self.set_global(SharedClient { client });
    }
}

impl AppContextClientExt for App {
    fn get_client(&self) -> Option<&MattermostClient> {
        self.global::<SharedClient>().client.as_ref()
    }

    fn set_client(&mut self, client: Option<MattermostClient>) {
        self.set_global(SharedClient { client });
    }
}

impl AsyncAppContextClientExt for AsyncApp {
    fn get_client(&self) -> Result<MattermostClient> {
        let shared_client =
            self.read_global::<SharedClient, _>(|shared, _app| shared.client.clone());
        shared_client.ok_or(anyhow!("No client found!"))
    }

    fn set_client(&self, client: Option<MattermostClient>) -> Result<()> {
        self.update_global::<SharedClient, _>(|_g, _app| SharedClient { client });

        Ok(())
    }
}
