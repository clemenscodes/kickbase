pub mod get_leagues;

use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use super::{HttpClient, HttpClientError};
use crate::HttpResponse;

#[derive(Debug, Default)]
pub struct League {
  pub id: String,
  pub name: String,
  pub creator: String,
  pub creation: String,
  pub image: String,
}

impl From<&Value> for League {
  fn from(value: &Value) -> Self {
    let value = value.get("league").unwrap_or(value);

    let name = value
      .get("name")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let id = value
      .get("id")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let creator = value
      .get("creator")
      .unwrap_or(value.get("creatorId").unwrap())
      .as_str()
      .unwrap_or_default()
      .to_string();

    let creation = value
      .get("creation")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let image = value
      .get("ci")
      .map(|v| v.as_str().unwrap_or_default().to_string())
      .unwrap_or_default();

    League {
      id,
      name,
      creator,
      creation,
      image,
    }
  }
}

impl HttpClient {
  pub async fn get_feed_comments(
    &self,
    league_id: &str,
    feed_item_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/feed/{}/comments", league_id, feed_item_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_feed(
    &self,
    league_id: &str,
    start: u64,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/feed?start={}", league_id, start);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_info(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/info", league_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_players_for_match_day(
    &self,
    league_id: &str,
    user_id: &str,
    match_day: u64,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!(
      "/leagues/{}/users/{}/players?matchDay={}",
      league_id, user_id, match_day
    );
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_stats(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/stats", league_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_users(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/users", league_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_user_profile(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/users/{}", league_id, user_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_user_stats(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/users/{}/stats", league_id, user_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_user_players(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/users/{}/players", league_id, user_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_me(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/me", league_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_quickstats(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/quickstats", league_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn get_comment_feed(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/commentfeed", league_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }

  pub async fn comment_league(
    &self,
    league_id: &str,
    comment: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("comment", comment.to_string());

    let url = format!("/leagues/{}/comment", league_id);
    let response = self.req(Method::POST, &url, &map).await?;
    Ok(response)
  }
}
