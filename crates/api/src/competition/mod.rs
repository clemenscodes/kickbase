use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use serde_json::Value;

impl HttpClient {
  pub async fn search_players(
    &self,
    query: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/competition/search?query={query}");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
