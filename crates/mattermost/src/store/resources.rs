use crate::MattermostClient;
use crate::client::channel::{Channel, ChannelCategory, ChannelLoadError, DefaultChannelClient};
use crate::client::team::{Team, TeamClient, TeamLoadError};

use super::Resource;

pub struct AllChannels;

impl Resource for AllChannels {
    type Value = Vec<Channel>;
    type Error = ChannelLoadError;

    fn cache_key(&self) -> String {
        "channels".to_string()
    }

    async fn fetch(&self, client: &MattermostClient) -> Result<Vec<Channel>, ChannelLoadError> {
        client.load_channels().await
    }
}

pub struct AllChannelCategories;

impl Resource for AllChannelCategories {
    type Value = Vec<ChannelCategory>;
    type Error = ChannelLoadError;

    fn cache_key(&self) -> String {
        "channel_categories".to_string()
    }

    async fn fetch(
        &self,
        client: &MattermostClient,
    ) -> Result<Vec<ChannelCategory>, ChannelLoadError> {
        client.load_channel_categories().await
    }
}

pub struct AllTeams;

impl Resource for AllTeams {
    type Value = Vec<Team>;
    type Error = TeamLoadError;

    fn cache_key(&self) -> String {
        "teams".to_string()
    }

    async fn fetch(&self, client: &MattermostClient) -> Result<Vec<Team>, TeamLoadError> {
        client.load_teams().await
    }
}
