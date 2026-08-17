use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::{MattermostClient, client::team::TeamClient};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChannelType {
    #[serde(rename = "D")]
    Direct,
    #[serde(rename = "O")]
    Open,
    #[serde(rename = "P")]
    Private,
    #[serde(rename = "G")]
    Group,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Channel {
    pub id: String,
    pub create_at: i64,
    pub update_at: i64,
    pub delete_at: i64,
    pub team_id: String,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub display_name: String,
    pub name: String,
    pub header: String,
    pub purpose: String,
    pub last_post_at: i64,
    pub total_msg_count: i64,
    pub extra_update_at: i64,
    pub creator_id: String,
    pub scheme_id: Value,
    pub props: Value,
    pub group_constrained: Value,
    pub autotranslation: bool,
    pub shared: Option<bool>,
    pub total_msg_count_root: i64,
    pub policy_id: Value,
    pub last_root_post_at: i64,
    pub banner_info: Value,
    pub policy_enforced: bool,
    pub policy_is_active: bool,
    pub default_category_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChannelCategoryType {
    #[serde(rename = "direct_messages")]
    DirectMessages,
    #[serde(rename = "favorites")]
    Favorites,
    #[default]
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "channels")]
    Channels,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelCategory {
    pub id: String,
    pub user_id: String,
    pub team_id: String,
    pub sort_order: i64,
    pub sorting: String,
    #[serde(rename = "type")]
    pub category_type: ChannelCategoryType,
    pub display_name: String,
    pub muted: bool,
    pub collapsed: bool,
    pub channel_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChannelCategoryResult {
    pub categories: Vec<ChannelCategory>,
    /// We can kind of ignore this as sort_order is present on the [ChannelCategory].
    pub order: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ChannelLoadError {
    #[error("failed to load channels: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("failed to parse channels: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("failed to load channels: {0}")]
    ResponseError(reqwest::StatusCode),
}

pub trait ChannelClient {
    fn load_channels_for_team(
        &self,
        team_id: &str,
    ) -> impl Future<Output = Result<Vec<Channel>, ChannelLoadError>>;

    fn load_channel_categories_for_team(
        &self,
        team_id: &str,
    ) -> impl Future<Output = Result<Vec<ChannelCategory>, ChannelLoadError>>;
}

pub trait DefaultChannelClient: ChannelClient + TeamClient {
    #[expect(async_fn_in_trait)]
    async fn load_channels(&self) -> Result<Vec<Channel>, ChannelLoadError> {
        let team_id = self
            .current_team()
            .await
            .ok_or(ChannelLoadError::ResponseError(
                reqwest::StatusCode::NOT_FOUND,
            ))?
            .id;
        self.load_channels_for_team(&team_id).await
    }

    #[expect(async_fn_in_trait)]
    async fn load_visible_channels(&self) -> Result<Vec<Channel>, ChannelLoadError> {
        let team_id = self
            .current_team()
            .await
            .ok_or(ChannelLoadError::ResponseError(
                reqwest::StatusCode::NOT_FOUND,
            ))?
            .id;
        let channels = self.load_channels().await?;
        let visible_channels = channels
            .into_iter()
            // invisible channels get marked with team_id == ""
            .filter(|c| c.team_id == team_id)
            .collect();
        Ok(visible_channels)
    }

    #[expect(async_fn_in_trait)]
    async fn load_channel_categories(&self) -> Result<Vec<ChannelCategory>, ChannelLoadError> {
        let team_id = self
            .current_team()
            .await
            .ok_or(ChannelLoadError::ResponseError(
                reqwest::StatusCode::NOT_FOUND,
            ))?
            .id;
        self.load_channel_categories_for_team(&team_id).await
    }
}

impl<T> DefaultChannelClient for T where T: ChannelClient + TeamClient {}

impl From<reqwest::StatusCode> for ChannelLoadError {
    fn from(status: reqwest::StatusCode) -> Self {
        ChannelLoadError::ResponseError(status)
    }
}

impl ChannelClient for MattermostClient {
    async fn load_channels_for_team(
        &self,
        team_id: &str,
    ) -> Result<Vec<Channel>, ChannelLoadError> {
        let url = format!(
            "{}/api/v4/users/me/teams/{}/channels?include_deleted=false",
            self.base_url, team_id
        );
        let request = self.client.get(&url);

        let response = request.send().await?;

        if response.status().is_success() {
            let channels: Vec<Channel> = response.json::<_>().await?;
            Ok(channels)
        } else {
            Err(response.status().into())
        }
    }

    async fn load_channel_categories_for_team(
        &self,
        team_id: &str,
    ) -> Result<Vec<ChannelCategory>, ChannelLoadError> {
        let url = format!(
            "{}/api/v4/users/me/teams/{}/channels/categories",
            self.base_url, team_id
        );
        let request = self.client.get(&url);

        let response = request.send().await?;

        if response.status().is_success() {
            let categories: ChannelCategoryResult = response.json::<_>().await?;
            Ok(categories.categories)
        } else {
            Err(response.status().into())
        }
    }
}
