use askama::Template;
use askama_axum::IntoResponse;

use crate::server::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct Html {}

pub async fn route() -> impl IntoResponse {
  let template = Html {};
  HtmlTemplate(template)
}
