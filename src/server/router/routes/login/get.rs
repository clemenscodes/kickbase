use crate::server::{html::HtmlTemplate, templates::login::GetLogin};
use askama_axum::IntoResponse;

pub async fn get_login() -> impl IntoResponse {
  let template = GetLogin {};
  HtmlTemplate(template)
}
