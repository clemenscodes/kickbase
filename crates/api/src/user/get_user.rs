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
    let user = User {
      id: user.get("id").unwrap().to_string().replace("\"", ""),
      name: user.get("name").unwrap().to_string().replace("\"", ""),
      image: user.get("profile").unwrap().to_string().replace("\"", ""),
      leagues,
    };

    Ok(user)
  }
}

#[cfg(test)]
mod tests {
  use crate::KICKBASE;

  #[tokio::test]
  async fn test_get_user() {
    let response = KICKBASE.read().await.get_user().await;
    println!("{response:#?}");
  }
}
