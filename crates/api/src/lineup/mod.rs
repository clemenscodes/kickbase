pub mod formation;
pub mod get_lineup;
pub mod get_lineup_extended;

use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct SetLineupPayload {
  pub lineup_type: String,
  pub players: Vec<Option<String>>,
}

impl HttpClient {
  pub async fn ligainsider(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/sso/ligainsider?leagueId={}", league_id);
    let response = self.get(Method::POST, &url, None).await?;
    Ok(response)
  }

  pub async fn set_lineup(
    &self,
    league_id: &str,
    payload: SetLineupPayload,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("type", payload.lineup_type);
    map.insert(
      "players",
      serde_json::to_string(&payload.players).unwrap_or_default(),
    );

    let url = format!("/leagues/{}/lineup", league_id);
    let response = self.req(Method::POST, &url, Some(&map), None).await?;
    Ok(response)
  }
}
