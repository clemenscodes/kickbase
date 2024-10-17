use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_current_gift(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self.get("/gifts/current").await?;
    Ok(response)
  }
}
