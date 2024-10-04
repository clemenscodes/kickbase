use crate::html::HtmlTemplate;
use api::{http::user::User, HTTP};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::Request, http::HeaderMap};
use reqwest::header;

#[derive(Template)]
#[template(path = "pages/dashboard/get.html")]
pub struct Html {
  pub user: User,
}

pub async fn route(request: Request) -> impl IntoResponse {
  let cookie = request.headers().get("cookie").unwrap();
  let mut headers = HeaderMap::new();
  headers.insert(header::COOKIE, cookie.clone());
  let user = HTTP.read().await.get_user(Some(headers)).await;
  let template = Html { user };
  HtmlTemplate(template)
}
