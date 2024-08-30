use crate::server::{
  html::HtmlTemplate, templates::login::PostLogin, KICKBASE_HTTP_CLIENT,
};
use askama_axum::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::collections::HashMap;

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

  KICKBASE_HTTP_CLIENT
    .post("/user/login", &map)
    .await
    .unwrap();

  let template = PostLogin {
    message: String::from("Logged in"),
  };

  HtmlTemplate(template)
}
