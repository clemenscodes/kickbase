use super::{league::League, HttpClient};
use crate::HTTP;
use axum::http::HeaderMap;
use reqwest::Method;

#[derive(Debug)]
pub struct User {
  pub name: String,
  pub id: String,
  pub image: String,
  pub leagues: Vec<League>,
}

impl HttpClient {
  pub async fn get_user(&self, headers: Option<HeaderMap>) -> User {
    let response = HTTP
      .read()
      .await
      .get(Method::GET, "/user/me", headers.clone())
      .await
      .unwrap();

    let user = response.value.get("user").unwrap();

    let leagues = self.get_leagues(headers).await;

    let user = User {
      id: user.get("id").unwrap().to_string().replace("\"", ""),
      name: user.get("name").unwrap().to_string().replace("\"", ""),
      image: user.get("profile").unwrap().to_string().replace("\"", ""),
      leagues,
    };

    user
  }
}
