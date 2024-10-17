pub mod get_user;
pub mod get_user_match_day_feed;
pub mod league_user_info;
pub mod login;
pub mod refresh_chat_token;
pub mod reset_password;

use serde_json::Value;

use crate::league::League;

#[derive(Debug)]
pub struct User {
  pub name: String,
  pub id: String,
  pub image: String,
  pub leagues: Vec<League>,
}

impl From<&Value> for User {
  fn from(value: &Value) -> Self {
    let user = value.get("user").unwrap();

    let id = user
      .get("id")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let name = user
      .get("name")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let image = user
      .get("profile")
      .map(|v| v.as_str().unwrap_or_default().to_string())
      .unwrap_or_default();

    let leagues = vec![];

    Self {
      id,
      name,
      image,
      leagues,
    }
  }
}

impl From<Value> for User {
  fn from(value: Value) -> Self {
    Self::from(&value)
  }
}
