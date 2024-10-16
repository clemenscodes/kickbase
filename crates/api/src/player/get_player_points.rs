use reqwest::Method;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_player_points(
    &self,
    player_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/players/{}/points", player_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}
