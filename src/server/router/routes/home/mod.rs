use crate::server::{
  constants::KICKBASE_API_ENDPOINT, html::HtmlTemplate, templates::home::Home,
};
use askama_axum::IntoResponse;

pub async fn home() -> impl IntoResponse {
  let template = Home {
    api: KICKBASE_API_ENDPOINT,
  };
  HtmlTemplate(template)
}
