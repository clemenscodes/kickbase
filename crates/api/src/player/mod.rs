use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;

#[derive(Debug, Clone, Copy)]
pub struct Player {}

impl HttpClient {
  pub async fn get_player_info(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/players/{}", league_id, player_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_player_feed(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/players/{}/feed", league_id, player_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_player_points(
    &self,
    player_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/players/{}/points", player_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_player_stats(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/players/{}/stats", league_id, player_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn search_competition_players(
    &self,
    query: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/competition/search?query={}", query);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}
