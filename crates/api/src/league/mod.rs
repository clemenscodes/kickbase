pub mod comment_league;
pub mod get_feed;
pub mod get_feed_comments;
pub mod get_info;
pub mod get_leagues;
pub mod get_players_for_match_day;
pub mod get_stats;
pub mod get_user_profile;
pub mod get_users;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{HttpClient, HttpClientError};
use crate::HttpResponse;

#[derive(Debug, Default, Serialize, Deserialize)]
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
  pub async fn get_user_stats(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/users/{user_id}/stats");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_user_players(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/users/{user_id}/players");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_me(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/me");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_quickstats(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/quickstats");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn get_comment_feed(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/commentfeed");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
