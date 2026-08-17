use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Post {
    id: String,
    create_at: u64,
    update_at: u64,
    delete_at: u64,
    edit_at: u64,
    user_id: String,
    channel_id: String,
    root_id: String,
    original_id: String,
    message: String,
    #[serde(rename = "type")]
    post_type: String,
    props: Value,
    hashtag: String,
    file_ids: String,
    pending_post_id: String,
    metadata: PostMetadata,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PostMetadata {
    embeds:
}

#[derive(Debug, Default)]
pub struct GetPostsOptions {
    channel_id: String,
    at: GetPostsAt,
    include_deleted: Option<bool>,
}

#[derive(Debug)]
pub enum GetPostsAt {
    Cursor(PostsCursor),
    Since(u64),
}

impl Default for GetPostsAt {
    fn default() -> Self {
        GetPostsAt::Cursor(PostsCursor::default())
    }
}

#[derive(Debug)]
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

pub trait PostsClient {
    fn get_posts(&self, options: GetPostsOptions) -> impl Future<Output = Result<Vec<Post>>>;
}
