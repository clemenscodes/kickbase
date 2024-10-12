use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;

impl HttpClient {
  pub async fn get_user_match_day_feed(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/users/{}/feed", league_id, user_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  #[tokio::test]
  async fn test_get_user_match_day_feed() {
    // Boilerplate test code for get_user_match_day_feed
  }
}
