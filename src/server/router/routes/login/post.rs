use crate::server::{html::HtmlTemplate, http::HTTP};
use askama::Template;
use askama_axum::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "pages/login/post.html")]
pub struct Html {
  pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct Payload {
  pub email: String,
  pub password: String,
}

pub async fn route(Json(payload): Json<Payload>) -> impl IntoResponse {
  let mut map = HashMap::new();

  map.insert("email", payload.email);
  map.insert("password", payload.password);

  HTTP.post("/user/login", &map).await.unwrap();

  let template = Html {
    message: String::from("Logged in"),
  };

  HtmlTemplate(template)
}
