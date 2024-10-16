use crate::{league::League, HttpClient, HttpClientError};
use reqwest::Method;

#[derive(Debug)]
pub struct User {
  pub name: String,
  pub id: String,
  pub image: String,
  pub leagues: Vec<League>,
}

impl HttpClient {
  pub async fn get_user(&self) -> Result<User, HttpClientError> {
    let response = self.get(Method::GET, "/user/me", None).await?;
    let user = response.value.get("user").unwrap();
    let leagues = self.get_leagues().await?;
    let image = user
      .get("profile")
      .map(|value| value.to_string().replace("\"", ""))
      .unwrap_or_default();

    let user = User {
      id: user.get("id").unwrap().to_string().replace("\"", ""),
      name: user.get("name").unwrap().to_string().replace("\"", ""),
      image,
      leagues,
    };

    Ok(user)
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
