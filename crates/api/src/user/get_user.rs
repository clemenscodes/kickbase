use reqwest::Method;

use super::User;
use crate::{HttpClient, HttpClientError};

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
