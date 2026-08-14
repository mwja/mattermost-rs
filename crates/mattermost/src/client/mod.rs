pub mod channel;
pub mod team;
pub mod user;

use std::sync::Arc;

use reqwest::{cookie::CookieStore, header::ToStrError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct MattermostClient {
    pub base_url: String,
    pub token: String,
    client: reqwest::Client,
    user_id: String,
    csrf_token: String,
}

/// The minimal, serializable slice of [`MattermostClient`] needed to reconstruct it
/// without logging in again.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub base_url: String,
    pub token: String,
    pub user_id: String,
    pub csrf_token: String,
}

#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("invalid base url provided")]
    InvalidUrl,
    #[error("unable to build client to interact with mattermost: {0}")]
    BuildError(reqwest::Error),
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("invalid credentials provided")]
    InvalidCredentials,
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("response error: {0}")]
    ResponseError(String),
    #[error("{0}")]
    BuildError(#[from] BuilderError),
    #[error("unable to pass response headers (for csrf token) to cookie jar: {0}")]
    CookieJarError(ToStrError),
}

impl MattermostClient {
    pub async fn new_with_login(
        base_url: String,
        username: String,
        password: String,
    ) -> Result<Self, LoginError> {
        // Use cookie jar to grab MMMCSRF token from set-cookie to store for future
        // requests
        let jar = Arc::new(reqwest::cookie::Jar::default());
        let client = reqwest::ClientBuilder::new()
            .cookie_provider(jar.clone())
            .build()
            .map_err(BuilderError::BuildError)?;

        let login_url = format!("{}/api/v4/users/login", base_url);
        let login_body = serde_json::json!({
            "login_id": username,
            "password": password,
            "deviceId": "",
            "token": "",
        });

        let response = client
            .post(&login_url)
            .header("Content-Type", "application/json")
            .header("Origin", base_url.clone())
            // https://forum.mattermost.com/t/cannot-get-heades-set-cookie/9108
            .header("X-Requested-With", "XMLHttpRequest")
            .json(&login_body)
            .send()
            .await?;

        if response.status().is_success() {
            let token = response
                .headers()
                .get("Token")
                .and_then(|t| t.to_str().ok())
                .map(|s| s.to_string());

            let body: user::User = response.json().await?;

            match token {
                Some(token) => {
                    let csrf_token: String = jar
                        .clone()
                        .cookies(&reqwest::Url::parse(base_url.as_str()).map_err(|_| {
                            LoginError::ResponseError("Invalid base URL".to_string())
                        })?)
                        .and_then(|cookies| {
                            cookies
                                .to_str()
                                .map_err(LoginError::CookieJarError)
                                .ok()?
                                .split(';')
                                .find_map(|cookie| {
                                    let cookie = cookie.trim();
                                    if cookie.starts_with("MMCSRF=") {
                                        Some(cookie.trim_start_matches("MMCSRF=").to_string())
                                    } else {
                                        None
                                    }
                                })
                        })
                        .ok_or_else(|| {
                            LoginError::ResponseError(
                                "MMCSRF token not found in cookies".to_string(),
                            )
                        })?;

                    Ok(MattermostClient::new_with_token(
                        base_url.as_str(),
                        &token,
                        &body.id,
                        &csrf_token,
                    )?)
                }
                None => Err(LoginError::ResponseError(
                    "Token not found in response headers".to_string(),
                )),
            }
        } else {
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                Err(LoginError::InvalidCredentials)
            } else {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(LoginError::ResponseError(error_text))
            }
        }
    }

    pub fn new_with_token(
        base_url: &str,
        token: &str,
        user_id: &str,
        csrf_token: &str,
    ) -> Result<Self, BuilderError> {
        use reqwest::cookie::Jar;
        let jar = Jar::default();

        // Add MMAUTHTOKEN, MMUSERID AND MMCSRF cookie to the jar
        let url = reqwest::Url::parse(base_url).map_err(|_| BuilderError::InvalidUrl)?;
        jar.add_cookie_str(&format!("MMAUTHTOKEN={}", token), &url);
        jar.add_cookie_str(&format!("MMUSERID={}", user_id), &url);
        jar.add_cookie_str(&format!("MMCSRF={}", csrf_token), &url);

        Ok(MattermostClient {
            base_url: base_url.to_string(),
            token: token.to_string(),
            client: reqwest::ClientBuilder::new()
                .cookie_provider(jar.into())
                .build()
                .map_err(BuilderError::BuildError)?,
            user_id: user_id.to_string(),
            csrf_token: csrf_token.to_string(),
        })
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn get_csrf_token(&self) -> &str {
        &self.csrf_token
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }
}

impl From<&MattermostClient> for Session {
    fn from(client: &MattermostClient) -> Self {
        Session {
            base_url: client.base_url.clone(),
            token: client.token.clone(),
            user_id: client.user_id.clone(),
            csrf_token: client.csrf_token.clone(),
        }
    }
}

impl TryFrom<Session> for MattermostClient {
    type Error = BuilderError;
    fn try_from(session: Session) -> Result<Self, Self::Error> {
        Self::new_with_token(
            &session.base_url,
            &session.token,
            &session.user_id,
            &session.csrf_token,
        )
    }
}
