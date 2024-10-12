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
}
