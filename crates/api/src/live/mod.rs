use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessagePayload {
  pub m: String,
}

impl HttpClient {
  pub async fn get_chat_history(
    &self,
    league_id: &str,
    start: u64,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/live/chat?start={start}");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_overview(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/live");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_events_history(
    &self,
    league_id: &str,
    match_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/live/matches/{match_id}");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_not_lined_up_players(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/live/notlinedupplayers");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_player_history(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/live/players/{player_id}");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_top_10(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = "/live/top10".to_string();
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_live_team_players(
    &self,
    team_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/live/teams/{team_id}/players");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_team_ranking(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self.get("/live/teamranking").await?;
    Ok(response)
  }

  pub async fn send_chat_message(
    &self,
    league_id: &str,
    payload: ChatMessagePayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/live/chat");
    let response = self.req(Method::POST, &url, &payload).await?;
    Ok(response)
  }
}
