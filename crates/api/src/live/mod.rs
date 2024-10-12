use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ChatMessagePayload {
  pub message: String,
}

impl HttpClient {
  pub async fn get_chat_history(
    &self,
    league_id: &str,
    start: u64,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/live/chat?start={}", league_id, start);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_overview(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/live", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_events_history(
    &self,
    league_id: &str,
    match_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/live/matches/{}", league_id, match_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_not_lined_up_players(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/live/notlinedupplayers", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_player_history(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/live/players/{}", league_id, player_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_top_10(&self) -> Result<HttpResponse, HttpClientError> {
    let url = "/live/top10".to_string();
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_live_team_players(
    &self,
    team_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/live/teams/{}/players", team_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_team_ranking(
    &self,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = "/live/teamranking".to_string();
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn send_chat_message(
    &self,
    league_id: &str,
    payload: ChatMessagePayload,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("m", payload.message);

    let url = format!("/leagues/{}/live/chat", league_id);
    let response = self.req(Method::POST, &url, Some(&map), None).await?;
    Ok(response)
  }
}
