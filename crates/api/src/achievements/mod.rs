use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use serde_json::Value;

impl HttpClient {
  pub async fn get_achievements(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self.get("/achievements").await?;
    Ok(response)
  }
}
