use super::{league::League, HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct User {
  pub name: String,
  pub id: String,
  pub image: String,
  pub leagues: Vec<League>,
}

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
  pub email: String,
  pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ResetPasswordPayload {
  pub password: String,
  pub token: String,
}

impl HttpClient {
  pub async fn login(
    &self,
    payload: LoginPayload,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();

    map.insert("email", payload.email);
    map.insert("password", payload.password);

    let response = self
      .req(Method::POST, "/user/login", Some(&map), None)
      .await?;

    Ok(response)
  }

  pub async fn reset_password(
    &self,
    payload: ResetPasswordPayload,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();

    map.insert("password", payload.password);
    map.insert("token", payload.token);

    let response = self
      .req(Method::POST, "/user/resetpassword", Some(&map), None)
      .await?;

    Ok(response)
  }

  pub async fn get_user(&self) -> Result<User, HttpClientError> {
    let response = self.get(Method::GET, "/user/me", None).await?;
    let user = response.value.get("user").unwrap();
    let leagues = self.get_leagues().await?;
    let user = User {
      id: user.get("id").unwrap().to_string().replace("\"", ""),
      name: user.get("name").unwrap().to_string().replace("\"", ""),
      image: user.get("profile").unwrap().to_string().replace("\"", ""),
      leagues,
    };

    Ok(user)
  }

  pub async fn league_user_info(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/me", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn get_user_match_day_feed(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/users/{}/feed", league_id, user_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn refresh_chat_token(
    &self,
  ) -> Result<HttpResponse, HttpClientError> {
    let response = self
      .get(Method::POST, "/user/refreshchattoken", None)
      .await?;
    Ok(response)
  }
}
