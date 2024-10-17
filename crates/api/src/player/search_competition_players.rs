use crate::{HttpClient, HttpClientError, HttpResponse};
use serde_json::Value;

impl HttpClient {
  pub async fn search_competition_players(
    &self,
    query: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/competition/search?query={query}");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
