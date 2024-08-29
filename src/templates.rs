use crate::constants::KICKBASE_API_ENDPOINT;
use crate::html::HtmlTemplate;
use askama::Template;
use axum::{
  extract::{Json, Request},
  http::HeaderValue,
  response::IntoResponse,
};
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

pub async fn get_login(request: Request) -> impl IntoResponse {
  debug!("request: {request:#?}");
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
  let email = payload.email;
  let password = payload.password;
  debug!("email: {email:#?}");
  debug!("password: {password:#?}");
  let template = PostLogin {
    message: format!("Entered email: {email}"),
  };
  (
    // [("HX-Location", HeaderValue::from_static("/dashboard"))],
    HtmlTemplate(template),
  )
}
