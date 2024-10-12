use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;

impl HttpClient {
  pub async fn league_user_info(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/me", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  #[tokio::test]
  async fn test_league_user_info() {
    // Boilerplate test code for league_user_info
  }
}
