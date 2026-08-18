use crate::MattermostClient;
use crate::client::channel::{Channel, ChannelCategory, ChannelLoadError, DefaultChannelClient};
use crate::client::post::{GetPostsOptions, Post, PostLoadError, PostsClient};
use crate::client::team::{Team, TeamClient, TeamLoadError};
use crate::client::user::{User, UserLoadError, UsersClient};

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

pub struct ChannelPosts(GetPostsOptions);

impl ChannelPosts {
    pub fn new(options: GetPostsOptions) -> Self {
        Self(options)
    }
}

impl Resource for ChannelPosts {
    type Value = Vec<Post>;
    type Error = PostLoadError;

    fn cache_key(&self) -> String {
        [
            self.0.channel_id.clone(),
            {
                match self.0.at.clone() {
                    crate::client::post::GetPostsAt::Cursor(cursor) => {
                        format!(
                            "{}_{}_{}_{}",
                            cursor.after.unwrap_or("".into()),
                            cursor.before.unwrap_or("".into()),
                            cursor.page,
                            cursor.per_page
                        )
                    }
                    crate::client::post::GetPostsAt::Since(ts) => ts.to_string(),
                }
            },
            self.0.include_deleted.unwrap_or(false).to_string(),
        ]
        .join("_")
    }

    async fn fetch(&self, client: &MattermostClient) -> Result<Vec<Post>, PostLoadError> {
        client.get_posts(self.0.clone()).await
    }
}

pub struct UserById(pub String);

impl Resource for UserById {
    type Value = User;
    type Error = UserLoadError;

    fn cache_key(&self) -> String {
        format!("user_{}", self.0)
    }

    async fn fetch(&self, client: &MattermostClient) -> Result<User, UserLoadError> {
        client.get_user(&self.0).await
    }
}
