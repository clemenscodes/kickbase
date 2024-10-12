use crate::html::HtmlTemplate;
use api::{user::login::LoginPayload, KICKBASE};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{http::HeaderValue, response::Response, Json};

#[derive(Template)]
#[template(path = "pages/login/post.html")]
pub struct Html {
  pub message: String,
}

pub async fn route(Json(payload): Json<LoginPayload>) -> Response {
  let response = KICKBASE.read().await.login(payload).await.unwrap();

  let status = response.status;

  if !status.is_success() {
    let template = Html {
      message: String::from("Invalid credentials"),
    };

    return HtmlTemplate(template).into_response();
  }

  let template = Html {
    message: String::from("Logged in"),
  };

  let mut html = HtmlTemplate(template).into_response();

  html
    .headers_mut()
    .insert("HX-Redirect", HeaderValue::from_str("/dashboard").unwrap());

  html
}
