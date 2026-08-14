use thiserror::Error;

use crate::MattermostClient;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Team {
    pub allow_open_invite: bool,
    pub allowed_domains: String,
    pub cloud_limits_archived: bool,
    pub company_name: String,
    pub create_at: i64,
    pub delete_at: i64,
    pub description: String,
    pub display_name: String,
    pub email: String,
    pub group_constrained: bool,
    pub id: String,
    pub invite_id: String,
    pub last_team_icon_update: i64,
    pub name: String,
    pub policy_id: Option<String>,
    pub scheme_id: String,
    #[serde(rename = "type")]
    pub team_type: String,
    pub update_at: i64,
}

#[derive(Debug, Error)]
pub enum TeamLoadError {
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("failed to load teams: {0}")]
    LoadTeamsError(String),
    #[error("no default team found")]
    NoTeams,
    #[error("failed to load channels: {0}")]
    ResponseError(reqwest::StatusCode),
}

impl From<reqwest::StatusCode> for TeamLoadError {
    fn from(status: reqwest::StatusCode) -> Self {
        TeamLoadError::ResponseError(status)
    }
}

pub trait TeamClient {
    fn load_teams(&self) -> impl Future<Output = Result<Vec<Team>, TeamLoadError>>;
    fn current_team(&self) -> impl Future<Output = Option<Team>>;
}

impl TeamClient for MattermostClient {
    async fn load_teams(&self) -> Result<Vec<Team>, TeamLoadError> {
        let url = format!("{}/api/v4/users/me/teams", self.base_url);
        let request = self.client.get(&url);

        let response = request.send().await?;

        if response.status().is_success() {
            let teams: Vec<Team> = response.json::<_>().await?;
            Ok(teams)
        } else {
            Err(response.status().into())
        }
    }

    async fn current_team(&self) -> Option<Team> {
        let teams = self.load_teams().await.ok()?;
        teams.first().cloned()
    }
}
