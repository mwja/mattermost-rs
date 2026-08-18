use std::collections::HashMap;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::MattermostClient;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Post {
    pub id: String,
    #[serde(default)]
    pub create_at: u64,
    #[serde(default)]
    pub update_at: u64,
    #[serde(default)]
    pub delete_at: u64,
    #[serde(default)]
    pub edit_at: u64,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub channel_id: String,
    #[serde(default)]
    pub root_id: String,
    #[serde(default)]
    pub original_id: String,
    #[serde(default)]
    pub message: String,
    #[serde(rename = "type", default)]
    pub post_type: String,
    #[serde(default)]
    pub props: Value,
    #[serde(default)]
    pub hashtag: String,
    #[serde(default)]
    pub file_ids: Vec<String>,
    #[serde(default)]
    pub pending_post_id: String,
    #[serde(default)]
    pub metadata: PostMetadata,
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// Our PartialEq is reflexive
impl Eq for Post {}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct PostMetadata {}

#[derive(Debug, Default, Clone)]
pub struct GetPostsOptions {
    pub channel_id: String,
    pub at: GetPostsAt,
    pub include_deleted: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum GetPostsAt {
    Cursor(PostsCursor),
    /// The API makes a point that using `since` removes all ordering.
    Since(u64),
}

impl Default for GetPostsAt {
    fn default() -> Self {
        GetPostsAt::Cursor(PostsCursor::default())
    }
}

#[derive(Debug, Clone)]
pub struct PostsCursor {
    pub after: Option<String>,
    pub before: Option<String>,
    pub page: u64,
    pub per_page: u64,
}

impl Default for PostsCursor {
    fn default() -> Self {
        PostsCursor {
            after: None,
            before: None,
            page: 0,
            per_page: 0,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
struct PostsResponse {
    /// Mattermost returns posts keyed by id, ordered separately via `order`.
    #[serde(default)]
    posts: HashMap<String, Post>,
    #[serde(default)]
    order: Vec<String>,
}

impl PostsResponse {
    /// Reassembles the id-keyed `posts` map into the sequence given by `order`.
    fn into_ordered_posts(mut self) -> Vec<Post> {
        self.order
            .iter()
            .filter_map(|id| self.posts.remove(id))
            .collect()
    }
}

#[derive(Debug, Error)]
pub enum PostLoadError {
    #[error("failed to load posts: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("failed to parse posts: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("failed to load posts: {0}")]
    ResponseError(reqwest::StatusCode),
    #[error("failed to construct url safely: {0}")]
    UrlParserError(#[from] url::ParseError),
}

impl From<reqwest::StatusCode> for PostLoadError {
    fn from(status: reqwest::StatusCode) -> Self {
        PostLoadError::ResponseError(status)
    }
}

pub trait PostsClient {
    fn get_posts(
        &self,
        options: GetPostsOptions,
    ) -> impl Future<Output = Result<Vec<Post>, PostLoadError>>;
}

impl PostsClient for MattermostClient {
    async fn get_posts(&self, options: GetPostsOptions) -> Result<Vec<Post>, PostLoadError> {
        let url = format!(
            "{}/api/v4/channels/{}/posts",
            self.base_url, options.channel_id
        );

        let mut params = Vec::new();

        match options.at {
            GetPostsAt::Cursor(cursor) => {
                if let Some(after) = cursor.after {
                    params.push(("after", after));
                }
                if let Some(before) = cursor.before {
                    params.push(("before", before));
                }
                if cursor.page > 0 {
                    params.push(("page", cursor.page.to_string()));
                }
                if cursor.per_page > 0 {
                    params.push(("per_page", cursor.per_page.to_string()));
                }
            }
            GetPostsAt::Since(since) => {
                params.push(("since", since.to_string()));
            }
        }

        if let Some(include_deleted) = options.include_deleted {
            params.push(("include_deleted", include_deleted.to_string()));
        }

        let url = Url::parse_with_params(&url, params)?;

        let request = self.client.get(url);
        let response = request.send().await?;

        if response.status().is_success() {
            let posts_response: PostsResponse = response.json::<_>().await?;
            Ok(posts_response.into_ordered_posts())
        } else {
            Err(response.status().into())
        }
    }
}
