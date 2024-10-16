use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;

impl HttpClient {
  pub async fn search_competition_players(
    &self,
    query: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/competition/search?query={}", query);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}
