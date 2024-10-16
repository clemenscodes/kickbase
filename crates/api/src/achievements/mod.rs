use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde_json::Value;

impl HttpClient {
  pub async fn get_achievements(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = "/achievements".to_string();
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }
}
