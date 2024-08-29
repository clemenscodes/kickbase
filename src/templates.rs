use std::collections::HashMap;

use crate::constants::KICKBASE_API_ENDPOINT;
use crate::html::HtmlTemplate;
use askama::Template;
use axum::{extract::Json, response::IntoResponse};
use serde::Deserialize;
use tracing::debug;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct Home<'a> {
  pub api: &'a str,
}

pub async fn home() -> impl IntoResponse {
  let template = Home {
    api: KICKBASE_API_ENDPOINT,
  };
  HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/login/get.html")]
pub struct GetLogin;

pub async fn get_login() -> impl IntoResponse {
  let template = GetLogin {};
  HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/login/post.html")]
pub struct PostLogin {
  message: String,
}

#[derive(Deserialize, Debug)]
pub struct PostLoginPayload {
  email: String,
  password: String,
}

pub async fn post_login(
  Json(payload): Json<PostLoginPayload>,
) -> impl IntoResponse {
  let mut map = HashMap::new();

  map.insert("email", payload.email);
  map.insert("password", payload.password);

  let client = reqwest::Client::new();

  let res = client
    .post(format!("{KICKBASE_API_ENDPOINT}/user/login"))
    .json(&map)
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

  debug!("{res:#?}");

  let template = PostLogin {
    message: String::from("Logged in"),
  };

  HtmlTemplate(template)
}
