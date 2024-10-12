use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;

impl HttpClient {
  pub async fn collect_gift(&self) -> Result<HttpResponse, HttpClientError> {
    let url = "/gifts/collect".to_string();
    let response = self.get(Method::POST, &url, None).await?;
    Ok(response)
  }

  pub async fn get_current_gift(
    &self,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = "/gifts/current".to_string();
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}
