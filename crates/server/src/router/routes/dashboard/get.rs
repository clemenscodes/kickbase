use crate::html::HtmlTemplate;
use api::{user::get_user::User, KICKBASE};
use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "pages/dashboard/get.html")]
pub struct Html {
  pub user: User,
}

pub async fn route() -> impl IntoResponse {
  let user = KICKBASE.read().await.get_user().await.unwrap();
  let template = Html { user };
  HtmlTemplate(template)
}
