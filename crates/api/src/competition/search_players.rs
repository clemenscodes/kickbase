use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

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
