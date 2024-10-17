use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde_json::Value;

impl HttpClient {
  pub async fn collect_gift(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = "/gifts/collect".to_string();
    let response = self.get(Method::POST, &url).await?;
    Ok(response)
  }

  pub async fn get_current_gift(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = "/gifts/current".to_string();
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }
}
