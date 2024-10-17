use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_achievements(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self.get("/achievements").await?;
    Ok(response)
  }
}
