use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

impl HttpClient {
  pub async fn add_favorite_player(
    &self,
    player_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("playerId", player_id.to_string());

    let url = "/competition/favorite/add".to_string();
    let response = self.req(Method::POST, &url, &map).await?;
    Ok(response)
  }

  pub async fn remove_favorite_player(
    &self,
    player_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("playerId", player_id.to_string());

    let url = "/competition/favorite/remove".to_string();
    let response = self.req(Method::POST, &url, &map).await?;
    Ok(response)
  }

  pub async fn get_top_25_players(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = "/competition/top25players".to_string();
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_matches(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = "/competition/matches".to_string();
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_table(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = "/competition/table".to_string();
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_team_players(
    &self,
    team_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/competition/teams/{}/players", team_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn search_players(
    &self,
    query: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/competition/search?query={}", query);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }
}
