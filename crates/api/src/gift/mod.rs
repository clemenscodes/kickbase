use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use serde_json::Value;

impl HttpClient {
  pub async fn collect_gift(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self.post("/gifts/collect").await?;
    Ok(response)
  }

  pub async fn get_current_gift(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self.get("/gifts/current").await?;
    Ok(response)
  }
}
