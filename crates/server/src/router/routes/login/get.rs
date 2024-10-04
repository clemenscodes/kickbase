use crate::html::HtmlTemplate;
use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "pages/login/get.html")]
pub struct Html;

pub async fn route() -> impl IntoResponse {
  let template = Html {};
  HtmlTemplate(template)
}
