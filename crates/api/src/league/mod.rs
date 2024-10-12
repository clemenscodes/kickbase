use std::collections::HashMap;

use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;

#[derive(Debug)]
pub struct League {
  pub id: String,
  pub name: String,
  pub creator: String,
  pub creation: String,
  pub image: String,
}

impl HttpClient {
  pub async fn get_leagues(&self) -> Result<Vec<League>, HttpClientError> {
    let response = self.get(Method::GET, "/leagues", None).await.unwrap();

    let leagues: Vec<League> = response
      .value
      .get("leagues")
      .unwrap()
      .as_array()
      .unwrap()
      .iter()
      .map(|league| {
        let name = league.get("name").unwrap().to_string().replace("\"", "");
        let id = league.get("id").unwrap().to_string().replace("\"", "");
        let creator =
          league.get("creator").unwrap().to_string().replace("\"", "");
        let creation = league
          .get("creation")
          .unwrap()
          .to_string()
          .replace("\"", "");
        let image = league.get("ci").unwrap().to_string().replace("\"", "");
        League {
          id,
          name,
          creator,
          creation,
          image,
        }
      })
      .collect();

    Ok(leagues)
  }

  pub async fn get_feed_comments(
    &self,
    league_id: &str,
    feed_item_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/feed/{}/comments", league_id, feed_item_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_feed(
    &self,
    league_id: &str,
    start: u64,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/feed?start={}", league_id, start);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_info(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/info", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_players_for_match_day(
    &self,
    league_id: &str,
    user_id: &str,
    match_day: u64,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!(
      "/leagues/{}/users/{}/players?matchDay={}",
      league_id, user_id, match_day
    );
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_stats(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/stats", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_users(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/users", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_user_profile(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/users/{}", league_id, user_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_user_stats(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/users/{}/stats", league_id, user_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_user_players(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/users/{}/players", league_id, user_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_me(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/me", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_quickstats(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/quickstats", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_comment_feed(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/commentfeed", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn comment_league(
    &self,
    league_id: &str,
    comment: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("comment", comment.to_string());

    let url = format!("/leagues/{}/comment", league_id);
    let response = self.req(Method::POST, &url, Some(&map), None).await?;
    Ok(response)
  }
}
