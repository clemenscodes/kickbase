use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_feed(
    &self,
    league_id: &str,
    start: u64,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/feed?start={start}");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
