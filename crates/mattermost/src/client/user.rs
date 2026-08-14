use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub create_at: i64,
    pub update_at: i64,
    pub delete_at: i64,

    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,

    pub email: String,
    pub email_verified: bool,

    pub auth_service: String,
    pub roles: String,
    pub position: String,
    pub locale: String,

    pub last_password_update: i64,
    pub last_picture_update: i64,

    pub disable_welcome_email: bool,

    pub notify_props: NotifyProps,
    pub props: UserProps,
    pub timezone: Timezone,
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

#[derive(Debug, Clone, Deserialize)]
pub struct UserProps {
    #[serde(rename = "customStatus")]
    pub custom_status: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timezone {
    pub automatic_timezone: String,
    pub manual_timezone: String,
    pub use_automatic_timezone: String,
}
