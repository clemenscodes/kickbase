use crate::{
  constants::KICKBASE_API_ENDPOINT,
  html::HtmlTemplate,
  http::HTTP_CLIENT,
  templates::{GetLogin, Home, PostLogin},
};
use askama_axum::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::collections::HashMap;

pub async fn home() -> impl IntoResponse {
  let template = Home {
    api: KICKBASE_API_ENDPOINT,
  };
  HtmlTemplate(template)
}

pub async fn get_login() -> impl IntoResponse {
  let template = GetLogin {};
  HtmlTemplate(template)
}

#[derive(Deserialize, Debug)]
pub struct PostLoginPayload {
  pub email: String,
  pub password: String,
}

pub async fn post_login(
  Json(payload): Json<PostLoginPayload>,
) -> impl IntoResponse {
  let mut map = HashMap::new();

  map.insert("email", payload.email);
  map.insert("password", payload.password);

  HTTP_CLIENT.post("/user/login", &map).await.unwrap();

  let template = PostLogin {
    message: String::from("Logged in"),
  };

  HtmlTemplate(template)
}
