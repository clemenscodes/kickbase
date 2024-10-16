use crate::{league::League, HttpClient, HttpClientError};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug)]
pub struct User {
  pub name: String,
  pub id: String,
  pub image: String,
  pub leagues: Vec<League>,
}

impl From<Value> for User {
  fn from(value: Value) -> Self {
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

impl HttpClient {
  pub async fn get_user(&self) -> Result<User, HttpClientError> {
    let mut response = self.get::<User>(Method::GET, "/user/me").await?;
    let leagues = self.get_leagues().await?;
    response.value.leagues = leagues;
    Ok(response.value)
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::get_test_client;

  #[ignore]
  #[tokio::test]
  async fn test_get_user() {
    let client = get_test_client();
    let result = client.get_user().await.unwrap();
    println!("{result:#?}");
  }
}
