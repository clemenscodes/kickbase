use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;

impl HttpClient {
  pub async fn get_achievements(
    &self,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = "/achievements".to_string();
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}
