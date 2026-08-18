use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::MattermostClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,

    #[serde(default)]
    pub create_at: i64,
    #[serde(default)]
    pub update_at: i64,
    #[serde(default)]
    pub delete_at: i64,

    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub nickname: String,

    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub email_verified: bool,

    #[serde(default)]
    pub auth_service: String,
    #[serde(default)]
    pub roles: String,
    #[serde(default)]
    pub locale: String,

    #[serde(default)]
    pub notify_props: UserNotifyProps,

    #[serde(default)]
    pub props: HashMap<String, Value>,

    #[serde(default)]
    pub last_password_update: i64,
    #[serde(default)]
    pub last_picture_update: i64,
    #[serde(default)]
    pub failed_attempts: i64,
    #[serde(default)]
    pub mfa_active: bool,

    #[serde(default)]
    pub timezone: Timezone,

    #[serde(default)]
    pub terms_of_service_id: Option<String>,

    #[serde(default)]
    pub terms_of_service_create_at: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotifyProps {
    pub auto_responder_active: String,
    pub auto_responder_message: String,
    pub calls_desktop_sound: String,
    pub calls_notification_sound: String,
    pub channel: String,
    pub comments: String,
    pub desktop: String,
    pub desktop_notification_sound: String,
    pub desktop_sound: String,
    pub desktop_threads: String,
    pub email: String,
    pub email_threads: String,
    pub first_name: String,
    pub highlight_keys: String,
    pub mention_keys: String,
    pub push: String,
    pub push_status: String,
    pub push_threads: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserNotifyProps {
    #[serde(flatten)]
    pub values: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Timezone {
    #[serde(default)]
    pub use_automatic_timezone: Option<bool>,

    #[serde(default)]
    pub manual_timezone: Option<String>,

    #[serde(default)]
    pub automatic_timezone: Option<String>,
}

#[derive(Debug, Error)]
pub enum UserLoadError {
    #[error("failed to load user: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("failed to parse user: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("failed to load user: {0}")]
    ResponseError(reqwest::StatusCode),
}

impl From<reqwest::StatusCode> for UserLoadError {
    fn from(status: reqwest::StatusCode) -> Self {
        UserLoadError::ResponseError(status)
    }
}

pub trait UsersClient {
    fn get_user(&self, user_id: &str) -> impl Future<Output = Result<User, UserLoadError>>;
}

impl UsersClient for MattermostClient {
    async fn get_user(&self, user_id: &str) -> Result<User, UserLoadError> {
        let url = format!("{}/api/v4/users/{}", self.base_url, user_id);
        let request = self.client.get(&url);
        let response = request.send().await?;

        if response.status().is_success() {
            let user: User = response.json::<_>().await?;
            Ok(user)
        } else {
            Err(response.status().into())
        }
    }
}
