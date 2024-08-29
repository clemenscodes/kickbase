use crate::constants::KICKBASE_API_ENDPOINT;
use crate::html::HtmlTemplate;
use askama::Template;
use axum::response::IntoResponse;

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
