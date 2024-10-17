use reqwest::Method;
use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_player_points(
    &self,
    player_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/players/{}/points", player_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }
}
