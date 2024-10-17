use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn collect_gift(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self.post("/gifts/collect").await?;
    Ok(response)
  }
}
